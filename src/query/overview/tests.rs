use super::*;
use rustdoc_types::{
    Crate, Generics, Id, Item, ItemEnum, Module, Struct, StructKind, Target, Union, Visibility,
};
use std::collections::HashMap;

fn item(id: u32, name: &str, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: Some(name.to_owned()),
        span: None,
        visibility: Visibility::Public,
        docs: None,
        links: HashMap::new(),
        attrs: vec![],
        deprecation: None,
        stability: None,
        const_stability: None,
        inner,
    }
}
fn module(items: &[u32]) -> ItemEnum {
    ItemEnum::Module(Module {
        is_crate: false,
        items: items.iter().copied().map(Id).collect(),
        is_stripped: false,
    })
}
fn generics() -> Generics {
    Generics {
        params: vec![],
        where_predicates: vec![],
    }
}
fn entry(items: Vec<Item>) -> CrateEntry {
    let krate = Crate {
        root: Id(0),
        crate_version: None,
        includes_private: false,
        index: items.into_iter().map(|item| (item.id, item)).collect(),
        paths: HashMap::new(),
        external_crates: HashMap::new(),
        target: Target {
            triple: String::new(),
            target_features: vec![],
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    };
    let (surface, _, _) = crate::rustdoc::Surface::build(&krate);
    CrateEntry::new(krate, surface, vec![], crate::search::FtsDb::dummy())
}
fn count(counts: &[KindCount], kind: rustdoc_types::ItemKind) -> usize {
    counts
        .iter()
        .find(|row| row.kind == kind)
        .map_or(0, |row| row.count)
}

#[test]
fn overview_preserves_direct_item_kinds_and_exact_counts() {
    let entry = entry(vec![
        item(0, "krate", module(&[1, 2, 3])),
        item(
            1,
            "Thing",
            ItemEnum::Struct(Struct {
                kind: StructKind::Unit,
                generics: generics(),
                impls: vec![],
            }),
        ),
        item(
            2,
            "Bits",
            ItemEnum::Union(Union {
                generics: generics(),
                fields: vec![],
                has_stripped_fields: false,
                impls: vec![],
            }),
        ),
        item(3, "sub", module(&[])),
    ]);
    let overview = build_overview(&entry, None, false).expect("root overview");
    assert_eq!(overview.items.len(), 3);
    assert!(overview
        .items
        .iter()
        .any(|item| item.kind == rustdoc_types::ItemKind::Union && item.name == "Bits"));
    assert_eq!(
        count(&overview.totals.direct, rustdoc_types::ItemKind::Struct),
        1
    );
    assert_eq!(
        count(&overview.totals.direct, rustdoc_types::ItemKind::Union),
        1
    );
    assert_eq!(
        count(&overview.totals.direct, rustdoc_types::ItemKind::Module),
        1
    );
}
