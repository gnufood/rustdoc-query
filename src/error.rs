//! Crate-wide error type for the fetch → load → cache → serve pipeline.
//!
//! Every fallible boundary in the crate returns [`Result`].

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    /// docs.rs has no rustdoc JSON for this crate/version (HTTP 404).
    ///
    /// Distinct from [`Error::Http`] so callers can report a clean
    /// "unknown crate/version" instead of a transport error.
    #[error("no rustdoc JSON for `{krate}` {version} on docs.rs")]
    NotFound {
        krate: String,
        /// The version (concrete or `"latest"`) that was requested.
        version: String,
    },

    #[error("offline cache miss for `{krate}` {version}")]
    OfflineCacheMiss { krate: String, version: String },

    /// The rustdoc JSON `format_version` is outside the supported range (50–60).
    #[error("rustdoc format_version {found} not supported (supported range: 50–60)")]
    UnsupportedFormat { found: u32 },

    #[error("docs.rs request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Filesystem or zstd-stream I/O failure.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// rustdoc JSON could not be deserialized into `rustdoc_types::Crate`.
    #[error("failed to parse rustdoc JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("cache sidecar codec error: {0}")]
    Sidecar(#[from] postcard::Error),

    /// `SQLite` FTS sidecar build or query failure.
    #[error("fts error: {0}")]
    Fts(String),

    /// No platform cache directory could be resolved (`directories` returned
    /// `None`, e.g. no `$HOME`).
    #[error("could not resolve a cache directory for this platform")]
    NoCacheDir,

    /// A coalesced cache load failed.
    ///
    /// The underlying error is available via [`std::error::Error::source`].
    #[error(transparent)]
    Shared(std::sync::Arc<Error>),
}

impl From<std::sync::Arc<Error>> for Error {
    /// Recover matchable error variants from a shared cache error.
    ///
    /// Other errors remain wrapped in [`Error::Shared`].
    fn from(shared: std::sync::Arc<Error>) -> Self {
        match &*shared {
            Error::NotFound { krate, version } => Error::NotFound {
                krate: krate.clone(),
                version: version.clone(),
            },
            Error::UnsupportedFormat { found } => Error::UnsupportedFormat { found: *found },
            _ => Error::Shared(shared),
        }
    }
}
