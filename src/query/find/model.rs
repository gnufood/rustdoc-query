#[derive(Debug)]
pub(crate) struct FindRow {
    pub(crate) path: String,
    /// Exact compiler-reported rustdoc kind.
    pub(crate) kind: rustdoc_types::ItemKind,
    /// First paragraph of docs joined to one line, capped at 120 chars. Present only when `with_summary` was set.
    pub(crate) summary: Option<String>,
}

#[derive(Debug)]
pub(crate) struct FindResult {
    /// Every matched row, in ranked or alphabetical order. Callers page it.
    pub(crate) rows: Vec<FindRow>,
    /// Set only when no rows match.
    pub(crate) hint: Option<String>,
    /// Deterministic private sort keys aligned with `rows`.
    pub(crate) order: Vec<SortKey>,
}

#[derive(Clone, Debug)]
pub(crate) enum SortKey {
    Alphabetical { path: String },
    Fuzzy { score: i64, path: String },
}

/// Ranked privately by the query domain.
pub(crate) struct DocumentPage {
    pub(crate) rows: Vec<FindRow>,
    pub(crate) has_more: bool,
    pub(crate) next_offset: Option<usize>,
}
