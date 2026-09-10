use super::{add_strict_option_deserializers, codegen_schema, defaults, generate_types, mcp};

const SCHEMA: &str = include_str!("../../../schema/rustdoc-query.v2.schema.json");

fn canonical_schema() -> serde_json::Value {
    serde_json::from_str(SCHEMA).expect("canonical schema is valid JSON")
}

#[test]
fn default_removal_follows_optional_request_references() {
    let mut schema = canonical_schema();
    let property = schema
        .pointer_mut("/$defs/FindRequest/properties/with_summary")
        .and_then(serde_json::Value::as_object_mut)
        .expect("with_summary is an object property");
    property.insert("default".to_owned(), serde_json::Value::Bool(false));

    let generated = codegen_schema(&schema).expect("codegen schema is valid");

    assert_eq!(
        generated.pointer("/$defs/FindRequest/properties/with_summary/default"),
        Some(&serde_json::Value::Bool(false))
    );
    assert!(generated.pointer("/$defs/IncludeSummary/default").is_none());
}

#[test]
fn referenced_type_defaults_do_not_make_request_fields_required() {
    let generated = generate_types(&canonical_schema()).expect("types generate");

    assert!(generated.contains("pub with_summary: ::std::option::Option<IncludeSummary>"));
    assert!(generated.contains("pub cursor: ::std::option::Option<PageCursor>"));
    assert!(generated.contains("pub page_size: ::std::option::Option<PageSize>"));
    assert!(generated.contains("pub output_format: ::std::option::Option<OutputFormat>"));
    assert!(generated.contains("impl ::std::default::Default for OutputFormat"));
}

#[test]
fn bounded_integer_definitions_generate_opaque_checked_scalars() {
    let generated = generate_types(&canonical_schema()).expect("types generate");

    assert!(generated.contains(
        "pub struct PageSize(#[schemars(range(min = 1, max = 200))] ::std::primitive::u8);"
    ));
    assert!(generated.contains("#[schemars(range(min = 1, max = 200))] ::std::primitive::u8"));
    assert!(generated.contains("impl ::std::convert::TryFrom<::std::primitive::u64> for PageSize"));
    assert!(generated.contains("PageSize must be between 1 and 200"));
    assert!(!generated.contains("pub struct PageSize(pub ::std::num::NonZeroU64);"));
}

#[test]
fn enum_defaults_belong_to_the_generated_type_not_private_service_constants() {
    let schema = canonical_schema();

    let private_defaults = defaults::source(&schema).expect("defaults generate");
    let generated = generate_types(&schema).expect("types generate");

    assert!(!private_defaults.contains("OUTPUT_FORMAT"));
    assert!(generated.contains("impl ::std::default::Default for OutputFormat"));
    assert!(generated.contains("Self::Json"));
}

#[test]
fn public_schema_nodes_are_documented_and_emit_variant_docs() {
    let schema = canonical_schema();
    let definitions = schema["$defs"]
        .as_object()
        .expect("canonical schema declares definitions");

    for (name, definition) in definitions {
        assert!(
            definition
                .get("description")
                .and_then(serde_json::Value::as_str)
                .is_some(),
            "{name} needs a description"
        );
        if let Some(properties) = definition
            .get("properties")
            .and_then(serde_json::Value::as_object)
        {
            for (property, schema) in properties {
                assert!(
                    schema
                        .get("description")
                        .and_then(serde_json::Value::as_str)
                        .is_some(),
                    "{name}.{property} needs a description"
                );
            }
        }
        if let Some(branches) = definition
            .get("oneOf")
            .and_then(serde_json::Value::as_array)
        {
            for (index, branch) in branches.iter().enumerate() {
                assert!(
                    branch
                        .get("description")
                        .and_then(serde_json::Value::as_str)
                        .is_some(),
                    "{name}.oneOf[{index}] needs a description"
                );
            }
        }
    }

    let generated = generate_types(&schema).expect("types generate");
    for documentation in [
        "Emit the canonical JSON outcome.",
        "A Rust module.",
        "Final page. No further results exist.",
        "The request violates the contract.",
        "Maximum rows to return in this page.",
    ] {
        assert!(generated.contains(documentation));
    }
}

#[test]
fn strict_option_deserializers_follow_schema_optional_fields_through_the_ast() {
    let source = r#"
        pub struct FindRequest {
            #[serde(skip_serializing_if = "::std::option::Option::is_none", default)]
            pub with_summary: ::std::option::Option<IncludeSummary>,
        }
    "#;

    let generated = add_strict_option_deserializers(&canonical_schema(), source)
        .expect("strict option attributes generate");

    assert!(generated.contains("deserialize_with = \"super::strict_option::deserialize\""));
}

#[test]
fn strict_option_deserializers_skip_option_fields_that_are_required_by_the_schema() {
    let source = r#"
        pub struct FindRequest {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            pub crate_name: ::std::option::Option<CrateName>,
        }
    "#;

    let generated = add_strict_option_deserializers(&canonical_schema(), source)
        .expect("strict option attributes generate");

    assert!(!generated.contains("deserialize_with = \"super::strict_option::deserialize\""));
}

#[test]
fn strict_option_deserializers_match_schema_fields_through_serde_renames() {
    let source = r#"
        pub struct FindRequest {
            #[serde(rename = "with_summary", default)]
            pub with_summary_value: ::std::option::Option<IncludeSummary>,
        }
    "#;

    let generated = add_strict_option_deserializers(&canonical_schema(), source)
        .expect("strict option attributes generate");

    assert!(generated.contains("deserialize_with = \"super::strict_option::deserialize\""));
}

#[test]
fn mcp_bindings_are_generated_from_the_canonical_tool_metadata() {
    let generated = mcp::source(&canonical_schema()).expect("MCP bindings generate");

    assert!(generated.contains("pub(crate) const TOOL_CONTRACTS"));
    assert!(generated.contains("name: \"find_items\""));
    assert!(generated.contains("impl McpRequest for crate::contract::generated::FindRequest"));
    assert!(generated.contains("impl McpOutcome for crate::contract::generated::FindOutcome"));
    assert!(generated.contains("matches!(self, Self::ErrorOutcome(_))"));
}

#[test]
fn mcp_bindings_reject_duplicate_tool_names() {
    let mut schema = canonical_schema();
    let tools = schema
        .get_mut("x-mcp-tools")
        .and_then(serde_json::Value::as_array_mut)
        .expect("canonical schema declares MCP tools");
    tools.push(tools[0].clone());

    let error = mcp::source(&schema).expect_err("duplicate MCP tool must fail");
    assert!(error.contains("repeats tool name crate_overview"));
}

#[test]
fn mcp_bindings_require_the_generated_error_branch() {
    let mut schema = canonical_schema();
    schema
        .pointer_mut("/$defs/FindOutcome/oneOf")
        .and_then(serde_json::Value::as_array_mut)
        .expect("FindOutcome declares oneOf")
        .retain(|branch| {
            branch.pointer("/$ref").and_then(serde_json::Value::as_str)
                != Some("#/$defs/ErrorOutcome")
        });

    let error = mcp::source(&schema).expect_err("missing error branch must fail");
    assert!(error.contains("MCP outcome FindOutcome must include ErrorOutcome"));
}
