use std::sync::Arc;

use super::to_api_error;
use crate::contract::generated::{ApiError, GetItemOutcome, GetItemRequest};
use crate::error::Error;

#[test]
fn a_missing_crate_becomes_crate_not_found() {
    let mapped = to_api_error(&Error::NotFound {
        krate: "definitely-not-a-crate".to_owned(),
        version: "1.2.3".to_owned(),
    });

    let ApiError::CrateNotFoundError(error) = mapped else {
        panic!("expected crate_not_found, got {mapped:?}");
    };
    assert_eq!(error.crate_name, "definitely-not-a-crate");
    assert_eq!(error.resolved_version, "1.2.3");
    assert!(!error.message.is_empty());
}

#[test]
fn an_unknown_format_reports_the_version_found() {
    let mapped = to_api_error(&Error::UnsupportedFormat { found: 99 });

    let ApiError::UnsupportedRustdocFormatError(error) = mapped else {
        panic!("expected unsupported_rustdoc_format, got {mapped:?}");
    };
    assert_eq!(error.found, 99);
}

#[test]
fn infrastructure_failures_become_operation_failed() {
    let failures = [
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "cache directory is not writable",
        )),
        Error::Fts("sqlite index build failed".to_owned()),
        Error::NoCacheDir,
    ];

    for failure in &failures {
        let mapped = to_api_error(failure);
        let ApiError::OperationFailedError(error) = mapped else {
            panic!("expected operation_failed for {failure:?}");
        };
        assert!(!error.retryable);
        assert!(!error.message.is_empty());
    }
}

#[test]
fn a_coalesced_failure_maps_to_the_error_it_wraps() {
    let mapped = to_api_error(&Error::Shared(Arc::new(Error::NotFound {
        krate: "serde".to_owned(),
        version: "1.0.0".to_owned(),
    })));

    let ApiError::CrateNotFoundError(error) = mapped else {
        panic!("a shared load failure must not hide its cause: {mapped:?}");
    };
    assert_eq!(error.crate_name, "serde");
}

#[tokio::test]
async fn public_service_returns_a_typed_missing_item_outcome_without_network_or_disk_state() {
    let service = crate::service::test_support::service().await;
    let request: GetItemRequest = serde_json::from_value(serde_json::json!({
        "crate_name": "krate",
        "version": "1.0.0",
        "path": "krate::Missing"
    }))
    .expect("request is valid");

    let GetItemOutcome::ErrorOutcome(outcome) = service.get_item(request).await else {
        panic!("a missing deterministic item must fail");
    };
    let ApiError::ItemNotFoundError(error) = outcome.error else {
        panic!("the public failure must retain item_not_found");
    };
    assert_eq!(error.crate_name, "krate");
    assert_eq!(error.resolved_version, "1.0.0");
    assert_eq!(error.path, "krate::Missing");
}
