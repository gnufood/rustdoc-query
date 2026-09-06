//! Query crate public APIs from rustdoc JSON.
//!
//! Exposes generated contract types, the service boundary, and MCP transport.
//!
//! ```compile_fail
//! use rustdoc_query::Error;
//! ```
//!
//! ```compile_fail
//! use rustdoc_query::error::Error;
//! ```

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

pub(crate) mod cache;
pub mod contract;
mod error;
pub mod mcp;
pub(crate) mod query;
pub(crate) mod rustdoc;
pub(crate) mod search;
pub mod service;

pub(crate) use error::{Error, Result};
