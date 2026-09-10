//! MCP transport and its tool handlers.
//!
//! All logging goes to stderr — the stdio transport owns stdout for JSON-RPC framing.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use std::sync::Arc;

use rmcp::model::{CallToolResult, Implementation, JsonObject, ServerCapabilities, ServerInfo};
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler};

mod generated;
mod response;
mod schema;

#[cfg(test)]
mod tests;

use crate::contract::generated::{
    ApiError, FindRequest, GetItemRequest, OperationFailedError, OperationFailedErrorCode,
    OutputFormat, OverviewRequest,
};
use crate::service::RustdocQueryService;
use generated::{McpRequest, TOOL_CONTRACTS};
use response::outcome;

const INSTRUCTIONS: &str = "Query the public API surface of Rust crates against \
ground-truth rustdoc JSON from docs.rs. Recommended workflow: (1) call \
`crate_overview` to orient and get a module map, (2) call `find_items` to locate \
specific items by name or kind, (3) call `get_item` for full detail on a specific item. \
\n\nNotes: `find_items` searches the indexed public surface, including dependency \
re-exports whose docs.rs metadata can be resolved. A zero result can mean there is no \
matching public item or that a dependency expansion was unavailable; inspect the \
dependency crate's overview when needed. `get_item` accepts public paths, canonical rustdoc \
inner paths for publicly reachable items, and `Parent::method` paths to retrieve a single \
method's signature.";

#[derive(Debug)]
/// Embeddable MCP server backed by [`RustdocQueryService`].
pub struct RustdocMcpServer {
    service: RustdocQueryService,
    output_format: OutputFormat,
    tool_router: ToolRouter<RustdocMcpServer>,
}

#[tool_router]
impl RustdocMcpServer {
    /// Build the MCP transport, resolving the on-disk cache directory.
    ///
    /// # Errors
    /// Returns a generated `operation_failed` error when initialization cannot
    /// complete.
    pub fn new() -> Result<Self, ApiError> {
        Self::with_output_format(OutputFormat::default())
    }

    /// Build an MCP transport with one fixed result representation.
    ///
    /// # Errors
    /// Returns a generated `operation_failed` error when initialization or
    /// canonical-schema projection cannot complete.
    pub fn with_output_format(output_format: OutputFormat) -> Result<Self, ApiError> {
        let mut tool_router = Self::tool_router();
        configure_tools(&mut tool_router, output_format)
            .map_err(|error| initialization_error(format!("MCP schema setup failed: {error}")))?;
        Ok(Self {
            service: RustdocQueryService::new()?,
            output_format,
            tool_router,
        })
    }

    /// Tool to search or list items in a crate's public surface by name, kind, or
    /// module. Use when you need to locate a specific item's full path before
    /// calling `get_item`, or to enumerate all items of a given kind in a module.
    /// Returns paths and kinds only — does not return method signatures or field
    /// details; use `get_item` for that. For name-based lookup use `query`; for
    /// concept/description search use `doc_query` (lexical matching over docs
    /// and item names, ranked for API applicability). Example:
    /// `find_items(crate_name="tokio", query="Mutex", kind="struct")` or
    /// `find_items(crate_name="reqwest", doc_query="send HTTP request")`.
    #[tool(title = "Find Items")]
    async fn find_items(
        &self,
        Parameters(mut request): Parameters<FindRequest>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let output_format = registration_format(&mut request, self.output_format)?;
        let result = self.service.find(request).await;
        outcome(&result, output_format)
    }

    /// Tool to get a high-level map of a crate's public API: module list,
    /// headline items grouped by kind, and total counts. Use when starting
    /// exploration of an unfamiliar crate — call this first, then `find_items`
    /// to locate specific items, then `get_item` for full detail. Does not
    /// return method signatures or field details. Example:
    /// `crate_overview(crate_name="tokio")` or
    /// `crate_overview(crate_name="tokio", module="tokio::sync")`.
    #[tool(title = "Crate Overview")]
    async fn crate_overview(
        &self,
        Parameters(mut request): Parameters<OverviewRequest>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let output_format = registration_format(&mut request, self.output_format)?;
        let result = self.service.overview(request).await;
        outcome(&result, output_format)
    }

    /// Tool to get full detail for one public item: fields, inherent methods with
    /// full signatures, associated constants, concrete trait impls with
    /// where-clause bounds, derived traits, auto-traits, and whether an enum is
    /// `#[non_exhaustive]`. Use when you have the exact item path and need its
    /// complete API surface. Call `find_items` first if you don't know the exact
    /// path. Blanket impls (`Into`, `From`, `TryFrom`, `Borrow`, `Any`) are
    /// omitted by default — pass `include_impls=true` to see them. Example:
    /// `get_item(crate_name="tokio", path="tokio::sync::Mutex")`.
    #[tool(title = "Get Item Detail")]
    async fn get_item(
        &self,
        Parameters(mut request): Parameters<GetItemRequest>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let output_format = registration_format(&mut request, self.output_format)?;
        let result = self.service.get_item(request).await;
        outcome(&result, output_format)
    }
}

fn initialization_error(message: String) -> ApiError {
    ApiError::OperationFailedError(OperationFailedError {
        code: OperationFailedErrorCode::OperationFailed,
        diagnostic_id: None,
        message,
        retryable: false,
    })
}

fn registration_format<T: McpRequest>(
    request: &mut T,
    registered: OutputFormat,
) -> Result<OutputFormat, ErrorData> {
    if request.requested_output_format().is_some() {
        return Err(ErrorData::invalid_params(
            "`output_format` is selected when the MCP adapter is registered",
            None,
        ));
    }
    let output_format = RustdocQueryService::output_format(Some(registered));
    request.select_output_format(output_format);
    Ok(output_format)
}

fn configure_tools(
    router: &mut ToolRouter<RustdocMcpServer>,
    output_format: OutputFormat,
) -> Result<(), String> {
    for contract in TOOL_CONTRACTS {
        let input_schema = schema_object(schema::input(*contract)?)?;
        let output_schema = if matches!(output_format, OutputFormat::Json) {
            Some(schema_object(schema::outcome(contract.outcome)?)?)
        } else {
            None
        };
        let route = router
            .map
            .get_mut(contract.name)
            .ok_or_else(|| format!("generated MCP route `{}` is missing", contract.name))?;
        route.attr.input_schema = input_schema;
        route.attr.output_schema = output_schema;
    }
    Ok(())
}

fn schema_object(schema: serde_json::Value) -> Result<Arc<JsonObject>, String> {
    match schema {
        serde_json::Value::Object(object) => Ok(Arc::new(object)),
        _ => Err("canonical MCP schema must be a JSON object".to_owned()),
    }
}

// RMCP's `tool_handler` macro generates async trait methods even when this
// implementation overrides only synchronous server metadata.
#[allow(unknown_lints, clippy::unused_async_trait_impl)]
#[tool_handler(router = self.tool_router)]
impl ServerHandler for RustdocMcpServer {
    fn get_info(&self) -> ServerInfo {
        // `Implementation::from_build_env` (the default) reports rmcp's own
        // crate name, so set ours explicitly.
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(INSTRUCTIONS)
    }
}
