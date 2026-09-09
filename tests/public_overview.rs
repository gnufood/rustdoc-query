//! Direct-library overview contract test.

use rustdoc_query::{
    contract::generated::{CacheMetadataStatus, CacheSource, OverviewOutcome, OverviewRequest},
    service::RustdocQueryService,
};

#[tokio::test]
#[ignore = "network + durable cache"]
async fn overview_returns_a_generated_success_outcome() {
    let service = RustdocQueryService::new().expect("service initializes");
    let request: OverviewRequest = serde_json::from_value(serde_json::json!({
        "crate_name": "reqwest",
        "version": "0.13.4"
    }))
    .expect("request is valid");

    let OverviewOutcome::OverviewSuccess(success) = service.overview(request).await else {
        panic!("pinned overview should succeed");
    };
    assert_eq!(success.result.resolved_version, "0.13.4");
    assert!(success.metadata.cache.is_some());
}

#[tokio::test]
#[ignore = "network + durable cache"]
async fn a_repeated_overview_is_served_from_memory() {
    let service = RustdocQueryService::new().expect("service initializes");
    let request = || {
        serde_json::from_value::<OverviewRequest>(serde_json::json!({
            "crate_name": "reqwest",
            "version": "0.13.4"
        }))
        .expect("request is valid")
    };

    let _ = service.overview(request()).await;
    let OverviewOutcome::OverviewSuccess(success) = service.overview(request()).await else {
        panic!("pinned overview should succeed");
    };

    let cache = success.metadata.cache.expect("cache metadata is reported");
    assert_eq!(cache.source, CacheSource::Memory);
    assert_eq!(cache.status, CacheMetadataStatus::Hit);
}

#[tokio::test]
#[ignore = "network + durable cache"]
async fn an_unknown_module_is_a_typed_error_outcome() {
    let service = RustdocQueryService::new().expect("service initializes");
    let request: OverviewRequest = serde_json::from_value(serde_json::json!({
        "crate_name": "reqwest",
        "version": "0.13.4",
        "module": "reqwest::definitely_not_a_module"
    }))
    .expect("request is valid");

    let OverviewOutcome::ErrorOutcome(outcome) = service.overview(request).await else {
        panic!("an absent module must not succeed");
    };
    let rustdoc_query::contract::generated::ApiError::ModuleNotFoundError(error) = outcome.error
    else {
        panic!("expected a module_not_found outcome");
    };
    assert_eq!(error.module, "reqwest::definitely_not_a_module");
    assert_eq!(error.resolved_version, "0.13.4");
}
