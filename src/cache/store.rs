use std::path::{Path, PathBuf};

use rustdoc_types::Crate;

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
            "{krate}-{version}.fv{}.postcard",
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
        match postcard::from_bytes::<Crate>(&bytes) {
            Ok(parsed) if parsed.format_version == rustdoc_types::FORMAT_VERSION => {
                SidecarRead::Hit(parsed)
            }
            Ok(parsed) => SidecarRead::Rejected(SidecarError::FormatMismatch {
                path: path.to_path_buf(),
                found: parsed.format_version,
                expected: rustdoc_types::FORMAT_VERSION,
            }),
            Err(source) => SidecarRead::Rejected(SidecarError::Decode {
                path: path.to_path_buf(),
                source,
            }),
        }
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
    use std::sync::atomic::{AtomicUsize, Ordering};

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
}
