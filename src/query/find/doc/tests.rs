use super::paginate;
use super::query::analyze;

#[test]
fn analysis_removes_stopwords_and_preserves_identifier_phrase() {
    let query = analyze("websocket upgrade with a handler");

    assert_eq!(query.terms, ["websocket", "upgrade", "handler"]);
    assert_eq!(query.joined, "websocketupgradehandler");
}

#[test]
fn punctuation_only_query_has_no_search_terms() {
    let query = analyze("--- :: <> !!!");

    assert!(query.terms.is_empty());
    assert!(query.joined.is_empty());
}

#[test]
fn analysis_caps_meaningful_terms_in_input_order() {
    let query = analyze("one two three four five six seven eight nine ten");

    assert_eq!(
        query.terms,
        ["one", "two", "three", "four", "five", "six", "seven", "eight"]
    );
    assert_eq!(query.joined, "onetwothreefourfivesixseveneight");
}

#[test]
fn pagination_is_gap_free_and_has_no_duplicate_boundary_row() {
    let rows = vec!["a", "b", "c", "d", "e"];
    let (first, first_has_more, first_next) = paginate(rows.clone(), None, 2);
    let (second, second_has_more, second_next) = paginate(rows, first_next, 2);

    assert_eq!(first, ["a", "b"]);
    assert!(first_has_more);
    assert_eq!(first_next, Some(2));
    assert_eq!(second, ["c", "d"]);
    assert!(second_has_more);
    assert_eq!(second_next, Some(4));
}
