//! The index model is separate from local traversal and dependency expansion.

use rustdoc_types::Crate;
#[cfg(test)]
use rustdoc_types::ItemKind;

mod expansion;
mod local;
mod methods;
mod model;

#[cfg(test)]
use local::parse_docs_rs_dependency;
pub(crate) use model::SurfaceEntry;
pub(crate) use model::{
    DocsRsDependency, ExpansionFailureReason, PendingExpansion, PendingNamedItem, Surface,
    SurfaceResolution,
};

impl Surface {
    #[must_use]
    pub(crate) fn build(krate: &Crate) -> (Self, Vec<PendingExpansion>, Vec<PendingNamedItem>) {
        local::build(krate)
    }

    pub(crate) fn apply_expansion(
        &mut self,
        dep: &Crate,
        exp: &PendingExpansion,
        dep_idx: u8,
    ) -> Result<(), ExpansionFailureReason> {
        expansion::apply_expansion(self, dep, exp, dep_idx)
    }

    /// Add public inherent methods whose canonical paths resolve unambiguously.
    pub(crate) fn add_resolvable_inherent_methods(&mut self, root: &Crate, dependencies: &[Crate]) {
        methods::add_resolvable_inherent_methods(self, root, dependencies);
    }
}

#[cfg(test)]
mod tests;
