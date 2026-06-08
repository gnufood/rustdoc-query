//! Application queries over prepared rustdoc crate entries.

mod detail;
mod find;
mod overview;

pub(crate) use detail::{
    resolve_item_detail, AssocTypeRow, ConstRow, FieldRow, ItemDetail, ItemDetailLookup, MethodRow,
    VariantRow,
};
pub(crate) use find::{find_document_items, find_items, FindRow, SortKey};
pub(crate) use overview::{build_overview, KindCount, Overview, OverviewItem, Totals};
