//! Public JSON/TOON serialization parity tests.

use rustdoc_query::{
    contract::generated::{FindOutcome, OutputFormat, OverviewRequest},
    service::RustdocQueryService,
};
use serde_json::Value;
use std::str::FromStr;

fn default_output_format() -> OutputFormat {
    let schema: Value =
        serde_json::from_str(include_str!("../schema/rustdoc-query.v1.schema.json"))
            .expect("canonical schema is valid JSON");
    let default = schema
        .pointer("/$defs/OutputFormat/default")
        .and_then(Value::as_str)
        .expect("output format has a string default");
    OutputFormat::from_str(default).expect("schema output format default is valid")
}

#[test]
fn generated_outcomes_have_matching_json_and_toon_serializations() {
    let expected: Value = serde_json::from_str(include_str!("fixtures/schema/find-success.json"))
        .expect("fixture JSON is valid");
    let outcome: FindOutcome =
        serde_json::from_value(expected.clone()).expect("fixture deserializes as a public outcome");

    assert_eq!(
        RustdocQueryService::json_value(&outcome).expect("JSON serialization succeeds"),
        expected
    );

    let toon = RustdocQueryService::toon(&outcome).expect("TOON serialization succeeds");
    let decoded: Value = toon_format::decode_default(&toon).expect("TOON decodes to JSON");
    assert_eq!(decoded, expected);
}

#[test]
fn a_request_omitting_the_output_format_selects_json() {
    let request: OverviewRequest =
        serde_json::from_value(serde_json::json!({ "crate_name": "reqwest" }))
            .expect("request is valid");

    assert_eq!(
        RustdocQueryService::output_format(request.output_format),
        default_output_format()
    );
}

#[test]
fn a_request_asking_for_toon_selects_toon() {
    let request: OverviewRequest = serde_json::from_value(
        serde_json::json!({ "crate_name": "reqwest", "output_format": "toon" }),
    )
    .expect("request is valid");

    assert_eq!(
        RustdocQueryService::output_format(request.output_format),
        OutputFormat::Toon
    );
}
