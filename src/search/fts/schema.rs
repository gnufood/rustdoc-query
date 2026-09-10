//! The DDL here is the single definition of the index shape. Tests build their
//! fixtures from it so they exercise the same tokenizer and column layout as
//! production.

use tokio_rusqlite::rusqlite;

use super::index::IndexRow;
use super::rank::RankFeatures;

/// Bumped whenever the DDL or the tokenizer's output changes, invalidating
/// existing sidecars whose stored tokens no longer match what a query produces.
pub(super) const FTS_SCHEMA_VERSION: i64 = 2;

/// `porter rust_ident` splits Rust identifiers before stemming.
///
/// Names use a separate FTS table to avoid BM25 length dilution from documentation.
pub(super) const CREATE_TABLES: &str = "CREATE VIRTUAL TABLE items USING fts5(\
        path UNINDEXED, \
        kind UNINDEXED, \
        docs, \
        sig, \
        tokenize = 'porter rust_ident'\
    );
    CREATE VIRTUAL TABLE items_name USING fts5(\
        name, \
        tokenize = 'porter rust_ident'\
    );
    CREATE VIRTUAL TABLE items_method_name USING fts5(\
        name, \
        tokenize = 'porter rust_ident'\
    );
    CREATE TABLE fts_metadata(\
        schema_version INTEGER NOT NULL, \
        fingerprint TEXT NOT NULL\
    );
    CREATE TABLE rank_terms(\
        term_id INTEGER PRIMARY KEY, \
        term TEXT NOT NULL UNIQUE\
    );
    CREATE TABLE rank_features(\
        rowid INTEGER PRIMARY KEY, \
        whole_name TEXT NOT NULL, \
        identifier_terms BLOB NOT NULL, \
        name_terms BLOB NOT NULL, \
        documentation_terms BLOB NOT NULL\
    );
    INSERT INTO items(items, rank) VALUES('rank', 'bm25(0.0, 0.0, 1.0, 3.0)');";

const DROP_TABLES: &str = "DROP TABLE IF EXISTS items;
     DROP TABLE IF EXISTS items_name;
     DROP TABLE IF EXISTS items_method_name;
     DROP TABLE IF EXISTS fts_metadata;
     DROP TABLE IF EXISTS rank_terms;
     DROP TABLE IF EXISTS rank_features;";

/// Build and populate the index unless the sidecar already matches
/// `fingerprint` at the current schema version.
pub(super) fn ensure_built(
    conn: &mut rusqlite::Connection,
    rows: &[IndexRow],
    fingerprint: &str,
) -> rusqlite::Result<RankFeatures> {
    if is_current(conn, fingerprint) {
        return RankFeatures::load(conn);
    }

    conn.execute_batch(DROP_TABLES)?;
    conn.execute_batch(CREATE_TABLES)?;

    let rank_features = RankFeatures::build(rows);
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT INTO items(rowid, path, kind, docs, sig) VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        let mut stmt_name = tx.prepare("INSERT INTO items_name(rowid, name) VALUES (?1,?2)")?;
        let mut stmt_method_name =
            tx.prepare("INSERT INTO items_method_name(rowid, name) VALUES (?1,?2)")?;
        for (index, row) in rows.iter().enumerate() {
            let rowid = i64::try_from(index + 1).unwrap_or(i64::MAX);
            stmt.execute(rusqlite::params![
                rowid, row.path, row.kind, row.docs, row.sig
            ])?;
            stmt_name.execute(rusqlite::params![rowid, row.name])?;
            if row.inherent_method {
                stmt_method_name.execute(rusqlite::params![rowid, row.name])?;
            }
        }
        rank_features.persist(&tx)?;
        tx.execute(
            "INSERT INTO fts_metadata(schema_version, fingerprint) VALUES (?1, ?2)",
            rusqlite::params![FTS_SCHEMA_VERSION, fingerprint],
        )?;
    }
    tx.commit()?;
    Ok(rank_features)
}

fn is_current(conn: &rusqlite::Connection, fingerprint: &str) -> bool {
    conn.query_row(
        "SELECT schema_version, fingerprint FROM fts_metadata LIMIT 1",
        [],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
    )
    .is_ok_and(|(version, stored)| version == FTS_SCHEMA_VERSION && stored == fingerprint)
}
