//! Query Rust crates' public APIs from rustdoc JSON.
//!
//! The supported API consists of generated request and outcome types in
//! [`contract::generated`], the direct-library [`service::RustdocQueryService`],
//! and the embeddable MCP adapter [`mcp::RustdocMcpServer`].
//!
//! Construct a service, submit a generated request, and match its generated
//! outcome. The service fetches and caches rustdoc data as needed.

#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::as_conversions
    )
)]
#![deny(unsafe_code)]

pub(crate) mod cache;
pub mod contract;
mod error;
pub mod mcp;
pub(crate) mod query;
pub(crate) mod rustdoc;
pub(crate) mod search;
pub mod service;

pub(crate) use error::{Error, Result};
