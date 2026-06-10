use rustdoc_types::{Id, ItemEnum, Visibility};

use super::ItemDetail;
use crate::rustdoc::{first_paragraph, render_fn_sig, render_inherent_impl_context, SurfaceEntry};

/// Render an inherent method already proven unique by the public surface.
pub(crate) fn build_recorded_method_detail(
    krate: &rustdoc_types::Crate,
    surface: &SurfaceEntry,
    parent: Id,
    implementation: Id,
    full_docs: bool,
) -> Option<ItemDetail> {
    let parent_item = krate.index.get(&parent)?;
    let implementations = match &parent_item.inner {
        ItemEnum::Struct(item) => &item.impls,
        ItemEnum::Enum(item) => &item.impls,
        _ => return None,
    };
    if !implementations.contains(&implementation) {
        return None;
    }
    let impl_item = krate.index.get(&implementation)?;
    let ItemEnum::Impl(impl_) = &impl_item.inner else {
        return None;
    };
    if impl_.trait_.is_some() || !impl_.items.contains(&surface.id) {
        return None;
    }
    let method = krate.index.get(&surface.id)?;
    let ItemEnum::Function(function) = &method.inner else {
        return None;
    };
    if method.visibility != Visibility::Public {
        return None;
    }
    let name = method.name.as_deref()?;
    let (docs, docs_truncated) = if full_docs {
        (method.docs.clone().filter(|docs| !docs.is_empty()), false)
    } else {
        first_paragraph(method.docs.as_deref())
    };
    Some(ItemDetail {
        path: surface.path.join("::"),
        kind: rustdoc_types::ItemKind::Function,
        docs,
        docs_truncated,
        signature: Some(render_fn_sig(krate, name, function)),
        impl_constraint: Some(render_inherent_impl_context(krate, impl_)),
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
        blanket_impls: None,
        version: None,
    })
}
