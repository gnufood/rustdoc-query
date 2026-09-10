//! Versioned request, result, and error types for rustdoc-query.
//!
//! Use [`generated`] for all public data models accepted or returned by
//! [`crate::service::RustdocQueryService`] and [`crate::mcp::RustdocMcpServer`].
//! Field names, defaults, and serialized representations are defined by this
//! contract version.

#[path = "../schema/generated/rustdoc-query.v2.rs"]
#[allow(
    clippy::module_name_repetitions,
    reason = "Schema generation owns these protocol type names."
)]
/// Generated, versioned request, result, and error types.
pub mod generated;

#[path = "../schema/generated/rustdoc-query.v2.defaults.rs"]
pub(crate) mod defaults;

#[path = "../schema/generated/rustdoc-query.v2.strict_option.rs"]
pub(crate) mod strict_option;
