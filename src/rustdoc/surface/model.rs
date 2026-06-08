use std::collections::HashMap;

use rustdoc_types::{Crate, Id, ItemKind};

#[derive(Debug, Clone)]
pub(crate) struct SurfaceEntry {
    pub(crate) id: Id,
    pub(crate) path: Vec<String>,
    pub(crate) kind: ItemKind,
    /// `0` is the main crate; higher values index prepared dependency crates.
    pub(crate) dep_idx: u8,
    pub(crate) resolution: SurfaceResolution,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum SurfaceResolution {
    Item,
    InherentMethod { parent: Id, implementation: Id },
}

#[derive(Debug, Default)]
pub(crate) struct Surface {
    entries: Vec<SurfaceEntry>,
    by_id: HashMap<(u8, Id), usize>,
    by_path: HashMap<String, usize>,
    by_alias: HashMap<String, usize>,
    by_kind: HashMap<ItemKind, Vec<usize>>,
    expansion_failures: Vec<ExpansionFailure>,
}

/// This stays private to the rustdoc/cache/query pipeline. The service renders
/// safe, actionable guidance from it through the existing public hint field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpansionFailureReason {
    DependencyMetadataMissing,
    DependencyMetadataInvalid,
    DependencyNotOnDocsRs,
    DocumentationUnavailable,
    TargetNotFound,
    ExpansionLimitReached,
}

impl ExpansionFailureReason {
    #[must_use]
    pub(crate) const fn description(self) -> &'static str {
        match self {
            Self::DependencyMetadataMissing => "its dependency metadata is unavailable",
            Self::DependencyMetadataInvalid => "its dependency metadata is malformed",
            Self::DependencyNotOnDocsRs => "its dependency documentation is not hosted on docs.rs",
            Self::DocumentationUnavailable => "its dependency documentation is unavailable",
            Self::TargetNotFound => "the re-export target is absent from dependency documentation",
            Self::ExpansionLimitReached => "the dependency expansion limit was reached",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExpansionFailure {
    pub(crate) local_path: Vec<String>,
    pub(crate) kind: ItemKind,
    pub(crate) dependency: Option<String>,
    pub(crate) reason: ExpansionFailureReason,
}

/// A module re-export whose dependency must be loaded before its children can
/// be stitched into the parent surface.
#[derive(Debug)]
pub(crate) struct PendingExpansion {
    pub(crate) dependency: DocsRsDependency,
    pub(crate) dep_module_path: Vec<String>,
    pub(crate) local_prefix: Vec<String>,
}

/// One external crate's Rust and docs.rs identities.
///
/// Package names may differ from crate identifiers (`-` vs `_`). Fetches use
/// the package name; rustdoc-path matching retains the Rust identifier.
#[derive(Debug, Clone)]
pub(crate) struct DocsRsDependency {
    pub(crate) rust_crate_name: String,
    pub(crate) package_name: String,
    pub(crate) version: String,
}

#[derive(Debug)]
pub(crate) struct PendingNamedItem {
    pub(crate) dependency: DocsRsDependency,
    pub(crate) dep_item_path: Vec<String>,
    pub(crate) local_path: Vec<String>,
    pub(crate) kind: ItemKind,
}

impl Surface {
    #[must_use]
    pub(crate) fn get_by_path(&self, path: &str) -> Option<&SurfaceEntry> {
        self.by_path
            .get(path)
            .or_else(|| self.by_alias.get(path))
            .map(|&index| &self.entries[index])
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) fn get_by_id(&self, id: Id) -> Option<&SurfaceEntry> {
        self.by_id.get(&(0, id)).map(|&index| &self.entries[index])
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &SurfaceEntry> {
        self.entries.iter()
    }

    pub(crate) fn iter_by_kind(&self, kind: ItemKind) -> impl Iterator<Item = &SurfaceEntry> {
        self.by_kind
            .get(&kind)
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .map(|&index| &self.entries[index])
    }

    pub(crate) fn expansion_failures(&self) -> impl Iterator<Item = &ExpansionFailure> {
        self.expansion_failures.iter()
    }

    pub(crate) fn record_expansion_failure(
        &mut self,
        local_path: Vec<String>,
        kind: ItemKind,
        dependency: Option<String>,
        reason: ExpansionFailureReason,
    ) {
        self.expansion_failures.push(ExpansionFailure {
            local_path,
            kind,
            dependency,
            reason,
        });
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn contains_path(&self, path: &[String]) -> bool {
        self.by_path.contains_key(&path.join("::"))
    }

    pub(crate) fn record_alias(&mut self, dep_idx: u8, id: Id, path: &[String]) {
        if let Some(&index) = self.by_id.get(&(dep_idx, id)) {
            self.by_alias.entry(path.join("::")).or_insert(index);
        }
    }

    pub(crate) fn record(&mut self, id: Id, path: Vec<String>, kind: ItemKind, dep_idx: u8) {
        let index = self.entries.len();
        self.by_id.insert((dep_idx, id), index);
        self.by_path.insert(path.join("::"), index);
        self.by_kind.entry(kind).or_default().push(index);
        self.entries.push(SurfaceEntry {
            id,
            path,
            kind,
            dep_idx,
            resolution: SurfaceResolution::Item,
        });
    }

    pub(crate) fn record_inherent_method(
        &mut self,
        id: Id,
        path: Vec<String>,
        dep_idx: u8,
        parent: Id,
        implementation: Id,
    ) {
        if self.contains_path(&path) {
            return;
        }
        let index = self.entries.len();
        self.by_id.insert((dep_idx, id), index);
        self.by_path.insert(path.join("::"), index);
        self.by_kind
            .entry(ItemKind::Function)
            .or_default()
            .push(index);
        self.entries.push(SurfaceEntry {
            id,
            path,
            kind: ItemKind::Function,
            dep_idx,
            resolution: SurfaceResolution::InherentMethod {
                parent,
                implementation,
            },
        });
    }

    pub(crate) fn apply_named_item(
        &mut self,
        dep: &Crate,
        dep_item_path: &[String],
        local_path: Vec<String>,
        kind: ItemKind,
        dep_idx: u8,
    ) -> Result<(), ExpansionFailureReason> {
        if self.contains_path(&local_path) {
            return Ok(());
        }
        let local_id = dep
            .paths
            .iter()
            .find(|(_, summary)| summary.path.as_slice() == dep_item_path && summary.crate_id == 0)
            .or_else(|| {
                dep.paths
                    .iter()
                    .find(|(_, summary)| summary.path.as_slice() == dep_item_path)
            })
            .map(|(&id, _)| id);
        let Some(local_id) = local_id else {
            return Err(ExpansionFailureReason::TargetNotFound);
        };
        if dep.index.contains_key(&local_id) {
            self.record(local_id, local_path, kind, dep_idx);
            Ok(())
        } else {
            Err(ExpansionFailureReason::TargetNotFound)
        }
    }
}
