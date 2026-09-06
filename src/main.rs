#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::as_conversions
    )
)]
#![forbid(unsafe_code)]

//! MCP server for querying crate public APIs from docs.rs rustdoc JSON.
//!
//! Logging must use stderr; stdout is reserved for JSON-RPC framing.

use rmcp::transport::io::stdio;
use rmcp::ServiceExt;
use rustdoc_query::contract::generated::OutputFormat;
use rustdoc_query::mcp::RustdocMcpServer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let environment_format = std::env::var("RUSTDOC_QUERY_OUTPUT_FORMAT").ok();
    let output_format =
        configured_output_format(std::env::args().skip(1), environment_format.as_deref())
            .map_err(std::io::Error::other)?;
    let server = RustdocMcpServer::with_output_format(output_format)
        .map_err(|error| std::io::Error::other(format!("MCP initialization failed: {error:?}")))?;
    tracing::info!(%output_format, "rustdoc-query serving on stdio");

    let running = server.serve(stdio()).await?;
    let reason = running.waiting().await?;
    tracing::info!(?reason, "rustdoc-query shutting down");

    Ok(())
}

/// The command-line flag overrides an environment default.
fn configured_output_format(
    arguments: impl IntoIterator<Item = String>,
    environment_format: Option<&str>,
) -> Result<OutputFormat, String> {
    let mut arguments = arguments.into_iter();
    let mut flag_format = None;

    while let Some(argument) = arguments.next() {
        let value = if argument == "--output-format" {
            arguments
                .next()
                .ok_or_else(|| "`--output-format` requires `json` or `toon`".to_owned())?
        } else if let Some(value) = argument.strip_prefix("--output-format=") {
            value.to_owned()
        } else {
            return Err(format!(
                "unknown argument `{argument}`; use `--output-format json|toon`"
            ));
        };
        flag_format = Some(parse_output_format(&value, "--output-format")?);
    }

    match flag_format {
        Some(format) => Ok(format),
        None => match environment_format {
            Some(value) => parse_output_format(value, "RUSTDOC_QUERY_OUTPUT_FORMAT"),
            None => Ok(OutputFormat::default()),
        },
    }
}

fn parse_output_format(value: &str, source: &str) -> Result<OutputFormat, String> {
    value
        .parse()
        .map_err(|_| format!("{source} must be `json` or `toon`, received `{value}`"))
}

#[cfg(test)]
mod tests {
    use super::configured_output_format;
    use rustdoc_query::contract::generated::OutputFormat;

    #[test]
    fn defaults_to_the_schema_output_format() {
        assert_eq!(
            configured_output_format([], None),
            Ok(OutputFormat::default())
        );
    }

    #[test]
    fn accepts_toon_from_the_environment() {
        assert_eq!(
            configured_output_format([], Some("toon")),
            Ok(OutputFormat::Toon)
        );
    }

    #[test]
    fn command_line_output_format_overrides_the_environment() {
        assert_eq!(
            configured_output_format(
                ["--output-format".to_owned(), "toon".to_owned()],
                Some("json")
            ),
            Ok(OutputFormat::Toon)
        );
    }

    #[test]
    fn accepts_equals_style_output_format_flag() {
        assert_eq!(
            configured_output_format(["--output-format=toon".to_owned()], None),
            Ok(OutputFormat::Toon)
        );
    }

    #[test]
    fn rejects_an_invalid_registration_format() {
        assert!(configured_output_format([], Some("xml")).is_err());
    }

    #[test]
    fn rejects_unknown_or_incomplete_arguments() {
        assert!(configured_output_format(["--unknown".to_owned()], None).is_err());
        assert!(configured_output_format(["--output-format".to_owned()], None).is_err());
    }
}
