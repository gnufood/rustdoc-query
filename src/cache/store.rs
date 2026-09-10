use std::path::{Path, PathBuf};

use rustdoc_types::Crate;

const SIDECAR_VERSION: u8 = 2;

#[derive(serde::Serialize, serde::Deserialize)]
struct SidecarEnvelope {
    version: u8,
    rustdoc_format: u32,
    krate: Crate,
}

/// Non-fatal failures while reading or refreshing a derived postcard sidecar.
#[derive(Debug, thiserror::Error)]
pub(crate) enum SidecarError {
    #[error("failed to read sidecar `{path}`")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to decode sidecar `{path}`")]
    Decode {
        path: PathBuf,
        #[source]
        source: postcard::Error,
    },

    #[error("sidecar `{path}` has rustdoc format {found}; expected {expected}")]
    FormatMismatch {
        path: PathBuf,
        found: u32,
        expected: u32,
    },

    #[error("sidecar `{path}` has application format {found}; expected {expected}")]
    VersionMismatch {
        path: PathBuf,
        found: u8,
        expected: u8,
    },

    #[error("failed to encode rustdoc crate as a postcard sidecar")]
    Encode {
        #[source]
        source: postcard::Error,
    },

    #[error("failed to persist sidecar `{path}`")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

/// Result of a sidecar probe. Rejections are diagnostics, not request failures.
#[derive(Debug)]
pub(super) enum SidecarRead {
    Hit(Crate),
    Miss,
    Rejected(SidecarError),
}

#[derive(Debug)]
pub(super) struct CacheStore {
    dir: PathBuf,
}

impl CacheStore {
    pub(super) fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub(super) fn blob_path(&self, krate: &str, version: &str) -> PathBuf {
        self.dir.join(format!("{krate}-{version}.json.zst"))
    }

    pub(super) fn fts_path(&self, krate: &str, version: &str) -> PathBuf {
        self.dir.join(format!("{krate}-{version}.fts2.db"))
    }

    pub(super) fn sidecar_path(&self, krate: &str, version: &str) -> PathBuf {
        self.dir.join(format!(
            "{krate}-{version}.cv{SIDECAR_VERSION}.fv{}.postcard",
            rustdoc_types::FORMAT_VERSION
        ))
    }

    #[cfg(test)]
    pub(super) fn dir(&self) -> &Path {
        &self.dir
    }

    pub(super) async fn read_sidecar(&self, path: &Path) -> SidecarRead {
        let bytes = match tokio::fs::read(path).await {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return SidecarRead::Miss;
            }
            Err(source) => {
                return SidecarRead::Rejected(SidecarError::Read {
                    path: path.to_path_buf(),
                    source,
                });
            }
        };
        match postcard::from_bytes::<SidecarEnvelope>(&bytes) {
            Ok(envelope) if envelope.version != SIDECAR_VERSION => {
                SidecarRead::Rejected(SidecarError::VersionMismatch {
                    path: path.to_path_buf(),
                    found: envelope.version,
                    expected: SIDECAR_VERSION,
                })
            }
            Ok(envelope) if envelope.rustdoc_format != rustdoc_types::FORMAT_VERSION => {
                SidecarRead::Rejected(SidecarError::FormatMismatch {
                    path: path.to_path_buf(),
                    found: envelope.rustdoc_format,
                    expected: rustdoc_types::FORMAT_VERSION,
                })
            }
            Ok(envelope) => SidecarRead::Hit(envelope.krate),
            Err(source) => SidecarRead::Rejected(SidecarError::Decode {
                path: path.to_path_buf(),
                source,
            }),
        }
    }

    pub(super) fn encode_sidecar(krate: &Crate) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_stdvec(&SidecarEnvelope {
            version: SIDECAR_VERSION,
            rustdoc_format: krate.format_version,
            krate: krate.clone(),
        })
    }

    pub(super) async fn write_atomic(&self, path: &Path, bytes: &[u8]) -> std::io::Result<()> {
        tokio::fs::create_dir_all(&self.dir).await?;
        let temporary = path.with_extension("tmp");
        tokio::fs::write(&temporary, bytes).await?;
        tokio::fs::rename(temporary, path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use rustdoc_types::{Crate, Id, Target};

    use super::*;

    static NEXT_TEMP_DIR: AtomicUsize = AtomicUsize::new(0);

    fn test_store() -> CacheStore {
        let sequence = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "rustdoc-query-cache-store-{}-{sequence}",
            std::process::id()
        ));
        CacheStore::new(dir)
    }

    fn fixture_krate() -> Crate {
        Crate {
            root: Id(0),
            crate_version: Some("1.0.0".to_owned()),
            includes_private: false,
            index: HashMap::new(),
            paths: HashMap::new(),
            external_crates: HashMap::new(),
            target: Target {
                triple: String::new(),
                target_features: Vec::new(),
            },
            format_version: rustdoc_types::FORMAT_VERSION,
        }
    }

    #[tokio::test]
    async fn atomic_write_replaces_content_without_leaving_a_temporary_file() {
        let store = test_store();
        let path = store.blob_path("fixture", "1.0.0");

        store.write_atomic(&path, b"first").await.unwrap();
        store.write_atomic(&path, b"second").await.unwrap();

        assert_eq!(tokio::fs::read(&path).await.unwrap(), b"second");
        assert!(!path.with_extension("tmp").exists());
        tokio::fs::remove_dir_all(store.dir()).await.unwrap();
    }

    #[tokio::test]
    async fn missing_sidecar_is_a_miss() {
        let store = test_store();
        let path = store.sidecar_path("fixture", "1.0.0");

        assert!(matches!(store.read_sidecar(&path).await, SidecarRead::Miss));
    }

    #[tokio::test]
    async fn corrupt_sidecar_reports_a_decode_rejection() {
        let store = test_store();
        let path = store.sidecar_path("fixture", "1.0.0");

        store.write_atomic(&path, b"not a postcard").await.unwrap();

        assert!(matches!(
            store.read_sidecar(&path).await,
            SidecarRead::Rejected(SidecarError::Decode { .. })
        ));
        tokio::fs::remove_dir_all(store.dir()).await.unwrap();
    }

    #[tokio::test]
    async fn v1_sidecar_is_rejected_and_v2_uses_a_distinct_path() {
        let store = test_store();
        let path = store.sidecar_path("fixture", "1.0.0");
        let bytes = postcard::to_stdvec(&SidecarEnvelope {
            version: 1,
            rustdoc_format: rustdoc_types::FORMAT_VERSION,
            krate: fixture_krate(),
        })
        .unwrap();

        store.write_atomic(&path, &bytes).await.unwrap();

        assert!(path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains(".cv2.fv"));
        assert!(matches!(
            store.read_sidecar(&path).await,
            SidecarRead::Rejected(SidecarError::VersionMismatch {
                found: 1,
                expected: 2,
                ..
            })
        ));
        tokio::fs::remove_dir_all(store.dir()).await.unwrap();
    }
}
