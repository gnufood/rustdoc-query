use super::{connection, insert_item};
use crate::search::fts::candidate::{retrieve, MAX_CANDIDATES};
use crate::search::fts::index::{fingerprint, IndexRow};

#[test]
fn exact_proc_attribute_filter_does_not_match_bang_macros() {
    let conn = connection();
    for (path, kind) in [("tokio::main", "proc_attribute"), ("tokio::join", "macro")] {
        insert_item(&conn, path, kind, "runtime entrypoint");
    }
    let rows = retrieve(
        &conn,
        &["entrypoint".to_owned()],
        "entrypoint",
        Some("proc_attribute"),
        Some("tokio::"),
    )
    .expect("exact proc-attribute candidates");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].path, "tokio::main");
    assert_eq!(rows[0].kind, rustdoc_types::ItemKind::ProcAttribute);
}

#[test]
fn query_words_colliding_with_fts5_operators_are_matched_as_text() {
    let conn = connection();
    insert_item(
        &conn,
        "demo::split",
        "function",
        "Splits a value and returns both halves.",
    );

    let terms = ["and", "or", "not", "near", "split"]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let rows = retrieve(&conn, &terms, "andornotsplit", None, None)
        .expect("operator-like terms must not be parsed as FTS5 syntax");

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].path, "demo::split");
}

#[test]
fn index_fingerprint_changes_with_indexed_surface_content() {
    let row = |docs: &str| {
        vec![IndexRow {
            path: "demo::Widget".to_owned(),
            kind: "struct",
            name: "Widget".to_owned(),
            inherent_method: false,
            docs: docs.to_owned(),
            sig: String::new(),
        }]
    };
    assert_ne!(
        fingerprint(&row("docs")),
        fingerprint(&row("expanded docs"))
    );
}

#[test]
fn retrieval_caps_the_total_deduplicated_candidate_set() {
    let conn = connection();
    let terms: Vec<_> = (0..16).map(|number| format!("term{number}")).collect();
    for (term_number, term) in terms.iter().enumerate() {
        for row_number in 0..16 {
            insert_item(
                &conn,
                &format!("demo::Item{term_number}_{row_number}"),
                "struct",
                term,
            );
        }
    }

    let rows = retrieve(&conn, &terms, "unmatched", None, None)
        .expect("candidate retrieval stays bounded");

    assert_eq!(rows.len(), MAX_CANDIDATES);
    assert!(rows.len() <= MAX_CANDIDATES);
}
