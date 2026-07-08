use rustdoc_fts5_tokenizer::register_rust_identifier_tokenizer;
use tokio_rusqlite::rusqlite;

use crate::search::fts::index::IndexRow;
use crate::search::fts::schema;

fn rows() -> Vec<IndexRow> {
    vec![IndexRow {
        path: "demo::Client".to_owned(),
        kind: "struct",
        name: "Client".to_owned(),
        inherent_method: false,
        docs: "Sends an HTTP request with a timeout.".to_owned(),
        sig: String::new(),
    }]
}

#[test]
fn rank_features_are_built_and_reloaded_with_the_fts_sidecar() {
    let mut conn = rusqlite::Connection::open_in_memory().expect("in-memory SQLite");
    register_rust_identifier_tokenizer(&conn).expect("register identifier tokenizer");
    let rows = rows();

    let built = schema::ensure_built(&mut conn, &rows, "fixture-fingerprint")
        .expect("build index and rank features");
    assert!(built.exact_identifier(1, "client"));
    assert_eq!(
        built.documentation_coverage(
            1,
            &[
                "send".to_owned(),
                "request".to_owned(),
                "timeout".to_owned()
            ]
        ),
        2
    );

    let loaded = schema::ensure_built(&mut conn, &rows, "fixture-fingerprint")
        .expect("load current index and rank features");
    assert!(loaded.exact_identifier(1, "client"));
    assert_eq!(
        loaded.documentation_coverage(
            1,
            &[
                "send".to_owned(),
                "request".to_owned(),
                "timeout".to_owned()
            ]
        ),
        2
    );
}
