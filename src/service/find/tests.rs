use rustdoc_types::ItemKind;

use super::{summaries, validate};
use crate::contract::generated::{ApiError, FindRequest, RustdocItemKind};

fn request(value: serde_json::Value) -> FindRequest {
    serde_json::from_value(value).expect("request is valid")
}

#[test]
fn a_request_with_both_queries_is_rejected() {
    let both = request(serde_json::json!({
        "crate_name": "reqwest", "query": "Client", "doc_query": "send a request"
    }));
    assert!(matches!(
        validate(&both),
        Err(ApiError::InvalidRequestError(_))
    ));
}

#[test]
fn cursor_and_page_size_are_presence_sensitive_contract_inputs() {
    let page = request(serde_json::json!({
        "crate_name": "reqwest", "cursor": "eyJ2IjoxfQ", "page_size": 12
    }));
    assert_eq!(page.page_size.map(|size| size.get()), Some(12));
    assert!(page.cursor.is_some());
}

#[test]
fn summary_and_exact_kind_contract_remain_stable() {
    assert_eq!(
        summaries(&request(serde_json::json!({ "crate_name": "reqwest" }))),
        crate::contract::defaults::INCLUDE_SUMMARY
    );
    assert_eq!(
        crate::service::kind::from_contract(RustdocItemKind::Union),
        ItemKind::Union
    );
    assert_eq!(
        crate::service::kind::to_contract(ItemKind::ProcAttribute),
        RustdocItemKind::ProcAttribute
    );
}

#[tokio::test]
async fn public_service_returns_a_generated_complete_or_continuation_page() {
    let service = crate::service::test_support::service().await;
    let outcome = service
        .find(request(serde_json::json!({
            "crate_name": "krate", "version": "1.0.0", "query": "Widget", "page_size": 1
        })))
        .await;
    let crate::contract::generated::FindOutcome::FindSuccess(success) = outcome else {
        panic!("deterministic fixture should succeed");
    };
    match success.result {
        crate::contract::generated::FindResult::CompletePage {
            has_more,
            resolved_version,
            ..
        } => {
            assert!(!has_more);
            assert_eq!(resolved_version, "1.0.0");
        }
        crate::contract::generated::FindResult::ContinuationPage {
            has_more,
            next_cursor,
            resolved_version,
            ..
        } => {
            assert!(has_more);
            assert!(!next_cursor.is_empty());
            assert_eq!(resolved_version, "1.0.0");
        }
    }
}

#[tokio::test]
async fn public_service_reports_reexport_provenance_in_the_existing_hint() {
    let service = crate::service::test_support::service().await;
    let outcome = service
        .find(request(serde_json::json!({
            "crate_name": "krate", "version": "1.0.0", "query": "MissingThing"
        })))
        .await;
    let crate::contract::generated::FindOutcome::FindSuccess(success) = outcome else {
        panic!("an unavailable re-export is a partial successful result");
    };
    let hint = match success.result {
        crate::contract::generated::FindResult::CompletePage { rows, hint, .. }
        | crate::contract::generated::FindResult::ContinuationPage { rows, hint, .. } => {
            assert!(rows.is_empty());
            hint.expect("zero result exposes a generated contract hint")
        }
    };
    assert!(hint.contains("krate::MissingThing"), "hint: {hint}");
    assert!(hint.contains("missing_dep"), "hint: {hint}");
    assert!(hint.contains("metadata"), "hint: {hint}");
}
