use super::*;
use rustdoc_types::{
    Crate, ExternalCrate, Function, FunctionHeader, FunctionSignature, Generics, Id, Impl, Item,
    ItemEnum, ItemSummary, Module, Struct, StructKind, Target, Type, Use, Visibility,
};
use std::collections::HashMap;

/// Build a bare [`Item`] with the fields the walk inspects; the rest are
/// defaulted to empty.
fn item(id: u32, name: Option<&str>, vis: Visibility, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: name.map(ToOwned::to_owned),
        span: None,
        visibility: vis,
        docs: None,
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
        is_crate: false,
        items: items.iter().copied().map(Id).collect(),
        is_stripped: false,
    })
}

fn use_(name: &str, target: u32, is_glob: bool) -> ItemEnum {
    ItemEnum::Use(Use {
        source: name.to_owned(),
        name: name.to_owned(),
        id: Some(Id(target)),
        is_glob,
    })
}

fn krate(root: u32, items: Vec<Item>) -> Crate {
    Crate {
        root: Id(root),
        crate_version: None,
        includes_private: false,
        index: items.into_iter().map(|i| (i.id, i)).collect(),
        paths: HashMap::new(),
        external_crates: HashMap::new(),
        target: Target {
            triple: String::new(),
            target_features: Vec::new(),
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    }
}

fn function() -> Function {
    Function {
        sig: FunctionSignature {
            inputs: vec![],
            output: None,
            is_c_variadic: false,
        },
        generics: Generics {
            params: vec![],
            where_predicates: vec![],
        },
        header: FunctionHeader {
            is_const: false,
            is_unsafe: false,
            is_async: false,
            abi: rustdoc_types::Abi::Rust,
        },
        has_body: true,
        default_unstable: None,
    }
}

fn inherent_impl(id: u32, items: &[u32]) -> Item {
    item(
        id,
        None,
        Visibility::Public,
        ItemEnum::Impl(Impl {
            is_unsafe: false,
            generics: Generics {
                params: vec![],
                where_predicates: vec![],
            },
            provided_trait_methods: vec![],
            trait_: None,
            for_: Type::Primitive("_".to_owned()),
            items: items.iter().copied().map(Id).collect(),
            is_negative: false,
            is_synthetic: false,
            blanket_impl: None,
        }),
    )
}

#[test]
fn finalization_adds_a_unique_public_inherent_method_to_the_surface() {
    let c = krate(
        0,
        vec![
            item(0, Some("krate"), Visibility::Public, module(&[1])),
            item(
                1,
                Some("Widget"),
                Visibility::Public,
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: Generics {
                        params: vec![],
                        where_predicates: vec![],
                    },
                    impls: vec![Id(10)],
                }),
            ),
            inherent_impl(10, &[11]),
            item(
                11,
                Some("run"),
                Visibility::Public,
                ItemEnum::Function(function()),
            ),
        ],
    );
    let (mut surface, _, _) = Surface::build(&c);

    surface.add_resolvable_inherent_methods(&c, &[]);

    let method = surface
        .get_by_path("krate::Widget::run")
        .expect("method is searchable");
    assert_eq!(method.id, Id(11));
    assert_eq!(method.kind, ItemKind::Function);
}

#[test]
fn finalization_excludes_private_and_ambiguous_inherent_methods() {
    let c = krate(
        0,
        vec![
            item(0, Some("krate"), Visibility::Public, module(&[1])),
            item(
                1,
                Some("Widget"),
                Visibility::Public,
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: Generics {
                        params: vec![],
                        where_predicates: vec![],
                    },
                    impls: vec![Id(10), Id(12)],
                }),
            ),
            inherent_impl(10, &[11, 13]),
            item(
                11,
                Some("hidden"),
                Visibility::Crate,
                ItemEnum::Function(function()),
            ),
            item(
                13,
                Some("duplicate"),
                Visibility::Public,
                ItemEnum::Function(function()),
            ),
            inherent_impl(12, &[14]),
            item(
                14,
                Some("duplicate"),
                Visibility::Public,
                ItemEnum::Function(function()),
            ),
        ],
    );
    let (mut surface, _, _) = Surface::build(&c);
    surface.add_resolvable_inherent_methods(&c, &[]);

    assert!(surface.get_by_path("krate::Widget::hidden").is_none());
    assert!(surface.get_by_path("krate::Widget::duplicate").is_none());
}

#[test]
fn walks_pub_items_reexports_and_globs() {
    // krate (root, id 0) -> [Foo(1), `use internal::Bar`(2->4), sub(3), `use sub::*`(6->3)]
    //   Foo(1): pub leaf
    //   Bar(4): leaf with NON-public visibility, reachable only via re-export 2
    //   sub(3): pub module -> [baz(5)]
    //   baz(5): pub leaf  (reachable as sub::baz AND, via glob 6, as krate::baz)
    let c = krate(
        0,
        vec![
            item(0, Some("krate"), Visibility::Public, module(&[1, 2, 3, 6])),
            item(1, Some("Foo"), Visibility::Public, ItemEnum::ExternType),
            item(2, Some("Bar"), Visibility::Public, use_("Bar", 4, false)),
            item(3, Some("sub"), Visibility::Public, module(&[5])),
            item(4, Some("Bar"), Visibility::Crate, ItemEnum::ExternType),
            item(5, Some("baz"), Visibility::Public, ItemEnum::ExternType),
            item(6, Some("glob"), Visibility::Public, use_("glob", 3, true)),
        ],
    );

    let (s, pending, _named) = Surface::build(&c);
    assert!(pending.is_empty());

    assert_eq!(s.get_by_path("krate").unwrap().kind, ItemKind::Module);
    assert_eq!(s.get_by_path("krate::Foo").unwrap().id, Id(1));
    assert_eq!(s.get_by_path("krate::Bar").unwrap().id, Id(4));
    assert_eq!(s.get_by_path("krate::sub").unwrap().kind, ItemKind::Module);

    // BFS: the glob makes `baz` reachable at depth 1 (krate::baz), beating the
    // depth-2 krate::sub::baz — shortest path is canonical, longer is alias.
    assert_eq!(s.get_by_id(Id(5)).unwrap().path, vec!["krate", "baz"]);
    assert_eq!(
        s.get_by_path("krate::sub::baz").map(|e| e.id),
        Some(Id(5)),
        "longer path should resolve via alias to the same item"
    );

    assert_eq!(
        s.get_by_path("krate::Foo").unwrap().id,
        s.get_by_id(Id(1)).unwrap().id
    );
}

#[test]
fn excludes_non_public_direct_children() {
    let c = krate(
        0,
        vec![
            item(0, Some("krate"), Visibility::Public, module(&[1])),
            item(1, Some("Hidden"), Visibility::Crate, ItemEnum::ExternType),
        ],
    );
    let (s, pending, _named) = Surface::build(&c);
    assert!(pending.is_empty());
    assert!(s.get_by_path("krate::Hidden").is_none());
    assert_eq!(s.len(), 1); // only the root module
}

#[test]
fn a_public_item_resolves_through_its_local_canonical_rustdoc_path() {
    let mut c = krate(
        0,
        vec![
            item(0, Some("krate"), Visibility::Public, module(&[1])),
            item(1, Some("Widget"), Visibility::Public, ItemEnum::ExternType),
        ],
    );
    c.paths.insert(
        Id(1),
        ItemSummary {
            crate_id: 0,
            path: vec!["krate".to_owned(), "inner".to_owned(), "Widget".to_owned()],
            kind: ItemKind::Struct,
        },
    );

    let (surface, _, _) = Surface::build(&c);
    let public = surface
        .get_by_path("krate::Widget")
        .expect("public path resolves");
    let canonical = surface.get_by_path("krate::inner::Widget");

    assert_eq!(canonical.map(|entry| entry.id), Some(public.id));
    assert_eq!(canonical.map(|entry| &entry.path), Some(&public.path));
}

#[test]
fn named_external_reexport_preserves_rust_and_docs_rs_identities() {
    let mut c = krate(
        0,
        vec![
            item(0, Some("axum"), Visibility::Public, module(&[1])),
            item(
                1,
                Some("Error"),
                Visibility::Public,
                use_("Error", 99, false),
            ),
        ],
    );
    c.paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 1,
            path: vec![
                "axum_core".to_owned(),
                "error".to_owned(),
                "Error".to_owned(),
            ],
            kind: ItemKind::Struct,
        },
    );
    c.external_crates.insert(
        1,
        ExternalCrate {
            name: "axum_core".to_owned(),
            html_root_url: Some(
                "https://docs.rs/axum-core/0.5.6/x86_64-unknown-linux-gnu/".to_owned(),
            ),
            path: std::path::PathBuf::new(),
        },
    );

    let (s, pending, named) = Surface::build(&c);

    assert!(
        pending.is_empty(),
        "named re-export should not produce a PendingExpansion"
    );
    assert_eq!(named.len(), 1, "one PendingNamedItem expected");
    let ni = &named[0];
    assert_eq!(ni.dependency.rust_crate_name, "axum_core");
    assert_eq!(ni.dependency.package_name, "axum-core");
    assert_eq!(ni.dependency.version, "0.5.6");
    assert_eq!(ni.dep_item_path, vec!["axum_core", "error", "Error"]);
    assert_eq!(ni.local_path, vec!["axum", "Error"]);
    assert_eq!(ni.kind, ItemKind::Struct);
    assert!(s.get_by_path("axum::Error").is_none());
}

/// Resolution uses dep.paths (keyed by canonical path), not the parent's ID,
/// because ID namespaces differ.
#[test]
fn apply_named_item_records_when_dep_has_item() {
    let (mut surface, _, _) = Surface::build(&krate(
        0,
        vec![item(0, Some("mycrate"), Visibility::Public, module(&[]))],
    ));

    let mut dep = krate(
        0,
        vec![
            item(0, Some("http"), Visibility::Public, module(&[99])),
            item(99, Some("Method"), Visibility::Public, ItemEnum::ExternType),
        ],
    );
    dep.paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 0,
            path: vec!["http".to_owned(), "Method".to_owned()],
            kind: ItemKind::Struct,
        },
    );

    surface
        .apply_named_item(
            &dep,
            &["http".to_owned(), "Method".to_owned()],
            vec!["mycrate".to_owned(), "Method".to_owned()],
            ItemKind::Struct,
            1,
        )
        .expect("named item resolves");

    let entry = surface
        .get_by_path("mycrate::Method")
        .expect("Method should be in surface");
    assert_eq!(entry.id, Id(99));
    assert_eq!(entry.dep_idx, 1);
    assert_eq!(entry.kind, ItemKind::Struct);
}

/// so the caller can retain failed-expansion provenance.
#[test]
fn apply_named_item_noop_when_absent() {
    let (mut surface, _, _) = Surface::build(&krate(
        0,
        vec![item(0, Some("mycrate"), Visibility::Public, module(&[]))],
    ));
    let dep = krate(
        0,
        vec![item(0, Some("http"), Visibility::Public, module(&[]))],
    );
    let result = surface.apply_named_item(
        &dep,
        &["http".to_owned(), "Method".to_owned()],
        vec!["mycrate".to_owned(), "Method".to_owned()],
        ItemKind::Struct,
        1,
    );

    assert_eq!(result, Err(ExpansionFailureReason::TargetNotFound));
    assert!(surface.get_by_path("mycrate::Method").is_none());
}

#[test]
fn external_module_reexport_yields_pending() {
    let mut c = krate(
        0,
        vec![
            item(0, Some("mycrate"), Visibility::Public, module(&[1])),
            item(
                1,
                Some("header"),
                Visibility::Public,
                use_("header", 99, false),
            ),
        ],
    );
    c.paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 1,
            path: vec!["http".to_owned(), "header".to_owned()],
            kind: ItemKind::Module,
        },
    );
    c.external_crates.insert(
        1,
        ExternalCrate {
            name: "http".to_owned(),
            html_root_url: Some("https://docs.rs/http/1.2.0/http/".to_owned()),
            path: std::path::PathBuf::new(),
        },
    );

    let (s, pending, _named) = Surface::build(&c);

    assert_eq!(
        pending.len(),
        1,
        "one module re-export should produce one pending"
    );
    let exp = &pending[0];
    assert_eq!(exp.dependency.rust_crate_name, "http");
    assert_eq!(exp.dependency.package_name, "http");
    assert_eq!(exp.dependency.version, "1.2.0");
    assert_eq!(exp.dep_module_path, vec!["http", "header"]);
    assert_eq!(exp.local_prefix, vec!["mycrate", "header"]);

    // The module itself is recorded immediately so it appears in path queries.
    assert_eq!(
        s.get_by_path("mycrate::header").unwrap().kind,
        ItemKind::Module
    );
}

#[test]
fn parse_docs_rs_dependency_keeps_package_and_rust_identifiers_distinct() {
    let dep = parse_docs_rs_dependency(
        "axum_core",
        "https://docs.rs/axum-core/0.5.6-beta.1/x86_64-unknown-linux-gnu/",
    )
    .expect("valid docs.rs dependency URL");
    assert_eq!(dep.rust_crate_name, "axum_core");
    assert_eq!(dep.package_name, "axum-core");
    assert_eq!(dep.version, "0.5.6-beta.1");
}

#[test]
fn parse_docs_rs_dependency_rejects_untrusted_or_invalid_urls() {
    for url in [
        "https://example.com/http/1.0.0/",
        "http://docs.rs/http/1.0.0/",
        "https://docs.rs/http/latest/",
        "https://docs.rs/http/1.0/",
    ] {
        assert!(
            parse_docs_rs_dependency("http", url).is_none(),
            "{url} must not become a docs.rs fetch target"
        );
    }
}

#[test]
fn apply_expansion_stitches_module_items() {
    let dep = krate(
        0,
        vec![
            item(0, Some("http"), Visibility::Public, module(&[1])),
            item(1, Some("header"), Visibility::Public, module(&[2])),
            item(
                2,
                Some("HeaderName"),
                Visibility::Public,
                ItemEnum::ExternType,
            ),
        ],
    );

    // Parent surface starts with just the "reqwest" root.
    let (mut surface, _, _) = Surface::build(&krate(
        0,
        vec![item(0, Some("reqwest"), Visibility::Public, module(&[]))],
    ));

    let exp = PendingExpansion {
        dependency: DocsRsDependency {
            rust_crate_name: "http".to_owned(),
            package_name: "http".to_owned(),
            version: "1.2.0".to_owned(),
        },
        dep_module_path: vec!["http".to_owned(), "header".to_owned()],
        local_prefix: vec!["reqwest".to_owned(), "header".to_owned()],
    };
    surface
        .apply_expansion(&dep, &exp, 1)
        .expect("module expansion resolves");

    assert!(
        surface.get_by_path("reqwest::header::HeaderName").is_some(),
        "HeaderName should be stitched under reqwest::header"
    );
}

#[test]
fn apply_expansion_skips_already_present_paths() {
    let dep = krate(
        0,
        vec![
            item(0, Some("http"), Visibility::Public, module(&[1])),
            item(1, Some("header"), Visibility::Public, module(&[2])),
            item(
                2,
                Some("HeaderName"),
                Visibility::Public,
                ItemEnum::ExternType,
            ),
        ],
    );

    let (mut surface, _, _) = Surface::build(&krate(
        0,
        vec![item(0, Some("reqwest"), Visibility::Public, module(&[]))],
    ));

    let exp = PendingExpansion {
        dependency: DocsRsDependency {
            rust_crate_name: "http".to_owned(),
            package_name: "http".to_owned(),
            version: "1.2.0".to_owned(),
        },
        dep_module_path: vec!["http".to_owned(), "header".to_owned()],
        local_prefix: vec!["reqwest".to_owned(), "header".to_owned()],
    };
    surface
        .apply_expansion(&dep, &exp, 1)
        .expect("module expansion resolves");
    let count_before = surface.len();
    surface
        .apply_expansion(&dep, &exp, 1)
        .expect("repeated module expansion resolves");
    assert_eq!(
        surface.len(),
        count_before,
        "second expansion should add nothing"
    );
}
