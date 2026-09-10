//! Three-tier crate cache: memory, parsed sidecar, and compressed source blob.
//!
//! The blob is authoritative; sidecars are disposable and rebuilt as needed.
//! Concurrent memory misses for the same crate are coalesced.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use moka::future::Cache;
use rustdoc_types::Crate;

use crate::error::{Error, Result};
use crate::rustdoc::{build_client, fetch_rustdoc_zst, load_crate, resolve_version, Surface};
use crate::search::FtsDb;

mod entry;
mod expand;
mod store;

pub(crate) use entry::CrateEntry;
use store::{CacheStore, SidecarError, SidecarRead};

/// Crates are large; disk is the real cache, this just keeps the hot set resident.
const MAX_IN_MEMORY: u64 = 32;
/// Latest-version mappings are inexpensive but must refresh: pinning one
/// forever would make the schema default silently stale.
const LATEST_VERSION_CAPACITY: u64 = 256;
const LATEST_VERSION_TTL: Duration = Duration::from_mins(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CacheSource {
    Memory,
    Sidecar,
    Blob,
    Network,
}

#[derive(Debug, Default)]
pub(crate) struct CacheTimings {
    pub(crate) sidecar_read: std::time::Duration,
    pub(crate) blob_read: std::time::Duration,
    pub(crate) network_fetch: std::time::Duration,
    pub(crate) decode: std::time::Duration,
    pub(crate) sidecar_encode: std::time::Duration,
    pub(crate) sidecar_write: std::time::Duration,
    pub(crate) surface_build: std::time::Duration,
    pub(crate) dependency_expand: std::time::Duration,
    pub(crate) fts_open: std::time::Duration,
}

#[derive(Debug)]
pub(crate) struct CacheLoadReport {
    pub(crate) source: CacheSource,
    pub(crate) sidecar_errors: Vec<SidecarError>,
    pub(crate) timings: CacheTimings,
}

impl CacheLoadReport {
    pub(crate) fn memory() -> Self {
        Self {
            source: CacheSource::Memory,
            sidecar_errors: Vec::new(),
            timings: CacheTimings::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LoadedCrate {
    krate: Crate,
    report: CacheLoadReport,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CacheAccessReport {
    pub(crate) source: CacheSource,
    pub(crate) load_duration: std::time::Duration,
}

#[derive(Debug)]
pub(crate) struct CachedEntry {
    pub(crate) entry: Arc<CrateEntry>,
    /// The concrete version this request resolved to, never `"latest"`.
    pub(crate) resolved_version: String,
    pub(crate) report: CacheAccessReport,
}

#[derive(Debug)]
pub(crate) struct CrateCache {
    mem: Cache<String, Arc<CrateEntry>>,
    latest_versions: Cache<String, String>,
    client: reqwest::Client,
    store: CacheStore,
    offline: bool,
}

impl CrateCache {
    /// Create a cache rooted at this platform's cache directory.
    ///
    /// # Errors
    /// [`Error::NoCacheDir`] if no cache directory can be resolved, or
    /// [`Error::Http`] if the HTTP client cannot be built.
    pub(crate) fn new() -> Result<Self> {
        let cache_dir = match std::env::var_os("RUSTDOC_QUERY_CACHE_DIR") {
            Some(path) => PathBuf::from(path),
            None => directories::ProjectDirs::from("foo", "gnu", "rustdoc-query")
                .ok_or(Error::NoCacheDir)?
                .cache_dir()
                .to_path_buf(),
        };
        Ok(Self {
            mem: Cache::new(MAX_IN_MEMORY),
            latest_versions: Cache::builder()
                .max_capacity(LATEST_VERSION_CAPACITY)
                .time_to_live(LATEST_VERSION_TTL)
                .build(),
            client: build_client()?,
            store: CacheStore::new(cache_dir),
            offline: matches!(
                std::env::var("RUSTDOC_QUERY_OFFLINE").as_deref(),
                Ok("1" | "true" | "TRUE")
            ),
        })
    }

    /// Get a crate's parsed entry, loading and caching it if necessary; reports which cache tier served it.
    pub(crate) async fn get_with_report(&self, krate: &str, version: &str) -> Result<CachedEntry> {
        let key = format!("{krate}@{version}");
        let started = std::time::Instant::now();
        if let Some(entry) = self.mem.get(&key).await {
            return Ok(CachedEntry {
                entry,
                resolved_version: version.to_owned(),
                report: CacheAccessReport {
                    source: CacheSource::Memory,
                    load_duration: started.elapsed(),
                },
            });
        }
        let entry = self
            .mem
            .try_get_with(key, self.load_entry(krate, version))
            .await
            .map_err(Error::from)?;
        Ok(CachedEntry {
            resolved_version: version.to_owned(),
            report: CacheAccessReport {
                source: entry.load_report.source,
                load_duration: started.elapsed(),
            },
            entry,
        })
    }

    /// Resolve `version` and load a crate with per-request cache observability.
    pub(crate) async fn get_resolved_with_report(
        &self,
        krate: &str,
        version: &str,
    ) -> Result<CachedEntry> {
        let concrete = self.resolve_requested_version(krate, version).await?;
        self.get_with_report(krate, &concrete).await
    }

    async fn resolve_requested_version(&self, krate: &str, requested: &str) -> Result<String> {
        if requested != "latest" {
            return Ok(requested.to_owned());
        }
        if self.offline {
            return Err(Error::OfflineCacheMiss {
                krate: krate.to_owned(),
                version: requested.to_owned(),
            });
        }
        if let Some(version) = self.latest_versions.get(krate).await {
            return Ok(version);
        }
        let version = resolve_version(&self.client, krate, requested).await?;
        self.latest_versions
            .insert(krate.to_owned(), version.clone())
            .await;
        Ok(version)
    }

    #[cfg(test)]
    pub(crate) async fn insert_for_test(&self, krate: &str, version: &str, entry: CrateEntry) {
        self.mem
            .insert(format!("{krate}@{version}"), Arc::new(entry))
            .await;
    }

    /// Builds the FTS sidecar only after the surface is fully expanded.
    async fn load_entry(&self, krate: &str, version: &str) -> Result<Arc<CrateEntry>> {
        let mut loaded = self.load_crate_cached(krate, version).await?;
        let parsed = loaded.krate;

        let started = std::time::Instant::now();
        let (mut surface, pending, pending_named) = Surface::build(&parsed);
        loaded.report.timings.surface_build = started.elapsed();

        let started = std::time::Instant::now();
        let dep_crates =
            expand::apply_pending_expansions(self, &mut surface, &pending, &pending_named).await;
        loaded.report.timings.dependency_expand = started.elapsed();

        surface.add_resolvable_inherent_methods(&parsed, &dep_crates);

        let fts_path = self.store.fts_path(krate, version);
        let started = std::time::Instant::now();
        let fts = FtsDb::open_or_build(&fts_path, &parsed, &surface, &dep_crates).await?;
        loaded.report.timings.fts_open = started.elapsed();

        let mut entry = CrateEntry::new(parsed, surface, dep_crates, fts);
        entry.load_report = loaded.report;
        tracing::debug!(
            crate = krate,
            version,
            source = ?entry.load_report.source,
            timings = ?entry.load_report.timings,
            sidecar_errors = ?entry.load_report.sidecar_errors,
            "cache entry prepared"
        );
        Ok(Arc::new(entry))
    }

    /// Dependency crates get the same disk-persistence treatment as the main crate.
    pub(crate) async fn load_crate_cached(
        &self,
        krate: &str,
        version: &str,
    ) -> Result<LoadedCrate> {
        let blob_path = self.store.blob_path(krate, version);
        let sidecar_path = self.store.sidecar_path(krate, version);
        let mut report = CacheLoadReport {
            source: CacheSource::Blob,
            sidecar_errors: Vec::new(),
            timings: CacheTimings::default(),
        };

        let started = std::time::Instant::now();
        let sidecar = self.store.read_sidecar(&sidecar_path).await;
        report.timings.sidecar_read = started.elapsed();
        match sidecar {
            SidecarRead::Hit(krate) => {
                report.source = CacheSource::Sidecar;
                return Ok(LoadedCrate { krate, report });
            }
            SidecarRead::Miss => {}
            SidecarRead::Rejected(error) => {
                tracing::warn!(
                    crate = krate,
                    version,
                    error = ?error,
                    "cache sidecar rejected; falling back to blob or network"
                );
                report.sidecar_errors.push(error);
            }
        }

        let started = std::time::Instant::now();
        let zst = match tokio::fs::read(&blob_path).await {
            Ok(bytes) => {
                report.timings.blob_read = started.elapsed();
                bytes
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                report.timings.blob_read = started.elapsed();
                if self.offline {
                    return Err(Error::OfflineCacheMiss {
                        krate: krate.to_owned(),
                        version: version.to_owned(),
                    });
                }
                report.source = CacheSource::Network;
                let started = std::time::Instant::now();
                let bytes = fetch_rustdoc_zst(&self.client, krate, version).await?;
                report.timings.network_fetch = started.elapsed();
                self.store.write_atomic(&blob_path, &bytes).await?;
                bytes
            }
            Err(error) => return Err(error.into()),
        };

        let started = std::time::Instant::now();
        let parsed = load_crate(&zst)?;
        report.timings.decode = started.elapsed();
        // Sidecar refresh failures are logged, never propagated; the `.zst` blob remains authoritative.
        let started = std::time::Instant::now();
        match CacheStore::encode_sidecar(&parsed) {
            Ok(encoded) => {
                report.timings.sidecar_encode = started.elapsed();
                let started = std::time::Instant::now();
                if let Err(source) = self.store.write_atomic(&sidecar_path, &encoded).await {
                    let error = SidecarError::Write {
                        path: sidecar_path,
                        source,
                    };
                    tracing::warn!(crate = krate, version, error = ?error, "cache sidecar refresh failed");
                    report.sidecar_errors.push(error);
                }
                report.timings.sidecar_write = started.elapsed();
            }
            Err(source) => {
                report.timings.sidecar_encode = started.elapsed();
                let error = SidecarError::Encode { source };
                tracing::warn!(
                    crate = krate,
                    version,
                    error = ?error,
                    failure_path = %postcard_failure_path(&parsed),
                    "cache sidecar refresh failed"
                );
                report.sidecar_errors.push(error);
            }
        }
        Ok(LoadedCrate {
            krate: parsed,
            report,
        })
    }
}

/// Runs only on the exceptional sidecar-refresh failure path — the linear scan cost is acceptable because it's rare.
fn postcard_failure_path(krate: &Crate) -> String {
    for (id, item) in &krate.index {
        if postcard::to_stdvec(item).is_err() {
            let name = item.name.as_deref().unwrap_or("<unnamed>");
            let prefix = format!("index[{id:?}] ({name})");
            if postcard::to_stdvec(&item.stability).is_err() {
                return format!("{prefix}.stability");
            }
            if postcard::to_stdvec(&item.const_stability).is_err() {
                return format!("{prefix}.const_stability");
            }
            if postcard::to_stdvec(&item.inner).is_err() {
                return format!("{prefix}.inner");
            }
            return prefix;
        }
    }
    "crate-level structure".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cache() -> CrateCache {
        CrateCache::new().expect("cache dir resolvable in test env")
    }

    #[test]
    fn synthetic_entries_default_to_a_memory_load_report() {
        let report = CacheLoadReport::memory();
        assert_eq!(report.source, CacheSource::Memory);
        assert!(report.sidecar_errors.is_empty());
        assert_eq!(report.timings.decode, std::time::Duration::ZERO);
    }

    #[tokio::test]
    async fn cached_latest_resolution_bypasses_network_lookup() {
        let cache = test_cache();
        cache
            .latest_versions
            .insert("demo".to_owned(), "1.2.3".to_owned())
            .await;

        assert_eq!(
            cache
                .resolve_requested_version("demo", "latest")
                .await
                .unwrap(),
            "1.2.3"
        );
    }

    #[test]
    fn paths_are_namespaced_and_versioned() {
        let c = test_cache();
        let blob = c.store.blob_path("serde", "1.0.0");
        let side = c.store.sidecar_path("serde", "1.0.0");

        assert!(blob.to_string_lossy().ends_with("serde-1.0.0.json.zst"));
        assert!(side
            .to_string_lossy()
            .ends_with(&format!("cv2.fv{}.postcard", rustdoc_types::FORMAT_VERSION)));
        assert_eq!(blob.parent(), Some(c.store.dir()));
    }

    #[tokio::test]
    #[ignore = "network + filesystem"]
    async fn cold_then_warm_load() {
        let cache = test_cache();
        let first = cache.get_with_report("anyhow", "1.0.102").await.unwrap();
        assert!(!first.entry.surface.is_empty());
        assert!(first.entry.surface.get_by_path("anyhow::Error").is_some());
        assert_ne!(first.report.source, CacheSource::Memory);

        let second = cache.get_with_report("anyhow", "1.0.102").await.unwrap();
        assert!(
            Arc::ptr_eq(&first.entry, &second.entry),
            "warm load should hit memory"
        );
        assert_eq!(second.report.source, CacheSource::Memory);
    }

    #[tokio::test]
    #[ignore = "network + filesystem"]
    async fn reqwest_expansion_includes_reexports() {
        let cache = test_cache();
        let entry = cache
            .get_with_report("reqwest", "0.12.15")
            .await
            .unwrap()
            .entry;
        let s = &entry.surface;

        assert!(
            s.get_by_path("reqwest::Method").is_some(),
            "reqwest::Method should be in surface"
        );
        assert!(
            s.get_by_path("reqwest::StatusCode").is_some(),
            "reqwest::StatusCode should be in surface"
        );

        assert!(
            s.get_by_path("reqwest::header").is_some(),
            "reqwest::header module should be in surface"
        );
        assert!(
            s.iter_by_kind(rustdoc_types::ItemKind::Struct).any(|e| e
                .path
                .starts_with(&["reqwest".to_owned(), "header".to_owned()])),
            "at least one struct should be stitched under reqwest::header"
        );
    }
}
