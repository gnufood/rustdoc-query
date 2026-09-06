//! These views deliberately originate in the checked-in JSON Schema rather
//! than `JsonSchema` derived from Typify output. The MCP adapter owns only the
//! removal of its registration-time `output_format` input.

use serde_json::Value;

use super::generated::ToolContract;

const CANONICAL_SCHEMA: &str = include_str!("../../schema/rustdoc-query.v1.schema.json");

pub(super) fn input(contract: ToolContract) -> Result<Value, String> {
    let mut schema = rooted(contract.request)?;
    require_root_object(&mut schema)?;
    omit_transport_fields(&mut schema, contract.omitted_input_fields)?;
    Ok(schema)
}

pub(super) fn outcome(outcome: &str) -> Result<Value, String> {
    let mut schema = rooted(outcome)?;
    require_root_object(&mut schema)?;
    Ok(schema)
}

fn require_root_object(schema: &mut Value) -> Result<(), String> {
    let root = schema
        .as_object_mut()
        .ok_or_else(|| "canonical MCP schema root is not an object".to_owned())?;
    root.insert("type".to_owned(), Value::String("object".to_owned()));
    Ok(())
}

fn rooted(definition: &str) -> Result<Value, String> {
    let mut schema: Value = serde_json::from_str(CANONICAL_SCHEMA)
        .map_err(|error| format!("canonical schema is invalid JSON: {error}"))?;
    let root = schema
        .as_object_mut()
        .ok_or_else(|| "canonical schema root is not an object".to_owned())?;
    // This is a modified transport view, not the canonical document named by
    // the catalog's identity. All retained references are local pointers.
    root.remove("$id");
    root.insert(
        "$ref".to_owned(),
        Value::String(format!("#/$defs/{definition}")),
    );
    Ok(schema)
}

/// Remove registration-only inputs from every request shape retained in an
/// MCP schema bundle, including sibling definitions not rooted by this tool.
fn omit_transport_fields(schema: &mut Value, fields: &[&str]) -> Result<(), String> {
    let definitions = schema
        .pointer_mut("/$defs")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "canonical MCP schema has no definitions object".to_owned())?;
    for (name, definition) in definitions {
        if !name.ends_with("Request") {
            continue;
        }
        let properties = definition
            .get_mut("properties")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| format!("canonical {name} definition has no properties object"))?;
        for field in fields {
            properties.remove(*field);
        }
    }
    Ok(())
}
