use std::collections::{HashSet, VecDeque};

use rustdoc_types::{Crate, Id, ItemEnum, ItemKind, Visibility};
use semver::Version;
use url::Url;

use super::{
    DocsRsDependency, ExpansionFailureReason, PendingExpansion, PendingNamedItem, Surface,
};

/// Returns the surface and unresolved cross-crate expansions.
#[must_use]
pub(crate) fn build(krate: &Crate) -> (Surface, Vec<PendingExpansion>, Vec<PendingNamedItem>) {
    let mut surface = Surface::default();
    let mut pending: Vec<PendingExpansion> = Vec::new();
    let mut pending_named: Vec<PendingNamedItem> = Vec::new();

    let Some(root) = krate.index.get(&krate.root) else {
        return (surface, pending, pending_named);
    };
    let root_name = root.name.clone().unwrap_or_default();

    let mut visited: HashSet<Id> = HashSet::new();
    let mut queue: VecDeque<(Id, Vec<String>)> = VecDeque::new();
    queue.push_back((krate.root, vec![root_name]));

    while let Some((id, path)) = queue.pop_front() {
        if !visited.insert(id) {
            // Already recorded at a shorter path. Register this longer path
            // as an alias so callers can look up either direction.
            surface.record_alias(0, id, &path);
            continue;
        }
        let Some(item) = krate.index.get(&id) else {
            // External item: not in this crate's index. Use paths for metadata.
            if let Some(summary) = krate.paths.get(&id) {
                if summary.kind == ItemKind::Module {
                    // Record the module itself, then defer child expansion to
                    // the cache layer (needs a network fetch to resolve children).
                    surface.record(id, path.clone(), ItemKind::Module, 0);
                    match docs_rs_dependency(krate, summary.crate_id) {
                        Ok(dependency) => pending.push(PendingExpansion {
                            dependency,
                            dep_module_path: summary.path.clone(),
                            local_prefix: path,
                        }),
                        Err(reason) => surface.record_expansion_failure(
                            path,
                            summary.kind,
                            dependency_name(krate, summary.crate_id),
                            reason,
                        ),
                    }
                } else {
                    // External named re-export; defer resolution to the cache layer.
                    match docs_rs_dependency(krate, summary.crate_id) {
                        Ok(dependency) => pending_named.push(PendingNamedItem {
                            dependency,
                            dep_item_path: summary.path.clone(),
                            local_path: path,
                            kind: summary.kind,
                        }),
                        Err(reason) => surface.record_expansion_failure(
                            path,
                            summary.kind,
                            dependency_name(krate, summary.crate_id),
                            reason,
                        ),
                    }
                }
            }
            continue;
        };
        surface.record(id, path.clone(), item.inner.item_kind(), 0);
        record_local_canonical_alias(&mut surface, krate, id);

        if let ItemEnum::Module(_) = &item.inner {
            let mut children = Vec::new();
            resolve_children(krate, id, &path, &mut children, &mut HashSet::new());
            queue.extend(children);
        }
    }

    (surface, pending, pending_named)
}

/// This intentionally never creates a new entry.
fn record_local_canonical_alias(surface: &mut Surface, krate: &Crate, id: Id) {
    let Some(summary) = krate.paths.get(&id) else {
        return;
    };
    if summary.crate_id == 0 {
        surface.record_alias(0, id, &summary.path);
    }
}

/// The `ExternalCrate::name` value remains the Rust crate identifier. The
/// docs.rs URL supplies the package spelling and the concrete dependency
/// version used for acquisition.
#[cfg(test)]
pub(crate) fn parse_docs_rs_dependency(
    rust_crate_name: &str,
    html_root_url: &str,
) -> Option<DocsRsDependency> {
    parse_docs_rs_dependency_result(rust_crate_name, html_root_url).ok()
}

fn parse_docs_rs_dependency_result(
    rust_crate_name: &str,
    html_root_url: &str,
) -> Result<DocsRsDependency, ExpansionFailureReason> {
    let url =
        Url::parse(html_root_url).map_err(|_| ExpansionFailureReason::DependencyMetadataInvalid)?;
    if url.scheme() != "https" || url.host_str() != Some("docs.rs") {
        return Err(ExpansionFailureReason::DependencyNotOnDocsRs);
    }
    let mut segments = url
        .path_segments()
        .ok_or(ExpansionFailureReason::DependencyMetadataInvalid)?
        .filter(|segment| !segment.is_empty());
    let package_name = segments
        .next()
        .ok_or(ExpansionFailureReason::DependencyMetadataInvalid)?
        .to_owned();
    let version = Version::parse(
        segments
            .next()
            .ok_or(ExpansionFailureReason::DependencyMetadataInvalid)?,
    )
    .map_err(|_| ExpansionFailureReason::DependencyMetadataInvalid)?
    .to_string();
    Ok(DocsRsDependency {
        rust_crate_name: rust_crate_name.to_owned(),
        package_name,
        version,
    })
}

fn docs_rs_dependency(
    krate: &Crate,
    crate_id: u32,
) -> Result<DocsRsDependency, ExpansionFailureReason> {
    let external = krate
        .external_crates
        .get(&crate_id)
        .ok_or(ExpansionFailureReason::DependencyMetadataMissing)?;
    let url = external
        .html_root_url
        .as_deref()
        .ok_or(ExpansionFailureReason::DependencyMetadataMissing)?;
    parse_docs_rs_dependency_result(&external.name, url)
}

fn dependency_name(krate: &Crate, crate_id: u32) -> Option<String> {
    krate
        .external_crates
        .get(&crate_id)
        .map(|external| external.name.clone())
}

/// Follows named and glob `use` re-exports, with cycle protection.
pub(crate) fn resolve_children(
    krate: &Crate,
    mod_id: Id,
    path: &[String],
    out: &mut Vec<(Id, Vec<String>)>,
    glob_seen: &mut HashSet<Id>,
) {
    let Some(item) = krate.index.get(&mod_id) else {
        return;
    };
    let ItemEnum::Module(module) = &item.inner else {
        return;
    };

    for &child_id in &module.items {
        let Some(child) = krate.index.get(&child_id) else {
            continue;
        };
        match &child.inner {
            ItemEnum::Use(use_) => {
                let Some(target) = use_.id else {
                    continue; // re-export of a primitive carries no id
                };
                if use_.is_glob {
                    if glob_seen.insert(target) {
                        resolve_children(krate, target, path, out, glob_seen);
                    }
                } else {
                    out.push((target, joined(path, &use_.name)));
                }
            }
            _ => {
                if matches!(child.visibility, Visibility::Public) {
                    if let Some(name) = &child.name {
                        out.push((child_id, joined(path, name)));
                    }
                }
            }
        }
    }
}

fn joined(path: &[String], segment: &str) -> Vec<String> {
    let mut next = Vec::with_capacity(path.len() + 1);
    next.extend_from_slice(path);
    next.push(segment.to_owned());
    next
}
