//! Generated public contract types for rustdoc-query API v1.
//!
//! The JSON Schema in `schema/rustdoc-query.v1.schema.json` is the source of
//! truth. Regenerate this module with `just generate-schema` after
//! an intentional schema change.

#[path = "../schema/generated/rustdoc-query.v1.rs"]
#[allow(
    clippy::module_name_repetitions,
    reason = "Schema generation owns these protocol type names."
)]
pub mod generated;

#[path = "../schema/generated/rustdoc-query.v1.defaults.rs"]
pub(crate) mod defaults;

#[path = "../schema/generated/rustdoc-query.v1.strict_option.rs"]
pub(crate) mod strict_option;
