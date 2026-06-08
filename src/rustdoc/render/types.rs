use rustdoc_types::{
    AssocItemConstraintKind, Crate, DynTrait, GenericArg, GenericArgs, Path, Term, Type,
};

use super::generics::render_bounds;

pub(crate) fn render_type(krate: &Crate, ty: &Type) -> String {
    match ty {
        Type::ResolvedPath(path) => render_path(krate, path),
        Type::DynTrait(trait_) => render_dyn_trait(krate, trait_),
        Type::Generic(name) | Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(_) => "fn(...)".to_owned(),
        Type::Tuple(types) => format!(
            "({})",
            types
                .iter()
                .map(|type_| render_type(krate, type_))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Type::Slice(type_) => format!("[{}]", render_type(krate, type_)),
        Type::Array { type_, len } => format!("[{}; {len}]", render_type(krate, type_)),
        Type::Pat { type_, .. } => render_type(krate, type_),
        Type::ImplTrait(bounds) => format!("impl {}", render_bounds(krate, bounds)),
        Type::Infer => "_".to_owned(),
        Type::RawPointer { is_mutable, type_ } => {
            let mutability = if *is_mutable { "mut" } else { "const" };
            format!("*{mutability} {}", render_type(krate, type_))
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let lifetime = lifetime
                .as_deref()
                .map(|value| format!("{value} "))
                .unwrap_or_default();
            let mutability = if *is_mutable { "mut " } else { "" };
            format!("&{lifetime}{mutability}{}", render_type(krate, type_))
        }
        Type::QualifiedPath {
            name,
            self_type,
            trait_,
            ..
        } => match trait_ {
            Some(trait_) => format!(
                "<{} as {}>::{name}",
                render_type(krate, self_type),
                path_name(krate, trait_)
            ),
            None => format!("{}::{name}", render_type(krate, self_type)),
        },
    }
}

pub(crate) fn render_path(krate: &Crate, path: &Path) -> String {
    let name = path_name(krate, path);
    let Some(args) = &path.args else { return name };
    match args.as_ref() {
        GenericArgs::AngleBracketed { args, constraints } => {
            let mut parts: Vec<String> = args
                .iter()
                .filter_map(|arg| match arg {
                    GenericArg::Lifetime(lifetime) => Some(lifetime.clone()),
                    GenericArg::Type(type_) => Some(render_type(krate, type_)),
                    GenericArg::Const(constant) => Some(constant.expr.clone()),
                    GenericArg::Infer => None,
                })
                .collect();
            for constraint in constraints {
                let value = match &constraint.binding {
                    AssocItemConstraintKind::Equality(term) => match term {
                        Term::Type(type_) => render_type(krate, type_),
                        Term::Constant(constant) => constant.expr.clone(),
                    },
                    AssocItemConstraintKind::Constraint(bounds) => render_bounds(krate, bounds),
                };
                let separator = match constraint.binding {
                    AssocItemConstraintKind::Equality(_) => " = ",
                    AssocItemConstraintKind::Constraint(_) => ": ",
                };
                parts.push(format!("{}{}{value}", constraint.name, separator));
            }
            if parts.is_empty() {
                name
            } else {
                format!("{name}<{}>", parts.join(", "))
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let inputs = inputs
                .iter()
                .map(|type_| render_type(krate, type_))
                .collect::<Vec<_>>()
                .join(", ");
            output.as_ref().map_or_else(
                || format!("{name}({inputs})"),
                |type_| format!("{name}({inputs}) -> {}", render_type(krate, type_)),
            )
        }
        GenericArgs::ReturnTypeNotation => format!("{name}(..)"),
    }
}

fn render_dyn_trait(krate: &Crate, dyn_trait: &DynTrait) -> String {
    let bounds = dyn_trait
        .traits
        .iter()
        .map(|poly_trait| render_path(krate, &poly_trait.trait_))
        .collect::<Vec<_>>()
        .join(" + ");
    dyn_trait.lifetime.as_ref().map_or_else(
        || format!("dyn {bounds}"),
        |lifetime| format!("dyn {bounds} + {lifetime}"),
    )
}

fn path_name(krate: &Crate, path: &Path) -> String {
    if path.path.is_empty() {
        krate
            .paths
            .get(&path.id)
            .and_then(|summary| summary.path.last())
            .cloned()
            .unwrap_or_else(|| "?".to_owned())
    } else {
        short_path(&path.path).to_owned()
    }
}

pub(crate) fn short_path(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}
