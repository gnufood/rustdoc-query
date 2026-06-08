#[derive(Debug, serde::Serialize)]
pub(crate) struct Overview {
    pub(crate) path: String,
    /// Resolved crate version that was queried, e.g. `"1.0.0"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) version: Option<String>,
    /// Module docs — first paragraph by default, full text when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) docs: Option<String>,
    /// Whether `docs` was truncated (more paragraphs followed).
    pub(crate) docs_truncated: bool,
    /// Distinct from `items.len()` (this module's direct children only).
    pub(crate) totals: Totals,
    pub(crate) items: Vec<OverviewItem>,
}

/// Per-kind counts for one scope (direct children or subtree).
#[derive(Debug, serde::Serialize)]
pub(crate) struct KindCount {
    pub(crate) kind: rustdoc_types::ItemKind,
    pub(crate) count: usize,
}

/// Item totals split by scope: `direct` counts only this module's immediate
/// children; `subtree` counts all descendants at any depth.
#[derive(Debug, serde::Serialize)]
pub(crate) struct Totals {
    pub(crate) direct: Vec<KindCount>,
    pub(crate) subtree: Vec<KindCount>,
}

/// `summary` is always present (empty string when the item has no docs) so
/// groups stay uniform and render as compact TOON tables.
#[derive(Debug, serde::Serialize)]
pub(crate) struct OverviewItem {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) kind: rustdoc_types::ItemKind,
    pub(crate) summary: String,
    /// Present only for `ItemKind::Function` items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) signature: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) deprecated: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) reexport: bool,
}
