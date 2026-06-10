use rustdoc_types::{Crate, Id, Impl, ItemEnum, Path};

use super::{ConstRow, MethodRow};
use crate::rustdoc::{
    render_fn_sig, render_inherent_impl_context, render_path, render_type, render_where_clause,
    short_path,
};

pub(crate) struct SplitImpls {
    pub(crate) methods: Vec<MethodRow>,
    pub(crate) consts: Vec<ConstRow>,
    pub(crate) trait_impls: Vec<String>,
    pub(crate) derives: Vec<String>,
    pub(crate) auto_traits: Vec<String>,
    pub(crate) blanket_impls: Option<Vec<String>>,
}

pub(crate) fn split_impls(krate: &Crate, impl_ids: &[Id], include_blanket: bool) -> SplitImpls {
    let mut methods = Vec::new();
    let mut consts = Vec::new();
    let mut trait_impls = Vec::new();
    let mut derives = Vec::new();
    let mut auto_traits = Vec::new();
    let mut blankets = Vec::new();

    for id in impl_ids {
        let Some(item) = krate.index.get(id) else {
            continue;
        };
        let ItemEnum::Impl(impl_) = &item.inner else {
            continue;
        };
        if impl_.blanket_impl.is_some() {
            if include_blanket {
                blankets.push(
                    impl_
                        .trait_
                        .as_ref()
                        .map(|path| render_trait_impl(krate, impl_, path))
                        .unwrap_or_default(),
                );
            }
            continue;
        }
        if let Some(trait_path) = &impl_.trait_ {
            let has_where = !impl_.generics.where_predicates.is_empty();
            if impl_.items.is_empty()
                && !has_where
                && is_auto_trait(krate, trait_path, impl_.is_synthetic)
            {
                auto_traits.push(short_path(&trait_path.path).to_owned());
                continue;
            }
            if impl_.items.is_empty() && !has_where {
                derives.push(short_path(&trait_path.path).to_owned());
            } else {
                trait_impls.push(render_trait_impl(krate, impl_, trait_path));
            }
            continue;
        }

        let constraint = render_inherent_impl_context(krate, impl_);
        for item_id in &impl_.items {
            let Some(method_item) = krate.index.get(item_id) else {
                continue;
            };
            let Some(name) = &method_item.name else {
                continue;
            };
            match &method_item.inner {
                ItemEnum::Function(function) => methods.push(MethodRow {
                    name: name.clone(),
                    signature: render_fn_sig(krate, name, function),
                    impl_constraint: Some(constraint.clone()),
                }),
                ItemEnum::AssocConst { type_, .. } => consts.push(ConstRow {
                    name: name.clone(),
                    type_: render_type(krate, type_),
                }),
                _ => {}
            }
        }
    }

    methods.sort_by(|left, right| {
        (&left.name, &left.impl_constraint, &left.signature).cmp(&(
            &right.name,
            &right.impl_constraint,
            &right.signature,
        ))
    });
    consts.sort_by(|left, right| left.name.cmp(&right.name));
    trait_impls.sort_unstable();
    derives.sort_unstable();
    auto_traits.sort_unstable();
    SplitImpls {
        methods,
        consts,
        trait_impls,
        derives,
        auto_traits,
        blanket_impls: include_blanket.then_some(blankets),
    }
}

/// The implemented type is essential because one queried item can
/// participate in several distinct implementations of the same generic
/// trait.
fn render_trait_impl(krate: &Crate, impl_: &Impl, trait_path: &Path) -> String {
    let mut rendered = format!(
        "{} for {}",
        render_path(krate, trait_path),
        render_type(krate, &impl_.for_)
    );
    if !impl_.generics.where_predicates.is_empty() {
        rendered.push(' ');
        rendered.push_str(&render_where_clause(
            krate,
            &impl_.generics.where_predicates,
        ));
    }
    rendered
}

/// Distinguishes an auto-trait impl from a derive: rustdoc marks
/// compiler-implied impls as synthetic, but dependency JSON can represent
/// explicit auto-trait impls with an unresolved external trait ID.
fn is_auto_trait(krate: &Crate, trait_path: &Path, is_synthetic: bool) -> bool {
    is_synthetic
        || matches!(
            krate.index.get(&trait_path.id).map(|item| &item.inner),
            Some(ItemEnum::Trait(trait_)) if trait_.is_auto
        )
        || is_standard_auto_trait_path(&trait_path.path)
}

/// Standard-library auto traits whose definitions are commonly absent from a
/// dependency crate's rustdoc index. Keep this fallback narrow: an unresolved
/// arbitrary empty trait impl remains subject to the existing classification.
fn is_standard_auto_trait_path(path: &str) -> bool {
    matches!(
        path.rsplit("::").next(),
        Some("Send" | "Sync" | "Unpin" | "UnwindSafe" | "RefUnwindSafe" | "Freeze")
    )
}
