use rustdoc_fts5_tokenizer::query_terms;

const STOPWORDS: &[&str] = &[
    "a", "an", "and", "for", "in", "of", "on", "or", "the", "to", "with",
];
const MAX_QUERY_TERMS: usize = 8;

pub(super) struct AnalyzedQuery {
    pub(super) terms: Vec<String>,
    pub(super) joined: String,
}

pub(super) fn analyze(input: &str) -> AnalyzedQuery {
    let terms: Vec<_> = query_terms(input)
        .into_iter()
        .filter(|term| !STOPWORDS.contains(&term.as_str()))
        .take(MAX_QUERY_TERMS)
        .collect();
    let joined = terms.concat();
    AnalyzedQuery { terms, joined }
}
