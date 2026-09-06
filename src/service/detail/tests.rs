use super::{super::test_support, to_contract};
use crate::contract::generated::{GetItemOutcome, GetItemRequest, RustdocItemKind, VariantKind};
use crate::query::{AssocTypeRow, ConstRow, FieldRow, ItemDetail, MethodRow, VariantRow};

#[tokio::test]
async fn public_service_applies_get_item_defaults_without_network_or_disk_state() {
    let service = test_support::service().await;
    let request: GetItemRequest = serde_json::from_value(serde_json::json!({
        "crate_name": "krate",
        "version": "1.0.0",
        "path": "krate::Widget"
    }))
    .expect("request is valid");

    let GetItemOutcome::GetItemSuccess(success) = service.get_item(request).await else {
        panic!("the deterministic item must succeed");
    };
    assert_eq!(success.result.kind, RustdocItemKind::Struct);
    assert_eq!(success.result.resolved_version, "1.0.0");
    assert_eq!(
        success.result.docs.as_deref(),
        Some("A deterministic widget.")
    );
    assert!(success.result.docs_truncated);
    assert!(success.metadata.cache.is_some());
}

#[tokio::test]
async fn public_service_resolves_a_canonical_local_item_path_to_its_public_path() {
    let service = test_support::service().await;
    let request: GetItemRequest = serde_json::from_value(serde_json::json!({
        "crate_name": "krate",
        "version": "1.0.0",
        "path": "krate::internal::Widget"
    }))
    .expect("request is valid");

    let GetItemOutcome::GetItemSuccess(success) = service.get_item(request).await else {
        panic!("the canonical path must resolve");
    };
    assert_eq!(success.result.path, "krate::Widget");
}

#[test]
fn detail_projection_preserves_every_contract_field() {
    let detail = to_contract(
        ItemDetail {
            path: "krate::Widget".to_owned(),
            kind: rustdoc_types::ItemKind::Struct,
            docs: Some("Docs".to_owned()),
            docs_truncated: true,
            signature: Some("struct Widget".to_owned()),
            impl_constraint: Some("<T: Codec>".to_owned()),
            fields: vec![FieldRow {
                name: "field".to_owned(),
                type_: "u8".to_owned(),
            }],
            variants: vec![VariantRow {
                name: "Variant".to_owned(),
                kind: rustdoc_types::VariantKind::Plain,
                fields: Vec::new(),
            }],
            non_exhaustive: true,
            methods: vec![method("method")],
            required_methods: vec![method("required")],
            provided_methods: vec![method("provided")],
            assoc_types: vec![AssocTypeRow {
                name: "Item".to_owned(),
                bounds: "Send".to_owned(),
                default: Some("u8".to_owned()),
            }],
            consts: vec![ConstRow {
                name: "MAX".to_owned(),
                type_: "usize".to_owned(),
            }],
            trait_impls: vec!["Debug".to_owned()],
            derives: vec!["Clone".to_owned()],
            auto_traits: vec!["Send".to_owned()],
            blanket_impls: Some(vec!["Into".to_owned()]),
            version: Some("ignored".to_owned()),
        },
        "1.0.0",
    );

    assert_eq!(detail.kind, RustdocItemKind::Struct);
    assert_eq!(detail.resolved_version, "1.0.0");
    assert_eq!(detail.impl_constraint.as_deref(), Some("<T: Codec>"));
    assert_eq!(detail.fields[0].type_, "u8");
    assert_eq!(detail.variants[0].kind, VariantKind::Unit);
    assert_eq!(detail.methods[0].impl_constraint, None);
    assert_eq!(detail.required_methods[0].name, "required");
    assert_eq!(detail.provided_methods[0].name, "provided");
    assert_eq!(detail.assoc_types[0].default.as_deref(), Some("u8"));
    assert_eq!(detail.consts[0].type_, "usize");
    assert_eq!(detail.blanket_impls, ["Into"]);
}

fn method(name: &str) -> MethodRow {
    MethodRow {
        name: name.to_owned(),
        signature: format!("fn {name}()"),
        impl_constraint: None,
    }
}
