use std::collections::HashMap;
use std::path::PathBuf;

use rustdoc_types::{
    Crate, ExternalCrate, Id, Item, ItemEnum, ItemKind, ItemSummary, Module, Target, Use,
    Visibility,
};

use super::*;

fn item(id: u32, name: &str, vis: Visibility, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: Some(name.to_owned()),
        span: None,
        visibility: vis,
        docs: None,
        links: HashMap::new(),
        attrs: vec![],
        deprecation: None,
        stability: None,
        const_stability: None,
        inner,
    }
}

fn item_with_docs(id: u32, name: &str, docs: &str, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: Some(name.to_owned()),
        span: None,
        visibility: Visibility::Public,
        docs: Some(docs.to_owned()),
        links: HashMap::new(),
        attrs: vec![],
        deprecation: None,
        stability: None,
        const_stability: None,
        inner,
    }
}

fn module_items(items: &[u32]) -> ItemEnum {
    ItemEnum::Module(Module {
        is_crate: false,
        items: items.iter().copied().map(Id).collect(),
        is_stripped: false,
    })
}

fn make_crate(root: u32, items: Vec<Item>) -> Crate {
    Crate {
        root: Id(root),
        crate_version: None,
        includes_private: false,
        index: items.into_iter().map(|i| (i.id, i)).collect(),
        paths: HashMap::new(),
        external_crates: HashMap::new(),
        target: Target {
            triple: String::new(),
            target_features: vec![],
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    }
}

fn test_entry() -> CrateEntry {
    // krate           (module, id=0)
    //  ├── Alpha      (ExternType, id=1)
    //  ├── Beta       (ExternType, id=2)
    //  └── sub        (module, id=3)
    //       └── Gamma (ExternType, id=4)
    let items = vec![
        item(0, "krate", Visibility::Public, module_items(&[1, 2, 3])),
        item(1, "Alpha", Visibility::Public, ItemEnum::ExternType),
        item(2, "Beta", Visibility::Public, ItemEnum::ExternType),
        item(3, "sub", Visibility::Public, module_items(&[4])),
        item(4, "Gamma", Visibility::Public, ItemEnum::ExternType),
    ];
    let mut c = make_crate(0, items);
    c.paths.insert(
        Id(3),
        ItemSummary {
            crate_id: 0,
            path: vec!["krate".to_owned(), "internal_sub".to_owned()],
            kind: ItemKind::Module,
        },
    );
    let (surface, _, _) = crate::rustdoc::Surface::build(&c);
    CrateEntry::new(c, surface, vec![], crate::search::FtsDb::dummy())
}

fn test_entry_with_docs() -> CrateEntry {
    let items = vec![
        item(0, "krate", Visibility::Public, module_items(&[1])),
        item_with_docs(
            1,
            "Widget",
            "A wonderful widget.\n\nMore details here.",
            ItemEnum::ExternType,
        ),
    ];
    let c = make_crate(0, items);
    let (surface, _, _) = crate::rustdoc::Surface::build(&c);
    CrateEntry::new(c, surface, vec![], crate::search::FtsDb::dummy())
}

#[test]
fn no_filters_returns_all_sorted_alpha() {
    let entry = test_entry();
    let result = find_items(&entry, None, None, None, false);
    assert_eq!(result.rows.len(), entry.surface.len());
    let paths: Vec<_> = result.rows.iter().map(|r| r.path.as_str()).collect();
    let mut sorted = paths.clone();
    sorted.sort_unstable();
    assert_eq!(paths, sorted, "results must be alphabetically ordered");
}

#[test]
fn kind_filter_returns_only_matching() {
    let entry = test_entry();
    let result = find_items(&entry, None, Some(ItemKind::Module), None, false);
    assert!(!result.rows.is_empty());
    assert!(
        result.rows.iter().all(|r| r.kind == ItemKind::Module),
        "got: {:?}",
        result.rows
    );
}

#[test]
fn module_filter_prefix() {
    let entry = test_entry();
    let result = find_items(&entry, None, None, Some("krate::sub"), false);
    assert!(!result.rows.is_empty());
    assert!(result
        .rows
        .iter()
        .all(|r| r.path.starts_with("krate::sub::")));
    assert!(result.rows.iter().all(|r| r.path != "krate::sub"));
}

#[test]
fn canonical_module_filter_uses_the_public_module_path() {
    let entry = test_entry();
    let result = find_items(&entry, None, None, Some("krate::internal_sub"), false);

    assert_eq!(
        result
            .rows
            .iter()
            .map(|row| row.path.as_str())
            .collect::<Vec<_>>(),
        ["krate::sub::Gamma"]
    );
}

#[test]
fn module_exact_match_excludes_longer() {
    let entry = test_entry();
    let result = find_items(&entry, None, None, Some("krate::sub"), false);
    for row in &result.rows {
        assert!(
            row.path.starts_with("krate::sub::"),
            "unexpected path: {}",
            row.path
        );
    }
}

#[test]
fn query_ranks_matching_entries() {
    let entry = test_entry();
    let result = find_items(&entry, Some("Alpha"), None, None, false);
    assert!(!result.rows.is_empty());
    assert!(result.rows.iter().any(|r| r.path.contains("Alpha")));
}

#[test]
fn query_no_match_returns_empty() {
    let entry = test_entry();
    let result = find_items(&entry, Some("zzxzxzxzxzx"), None, None, false);
    assert!(result.rows.is_empty());
}

#[test]
fn zero_match_explains_when_kind_filter_excludes_name_matches() {
    let entry = test_entry();

    let result = find_items(&entry, Some("Alpha"), Some(ItemKind::Struct), None, false);

    assert!(result.rows.is_empty());
    let hint = result.hint.expect("zero results include a hint");
    assert!(hint.contains("kind `struct`"), "hint: {hint}");
    assert!(hint.contains("extern_type"), "hint: {hint}");
    assert!(
        !hint.contains("re-exported from dependency"),
        "kind mismatch must not suggest a dependency re-export: {hint}"
    );
}

#[test]
fn zero_match_identifies_an_unexpandable_dependency_reexport() {
    // `krate::Thing` is public, but its dependency has no docs.rs URL from
    // which expansion can load rustdoc JSON.
    let mut krate = make_crate(
        0,
        vec![
            item(0, "krate", Visibility::Public, module_items(&[2])),
            item(
                2,
                "Thing",
                Visibility::Public,
                ItemEnum::Use(Use {
                    source: "some_dep::Thing".to_owned(),
                    name: "Thing".to_owned(),
                    id: Some(Id(1)),
                    is_glob: false,
                }),
            ),
        ],
    );
    krate.paths.insert(
        Id(1),
        ItemSummary {
            crate_id: 1,
            path: vec!["some_dep".to_owned(), "Thing".to_owned()],
            kind: ItemKind::ExternType,
        },
    );
    krate.external_crates.insert(
        1,
        ExternalCrate {
            name: "some_dep".to_owned(),
            html_root_url: None,
            path: PathBuf::new(),
        },
    );
    let (surface, _, _) = crate::rustdoc::Surface::build(&krate);
    let entry = CrateEntry::new(krate, surface, vec![], crate::search::FtsDb::dummy());

    let result = find_items(&entry, Some("Thing"), None, None, false);

    assert!(result.rows.is_empty());
    let hint = result.hint.expect("zero results include a hint");
    assert!(hint.contains("krate::Thing"), "hint: {hint}");
    assert!(hint.contains("some_dep"), "hint: {hint}");
    assert!(hint.contains("metadata"), "hint: {hint}");
}

#[test]
fn with_summary_true_populates_field() {
    let entry = test_entry_with_docs();
    let result = find_items(&entry, None, Some(ItemKind::ExternType), None, true);
    assert!(!result.rows.is_empty());
    assert!(
        result.rows[0].summary.is_some(),
        "summary must be populated when with_summary=true"
    );
}

#[test]
fn with_summary_false_omits_field() {
    let entry = test_entry_with_docs();
    let result = find_items(&entry, None, Some(ItemKind::ExternType), None, false);
    assert!(!result.rows.is_empty());
    assert!(result.rows[0].summary.is_none());
}

#[test]
fn kind_and_module_combined() {
    let entry = test_entry();
    let result = find_items(
        &entry,
        None,
        Some(ItemKind::ExternType),
        Some("krate::sub"),
        false,
    );
    assert!(!result.rows.is_empty());
    for row in &result.rows {
        assert_eq!(row.kind, ItemKind::ExternType);
        assert!(row.path.starts_with("krate::sub::"));
    }
}

/// ISSUES.md #9 against ground-truth rustdoc JSON. These are assertions about
/// query relevance, so they live with the documentation-query implementation
/// rather than the cache that supplies its prepared crate entries.
#[tokio::test]
#[ignore = "network + filesystem"]
async fn documentation_queries_rank_the_applicable_public_api_first() {
    let cache = crate::cache::CrateCache::new().expect("test cache initializes");
    for (krate, module, query, expected) in [
        (
            "reqwest",
            None,
            "send HTTP request with timeout",
            "reqwest::Request",
        ),
        (
            "axum",
            Some("axum::extract"),
            "websocket upgrade",
            "axum::extract::WebSocketUpgrade",
        ),
        (
            "tokio",
            None,
            "spawn a blocking task on a dedicated thread pool",
            "tokio::task::spawn_blocking",
        ),
    ] {
        let entry = cache
            .get_with_report(krate, "latest")
            .await
            .unwrap_or_else(|error| panic!("{krate}: {error:?}"))
            .entry;
        let page = find_document_items(&entry, query, None, module, None, 3, true)
            .await
            .expect("documentation search");
        let found: Vec<_> = page.rows.iter().map(|row| row.path.as_str()).collect();

        assert_eq!(
            page.rows.first().map(|row| row.path.as_str()),
            Some(expected),
            "{krate}: {query:?} must rank the applicable API first, got {found:?}"
        );
    }

    let entry = cache
        .get_with_report("tokio", "latest")
        .await
        .expect("Tokio cache entry")
        .entry;
    let page = find_document_items(
        &entry,
        "lock an asynchronous mutex",
        None,
        None,
        None,
        50,
        true,
    )
    .await
    .expect("method discovery search");
    assert!(
        page.rows
            .iter()
            .any(|row| row.path == "tokio::sync::Mutex::lock"),
        "doc query must discover the inherent method"
    );
    assert!(matches!(
        crate::query::resolve_item_detail(&entry, "tokio::sync::Mutex::lock", false, false),
        crate::query::ItemDetailLookup::Found(_)
    ));
}
