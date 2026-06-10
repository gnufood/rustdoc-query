use rustdoc_types::{Crate, ItemEnum, Trait};

use super::{AssocTypeRow, MethodRow};
use crate::rustdoc::{
    render_bounds, render_fn_sig, render_generic_params, render_type, render_where_clause,
};

/// Trait fields that are not shared by other public item kinds.
pub(crate) struct TraitDetail {
    pub(crate) signature: Option<String>,
    pub(crate) required_methods: Vec<MethodRow>,
    pub(crate) provided_methods: Vec<MethodRow>,
    pub(crate) assoc_types: Vec<AssocTypeRow>,
}

pub(crate) fn render_trait(krate: &Crate, trait_: &Trait) -> TraitDetail {
    let params = render_generic_params(krate, &trait_.generics.params);
    let bounds = render_bounds(krate, &trait_.bounds);
    let where_clause = render_where_clause(krate, &trait_.generics.where_predicates);

    let mut required_methods = Vec::new();
    let mut provided_methods = Vec::new();
    let mut assoc_types = Vec::new();
    for id in &trait_.items {
        let Some(item) = krate.index.get(id) else {
            continue;
        };
        let Some(name) = item.name.as_ref() else {
            continue;
        };
        match &item.inner {
            ItemEnum::Function(function) => {
                let method = MethodRow {
                    name: name.clone(),
                    signature: render_fn_sig(krate, name, function),
                    impl_constraint: None,
                };
                if function.has_body {
                    provided_methods.push(method);
                } else {
                    required_methods.push(method);
                }
            }
            ItemEnum::AssocType { bounds, type_, .. } => assoc_types.push(AssocTypeRow {
                name: name.clone(),
                bounds: render_bounds(krate, bounds),
                default: type_.as_ref().map(|type_| render_type(krate, type_)),
            }),
            _ => {}
        }
    }
    required_methods.sort_by(|left, right| left.name.cmp(&right.name));
    provided_methods.sort_by(|left, right| left.name.cmp(&right.name));
    assoc_types.sort_by(|left, right| left.name.cmp(&right.name));

    TraitDetail {
        signature: non_empty(join_trait_signature(&params, &bounds, &where_clause)),
        required_methods,
        provided_methods,
        assoc_types,
    }
}

fn join_trait_signature(params: &str, bounds: &str, where_clause: &str) -> String {
    let mut signature = String::new();
    if !params.is_empty() {
        signature.push_str(params);
    }
    if !bounds.is_empty() {
        signature.push_str(": ");
        signature.push_str(bounds);
    }
    if !where_clause.is_empty() {
        if !signature.is_empty() {
            signature.push(' ');
        }
        signature.push_str(where_clause);
    }
    signature
}

fn non_empty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}
