//! `SQLite` FTS5 sidecar.
//!
//! Tokenizer FFI is isolated in the tokenizer module; this module owns safe
//! database lifecycle and delegates index shape, match construction, and
//! paginated reads to its children.

use std::path::Path;

use rustdoc_types::{Crate, ItemKind};
use tokio_rusqlite::rusqlite;

use crate::rustdoc::Surface;

mod candidate;
mod index;
mod query;
mod rank;
mod schema;
#[cfg(test)]
mod tests;
pub(crate) mod tokenizer;

/// Relevance remains the responsibility of the query layer, which has the
/// crate's public-surface and rustdoc context.
#[derive(Debug)]
pub(crate) struct SearchCandidate {
    pub(crate) rowid: i64,
    pub(crate) path: String,
    pub(crate) kind: ItemKind,
}

/// `SQLite` FTS5 connection for a single crate, valid for its lifetime in the
/// in-memory cache.
#[derive(Debug)]
pub(crate) struct FtsDb {
    conn: tokio_rusqlite::Connection,
    rank_features: rank::RankFeatures,
}

impl FtsDb {
    /// Must be called **after** `apply_pending_expansions` so dep-expanded
    /// items are included in the index.
    pub(crate) async fn open_or_build(
        path: &Path,
        krate: &Crate,
        surface: &Surface,
        dep_crates: &[Crate],
    ) -> crate::Result<Self> {
        let rows = index::collect_rows(krate, surface, dep_crates);
        let fingerprint = index::fingerprint(&rows);

        let conn = tokio_rusqlite::Connection::open(path)
            .await
            .map_err(|e| crate::Error::Fts(e.to_string()))?;

        let rank_features = conn
            .call(move |conn| -> rusqlite::Result<rank::RankFeatures> {
                // Register on every open: FTS5 needs the tokenizer for both cold
                // indexing and warm queries.
                tokenizer::register_rust_identifier_tokenizer(conn)?;
                schema::ensure_built(conn, &rows, &fingerprint)
            })
            .await
            .map_err(|e| crate::Error::Fts(e.to_string()))?;

        Ok(Self {
            conn,
            rank_features,
        })
    }

    #[cfg(test)]
    pub(crate) fn dummy() -> Self {
        let raw = rusqlite::Connection::open_in_memory().expect("in-memory SQLite for test stub");
        Self {
            conn: tokio_rusqlite::Connection::from(raw),
            rank_features: rank::RankFeatures::empty(),
        }
    }

    /// This deliberately discards `SQLite`'s BM25 ordering. BM25 is useful for
    /// recall, but the application query owns the user-facing relevance order.
    pub(crate) async fn search_candidates(
        &self,
        terms: &[String],
        joined_identifier: &str,
        kind_filter: Option<&str>,
        module: Option<&str>,
    ) -> crate::Result<Vec<SearchCandidate>> {
        if terms.is_empty() {
            return Ok(Vec::new());
        }
        let terms = terms.to_vec();
        let joined_identifier = joined_identifier.to_owned();
        let kind = kind_filter.map(str::to_owned);
        let module_prefix = module.map(|value| format!("{value}::"));
        self.conn
            .call(move |conn| {
                candidate::retrieve(
                    conn,
                    &terms,
                    &joined_identifier,
                    kind.as_deref(),
                    module_prefix.as_deref(),
                )
            })
            .await
            .map_err(|error| crate::Error::Fts(error.to_string()))
    }

    pub(crate) fn exact_identifier(&self, rowid: i64, joined: &str) -> bool {
        self.rank_features.exact_identifier(rowid, joined)
    }

    pub(crate) fn name_coverage(&self, rowid: i64, terms: &[String]) -> u16 {
        self.rank_features.name_coverage(rowid, terms)
    }

    pub(crate) fn identifier_coverage(&self, rowid: i64, terms: &[String]) -> u16 {
        self.rank_features.identifier_coverage(rowid, terms)
    }

    pub(crate) fn documentation_coverage(&self, rowid: i64, terms: &[String]) -> u16 {
        self.rank_features.documentation_coverage(rowid, terms)
    }
}
