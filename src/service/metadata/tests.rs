use std::time::Duration;

use super::{cache_metadata, version_comparison};
use crate::cache::{CacheAccessReport, CacheSource};
use crate::contract::generated::{
    CacheMetadataStatus, CacheSource as ContractCacheSource, VersionComparisonStatus,
};

#[test]
fn absent_dependency_version_is_not_compared() {
    let requested = crate::contract::defaults::REQUESTED_VERSION;
    let comparison = version_comparison(requested, "0.13.4", None);

    assert_eq!(comparison.status, VersionComparisonStatus::NotCompared);
    assert_eq!(comparison.requested_version, requested);
    assert_eq!(comparison.resolved_version, "0.13.4");
    assert!(comparison.dependency_version.is_none());
    assert!(!comparison.message.is_empty());
}

#[test]
fn equal_dependency_version_matches() {
    let comparison = version_comparison(
        crate::contract::defaults::REQUESTED_VERSION,
        "0.13.4",
        Some("0.13.4"),
    );

    assert_eq!(comparison.status, VersionComparisonStatus::Matched);
    assert_eq!(comparison.dependency_version.as_deref(), Some("0.13.4"));
}

#[test]
fn differing_dependency_version_mismatches() {
    let comparison = version_comparison("0.13.4", "0.13.4", Some("0.12.0"));

    assert_eq!(comparison.status, VersionComparisonStatus::Mismatch);
    assert_eq!(comparison.dependency_version.as_deref(), Some("0.12.0"));
    assert!(comparison.message.contains("0.12.0"));
    assert!(comparison.message.contains("0.13.4"));
}

#[test]
fn immutable_durable_tiers_report_a_fresh_hit_without_age_diagnostics() {
    for (source, expected) in [
        (CacheSource::Memory, ContractCacheSource::Memory),
        (CacheSource::Sidecar, ContractCacheSource::Sidecar),
        (CacheSource::Blob, ContractCacheSource::Blob),
    ] {
        let metadata = cache_metadata(CacheAccessReport {
            source,
            load_duration: Duration::from_millis(7),
        });

        assert_eq!(metadata.source, expected);
        assert_eq!(metadata.status, CacheMetadataStatus::Hit);
        assert_eq!(metadata.load_duration_ms, 7);
        assert!(!metadata.stale);
        assert!(metadata.age_ms.is_none());
    }
}

#[test]
fn a_network_load_reports_a_miss() {
    let metadata = cache_metadata(CacheAccessReport {
        source: CacheSource::Network,
        load_duration: Duration::from_millis(412),
    });

    assert_eq!(metadata.source, ContractCacheSource::Network);
    assert_eq!(metadata.status, CacheMetadataStatus::Miss);
    assert_eq!(metadata.load_duration_ms, 412);
    assert!(!metadata.stale);
    assert!(metadata.age_ms.is_none());
}
