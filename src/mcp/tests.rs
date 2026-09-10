use rmcp::model::Content;
use serde_json::Value;

use super::response;
use super::schema;
use super::RustdocMcpServer;
use crate::contract::generated::{FindOutcome, GetItemOutcome, OutputFormat, OverviewOutcome};
use crate::service::RustdocQueryService;

const SCHEMA: &str = include_str!("../../schema/rustdoc-query.v2.schema.json");

fn frame<T: serde::Serialize + super::generated::McpOutcome>(
    outcome: &T,
    format: OutputFormat,
) -> rmcp::model::CallToolResult {
    response::outcome(outcome, format).unwrap()
}

#[test]
fn projected_find_input_keeps_canonical_contract_except_transport_format() {
    let schema = schema::input(super::generated::TOOL_CONTRACTS[1]).expect("find schema projects");
    let canonical: Value = serde_json::from_str(SCHEMA).expect("canonical schema is valid JSON");

    assert_eq!(
        schema.pointer("/$ref"),
        Some(&Value::String("#/$defs/FindRequest".to_owned()))
    );
    assert_eq!(
        schema.pointer("/$defs/FindRequest/properties/crate_name"),
        canonical.pointer("/$defs/FindRequest/properties/crate_name")
    );
    assert_eq!(
        schema.pointer("/$defs/FindRequest/properties/query/minLength"),
        canonical.pointer("/$defs/FindRequest/properties/query/minLength")
    );
    assert_eq!(
        schema.pointer("/$defs/FindRequest/properties/output_format"),
        None
    );
    assert_eq!(
        schema.pointer("/$defs/FindRequest/required"),
        canonical.pointer("/$defs/FindRequest/required")
    );
    assert_eq!(
        schema.pointer("/type"),
        Some(&Value::String("object".to_owned()))
    );
    assert_eq!(schema.pointer("/$id"), None);
}

#[test]
fn projected_inputs_hide_transport_fields_from_every_request_definition() {
    for contract in super::generated::TOOL_CONTRACTS {
        let schema = schema::input(*contract).expect("input schema projects");
        let definitions = schema
            .pointer("/$defs")
            .and_then(Value::as_object)
            .expect("projected schema retains definitions");
        for (name, definition) in definitions {
            if !name.ends_with("Request") {
                continue;
            }
            assert_eq!(
                definition.pointer("/properties/output_format"),
                None,
                "{name} must not expose a transport-only field",
            );
        }
    }
}

#[test]
fn projected_schemas_do_not_reuse_the_canonical_document_identity() {
    let canonical: Value = serde_json::from_str(SCHEMA).expect("canonical schema is valid JSON");
    assert!(canonical.pointer("/$id").is_some());

    for schema in [
        schema::input(super::generated::TOOL_CONTRACTS[0]).expect("overview input projects"),
        schema::input(super::generated::TOOL_CONTRACTS[1]).expect("find input projects"),
        schema::input(super::generated::TOOL_CONTRACTS[2]).expect("item input projects"),
        schema::outcome("OverviewOutcome").expect("overview outcome projects"),
        schema::outcome("FindOutcome").expect("find outcome projects"),
        schema::outcome("GetItemOutcome").expect("item outcome projects"),
    ] {
        assert_eq!(schema.pointer("/$id"), None);
    }
}

#[test]
fn projected_json_outcome_schema_is_the_canonical_outcome() {
    let schema = schema::outcome("GetItemOutcome").expect("outcome schema projects");
    let canonical: Value = serde_json::from_str(SCHEMA).expect("canonical schema is valid JSON");

    assert_eq!(
        schema.pointer("/$ref"),
        Some(&Value::String("#/$defs/GetItemOutcome".to_owned()))
    );
    assert_eq!(
        schema.pointer("/$defs/GetItemOutcome"),
        canonical.pointer("/$defs/GetItemOutcome")
    );
    assert_eq!(
        schema.pointer("/$defs/ApiError"),
        canonical.pointer("/$defs/ApiError")
    );
    assert_eq!(
        schema.pointer("/type"),
        Some(&Value::String("object".to_owned()))
    );
    assert_eq!(schema.pointer("/$id"), None);
}

#[test]
fn json_mode_returns_generated_success_with_a_json_text_fallback() {
    let outcome: OverviewOutcome = serde_json::from_str(include_str!(
        "../../tests/fixtures/schema/overview-success.json"
    ))
    .expect("fixture is a generated overview outcome");

    let result = frame(&outcome, OutputFormat::Json);

    let value = RustdocQueryService::json_value(&outcome).unwrap();
    assert_eq!(result.content, vec![Content::text(value.to_string())]);
    assert_eq!(result.structured_content, Some(value));
    assert_eq!(result.is_error, Some(false));
}

#[test]
fn json_mode_preserves_typed_errors_for_every_tool_outcome() {
    let fixture = include_str!("../../tests/fixtures/schema/get-item-error.json");

    let overview: OverviewOutcome =
        serde_json::from_str(fixture).expect("fixture is a generated overview outcome");
    let find: FindOutcome =
        serde_json::from_str(fixture).expect("fixture is a generated find outcome");
    let item: GetItemOutcome =
        serde_json::from_str(fixture).expect("fixture is a generated item outcome");

    for result in [
        frame(&overview, OutputFormat::Json),
        frame(&find, OutputFormat::Json),
        frame(&item, OutputFormat::Json),
    ] {
        let structured = result
            .structured_content
            .as_ref()
            .expect("JSON mode provides structured content");
        assert_eq!(result.content, vec![Content::text(structured.to_string())]);
        assert_eq!(structured["status"], "error");
        assert_eq!(structured["error"]["code"], "item_not_found");
        assert_eq!(result.is_error, Some(true));
    }
}

#[test]
fn default_registration_advertises_canonical_json_contracts() {
    let server = RustdocMcpServer::new().expect("MCP server initializes");

    for contract in super::generated::TOOL_CONTRACTS {
        let tool_name = contract.name;
        let tool = server
            .tool_router
            .get(tool_name)
            .expect("generated route is registered");

        assert_eq!(
            Value::Object(tool.input_schema.as_ref().clone()),
            schema::input(*contract).expect("request schema projects")
        );
        assert_eq!(
            tool.output_schema
                .as_ref()
                .map(|schema| Value::Object(schema.as_ref().clone())),
            Some(schema::outcome(contract.outcome).expect("outcome schema projects"))
        );
    }
}

#[test]
fn toon_registration_keeps_inputs_and_omits_output_schemas() {
    let server = RustdocMcpServer::with_output_format(OutputFormat::Toon)
        .expect("TOON MCP server initializes");

    for contract in super::generated::TOOL_CONTRACTS {
        let tool_name = contract.name;
        let tool = server
            .tool_router
            .get(tool_name)
            .expect("generated route is registered");

        assert_eq!(
            Value::Object(tool.input_schema.as_ref().clone()),
            schema::input(*contract).expect("request schema projects")
        );
        assert!(tool.output_schema.is_none());
    }
}

#[test]
fn toon_mode_returns_generated_success_as_text_only() {
    let outcome: OverviewOutcome = serde_json::from_str(include_str!(
        "../../tests/fixtures/schema/overview-success.json"
    ))
    .expect("fixture is a generated overview outcome");

    let result = frame(&outcome, OutputFormat::Toon);

    assert_eq!(
        result.content,
        vec![Content::text(
            RustdocQueryService::toon(&outcome).expect("generated outcome encodes as TOON")
        )]
    );
    assert_eq!(result.structured_content, None);
    assert_eq!(result.is_error, Some(false));
}

#[test]
fn toon_mode_returns_generated_error_as_text_only() {
    let outcome: GetItemOutcome = serde_json::from_str(include_str!(
        "../../tests/fixtures/schema/get-item-error.json"
    ))
    .expect("fixture is a generated get-item outcome");

    let result = frame(&outcome, OutputFormat::Toon);

    assert_eq!(
        result.content,
        vec![Content::text(
            RustdocQueryService::toon(&outcome).expect("generated error encodes as TOON")
        )]
    );
    assert_eq!(result.structured_content, None);
    assert_eq!(result.is_error, Some(true));
}
