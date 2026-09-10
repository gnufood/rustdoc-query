//! Safe registration of the Rust-identifier tokenizer for `SQLite` FTS5.
//!
//! Its crate-visible API contains no raw `SQLite` pointers.

mod identifier;
mod sqlite_ffi;

pub(crate) use sqlite_ffi::register_rust_identifier_tokenizer;

/// Normalize text into the same query terms used by the FTS tokenizer.
#[must_use]
pub(crate) fn query_terms(text: &str) -> Vec<String> {
    identifier::tokenize_query(text)
        .into_iter()
        .map(|token| token.term)
        .collect()
}

/// Normalize indexed text into FTS5 terms, including identifier parts and joins.
///
/// This intentionally differs from the narrower query tokenizer.
#[must_use]
pub(crate) fn document_terms(text: &str) -> Vec<String> {
    identifier::tokenize_document(text)
        .into_iter()
        .map(|token| token.term)
        .collect()
}

#[cfg(test)]
mod tests;
