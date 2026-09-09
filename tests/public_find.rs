//! Direct-library find contract tests.

use rustdoc_query::{
    contract::generated::{ApiError, FindOutcome, FindRequest, FindResult, RustdocItemKind},
    service::RustdocQueryService,
};

fn request(value: serde_json::Value) -> FindRequest {
    serde_json::from_value(value).expect("request is valid")
}

async fn find(value: serde_json::Value) -> FindOutcome {
    RustdocQueryService::new()
        .expect("service initializes")
        .find(request(value))
        .await
}

#[tokio::test]
#[ignore = "network + durable cache"]
async fn a_name_search_returns_a_generated_page_of_summarized_matches() {
    let outcome = find(serde_json::json!({
        "crate_name": "reqwest", "version": "0.13.4", "query": "Client", "kind": "struct", "page_size": 5
    })).await;
    let FindOutcome::FindSuccess(success) = outcome else {
        panic!("pinned name search should succeed");
    };
    let rows = match success.result {
        FindResult::CompletePage {
            rows,
            resolved_version,
            ..
        }
        | FindResult::ContinuationPage {
            rows,
            resolved_version,
            ..
        } => {
            assert_eq!(resolved_version, "0.13.4");
            rows
        }
    };
    assert!(rows.len() <= 5);
    assert!(rows.iter().any(|row| row.path == "reqwest::Client"
        && row.kind == RustdocItemKind::Struct
        && row.summary.is_some()));
}

#[tokio::test]
#[ignore = "network + durable cache"]
async fn a_continuation_cursor_has_no_gaps_or_duplicates() {
    let first = find(serde_json::json!({
        "crate_name": "reqwest", "version": "0.13.4", "kind": "struct", "page_size": 2, "with_summary": false
    })).await;
    let FindOutcome::FindSuccess(first) = first else {
        panic!("pinned browse should succeed");
    };
    let FindResult::ContinuationPage {
        rows: first_rows,
        next_cursor,
        ..
    } = first.result
    else {
        return;
    };
    let second = find(serde_json::json!({
        "crate_name": "reqwest", "version": "0.13.4", "kind": "struct", "page_size": 2, "with_summary": false, "cursor": next_cursor
    })).await;
    let FindOutcome::FindSuccess(second) = second else {
        panic!("continuation should succeed");
    };
    let second_rows = match second.result {
        FindResult::CompletePage { rows, .. } | FindResult::ContinuationPage { rows, .. } => rows,
    };
    assert!(second_rows
        .iter()
        .all(|row| !first_rows.iter().any(|prior| prior.path == row.path)));
    assert!(second_rows.iter().all(|row| row.summary.is_none()));
}

#[tokio::test]
#[ignore = "durable cache"]
async fn asking_both_query_kinds_is_an_invalid_request() {
    let outcome = find(serde_json::json!({ "crate_name": "reqwest", "query": "Client", "doc_query": "send an HTTP request" })).await;
    let FindOutcome::ErrorOutcome(failure) = outcome else {
        panic!("mutually exclusive queries should fail");
    };
    assert!(matches!(failure.error, ApiError::InvalidRequestError(_)));
}
