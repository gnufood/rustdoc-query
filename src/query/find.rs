use std::collections::BTreeSet;

use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};
use nucleo_matcher::{Config, Matcher, Utf32Str};
use rustdoc_types::ItemKind;

use crate::cache::CrateEntry;
use crate::rustdoc::{summary_line, SurfaceEntry};
mod doc;
mod model;
pub(crate) use model::{DocumentPage, FindResult, FindRow, SortKey};

/// When `query` is set, results are fuzzy-ranked by score (highest first) and
/// non-matching entries are excluded. Without a query, results are sorted
/// alphabetically by path. Operates on in-RAM data only — no I/O.
#[must_use]
pub(crate) fn find_items(
    entry: &CrateEntry,
    query: Option<&str>,
    kind_filter: Option<ItemKind>,
    module: Option<&str>,
    with_summary: bool,
) -> FindResult {
    let module_scope = module.map(|path| {
        entry
            .surface
            .get_by_path(path)
            .filter(|entry| entry.kind == ItemKind::Module)
            .map(|entry| entry.path.join("::"))
    });
    // Use the secondary kind index when available to avoid scanning unrelated kinds.
    let filtered: Vec<_> = match kind_filter {
        Some(k) => entry.surface.iter_by_kind(k).collect::<Vec<_>>(),
        None => entry.surface.iter().collect::<Vec<_>>(),
    }
    .into_iter()
    .filter(|e| {
        // Strict descendant check: require the `module::` prefix so the module
        // itself is excluded and names like `tokio::sync_extra` don't match `tokio::sync`.
        module_scope.as_ref().is_none_or(|scope| {
            scope
                .as_ref()
                .is_some_and(|path| e.path.join("::").starts_with(&format!("{path}::")))
        })
    })
    .collect();

    let (all, order): (Vec<FindRow>, Vec<SortKey>) = if let Some(q) = query {
        if q.is_empty() {
            let mut out: Vec<_> = filtered
                .into_iter()
                .map(|e| make_row(entry, e, with_summary))
                .collect();
            out.sort_unstable_by(|a, b| a.path.cmp(&b.path));
            let order = out
                .iter()
                .map(|row| SortKey::Alphabetical {
                    path: row.path.clone(),
                })
                .collect();
            (out, order)
        } else {
            let atom = Atom::new(
                q,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Fuzzy,
                false,
            );
            let mut matcher = Matcher::new(Config::DEFAULT);
            let mut buf = Vec::new();
            let mut scored: Vec<_> = filtered
                .into_iter()
                .filter_map(|e| {
                    let name = e.path.last().map_or("", String::as_str);
                    let score = atom.score(Utf32Str::new(name, &mut buf), &mut matcher)?;
                    Some((e, score))
                })
                .collect();
            scored.sort_unstable_by(|(left, left_score), (right, right_score)| {
                right_score
                    .cmp(left_score)
                    .then_with(|| left.path.cmp(&right.path))
            });
            let mut rows = Vec::with_capacity(scored.len());
            let mut order = Vec::with_capacity(scored.len());
            for (surface_entry, score) in scored {
                let row = make_row(entry, surface_entry, with_summary);
                order.push(SortKey::Fuzzy {
                    score: i64::from(score),
                    path: row.path.clone(),
                });
                rows.push(row);
            }
            (rows, order)
        }
    } else {
        let mut out: Vec<_> = filtered
            .into_iter()
            .map(|e| make_row(entry, e, with_summary))
            .collect();
        out.sort_unstable_by(|a, b| a.path.cmp(&b.path));
        let order = out
            .iter()
            .map(|row| SortKey::Alphabetical {
                path: row.path.clone(),
            })
            .collect();
        (out, order)
    };

    let hint = all.is_empty().then(|| {
        zero_result_hint(
            entry,
            query,
            kind_filter,
            module_scope.as_ref().and_then(|scope| scope.as_deref()),
            module.is_some(),
        )
    });
    FindResult {
        rows: all,
        hint,
        order,
    }
}

fn zero_result_hint(
    entry: &CrateEntry,
    query: Option<&str>,
    kind_filter: Option<ItemKind>,
    module_scope: Option<&str>,
    has_module_filter: bool,
) -> String {
    kind_mismatch_hint(entry, query, kind_filter, module_scope, has_module_filter)
        .or_else(|| expansion_failure_hint(entry, query, kind_filter, module_scope))
        .unwrap_or_else(|| {
            "No matches in this crate's directly indexed surface. Items re-exported from dependency \
             crates may not appear here — try `get_item` with the known re-export path (e.g. the \
             crate-root path), or check the dep crate's own overview."
                .to_owned()
        })
}

/// Explain a zero result when name matching succeeded but the requested kind
/// removed every candidate. This fallback runs only after the indexed query
/// has already produced no rows, so normal search keeps its kind-index fast
/// path.
fn kind_mismatch_hint(
    entry: &CrateEntry,
    query: Option<&str>,
    kind_filter: Option<ItemKind>,
    module_scope: Option<&str>,
    has_module_filter: bool,
) -> Option<String> {
    let requested_kind = kind_filter?;
    // An unknown module has no resolved scope; do not scan the whole crate and
    // accidentally attribute that separate mismatch to the kind filter.
    if has_module_filter && module_scope.is_none() {
        return None;
    }
    let actual_kinds = entry
        .surface
        .iter()
        .filter(|surface| matches_module_scope(surface, module_scope))
        .filter(|surface| path_matches_query(&surface.path, query))
        .map(|surface| surface.kind)
        .filter(|&kind| kind != requested_kind)
        .map(crate::rustdoc::label)
        .collect::<BTreeSet<_>>();
    (!actual_kinds.is_empty()).then(|| {
        format!(
            "Name matches exist, but none have kind `{}`; matching kinds: {}. Try one of those \
             kinds or omit `kind`.",
            crate::rustdoc::label(requested_kind),
            actual_kinds.into_iter().collect::<Vec<_>>().join(", ")
        )
    })
}

fn expansion_failure_hint(
    entry: &CrateEntry,
    query: Option<&str>,
    kind_filter: Option<ItemKind>,
    module_scope: Option<&str>,
) -> Option<String> {
    let failure = entry.surface.expansion_failures().find(|failure| {
        kind_filter.is_none_or(|kind| kind == failure.kind)
            && module_scope.is_none_or(|scope| {
                failure
                    .local_path
                    .join("::")
                    .starts_with(&format!("{scope}::"))
            })
            && path_matches_query(&failure.local_path, query)
    })?;
    let path = failure.local_path.join("::");
    let dependency = failure
        .dependency
        .as_deref()
        .unwrap_or("an unknown dependency");
    Some(format!(
        "No match found: public re-export `{path}` from dependency `{dependency}` could not be \
         expanded because {}. Check that dependency's documentation or query its own crate.",
        failure.reason.description()
    ))
}

fn matches_module_scope(surface: &SurfaceEntry, module_scope: Option<&str>) -> bool {
    module_scope.is_none_or(|scope| surface.path.join("::").starts_with(&format!("{scope}::")))
}

fn path_matches_query(path: &[String], query: Option<&str>) -> bool {
    let Some(query) = query.filter(|query| !query.is_empty()) else {
        return true;
    };
    let Some(name) = path.last() else {
        return false;
    };
    let atom = Atom::new(
        query,
        CaseMatching::Smart,
        Normalization::Smart,
        AtomKind::Fuzzy,
        false,
    );
    let mut matcher = Matcher::new(Config::DEFAULT);
    let mut buf = Vec::new();
    atom.score(Utf32Str::new(name, &mut buf), &mut matcher)
        .is_some()
}

pub(crate) async fn find_document_items(
    entry: &CrateEntry,
    query: &str,
    kind_filter: Option<&str>,
    module: Option<&str>,
    offset: Option<usize>,
    page_size: usize,
    with_summary: bool,
) -> crate::Result<DocumentPage> {
    doc::search(
        entry,
        query,
        kind_filter,
        module,
        offset,
        page_size,
        with_summary,
    )
    .await
}

fn make_row(entry: &CrateEntry, e: &SurfaceEntry, with_summary: bool) -> FindRow {
    let path = e.path.join("::");
    let summary = with_summary
        .then(|| {
            entry
                .krate_for(e)
                .index
                .get(&e.id)
                .and_then(|i| summary_line(i.docs.as_deref()))
        })
        .flatten();
    FindRow {
        path,
        kind: e.kind,
        summary,
    }
}

#[cfg(test)]
#[path = "find/tests.rs"]
mod tests;
