use std::collections::HashMap;
use std::path::PathBuf;

use rustdoc_types::{
    Crate, ExternalCrate, Generics, Id, Item, ItemEnum, ItemKind, ItemSummary, Module, Struct,
    StructKind, Target, Use, Visibility,
};

use crate::cache::CrateEntry;
use crate::rustdoc::Surface;
use crate::search::FtsDb;

pub(super) fn entry() -> CrateEntry {
    let krate = Crate {
        root: Id(0),
        crate_version: Some("1.0.0".to_owned()),
        includes_private: false,
        index: [
            (Id(0), item(0, "krate", None, module(&[1, 6]))),
            (
                Id(1),
                item(
                    1,
                    "Widget",
                    Some("A deterministic widget.\n\nMore details."),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: Generics {
                            params: Vec::new(),
                            where_predicates: Vec::new(),
                        },
                        impls: vec![],
                    }),
                ),
            ),
            (
                Id(6),
                item(
                    6,
                    "MissingThing",
                    None,
                    ItemEnum::Use(Use {
                        source: "missing_dep::MissingThing".to_owned(),
                        name: "MissingThing".to_owned(),
                        id: Some(Id(7)),
                        is_glob: false,
                    }),
                ),
            ),
        ]
        .into_iter()
        .collect(),
        paths: HashMap::from([
            (
                Id(1),
                ItemSummary {
                    crate_id: 0,
                    path: vec![
                        "krate".to_owned(),
                        "internal".to_owned(),
                        "Widget".to_owned(),
                    ],
                    kind: ItemKind::Struct,
                },
            ),
            (
                Id(7),
                ItemSummary {
                    crate_id: 1,
                    path: vec!["missing_dep".to_owned(), "MissingThing".to_owned()],
                    kind: ItemKind::Struct,
                },
            ),
        ]),
        external_crates: HashMap::from([(
            1,
            ExternalCrate {
                name: "missing_dep".to_owned(),
                html_root_url: None,
                path: PathBuf::new(),
            },
        )]),
        target: Target {
            triple: String::new(),
            target_features: Vec::new(),
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    };
    let (mut surface, _, _) = Surface::build(&krate);
    surface.add_resolvable_inherent_methods(&krate, &[]);
    CrateEntry::new(krate, surface, Vec::new(), FtsDb::dummy())
}

fn item(id: u32, name: &str, docs: Option<&str>, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: Some(name.to_owned()),
        span: None,
        visibility: Visibility::Public,
        docs: docs.map(ToOwned::to_owned),
        links: HashMap::new(),
        attrs: Vec::new(),
        deprecation: None,
        stability: None,
        const_stability: None,
        inner,
    }
}

fn module(items: &[u32]) -> ItemEnum {
    ItemEnum::Module(Module {
        is_crate: true,
        items: items.iter().copied().map(Id).collect(),
        is_stripped: false,
    })
}
