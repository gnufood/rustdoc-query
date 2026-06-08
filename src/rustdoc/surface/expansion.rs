use std::collections::{HashSet, VecDeque};

use rustdoc_types::{Crate, Id, ItemEnum, ItemKind};

use super::{local::resolve_children, ExpansionFailureReason, PendingExpansion, Surface};

/// Preserves items reachable through multiple dependency paths.
///
/// Nested cross-crate module re-exports are not followed
/// (one-level limit: missing id in dep.index + dep.paths module → skip).
pub(crate) fn apply_expansion(
    surface: &mut Surface,
    dep: &Crate,
    exp: &PendingExpansion,
    dep_idx: u8,
) -> Result<(), ExpansionFailureReason> {
    let Some(module_id) = find_dep_module(dep, &exp.dep_module_path) else {
        return Err(ExpansionFailureReason::TargetNotFound);
    };

    if !dep.index.contains_key(&module_id) {
        return Err(ExpansionFailureReason::TargetNotFound);
    }

    // BFS from the module's children (module itself already in surface).
    let mut visited: HashSet<Id> = HashSet::new();
    let mut queue: VecDeque<(Id, Vec<String>)> = VecDeque::new();
    let mut children = Vec::new();
    resolve_children(
        dep,
        module_id,
        &exp.local_prefix,
        &mut children,
        &mut HashSet::new(),
    );
    queue.extend(children);

    while let Some((id, path)) = queue.pop_front() {
        if !visited.insert(id) {
            continue;
        }
        if surface.contains_path(&path) {
            continue;
        }
        let Some(item) = dep.index.get(&id) else {
            // External to dep: record leaf if kind known and not a module.
            if let Some(summary) = dep.paths.get(&id) {
                if summary.kind != ItemKind::Module {
                    surface.record(id, path, summary.kind, dep_idx);
                }
                // Nested cross-crate module → skip (one-level limit).
            }
            continue;
        };
        surface.record(id, path.clone(), item.inner.item_kind(), dep_idx);
        if let ItemEnum::Module(_) = &item.inner {
            let mut ch = Vec::new();
            resolve_children(dep, id, &path, &mut ch, &mut HashSet::new());
            queue.extend(ch);
        }
    }
    Ok(())
}
/// Tries `dep.paths` first (fast for real crates with populated paths maps),
/// then falls back to walking `dep.index` from the root by following path
/// segments (needed when `dep.paths` is absent or stripped).
fn find_dep_module(dep: &Crate, path: &[String]) -> Option<Id> {
    if let Some((&id, _)) = dep
        .paths
        .iter()
        .find(|(_, s)| s.path == *path && s.kind == ItemKind::Module)
    {
        if dep.index.contains_key(&id) {
            return Some(id);
        }
    }

    // path[0] is the crate root name (matches dep.root by convention).
    let mut current = dep.root;
    for segment in path.iter().skip(1) {
        let item = dep.index.get(&current)?;
        let ItemEnum::Module(module) = &item.inner else {
            return None;
        };
        let mut next = None;
        for &child_id in &module.items {
            let Some(child) = dep.index.get(&child_id) else {
                continue;
            };
            let candidate = match &child.inner {
                ItemEnum::Use(u) if u.name == *segment => u.id?,
                _ if child.name.as_deref() == Some(segment.as_str()) => child_id,
                _ => continue,
            };
            next = Some(candidate);
            break;
        }
        current = next?;
    }
    Some(current)
}
