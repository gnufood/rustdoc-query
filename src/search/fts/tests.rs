use rustdoc_fts5_tokenizer::register_rust_identifier_tokenizer;
use tokio_rusqlite::rusqlite;

use super::schema::CREATE_TABLES;

mod indexing;
mod rank;

/// Build a fixture index from the production DDL.
fn connection() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().expect("in-memory SQLite");
    register_rust_identifier_tokenizer(&conn).expect("register identifier tokenizer");
    conn.execute_batch(CREATE_TABLES).expect("FTS tables");
    conn
}

/// Index one item, mirroring the production write across both tables.
fn insert_item(conn: &rusqlite::Connection, path: &str, kind: &str, docs: &str) {
    let name = path.rsplit("::").next().unwrap_or(path);
    conn.execute(
        "INSERT INTO items(path, kind, docs, sig) VALUES (?1, ?2, ?3, '')",
        rusqlite::params![path, kind, docs],
    )
    .expect("insert FTS row");
    conn.execute(
        "INSERT INTO items_name(rowid, name) VALUES (?1, ?2)",
        rusqlite::params![conn.last_insert_rowid(), name],
    )
    .expect("insert name row");
}
