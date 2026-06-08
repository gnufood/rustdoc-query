use rusqlite::Connection;

use crate::register_rust_identifier_tokenizer;

#[test]
fn registered_tokenizer_preserves_identifier_matching_and_snippet_offsets() {
    let connection = Connection::open_in_memory().expect("in-memory SQLite opens");
    register_rust_identifier_tokenizer(&connection).expect("tokenizer registers");
    connection
        .execute_batch(
            "CREATE VIRTUAL TABLE items USING fts5(docs, tokenize = 'porter rust_ident');
             INSERT INTO items(docs) VALUES
                ('A ClientBuilder constructs clients.'),
                ('XMLParser handles markup.'),
                ('A snake_case helper.');",
        )
        .expect("FTS table and fixture rows are valid");

    for query in [
        "ClientBuilder",
        "client",
        "builder",
        "clients",
        "xml",
        "parser",
        "snake",
        "case",
    ] {
        let count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM items WHERE items MATCH ?1",
                [query],
                |row| row.get(0),
            )
            .expect("identifier query succeeds");
        assert_eq!(count, 1, "{query} should match exactly one fixture row");
    }

    let snippet: String = connection
        .query_row(
            "SELECT snippet(items, 0, '[', ']', '…', 12) \
             FROM items WHERE items MATCH 'client'",
            [],
            |row| row.get(0),
        )
        .expect("component query returns a snippet");
    assert!(snippet.contains("[ClientBuilder]"));
}
