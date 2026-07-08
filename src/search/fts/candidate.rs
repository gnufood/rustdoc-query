//! This layer owns only recall. The query domain evaluates candidate
//! applicability and orders the final response.

use std::collections::BTreeMap;

use tokio_rusqlite::rusqlite;

use super::query;
use super::SearchCandidate;
use crate::rustdoc::parse_label;

const EXACT_IDENTIFIER_LIMIT: i64 = 16;
const PRECISE_LIMIT: i64 = 48;
const RELAXED_PER_TERM_LIMIT: i64 = 16;
pub(super) const MAX_CANDIDATES: usize = 128;

pub(super) fn retrieve(
    conn: &rusqlite::Connection,
    terms: &[String],
    joined_identifier: &str,
    kind: Option<&str>,
    module_prefix: Option<&str>,
) -> rusqlite::Result<Vec<SearchCandidate>> {
    let mut candidates = BTreeMap::new();
    if joined_identifier.chars().count() >= 3 {
        extend(
            &mut candidates,
            name_matches(
                conn,
                &query::term_expression(joined_identifier),
                kind,
                module_prefix,
                EXACT_IDENTIFIER_LIMIT,
            )?,
        );
    }
    extend(
        &mut candidates,
        document_matches(
            conn,
            &query::all_expression(terms),
            kind,
            module_prefix,
            PRECISE_LIMIT,
        )?,
    );
    // A broad OR query must score every matching row before SQLite can order
    // it. Query bounded postings per term instead: this makes recall explicit
    // and keeps the amount of downstream ranking work bounded by the query
    // length rather than the crate's vocabulary fan-out.
    extend(
        &mut candidates,
        method_name_matches(
            conn,
            &query::any_expression(terms),
            kind,
            module_prefix,
            RELAXED_PER_TERM_LIMIT,
        )?,
    );
    for term in terms {
        extend(
            &mut candidates,
            document_matches(
                conn,
                &query::term_expression(term),
                kind,
                module_prefix,
                RELAXED_PER_TERM_LIMIT,
            )?,
        );
    }
    Ok(candidates.into_values().collect())
}

fn extend(
    target: &mut BTreeMap<String, SearchCandidate>,
    rows: impl IntoIterator<Item = SearchCandidate>,
) {
    for row in rows {
        if target.len() == MAX_CANDIDATES {
            break;
        }
        target.entry(row.path.clone()).or_insert(row);
    }
}

fn name_matches(
    conn: &rusqlite::Connection,
    expression: &str,
    kind: Option<&str>,
    module_prefix: Option<&str>,
    limit: i64,
) -> rusqlite::Result<Vec<SearchCandidate>> {
    let mut stmt = conn.prepare(
        "SELECT items.rowid, items.path, items.kind
         FROM items_name JOIN items ON items.rowid = items_name.rowid
         WHERE items_name MATCH ?1
           AND (?2 IS NULL OR items.kind = ?2)
           AND (?3 IS NULL OR substr(items.path, 1, length(?3)) = ?3)
         LIMIT ?4",
    )?;
    let rows = collect(stmt.query_map(
        rusqlite::params![expression, kind, module_prefix, limit],
        decode,
    )?);
    rows
}

fn method_name_matches(
    conn: &rusqlite::Connection,
    expression: &str,
    kind: Option<&str>,
    module_prefix: Option<&str>,
    limit: i64,
) -> rusqlite::Result<Vec<SearchCandidate>> {
    let mut stmt = conn.prepare(
        "SELECT items.rowid, items.path, items.kind
         FROM items_method_name JOIN items ON items.rowid = items_method_name.rowid
         WHERE items_method_name MATCH ?1
           AND (?2 IS NULL OR items.kind = ?2)
           AND (?3 IS NULL OR substr(items.path, 1, length(?3)) = ?3)
         LIMIT ?4",
    )?;
    let rows = collect(stmt.query_map(
        rusqlite::params![expression, kind, module_prefix, limit],
        decode,
    )?);
    rows
}

fn document_matches(
    conn: &rusqlite::Connection,
    expression: &str,
    kind: Option<&str>,
    module_prefix: Option<&str>,
    limit: i64,
) -> rusqlite::Result<Vec<SearchCandidate>> {
    let mut stmt = conn.prepare(
        "SELECT rowid, path, kind FROM items
         WHERE items MATCH ?1
           AND (?2 IS NULL OR kind = ?2)
           AND (?3 IS NULL OR substr(path, 1, length(?3)) = ?3)
         LIMIT ?4",
    )?;
    let rows = collect(stmt.query_map(
        rusqlite::params![expression, kind, module_prefix, limit],
        decode,
    )?);
    rows
}

fn collect(
    rows: impl Iterator<Item = rusqlite::Result<SearchCandidate>>,
) -> rusqlite::Result<Vec<SearchCandidate>> {
    rows.collect()
}

fn decode(row: &rusqlite::Row<'_>) -> rusqlite::Result<SearchCandidate> {
    let kind = row.get::<_, String>(2)?;
    let kind = parse_label(&kind).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            2,
            rusqlite::types::Type::Text,
            format!("unknown exact rustdoc item kind `{kind}").into(),
        )
    })?;
    Ok(SearchCandidate {
        rowid: row.get(0)?,
        path: row.get(1)?,
        kind,
    })
}
