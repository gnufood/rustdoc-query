//! Fetches compressed rustdoc JSON from docs.rs without decompression.
//! Decompression occurs when the cached blob is loaded.

use crate::error::{Error, Result};

const DOCS_RS: &str = "https://docs.rs";

const CRATES_IO: &str = "https://crates.io";

/// Reused across fetches for connection pooling; identifies itself with a
/// descriptive user agent.
pub(crate) fn build_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;
    Ok(client)
}

/// `version` must be a concrete semver — callers resolve `"latest"` beforehand
/// via [`resolve_version`]. Returns the compressed bytes exactly as docs.rs
/// served them.
///
/// A `404` is mapped to [`Error::NotFound`] (crate/version has no rustdoc JSON);
/// any other non-success status becomes [`Error::Http`].
pub(crate) async fn fetch_rustdoc_zst(
    client: &reqwest::Client,
    krate: &str,
    version: &str,
) -> Result<Vec<u8>> {
    let url = format!("{DOCS_RS}/crate/{krate}/{version}/json");
    let response = client.get(&url).send().await?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::NotFound {
            krate: krate.to_owned(),
            version: version.to_owned(),
        });
    }

    let bytes = response.error_for_status()?.bytes().await?;
    Ok(bytes.to_vec())
}

/// Resolves `"latest"` to the highest stable release, falling back to the
/// highest available release.
///
/// # Errors
/// Returns an error when resolution or decoding fails.
pub(crate) async fn resolve_version(
    client: &reqwest::Client,
    krate: &str,
    requested: &str,
) -> Result<String> {
    if requested != "latest" {
        return Ok(requested.to_owned());
    }

    let url = format!("{CRATES_IO}/api/v1/crates/{krate}");
    let response = client.get(&url).send().await?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::NotFound {
            krate: krate.to_owned(),
            version: requested.to_owned(),
        });
    }

    // reqwest is built without its `json` feature, so decode the body ourselves.
    let bytes = response.error_for_status()?.bytes().await?;
    let parsed: CratesIoResponse = serde_json::from_slice(&bytes)?;
    Ok(parsed
        .krate
        .max_stable_version
        .unwrap_or(parsed.krate.max_version))
}

/// Minimal shape of the crates.io `GET /api/v1/crates/{crate}` response — only
/// the version fields we need to resolve `"latest"`.
#[derive(serde::Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    krate: CratesIoCrate,
}

#[derive(serde::Deserialize)]
struct CratesIoCrate {
    /// Highest non-prerelease version; absent if the crate has only prereleases.
    max_stable_version: Option<String>,
    /// Highest version overall, used as a fallback.
    max_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms we receive a real zstd stream (magic bytes `28 B5 2F FD`)
    /// rather than auto-decompressed JSON.
    #[tokio::test]
    #[ignore = "hits the network"]
    async fn fetches_zstd_blob() {
        let client = build_client().unwrap();
        let bytes = fetch_rustdoc_zst(&client, "rustdoc-types", "0.57.3")
            .await
            .unwrap();
        assert_eq!(
            &bytes[..4],
            &[0x28, 0xB5, 0x2F, 0xFD],
            "expected raw zstd magic"
        );
    }

    #[tokio::test]
    #[ignore = "hits the network"]
    async fn missing_crate_is_not_found() {
        let client = build_client().unwrap();
        let err = fetch_rustdoc_zst(&client, "definitely-not-a-real-crate-xyzzy", "1.0.0")
            .await
            .unwrap_err();
        assert!(matches!(err, Error::NotFound { .. }));
    }

    #[tokio::test]
    async fn resolve_version_passes_concrete_through() {
        let client = build_client().unwrap();
        let v = resolve_version(&client, "serde", "1.0.0").await.unwrap();
        assert_eq!(v, "1.0.0");
    }

    #[tokio::test]
    #[ignore = "hits the network"]
    async fn resolve_version_latest_is_concrete() {
        let client = build_client().unwrap();
        let v = resolve_version(&client, "serde", "latest").await.unwrap();
        assert_ne!(v, "latest");
        assert!(v.starts_with("1."), "serde latest should be 1.x, got {v}");
    }
}
