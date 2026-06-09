//! FTS supplies lexical candidates only. This module evaluates their public
//! identity and rustdoc content so applicability is not coupled to `SQLite`'s
//! corpus-dependent BM25 values.

use rustdoc_types::ItemKind;

use super::{DocumentPage, FindRow};
use crate::cache::CrateEntry;
use crate::rustdoc::{summary_line, SurfaceResolution};

mod query;

#[cfg(test)]
mod tests;

pub(super) async fn search(
    entry: &CrateEntry,
    query: &str,
    kind_filter: Option<&str>,
    module: Option<&str>,
    offset: Option<usize>,
    page_size: usize,
    with_summary: bool,
) -> crate::Result<DocumentPage> {
    let analyzed = query::analyze(query);
    if analyzed.terms.is_empty() {
        return Ok(DocumentPage {
            rows: Vec::new(),
            has_more: false,
            next_offset: None,
        });
    }
    let candidates = entry
        .fts
        .search_candidates(&analyzed.terms, &analyzed.joined, kind_filter, module)
        .await?;
    let query_identifier = analyzed.joined;
    let mut ranked: Vec<_> = candidates
        .into_iter()
        .filter_map(|candidate| {
            let surface = entry.surface.get_by_path(&candidate.path)?;
            Some(Ranked {
                row: FindRow {
                    path: candidate.path,
                    kind: candidate.kind,
                    summary: None,
                },
                exact_name: entry
                    .fts
                    .exact_identifier(candidate.rowid, &query_identifier),
                whole_name_matches: entry
                    .fts
                    .identifier_coverage(candidate.rowid, &analyzed.terms),
                name_coverage: entry.fts.name_coverage(candidate.rowid, &analyzed.terms),
                docs_coverage: entry
                    .fts
                    .documentation_coverage(candidate.rowid, &analyzed.terms),
                inherent_method: matches!(
                    surface.resolution,
                    SurfaceResolution::InherentMethod { .. }
                ),
            })
        })
        .collect();
    ranked.sort_unstable_by(|left, right| right.cmp(left));

    // The cursor's rowid is a one-based position in this deterministic private
    // ordering. Cursor context binds it to the normalized request and crate
    // version; it never exposes a storage rowid.
    let (selected, has_more, next_offset) = paginate(ranked, offset, page_size);
    Ok(DocumentPage {
        rows: selected
            .into_iter()
            .map(|mut ranked| {
                ranked.row.summary = with_summary
                    .then(|| {
                        entry
                            .surface
                            .get_by_path(&ranked.row.path)
                            .and_then(|surface| entry.krate_for(surface).index.get(&surface.id))
                            .and_then(|item| summary_line(item.docs.as_deref()))
                    })
                    .flatten();
                ranked.row
            })
            .collect(),
        has_more,
        next_offset,
    })
}

fn paginate<T>(
    items: Vec<T>,
    offset: Option<usize>,
    page_size: usize,
) -> (Vec<T>, bool, Option<usize>) {
    if page_size == 0 {
        return (Vec::new(), false, None);
    }
    let start = offset.unwrap_or(0);
    let mut selected: Vec<_> = items
        .into_iter()
        .skip(start)
        .take(page_size.saturating_add(1))
        .collect();
    let has_more = selected.len() > page_size;
    if has_more {
        selected.pop();
    }
    let end = start.saturating_add(selected.len());
    (selected, has_more, has_more.then_some(end))
}

struct Ranked {
    row: FindRow,
    exact_name: bool,
    whole_name_matches: u16,
    name_coverage: u16,
    docs_coverage: u16,
    inherent_method: bool,
}

impl Ord for Ranked {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.exact_name
            .cmp(&other.exact_name)
            .then_with(|| other.inherent_method.cmp(&self.inherent_method))
            .then_with(|| self.name_coverage.cmp(&other.name_coverage))
            .then_with(|| self.whole_name_matches.cmp(&other.whole_name_matches))
            .then_with(|| self.docs_coverage.cmp(&other.docs_coverage))
            .then_with(|| {
                (other.row.kind == ItemKind::Module).cmp(&(self.row.kind == ItemKind::Module))
            })
            .then_with(|| other.row.path.cmp(&self.row.path))
    }
}

impl PartialOrd for Ranked {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Ranked {
    fn eq(&self, other: &Self) -> bool {
        self.row.path == other.row.path
    }
}

impl Eq for Ranked {}
