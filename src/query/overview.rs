//! Builds a low-token crate orientation view from [`CrateEntry`].
//!
//! Includes summary docs, headline items, and per-kind counts; detailed item
//! data is provided by `get_item`.

use rustdoc_types::ItemKind;

use crate::cache::CrateEntry;
use crate::rustdoc::{first_paragraph, render_fn_sig, summary_line};

mod model;
pub(crate) use model::{KindCount, Overview, OverviewItem, Totals};

/// Build the orientation view for a public module, or the crate root if `module` is `None`.
///
/// Returns `None` if `module` is not publicly reachable.
pub(crate) fn build_overview(
    entry: &CrateEntry,
    module: Option<&str>,
    full_docs: bool,
) -> Option<Overview> {
    let krate = &entry.krate;
    let surface = &entry.surface;

    let module_path: Vec<String> = if let Some(m) = module {
        let module = surface.get_by_path(m)?;
        if module.kind != ItemKind::Module {
            return None;
        }
        module.path.clone()
    } else {
        let root_name = krate.index.get(&krate.root)?.name.clone()?;
        vec![root_name]
    };

    let module_entry = surface.get_by_path(&module_path.join("::"))?;
    let module_docs = krate
        .index
        .get(&module_entry.id)
        .and_then(|i| i.docs.as_deref());
    let (docs, docs_truncated) = if full_docs {
        (
            module_docs.map(ToOwned::to_owned).filter(|s| !s.is_empty()),
            false,
        )
    } else {
        first_paragraph(module_docs)
    };

    let mut items = Vec::new();

    let depth = module_path.len();
    for child in surface.iter() {
        // Direct children only: one segment deeper, sharing the module prefix.
        if child.path.len() != depth + 1 || child.path[..depth] != module_path[..] {
            continue;
        }
        items.push(item(entry, child));
    }
    items.sort_by(|a, b| a.path.cmp(&b.path));

    Some(Overview {
        path: module_path.join("::"),
        version: entry.krate.crate_version.clone(),
        docs,
        docs_truncated,
        totals: compute_totals(entry, &module_path),
        items,
    })
}

fn item(ce: &CrateEntry, entry: &crate::rustdoc::SurfaceEntry) -> OverviewItem {
    let krate = ce.krate_for(entry);
    let raw = krate.index.get(&entry.id);
    let summary = raw
        .and_then(|i| summary_line(i.docs.as_deref()))
        .unwrap_or_default();
    let deprecated = raw.is_some_and(|i| i.deprecation.is_some());
    // Re-export: the public path differs from rustdoc's private definition path.
    let reexport = ce
        .krate
        .paths
        .get(&entry.id)
        .is_some_and(|s| s.path != entry.path);

    let signature = if entry.kind == ItemKind::Function {
        raw.and_then(|i| {
            let name = entry.path.last().map_or("", String::as_str);
            if let rustdoc_types::ItemEnum::Function(f) = &i.inner {
                Some(render_fn_sig(krate, name, f))
            } else {
                None
            }
        })
    } else {
        None
    };

    OverviewItem {
        name: entry.path.last().cloned().unwrap_or_default(),
        path: entry.path.join("::"),
        kind: entry.kind,
        summary,
        signature,
        deprecated,
        reexport,
    }
}

fn compute_totals(entry: &CrateEntry, module_path: &[String]) -> Totals {
    let depth = module_path.len();
    let mut direct = Vec::new();
    let mut subtree = Vec::new();
    for e in entry.surface.iter() {
        if e.path.len() <= depth || e.path[..depth] != *module_path {
            continue;
        }
        increment(&mut subtree, e.kind);
        if e.path.len() == depth + 1 {
            increment(&mut direct, e.kind);
        }
    }
    Totals { direct, subtree }
}

fn increment(counts: &mut Vec<KindCount>, kind: ItemKind) {
    if let Some(entry) = counts.iter_mut().find(|entry| entry.kind == kind) {
        entry.count += 1;
    } else {
        counts.push(KindCount { kind, count: 1 });
    }
}

#[cfg(test)]
#[path = "overview/tests.rs"]
mod tests;
