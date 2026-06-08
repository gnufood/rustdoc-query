//! Safe registration of the Rust-identifier tokenizer for `SQLite` FTS5.
//!
//! The public API contains no raw `SQLite` pointers.

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

mod identifier;
mod sqlite_ffi;

pub use sqlite_ffi::register_rust_identifier_tokenizer;

/// Normalize text into the same query terms used by the FTS tokenizer.
#[must_use]
pub fn query_terms(text: &str) -> Vec<String> {
    identifier::tokenize_query(text)
        .into_iter()
        .map(|token| token.term)
        .collect()
}

/// Normalize indexed text into FTS5 terms, including identifier parts and joins.
///
/// This intentionally differs from the narrower query tokenizer.
#[must_use]
pub fn document_terms(text: &str) -> Vec<String> {
    identifier::tokenize_document(text)
        .into_iter()
        .map(|token| token.term)
        .collect()
}

#[cfg(test)]
mod tests;
