use rustdoc_types::{Crate, Impl};

use super::{render_generic_params, render_type, render_where_clause};

pub(crate) fn render_inherent_impl_context(krate: &Crate, impl_: &Impl) -> String {
    let params = render_generic_params(krate, &impl_.generics.params);
    let target = render_type(krate, &impl_.for_);
    let where_clause = render_where_clause(krate, &impl_.generics.where_predicates);
    let mut rendered = format!("impl{params} {target}");
    if !where_clause.is_empty() {
        rendered.push(' ');
        rendered.push_str(&where_clause);
    }
    rendered
}
