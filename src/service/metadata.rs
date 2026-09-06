//! Response metadata for version comparison and cache observability.

use crate::cache::{CacheAccessReport, CacheSource};
use crate::contract::generated::{
    CacheMetadata, CacheMetadataStatus, CacheSource as ContractCacheSource, VersionComparison,
    VersionComparisonStatus,
};

/// Dependency versions agree only on exact equality; semver matching is not used.
pub(crate) fn version_comparison(
    requested: &str,
    resolved: &str,
    dependency: Option<&str>,
) -> VersionComparison {
    let (status, message) = match dependency {
        None => (
            VersionComparisonStatus::NotCompared,
            format!("no dependency version was supplied to compare against `{resolved}`"),
        ),
        Some(dependency) if dependency == resolved => (
            VersionComparisonStatus::Matched,
            format!("dependency version `{dependency}` matches the documented version"),
        ),
        Some(dependency) => (
            VersionComparisonStatus::Mismatch,
            format!(
                "dependency version `{dependency}` differs from the documented version \
                 `{resolved}`"
            ),
        ),
    };

    VersionComparison {
        requested_version: requested.to_owned(),
        resolved_version: resolved.to_owned(),
        dependency_version: dependency.map(ToOwned::to_owned),
        status,
        message,
    }
}

pub(crate) fn cache_metadata(report: CacheAccessReport) -> CacheMetadata {
    let (source, status) = match report.source {
        CacheSource::Memory => (ContractCacheSource::Memory, CacheMetadataStatus::Hit),
        CacheSource::Sidecar => (ContractCacheSource::Sidecar, CacheMetadataStatus::Hit),
        CacheSource::Blob => (ContractCacheSource::Blob, CacheMetadataStatus::Hit),
        CacheSource::Network => (ContractCacheSource::Network, CacheMetadataStatus::Miss),
    };

    CacheMetadata {
        source,
        status,
        load_duration_ms: u64::try_from(report.load_duration.as_millis()).unwrap_or(u64::MAX),
        // Cache age is diagnostic only; missing provenance does not imply staleness.
        stale: false,
        age_ms: None,
    }
}

#[cfg(test)]
#[path = "metadata/tests.rs"]
mod tests;
