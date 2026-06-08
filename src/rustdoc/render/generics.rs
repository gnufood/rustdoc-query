use rustdoc_types::{
    Crate, GenericBound, GenericParamDef, GenericParamDefKind, Generics, Term, TraitBoundModifier,
    WherePredicate,
};

use super::types::{render_path, render_type};

pub(crate) fn render_generics(krate: &Crate, generics: &Generics) -> String {
    let params = render_generic_params(krate, &generics.params);
    let where_clause = render_where_clause(krate, &generics.where_predicates);
    match (params.is_empty(), where_clause.is_empty()) {
        (true, true) => String::new(),
        (false, true) => params,
        (true, false) => where_clause,
        (false, false) => format!("{params} {where_clause}"),
    }
}

pub(crate) fn render_generic_params(krate: &Crate, params: &[GenericParamDef]) -> String {
    let parts: Vec<String> = params
        .iter()
        .filter_map(|param| match &param.kind {
            GenericParamDefKind::Lifetime { outlives } if outlives.is_empty() => {
                Some(param.name.clone())
            }
            GenericParamDefKind::Lifetime { outlives } => {
                Some(format!("{}: {}", param.name, outlives.join(" + ")))
            }
            GenericParamDefKind::Type {
                is_synthetic: true, ..
            } => None,
            GenericParamDefKind::Type { bounds, .. } if bounds.is_empty() => {
                Some(param.name.clone())
            }
            GenericParamDefKind::Type { bounds, .. } => {
                Some(format!("{}: {}", param.name, render_bounds(krate, bounds)))
            }
            GenericParamDefKind::Const { type_, .. } => Some(format!(
                "const {}: {}",
                param.name,
                render_type(krate, type_)
            )),
        })
        .collect();
    if parts.is_empty() {
        String::new()
    } else {
        format!("<{}>", parts.join(", "))
    }
}

pub(crate) fn render_where_clause(krate: &Crate, predicates: &[WherePredicate]) -> String {
    if predicates.is_empty() {
        return String::new();
    }
    let parts = predicates
        .iter()
        .map(|predicate| match predicate {
            WherePredicate::BoundPredicate { type_, bounds, .. } => format!(
                "{}: {}",
                render_type(krate, type_),
                render_bounds(krate, bounds)
            ),
            WherePredicate::LifetimePredicate { lifetime, outlives } if outlives.is_empty() => {
                lifetime.clone()
            }
            WherePredicate::LifetimePredicate { lifetime, outlives } => {
                format!("{lifetime}: {}", outlives.join(" + "))
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                let rhs = match rhs {
                    Term::Type(type_) => render_type(krate, type_),
                    Term::Constant(constant) => constant.expr.clone(),
                };
                format!("{} = {rhs}", render_type(krate, lhs))
            }
        })
        .collect::<Vec<_>>();
    format!("where {}", parts.join(", "))
}

pub(crate) fn render_bounds(krate: &Crate, bounds: &[GenericBound]) -> String {
    bounds
        .iter()
        .filter_map(|bound| match bound {
            GenericBound::TraitBound {
                trait_, modifier, ..
            } => {
                let prefix = match modifier {
                    TraitBoundModifier::Maybe => "?",
                    TraitBoundModifier::MaybeConst => "~const ",
                    TraitBoundModifier::None => "",
                };
                Some(format!("{prefix}{}", render_path(krate, trait_)))
            }
            GenericBound::Outlives(lifetime) => Some(lifetime.clone()),
            GenericBound::Use(_) => None,
        })
        .collect::<Vec<_>>()
        .join(" + ")
}
