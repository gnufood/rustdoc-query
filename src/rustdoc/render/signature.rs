use std::fmt::Write;

use rustdoc_types::{Crate, Function, Type};

use super::generics::{render_generic_params, render_where_clause};
use super::types::render_type;

pub(crate) fn render_fn_sig(krate: &Crate, name: &str, function: &Function) -> String {
    let mut output = String::new();
    if function.header.is_const {
        output.push_str("const ");
    }
    if function.header.is_async {
        output.push_str("async ");
    }
    if function.header.is_unsafe {
        output.push_str("unsafe ");
    }
    output.push_str("fn ");
    output.push_str(name);
    output.push_str(&render_generic_params(krate, &function.generics.params));
    let inputs = function
        .sig
        .inputs
        .iter()
        .map(|(name, type_)| render_param(krate, name, type_))
        .collect::<Vec<_>>();
    output.push('(');
    output.push_str(&inputs.join(", "));
    if function.sig.is_c_variadic {
        if !inputs.is_empty() {
            output.push_str(", ");
        }
        output.push_str("...");
    }
    output.push(')');
    if let Some(type_) = &function.sig.output {
        if write!(output, " -> {}", render_type(krate, type_)).is_err() {
            return output;
        }
    }
    let where_clause = render_where_clause(krate, &function.generics.where_predicates);
    if !where_clause.is_empty() {
        output.push(' ');
        output.push_str(&where_clause);
    }
    output
}

fn render_param(krate: &Crate, name: &str, type_: &Type) -> String {
    if name == "self" {
        match type_ {
            Type::BorrowedRef {
                is_mutable: false,
                lifetime: None,
                type_,
            } if matches!(type_.as_ref(), Type::Generic(value) if value == "Self") => {
                return "&self".to_owned()
            }
            Type::BorrowedRef {
                is_mutable: true,
                lifetime: None,
                type_,
            } if matches!(type_.as_ref(), Type::Generic(value) if value == "Self") => {
                return "&mut self".to_owned()
            }
            Type::Generic(value) if value == "Self" => return "self".to_owned(),
            _ => {}
        }
    }
    format!("{name}: {}", render_type(krate, type_))
}
