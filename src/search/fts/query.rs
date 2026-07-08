//! Callers supply prose. FTS5 treats several ASCII characters as operators, so
//! nothing here may reach the database without passing through this module.

pub(super) fn all_expression(terms: &[String]) -> String {
    expression(terms, " AND ")
}

pub(super) fn any_expression(terms: &[String]) -> String {
    expression(terms, " OR ")
}

/// Safely match one already-normalized identifier form.
pub(super) fn term_expression(term: &str) -> String {
    format!("\"{term}\"")
}

fn expression(terms: &[String], separator: &str) -> String {
    terms
        .iter()
        .map(|term| term_expression(term))
        .collect::<Vec<_>>()
        .join(separator)
}
