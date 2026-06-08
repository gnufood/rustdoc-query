//! Best-effort expansion of public dependency re-exports.
//!
//! Dependency crates use the same three-tier loader as primary crates, while
//! an individual failed expansion remains non-fatal to the parent crate.

use std::collections::HashMap;

use rustdoc_types::Crate;

use super::CrateCache;
use crate::rustdoc::{
    DocsRsDependency, ExpansionFailureReason, PendingExpansion, PendingNamedItem, Surface,
};

/// Returns the dep crates in expansion order so `CrateEntry` can route
/// index lookups to the correct crate for dep-expanded surface entries.
pub(crate) async fn apply_pending_expansions(
    cache: &CrateCache,
    surface: &mut Surface,
    pending: &[PendingExpansion],
    pending_named: &[PendingNamedItem],
) -> Vec<Crate> {
    let mut dep_crates: Vec<Crate> = Vec::new();
    // `None` records a failed/limited load too, so several re-exports from one
    // dependency cannot repeatedly issue the same failed request.
    let mut dep_idx_by_source: HashMap<(String, String), Result<u8, ExpansionFailureReason>> =
        HashMap::new();

    for exp in pending {
        let dep_idx = match load_dependency(
            cache,
            &mut dep_crates,
            &mut dep_idx_by_source,
            &exp.dependency,
        )
        .await
        {
            Ok(dep_idx) => dep_idx,
            Err(reason) => {
                surface.record_expansion_failure(
                    exp.local_prefix.clone(),
                    rustdoc_types::ItemKind::Module,
                    Some(exp.dependency.rust_crate_name.clone()),
                    reason,
                );
                continue;
            }
        };
        let dep = &dep_crates[usize::from(dep_idx - 1)];
        if let Err(reason) = surface.apply_expansion(dep, exp, dep_idx) {
            surface.record_expansion_failure(
                exp.local_prefix.clone(),
                rustdoc_types::ItemKind::Module,
                Some(exp.dependency.rust_crate_name.clone()),
                reason,
            );
        }
    }

    // Reuse already-fetched dependency crates, including remembered failures,
    // keyed by docs.rs package/version.
    for item in pending_named {
        let dep_idx = match load_dependency(
            cache,
            &mut dep_crates,
            &mut dep_idx_by_source,
            &item.dependency,
        )
        .await
        {
            Ok(dep_idx) => dep_idx,
            Err(reason) => {
                surface.record_expansion_failure(
                    item.local_path.clone(),
                    item.kind,
                    Some(item.dependency.rust_crate_name.clone()),
                    reason,
                );
                continue;
            }
        };
        if let Err(reason) = surface.apply_named_item(
            &dep_crates[usize::from(dep_idx - 1)],
            &item.dep_item_path,
            item.local_path.clone(),
            item.kind,
            dep_idx,
        ) {
            surface.record_expansion_failure(
                item.local_path.clone(),
                item.kind,
                Some(item.dependency.rust_crate_name.clone()),
                reason,
            );
        }
    }

    dep_crates
}

/// Fetch a dependency once and return its one-based index in `dep_crates`.
///
/// docs.rs package names may differ from the crate identifiers carried by
/// rustdoc paths (`-` vs `_`).
async fn load_dependency(
    cache: &CrateCache,
    dep_crates: &mut Vec<Crate>,
    dep_idx_by_source: &mut HashMap<(String, String), Result<u8, ExpansionFailureReason>>,
    dependency: &DocsRsDependency,
) -> Result<u8, ExpansionFailureReason> {
    let key = source_key(dependency);
    if let Some(result) = dep_idx_by_source.get(&key) {
        return *result;
    }

    let result = if let Some(dep_idx) = next_dep_idx(dep_crates.len()) {
        match cache
            .load_crate_cached(&dependency.package_name, &dependency.version)
            .await
        {
            Ok(loaded) => {
                dep_crates.push(loaded.krate);
                Ok(dep_idx)
            }
            Err(error) => {
                tracing::warn!(
                    rust_crate = dependency.rust_crate_name,
                    package = dependency.package_name,
                    version = dependency.version,
                    "dependency expansion load failed: {error}"
                );
                Err(ExpansionFailureReason::DocumentationUnavailable)
            }
        }
    } else {
        tracing::warn!(
            rust_crate = dependency.rust_crate_name,
            package = dependency.package_name,
            "dependency expansion limit reached"
        );
        Err(ExpansionFailureReason::ExpansionLimitReached)
    };
    dep_idx_by_source.insert(key, result);
    result
}

fn source_key(dependency: &DocsRsDependency) -> (String, String) {
    (dependency.package_name.clone(), dependency.version.clone())
}

/// Return the next one-based dependency index, reserving zero for the root crate.
fn next_dep_idx(dep_count: usize) -> Option<u8> {
    u8::try_from(dep_count).ok()?.checked_add(1)
}

#[cfg(test)]
mod tests {
    use super::{next_dep_idx, source_key};
    use crate::rustdoc::DocsRsDependency;

    #[test]
    fn dependency_indices_are_one_based_and_bounded() {
        assert_eq!(next_dep_idx(0), Some(1));
        assert_eq!(next_dep_idx(254), Some(u8::MAX));
        assert_eq!(next_dep_idx(255), None);
    }

    #[test]
    fn expansion_deduplication_uses_docs_rs_source_identity() {
        let package = DocsRsDependency {
            rust_crate_name: "axum_core".to_owned(),
            package_name: "axum-core".to_owned(),
            version: "0.5.6".to_owned(),
        };
        let renamed = DocsRsDependency {
            rust_crate_name: "axum_core_renamed".to_owned(),
            package_name: "axum-core".to_owned(),
            version: "0.5.6".to_owned(),
        };
        assert_eq!(source_key(&package), source_key(&renamed));
    }
}
