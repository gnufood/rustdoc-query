use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;

use crate::contract::generated::OutputFormat;
use crate::service::RustdocQueryService;

use super::generated::McpOutcome;

pub(super) fn outcome<T: serde::Serialize + McpOutcome>(
    outcome: &T,
    format: OutputFormat,
) -> Result<CallToolResult, ErrorData> {
    let is_error = outcome.is_tool_error();
    match format {
        OutputFormat::Json => {
            let encoded = RustdocQueryService::json(outcome).map_err(|error| {
                ErrorData::internal_error(format!("JSON encode failed: {error:?}"), None)
            })?;
            let mut result = CallToolResult::success(vec![Content::text(encoded.text)]);
            result.structured_content = Some(encoded.value);
            result.is_error = Some(is_error);
            Ok(result)
        }
        OutputFormat::Toon => {
            let text = RustdocQueryService::toon(outcome).map_err(|error| {
                ErrorData::internal_error(format!("TOON encode failed: {error:?}"), None)
            })?;
            let mut result = CallToolResult::success(vec![Content::text(text)]);
            result.is_error = Some(is_error);
            Ok(result)
        }
    }
}
