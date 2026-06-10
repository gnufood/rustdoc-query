mod adt;
mod impls;
mod method;
mod model;
mod query;
mod shape;
mod trait_detail;

#[cfg(test)]
mod tests;

pub(crate) use adt::{render_enum, render_struct, render_union, AdtDetail};
pub(crate) use impls::{split_impls, SplitImpls};
pub(crate) use method::build_recorded_method_detail;
pub(crate) use model::{AssocTypeRow, ConstRow, FieldRow, ItemDetail, MethodRow, VariantRow};
#[cfg(test)]
pub(crate) use query::build_item_detail;
pub(crate) use query::{resolve_item_detail, ItemDetailLookup};
pub(crate) use shape::{render_fields, render_tuple_fields, render_variants};
pub(crate) use trait_detail::{render_trait, TraitDetail};
