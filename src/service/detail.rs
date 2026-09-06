//! Projects private item-detail query results onto the generated contract.

use crate::contract::generated::ItemDetail as ContractItemDetail;
use crate::query::ItemDetail;

mod map;

pub(crate) fn to_contract(detail: ItemDetail, resolved_version: &str) -> ContractItemDetail {
    ContractItemDetail {
        path: detail.path,
        kind: crate::service::kind::to_contract(detail.kind),
        docs: detail.docs,
        docs_truncated: detail.docs_truncated,
        signature: detail.signature,
        impl_constraint: detail.impl_constraint,
        fields: detail.fields.into_iter().map(map::field).collect(),
        variants: detail.variants.into_iter().map(map::variant).collect(),
        non_exhaustive: detail.non_exhaustive,
        methods: detail.methods.into_iter().map(map::method).collect(),
        required_methods: detail
            .required_methods
            .into_iter()
            .map(map::method)
            .collect(),
        provided_methods: detail
            .provided_methods
            .into_iter()
            .map(map::method)
            .collect(),
        assoc_types: detail
            .assoc_types
            .into_iter()
            .map(map::associated_type)
            .collect(),
        consts: detail
            .consts
            .into_iter()
            .map(map::associated_const)
            .collect(),
        trait_impls: detail.trait_impls,
        derives: detail.derives,
        auto_traits: detail.auto_traits,
        blanket_impls: detail.blanket_impls.unwrap_or_default(),
        resolved_version: resolved_version.to_owned(),
    }
}

#[cfg(test)]
#[path = "detail/tests.rs"]
mod tests;
