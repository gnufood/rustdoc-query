mod item;

use crate::cache::CrateEntry;
use crate::query::detail::{build_recorded_method_detail, ItemDetail};
use crate::rustdoc::SurfaceResolution;

pub(crate) enum ItemDetailLookup {
    Found(Box<ItemDetail>),
    Missing,
}

#[cfg(test)]
pub(crate) fn build_item_detail(
    entry: &CrateEntry,
    path: &str,
    include_impls: bool,
    full_docs: bool,
) -> Option<ItemDetail> {
    match resolve_item_detail(entry, path, include_impls, full_docs) {
        ItemDetailLookup::Found(detail) => Some(*detail),
        ItemDetailLookup::Missing => None,
    }
}

pub(crate) fn resolve_item_detail(
    entry: &CrateEntry,
    path: &str,
    include_impls: bool,
    full_docs: bool,
) -> ItemDetailLookup {
    // Alias-aware surface lookup (handles longer alternate paths for items
    // already recorded at a shorter re-export path).
    let Some(se) = entry.surface.get_by_path(path) else {
        return ItemDetailLookup::Missing;
    };
    let owner = entry.krate_for(se);
    match se.resolution {
        SurfaceResolution::Item => item::build(owner, se, include_impls, full_docs),
        SurfaceResolution::InherentMethod {
            parent,
            implementation,
        } => build_recorded_method_detail(owner, se, parent, implementation, full_docs),
    }
    .map_or(ItemDetailLookup::Missing, |detail| {
        ItemDetailLookup::Found(Box::new(detail))
    })
}
