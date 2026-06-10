use rustdoc_types::{Crate, Id, ItemEnum, VariantKind};

use super::{FieldRow, VariantRow};
use crate::rustdoc::render_type;

pub(crate) fn render_fields(krate: &Crate, field_ids: &[Id]) -> Vec<FieldRow> {
    field_ids
        .iter()
        .filter_map(|id| {
            let item = krate.index.get(id)?;
            let name = item.name.clone()?;
            let ItemEnum::StructField(type_) = &item.inner else {
                return None;
            };
            Some(FieldRow {
                name,
                type_: render_type(krate, type_),
            })
        })
        .collect()
}

pub(crate) fn render_tuple_fields(krate: &Crate, ids: &[Option<Id>]) -> Vec<FieldRow> {
    ids.iter()
        .enumerate()
        .filter_map(|(index, id)| {
            let item = krate.index.get(id.as_ref()?)?;
            let ItemEnum::StructField(type_) = &item.inner else {
                return None;
            };
            Some(FieldRow {
                name: index.to_string(),
                type_: render_type(krate, type_),
            })
        })
        .collect()
}

pub(crate) fn render_variants(krate: &Crate, variant_ids: &[Id]) -> Vec<VariantRow> {
    variant_ids
        .iter()
        .filter_map(|id| {
            let item = krate.index.get(id)?;
            let name = item.name.clone()?;
            let ItemEnum::Variant(variant) = &item.inner else {
                return None;
            };
            let (kind, fields) = match &variant.kind {
                VariantKind::Plain => (VariantKind::Plain, Vec::new()),
                VariantKind::Tuple(ids) => (
                    VariantKind::Tuple(ids.clone()),
                    render_tuple_fields(krate, ids),
                ),
                VariantKind::Struct {
                    fields,
                    has_stripped_fields,
                } => (
                    VariantKind::Struct {
                        fields: fields.clone(),
                        has_stripped_fields: *has_stripped_fields,
                    },
                    render_fields(krate, fields),
                ),
            };
            Some(VariantRow { name, kind, fields })
        })
        .collect()
}
