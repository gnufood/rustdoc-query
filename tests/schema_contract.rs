//! Contract fixtures and schema-to-Rust generation spike for public API v1.

use serde_json::Value;

#[path = "../schema/generated/rustdoc-query.v1.strict_option.rs"]
mod strict_option;

#[allow(dead_code, unreachable_pub)]
#[path = "../schema/generated/rustdoc-query.v1.rs"]
mod generated;

const SCHEMA: &str = include_str!("../schema/rustdoc-query.v1.schema.json");

fn schema_for(definition: &str) -> Value {
    let mut schema: Value = serde_json::from_str(SCHEMA).expect("schema JSON is valid");
    schema.as_object_mut().expect("schema is an object").insert(
        "$ref".to_owned(),
        Value::String(format!("#/$defs/{definition}")),
    );
    schema
}

fn assert_valid(definition: &str, fixture: &str) {
    let schema = schema_for(definition);
    let instance: Value = serde_json::from_str(fixture).expect("fixture JSON is valid");
    jsonschema::draft202012::validate(&schema, &instance)
        .unwrap_or_else(|error| panic!("{definition} fixture should validate: {error}"));
}

fn assert_invalid(definition: &str, fixture: &str) {
    let schema = schema_for(definition);
    let instance: Value = serde_json::from_str(fixture).expect("fixture JSON is valid");
    assert!(
        jsonschema::draft202012::validate(&schema, &instance).is_err(),
        "{definition} fixture should be rejected"
    );
}

#[test]
fn schema_accepts_public_request_fixtures() {
    assert_valid(
        "OverviewRequest",
        include_str!("fixtures/schema/overview-request.json"),
    );
    assert_valid(
        "FindRequest",
        include_str!("fixtures/schema/find-name-request.json"),
    );
    assert_valid(
        "FindRequest",
        include_str!("fixtures/schema/find-doc-request.json"),
    );
    assert_valid(
        "GetItemRequest",
        include_str!("fixtures/schema/get-item-request.json"),
    );
}

#[test]
fn schema_accepts_approved_v1_request_names_and_defaults() {
    assert_valid(
        "FindRequest",
        include_str!("fixtures/schema/find-approved-v1-request.json"),
    );

    let schema: Value = serde_json::from_str(SCHEMA).expect("schema JSON is valid");
    assert_eq!(
        schema.pointer("/$defs/OutputFormat/default"),
        Some(&Value::String("json".to_owned()))
    );
    assert_eq!(
        schema.pointer("/$defs/FindRequest/properties/query/minLength"),
        Some(&Value::Number(1.into()))
    );
}

#[test]
fn schema_rejects_superseded_or_empty_find_request_fields() {
    assert_invalid(
        "FindRequest",
        include_str!("fixtures/schema/find-legacy-crate-request.json"),
    );
    assert_invalid(
        "FindRequest",
        include_str!("fixtures/schema/find-empty-name-request.json"),
    );
}

#[test]
fn schema_rejects_conflicting_find_queries() {
    assert_invalid(
        "FindRequest",
        include_str!("fixtures/schema/find-conflicting-request.json"),
    );
}

#[test]
fn schema_rejects_null_for_presence_sensitive_inputs() {
    for fixture in [
        r#"{ "crate_name": "reqwest", "with_summary": null }"#,
        r#"{ "crate_name": "reqwest", "cursor": null }"#,
        r#"{ "crate_name": "reqwest", "page_size": null }"#,
    ] {
        assert_invalid("FindRequest", fixture);
    }

    assert_invalid(
        "OverviewRequest",
        r#"{ "crate_name": "reqwest", "full_docs": null }"#,
    );
    for fixture in [
        r#"{ "crate_name": "reqwest", "path": "reqwest::Client", "full_docs": null }"#,
        r#"{ "crate_name": "reqwest", "path": "reqwest::Client", "include_impls": null }"#,
    ] {
        assert_invalid("GetItemRequest", fixture);
    }
}

#[test]
fn schema_accepts_success_and_error_outcomes() {
    assert_valid(
        "OverviewOutcome",
        include_str!("fixtures/schema/overview-success.json"),
    );
    assert_valid(
        "FindOutcome",
        include_str!("fixtures/schema/find-success.json"),
    );
    assert_valid(
        "GetItemOutcome",
        include_str!("fixtures/schema/get-item-error.json"),
    );
}

#[test]
fn schema_allows_success_without_cache_observability() {
    assert_valid(
        "OverviewOutcome",
        include_str!("fixtures/schema/overview-success-without-cache.json"),
    );
}

#[test]
fn generated_public_contract_types_compile() {
    fn generated<T: serde::Serialize + serde::de::DeserializeOwned>() {}

    generated::<generated::OverviewRequest>();
    generated::<generated::FindRequest>();
    generated::<generated::GetItemRequest>();
    generated::<generated::OverviewOutcome>();
    generated::<generated::FindOutcome>();
    generated::<generated::GetItemOutcome>();
}
