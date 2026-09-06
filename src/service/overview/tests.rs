use super::to_contract;
use crate::contract::generated::{ApiError, RustdocItemKind};
use crate::query::{KindCount, Overview, OverviewItem, Totals};
use crate::service::error::module_not_found;
use rustdoc_types::ItemKind;

fn sample() -> Overview {
    Overview {
        path: "reqwest".to_owned(),
        version: Some("0.13.3".to_owned()),
        docs: Some("An ergonomic HTTP client.".to_owned()),
        docs_truncated: true,
        totals: Totals {
            direct: vec![KindCount {
                kind: ItemKind::Struct,
                count: 1,
            }],
            subtree: vec![
                KindCount {
                    kind: ItemKind::Struct,
                    count: 2,
                },
                KindCount {
                    kind: ItemKind::Module,
                    count: 1,
                },
            ],
        },
        items: vec![OverviewItem {
            name: "Client".to_owned(),
            path: "reqwest::Client".to_owned(),
            kind: ItemKind::Struct,
            summary: "An asynchronous HTTP client.".to_owned(),
            signature: None,
            deprecated: false,
            reexport: true,
        }],
    }
}

#[test]
fn projection_preserves_exact_items_and_counts() {
    let contract = to_contract(sample(), "0.13.4");
    assert_eq!(contract.resolved_version, "0.13.4");
    assert_eq!(contract.totals.direct[0].kind, RustdocItemKind::Struct);
    assert_eq!(contract.totals.direct[0].count, 1);
    let [client] = contract.items.as_slice() else {
        panic!("expected one item")
    };
    assert_eq!(client.kind, RustdocItemKind::Struct);
    assert!(client.reexport);
}

#[test]
fn an_absent_module_names_the_crate_and_version() {
    let mapped = module_not_found("reqwest", "0.13.4", "reqwest::nope");
    let ApiError::ModuleNotFoundError(error) = mapped else {
        panic!("expected module_not_found")
    };
    assert_eq!(error.module, "reqwest::nope");
    assert_eq!(error.crate_name, "reqwest");
    assert_eq!(error.resolved_version, "0.13.4");
}
