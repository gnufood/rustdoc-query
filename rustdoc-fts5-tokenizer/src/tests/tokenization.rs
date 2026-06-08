use crate::identifier::{tokenize_document, tokenize_query, Token};

fn terms(tokens: Vec<Token>) -> Vec<(String, bool)> {
    tokens
        .into_iter()
        .map(|token| (token.term, token.colocated))
        .collect()
}

#[test]
fn document_tokenization_keeps_identifier_parts_colocated() {
    assert_eq!(
        terms(tokenize_document("ClientBuilder")),
        [
            ("clientbuilder".into(), false),
            ("client".into(), true),
            ("builder".into(), true)
        ]
    );
}

#[test]
fn query_tokenization_does_not_expand_identifier_parts() {
    assert_eq!(
        terms(tokenize_query("ClientBuilder")),
        [("clientbuilder".into(), false)]
    );
}

#[test]
fn document_tokenization_joins_adjacent_identifier_parts() {
    let indexed = terms(tokenize_document("WebSocketUpgrade"))
        .into_iter()
        .map(|(term, _)| term)
        .collect::<Vec<_>>();

    assert!(
        indexed.contains(&"websocket".to_owned()),
        "adjacent parts must be searchable as one word, got {indexed:?}"
    );
    assert!(
        indexed.contains(&"socketupgrade".to_owned()),
        "every adjacent pair must be joined, got {indexed:?}"
    );
}
