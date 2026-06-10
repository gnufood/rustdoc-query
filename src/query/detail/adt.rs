use rustdoc_types::{Attribute, Crate, Enum, Struct, StructKind, Union};

use super::{
    render_fields, render_tuple_fields, render_variants, split_impls, ConstRow, FieldRow,
    MethodRow, SplitImpls, VariantRow,
};
use crate::rustdoc::render_generics;

pub(crate) struct AdtDetail {
    pub(crate) signature: Option<String>,
    pub(crate) fields: Vec<FieldRow>,
    pub(crate) variants: Vec<VariantRow>,
    pub(crate) non_exhaustive: bool,
    pub(crate) methods: Vec<MethodRow>,
    pub(crate) consts: Vec<ConstRow>,
    pub(crate) trait_impls: Vec<String>,
    pub(crate) derives: Vec<String>,
    pub(crate) auto_traits: Vec<String>,
    pub(crate) blanket_impls: Option<Vec<String>>,
}

pub(crate) fn render_struct(krate: &Crate, item: &Struct, include_impls: bool) -> AdtDetail {
    let fields = match &item.kind {
        StructKind::Plain { fields, .. } => render_fields(krate, fields),
        StructKind::Tuple(ids) => render_tuple_fields(krate, ids),
        StructKind::Unit => Vec::new(),
    };
    from_impls(
        non_empty(render_generics(krate, &item.generics)),
        fields,
        Vec::new(),
        false,
        split_impls(krate, &item.impls, include_impls),
    )
}

pub(crate) fn render_union(krate: &Crate, item: &Union, include_impls: bool) -> AdtDetail {
    from_impls(
        non_empty(render_generics(krate, &item.generics)),
        render_fields(krate, &item.fields),
        Vec::new(),
        false,
        split_impls(krate, &item.impls, include_impls),
    )
}

pub(crate) fn render_enum(
    krate: &Crate,
    item: &Enum,
    attrs: &[Attribute],
    include_impls: bool,
) -> AdtDetail {
    from_impls(
        non_empty(render_generics(krate, &item.generics)),
        Vec::new(),
        render_variants(krate, &item.variants),
        attrs
            .iter()
            .any(|attr| matches!(attr, Attribute::NonExhaustive)),
        split_impls(krate, &item.impls, include_impls),
    )
}

fn from_impls(
    signature: Option<String>,
    fields: Vec<FieldRow>,
    variants: Vec<VariantRow>,
    non_exhaustive: bool,
    SplitImpls {
        methods,
        consts,
        trait_impls,
        derives,
        auto_traits,
        blanket_impls,
    }: SplitImpls,
) -> AdtDetail {
    AdtDetail {
        signature,
        fields,
        variants,
        non_exhaustive,
        methods,
        consts,
        trait_impls,
        derives,
        auto_traits,
        blanket_impls,
    }
}

fn non_empty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}
