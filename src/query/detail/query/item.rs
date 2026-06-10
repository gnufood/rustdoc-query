use rustdoc_types::{Crate, Item, ItemEnum};

use crate::query::detail::{
    render_enum, render_struct, render_trait, render_union, AdtDetail, ItemDetail, TraitDetail,
};
use crate::rustdoc::{first_paragraph, render_fn_sig, render_generics, render_type, SurfaceEntry};

pub(super) fn build(
    krate: &Crate,
    surface: &SurfaceEntry,
    include_impls: bool,
    full_docs: bool,
) -> Option<ItemDetail> {
    let item = krate.index.get(&surface.id)?;
    let base = Base::new(surface, item, full_docs);
    Some(match &item.inner {
        ItemEnum::Struct(value) => adt(base, render_struct(krate, value, include_impls)),
        ItemEnum::Union(value) => adt(base, render_union(krate, value, include_impls)),
        ItemEnum::Enum(value) => adt(base, render_enum(krate, value, &item.attrs, include_impls)),
        ItemEnum::Function(value) => function(base, krate, surface, value, include_impls),
        ItemEnum::Trait(value) => trait_detail(base, render_trait(krate, value), include_impls),
        ItemEnum::TypeAlias(value) => type_alias(base, krate, value, include_impls),
        _ => empty(base, include_impls),
    })
}

struct Base {
    path: String,
    kind: rustdoc_types::ItemKind,
    docs: Option<String>,
    docs_truncated: bool,
}

impl Base {
    fn new(surface: &SurfaceEntry, item: &Item, full_docs: bool) -> Self {
        let (docs, docs_truncated) = if full_docs {
            (item.docs.clone().filter(|docs| !docs.is_empty()), false)
        } else {
            first_paragraph(item.docs.as_deref())
        };
        Self {
            path: surface.path.join("::"),
            kind: surface.kind,
            docs,
            docs_truncated,
        }
    }
}

fn empty(base: Base, include_impls: bool) -> ItemDetail {
    ItemDetail {
        path: base.path,
        kind: base.kind,
        docs: base.docs,
        docs_truncated: base.docs_truncated,
        signature: None,
        impl_constraint: None,
        fields: vec![],
        variants: vec![],
        non_exhaustive: false,
        methods: vec![],
        required_methods: vec![],
        provided_methods: vec![],
        assoc_types: vec![],
        consts: vec![],
        trait_impls: vec![],
        derives: vec![],
        auto_traits: vec![],
        blanket_impls: include_impls.then(Vec::new),
        version: None,
    }
}

fn adt(base: Base, value: AdtDetail) -> ItemDetail {
    let AdtDetail {
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
    } = value;
    let mut detail = empty(base, false);
    detail.signature = signature;
    detail.fields = fields;
    detail.variants = variants;
    detail.non_exhaustive = non_exhaustive;
    detail.methods = methods;
    detail.consts = consts;
    detail.trait_impls = trait_impls;
    detail.derives = derives;
    detail.auto_traits = auto_traits;
    detail.blanket_impls = blanket_impls;
    detail
}

fn function(
    base: Base,
    krate: &Crate,
    surface: &SurfaceEntry,
    value: &rustdoc_types::Function,
    include_impls: bool,
) -> ItemDetail {
    let mut detail = empty(base, include_impls);
    let name = surface.path.last().map_or("", String::as_str);
    detail.signature = Some(render_fn_sig(krate, name, value));
    detail
}

fn trait_detail(base: Base, value: TraitDetail, include_impls: bool) -> ItemDetail {
    let TraitDetail {
        signature,
        required_methods,
        provided_methods,
        assoc_types,
    } = value;
    let mut detail = empty(base, include_impls);
    detail.signature = signature;
    detail.required_methods = required_methods;
    detail.provided_methods = provided_methods;
    detail.assoc_types = assoc_types;
    detail
}

fn type_alias(
    base: Base,
    krate: &Crate,
    value: &rustdoc_types::TypeAlias,
    include_impls: bool,
) -> ItemDetail {
    let generics = render_generics(krate, &value.generics);
    let type_ = render_type(krate, &value.type_);
    let signature = if generics.is_empty() {
        type_
    } else {
        format!("{generics} = {type_}")
    };
    let mut detail = empty(base, include_impls);
    detail.signature = Some(signature);
    detail
}
