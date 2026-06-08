//! Rustdoc acquisition, decoding, public-surface resolution, and rendering.

mod docs;
mod fetch;
mod kind;
mod load;
mod render;
mod surface;

pub(crate) use docs::{first_paragraph, summary_line};
pub(crate) use fetch::{build_client, fetch_rustdoc_zst, resolve_version};
pub(crate) use kind::{label, parse_label};
pub(crate) use load::load_crate;
pub(crate) use render::{
    render_bounds, render_fn_sig, render_generic_params, render_generics,
    render_inherent_impl_context, render_path, render_type, render_where_clause, short_path,
};
pub(crate) use surface::{
    DocsRsDependency, ExpansionFailureReason, PendingExpansion, PendingNamedItem, Surface,
    SurfaceEntry, SurfaceResolution,
};
