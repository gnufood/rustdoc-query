use std::collections::HashMap;

use rustdoc_types::{
    Attribute, Crate, Enum, Function, FunctionHeader, FunctionSignature, GenericArg, GenericArgs,
    Generics, Id, Impl, Item, ItemEnum, ItemKind, ItemSummary, Module, Struct, Target, Type,
    Variant, VariantKind, Visibility,
};

use crate::cache::CrateEntry;
use crate::query::detail::{
    build_item_detail, render_fields, render_variants, resolve_item_detail, split_impls,
    ItemDetailLookup,
};
use crate::rustdoc::{render_fn_sig, render_generics, render_path, render_type, Surface};

fn item_bare(id: u32, name: Option<&str>, vis: Visibility, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: name.map(ToOwned::to_owned),
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

fn item_with_attrs(id: u32, name: &str, attrs: Vec<Attribute>, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: Some(name.to_owned()),
        span: None,
        visibility: Visibility::Public,
        docs: None,
        links: HashMap::new(),
        attrs,
        deprecation: None,
        stability: None,
        const_stability: None,
        inner,
    }
}

fn item_docs(id: u32, name: &str, docs: &str, inner: ItemEnum) -> Item {
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

fn module(items: &[u32]) -> ItemEnum {
    ItemEnum::Module(Module {
        is_crate: false,
        items: items.iter().copied().map(Id).collect(),
        is_stripped: false,
    })
}

fn stub_generics() -> Generics {
    Generics {
        params: vec![],
        where_predicates: vec![],
    }
}

fn stub_fn_inner() -> Function {
    Function {
        sig: FunctionSignature {
            inputs: vec![],
            output: None,
            is_c_variadic: false,
        },
        generics: stub_generics(),
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

fn make_crate_with_paths(root: u32, items: Vec<Item>, paths: HashMap<Id, ItemSummary>) -> Crate {
    Crate {
        root: Id(root),
        crate_version: None,
        includes_private: false,
        index: items.into_iter().map(|i| (i.id, i)).collect(),
        paths,
        external_crates: HashMap::new(),
        target: Target {
            triple: String::new(),
            target_features: vec![],
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    }
}

fn empty_krate() -> Crate {
    make_crate(0, vec![])
}

fn entry(c: Crate) -> CrateEntry {
    let (mut surface, _, _) = Surface::build(&c);
    surface.add_resolvable_inherent_methods(&c, &[]);
    CrateEntry::new(c, surface, vec![], crate::search::FtsDb::dummy())
}

#[test]
fn render_type_primitives() {
    let k = empty_krate();
    assert_eq!(render_type(&k, &Type::Primitive("u32".into())), "u32");
    assert_eq!(render_type(&k, &Type::Generic("T".into())), "T");
    assert_eq!(render_type(&k, &Type::Infer), "_");
}

#[test]
fn render_type_borrowed_ref() {
    let k = empty_krate();
    let t = Type::BorrowedRef {
        lifetime: None,
        is_mutable: false,
        type_: Box::new(Type::Primitive("str".into())),
    };
    assert_eq!(render_type(&k, &t), "&str");

    let t_mut = Type::BorrowedRef {
        lifetime: Some("'a".into()),
        is_mutable: true,
        type_: Box::new(Type::Generic("T".into())),
    };
    assert_eq!(render_type(&k, &t_mut), "&'a mut T");
}

#[test]
fn render_type_compound() {
    let k = empty_krate();
    let tuple = Type::Tuple(vec![
        Type::Primitive("u32".into()),
        Type::Primitive("bool".into()),
    ]);
    assert_eq!(render_type(&k, &tuple), "(u32, bool)");

    let slice = Type::Slice(Box::new(Type::Primitive("u8".into())));
    assert_eq!(render_type(&k, &slice), "[u8]");

    let array = Type::Array {
        type_: Box::new(Type::Primitive("u8".into())),
        len: "16".into(),
    };
    assert_eq!(render_type(&k, &array), "[u8; 16]");

    let raw = Type::RawPointer {
        is_mutable: false,
        type_: Box::new(Type::Primitive("u8".into())),
    };
    assert_eq!(render_type(&k, &raw), "*const u8");
}

#[test]
fn render_path_empty_path_falls_back_to_paths_table() {
    let mut paths = HashMap::new();
    paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 2,
            path: vec![
                "core".into(),
                "future".into(),
                "future".into(),
                "Future".into(),
            ],
            kind: rustdoc_types::ItemKind::Trait,
        },
    );
    let k = make_crate_with_paths(0, vec![], paths);
    let p = rustdoc_types::Path {
        path: String::new(),
        id: Id(99),
        args: None,
    };
    assert_eq!(render_path(&k, &p), "Future");
}

#[test]
fn render_type_qualified_path_with_empty_trait_path() {
    let mut paths = HashMap::new();
    paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 2,
            path: vec![
                "core".into(),
                "future".into(),
                "future".into(),
                "Future".into(),
            ],
            kind: rustdoc_types::ItemKind::Trait,
        },
    );
    let k = make_crate_with_paths(0, vec![], paths);
    let ty = Type::QualifiedPath {
        name: "Output".into(),
        args: Some(Box::new(GenericArgs::AngleBracketed {
            args: vec![],
            constraints: vec![],
        })),
        self_type: Box::new(Type::Generic("F".into())),
        trait_: Some(rustdoc_types::Path {
            path: String::new(),
            id: Id(99),
            args: None,
        }),
    };
    assert_eq!(render_type(&k, &ty), "<F as Future>::Output");
}

#[test]
fn render_generics_empty() {
    let k = empty_krate();
    assert_eq!(render_generics(&k, &stub_generics()), "");
}

#[test]
fn render_generics_type_with_bounds() {
    use rustdoc_types::{
        GenericBound, GenericParamDef, GenericParamDefKind, Path, TraitBoundModifier,
    };
    let k = empty_krate();
    let gen = Generics {
        params: vec![GenericParamDef {
            name: "T".into(),
            kind: GenericParamDefKind::Type {
                bounds: vec![GenericBound::TraitBound {
                    trait_: Path {
                        path: "core::fmt::Debug".into(),
                        id: Id(0),
                        args: None,
                    },
                    generic_params: vec![],
                    modifier: TraitBoundModifier::None,
                }],
                default: None,
                is_synthetic: false,
            },
        }],
        where_predicates: vec![],
    };
    assert_eq!(render_generics(&k, &gen), "<T: Debug>");
}

#[test]
fn render_generics_with_where() {
    use rustdoc_types::{
        GenericBound, GenericParamDef, GenericParamDefKind, Path, TraitBoundModifier,
        WherePredicate,
    };
    let k = empty_krate();
    let gen = Generics {
        params: vec![GenericParamDef {
            name: "T".into(),
            kind: GenericParamDefKind::Type {
                bounds: vec![],
                default: None,
                is_synthetic: false,
            },
        }],
        where_predicates: vec![WherePredicate::BoundPredicate {
            type_: Type::Generic("T".into()),
            bounds: vec![GenericBound::TraitBound {
                trait_: Path {
                    path: "core::fmt::Debug".into(),
                    id: Id(0),
                    args: None,
                },
                generic_params: vec![],
                modifier: TraitBoundModifier::None,
            }],
            generic_params: vec![],
        }],
    };
    assert_eq!(render_generics(&k, &gen), "<T> where T: Debug");
}

#[test]
fn render_fn_sig_simple() {
    let k = empty_krate();
    let f = stub_fn_inner();
    assert_eq!(render_fn_sig(&k, "foo", &f), "fn foo()");
}

#[test]
fn render_fn_sig_async_with_return() {
    let k = empty_krate();
    let mut f = stub_fn_inner();
    f.header.is_async = true;
    f.sig.output = Some(Type::Primitive("u32".into()));
    f.sig
        .inputs
        .push(("x".into(), Type::Primitive("i32".into())));
    assert_eq!(render_fn_sig(&k, "bar", &f), "async fn bar(x: i32) -> u32");
}

#[test]
fn render_fn_sig_self_receiver() {
    let k = empty_krate();
    let mut f = stub_fn_inner();
    f.sig.inputs.push((
        "self".into(),
        Type::BorrowedRef {
            lifetime: None,
            is_mutable: false,
            type_: Box::new(Type::Generic("Self".into())),
        },
    ));
    f.sig.inputs.push(("url".into(), Type::Generic("U".into())));
    assert_eq!(render_fn_sig(&k, "get", &f), "fn get(&self, url: U)");
}

#[test]
fn render_fields_plain_struct() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1, 2, 3])),
        item_bare(
            1,
            Some("MyStruct"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Plain {
                    fields: vec![Id(2), Id(3)],
                    has_stripped_fields: false,
                },
                generics: stub_generics(),
                impls: vec![],
            }),
        ),
        item_bare(
            2,
            Some("x"),
            Visibility::Public,
            ItemEnum::StructField(Type::Primitive("i32".into())),
        ),
        item_bare(
            3,
            Some("y"),
            Visibility::Public,
            ItemEnum::StructField(Type::Primitive("bool".into())),
        ),
    ];
    let c = make_crate(0, items);
    let fields = render_fields(&c, &[Id(2), Id(3)]);
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "x");
    assert_eq!(fields[0].type_, "i32");
    assert_eq!(fields[1].name, "y");
    assert_eq!(fields[1].type_, "bool");
}

#[test]
fn render_variants_all_kinds() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("MyEnum"),
            Visibility::Public,
            ItemEnum::Enum(Enum {
                generics: stub_generics(),
                has_stripped_variants: false,
                variants: vec![Id(10), Id(11), Id(12)],
                impls: vec![],
            }),
        ),
        item_bare(
            10,
            Some("Unit"),
            Visibility::Public,
            ItemEnum::Variant(Variant {
                kind: VariantKind::Plain,
                discriminant: None,
            }),
        ),
        item_bare(
            11,
            Some("Tuple"),
            Visibility::Public,
            ItemEnum::Variant(Variant {
                kind: VariantKind::Tuple(vec![Some(Id(20))]),
                discriminant: None,
            }),
        ),
        item_bare(
            20,
            None,
            Visibility::Public,
            ItemEnum::StructField(Type::Primitive("u32".into())),
        ),
        item_bare(
            12,
            Some("Struct"),
            Visibility::Public,
            ItemEnum::Variant(Variant {
                kind: VariantKind::Struct {
                    fields: vec![Id(21)],
                    has_stripped_fields: false,
                },
                discriminant: None,
            }),
        ),
        item_bare(
            21,
            Some("val"),
            Visibility::Public,
            ItemEnum::StructField(Type::Primitive("String".into())),
        ),
    ];
    let c = make_crate(0, items);
    let rows = render_variants(&c, &[Id(10), Id(11), Id(12)]);
    assert_eq!(rows[0].kind, VariantKind::Plain);
    assert!(rows[0].fields.is_empty());
    assert!(matches!(rows[1].kind, VariantKind::Tuple(_)));
    assert_eq!(rows[1].fields[0].type_, "u32");
    assert!(matches!(rows[2].kind, VariantKind::Struct { .. }));
    assert_eq!(rows[2].fields[0].name, "val");
}

fn make_impl(
    id: u32,
    trait_path: Option<&str>,
    item_ids: &[u32],
    is_synthetic: bool,
    blanket: bool,
) -> Item {
    make_impl_with_where(id, trait_path, item_ids, is_synthetic, blanket, vec![])
}

fn make_impl_with_where(
    id: u32,
    trait_path: Option<&str>,
    item_ids: &[u32],
    is_synthetic: bool,
    blanket: bool,
    where_predicates: Vec<rustdoc_types::WherePredicate>,
) -> Item {
    item_bare(
        id,
        None,
        Visibility::Public,
        ItemEnum::Impl(Impl {
            is_unsafe: false,
            generics: Generics {
                params: vec![],
                where_predicates,
            },
            provided_trait_methods: vec![],
            trait_: trait_path.map(|p| rustdoc_types::Path {
                path: p.into(),
                id: Id(0),
                args: None,
            }),
            for_: Type::Primitive("_".into()),
            items: item_ids.iter().copied().map(Id).collect(),
            is_negative: false,
            is_synthetic,
            blanket_impl: if blanket {
                Some(Type::Primitive("_".into()))
            } else {
                None
            },
        }),
    )
}

#[test]
fn split_impls_partitions_correctly() {
    let method = Item {
        id: Id(100),
        crate_id: 0,
        name: Some("run".into()),
        span: None,
        visibility: Visibility::Public,
        docs: None,
        links: HashMap::new(),
        attrs: vec![],
        deprecation: None,
        stability: None,
        const_stability: None,
        inner: ItemEnum::Function(stub_fn_inner()),
    };
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Foo"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10), Id(11), Id(12), Id(13), Id(14)],
            }),
        ),
        make_impl(10, None, &[100], false, false),
        make_impl(11, Some("core::marker::Send"), &[], true, false),
        make_impl(12, Some("core::clone::Clone"), &[], false, false),
        make_impl(13, Some("core::fmt::Debug"), &[200], false, false),
        make_impl(14, Some("core::convert::Into"), &[], false, true),
        method,
        item_bare(
            200,
            Some("fmt"),
            Visibility::Public,
            ItemEnum::Function(stub_fn_inner()),
        ),
    ];
    let c = make_crate(0, items);
    let split = split_impls(&c, &[Id(10), Id(11), Id(12), Id(13), Id(14)], false);

    assert_eq!(split.methods.len(), 1);
    assert_eq!(split.methods[0].name, "run");
    assert_eq!(split.auto_traits, ["Send"]);
    assert_eq!(split.derives, ["Clone"]);
    assert_eq!(split.trait_impls, ["Debug for _"]);
    assert!(split.blanket_impls.is_none());
}

#[test]
fn split_impls_labels_include_the_implementing_type() {
    let krate = make_crate(
        0,
        vec![
            item_bare(0, Some("krate"), Visibility::Public, module(&[])),
            make_impl(10, Some("core::fmt::Debug"), &[99], false, false),
            make_impl(11, Some("core::convert::From"), &[], false, true),
        ],
    );

    let split = split_impls(&krate, &[Id(10), Id(11)], true);

    assert_eq!(split.trait_impls, ["Debug for _"]);
    assert_eq!(split.blanket_impls, Some(vec!["From for _".to_owned()]));
}

#[test]
fn split_impls_distinguishes_same_signature_concrete_impls() {
    let method = |id| {
        item_bare(
            id,
            Some("new"),
            Visibility::Public,
            ItemEnum::Function(stub_fn_inner()),
        )
    };
    let concrete_impl = |id, state: &str, method_id| {
        item_bare(
            id,
            None,
            Visibility::Public,
            ItemEnum::Impl(Impl {
                is_unsafe: false,
                generics: stub_generics(),
                provided_trait_methods: vec![],
                trait_: None,
                for_: Type::ResolvedPath(rustdoc_types::Path {
                    path: "krate::Connection".into(),
                    id: Id(1),
                    args: Some(Box::new(GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Primitive(state.into()))],
                        constraints: vec![],
                    })),
                }),
                items: vec![Id(method_id)],
                is_negative: false,
                is_synthetic: false,
                blanket_impl: None,
            }),
        )
    };
    let c = make_crate(
        0,
        vec![
            item_bare(0, Some("krate"), Visibility::Public, module(&[])),
            concrete_impl(10, "Connected", 100),
            concrete_impl(11, "Closed", 101),
            method(100),
            method(101),
        ],
    );

    let split = split_impls(&c, &[Id(10), Id(11)], false);

    assert_eq!(
        split.methods.len(),
        2,
        "distinct impl methods must not collapse"
    );
    assert_eq!(split.methods[0].name, "new");
    assert_eq!(
        split.methods[0].impl_constraint.as_deref(),
        Some("impl Connection<Closed>")
    );
    assert_eq!(split.methods[1].name, "new");
    assert_eq!(
        split.methods[1].impl_constraint.as_deref(),
        Some("impl Connection<Connected>"),
        "concrete impl targets distinguish same-signature methods"
    );
}

#[test]
fn split_impls_include_blanket_true() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Foo"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10)],
            }),
        ),
        make_impl(10, Some("core::convert::Into"), &[], false, true),
    ];
    let c = make_crate(0, items);
    let split = split_impls(&c, &[Id(10)], true);
    assert!(split.blanket_impls.is_some());
    assert_eq!(split.blanket_impls.as_ref().unwrap(), &["Into for _"]);
}

#[test]
fn split_impls_collects_assoc_consts() {
    let const_item = Item {
        id: Id(101),
        crate_id: 0,
        name: Some("MAX".into()),
        span: None,
        visibility: Visibility::Public,
        docs: None,
        links: HashMap::new(),
        attrs: vec![],
        deprecation: None,
        stability: None,
        const_stability: None,
        inner: ItemEnum::AssocConst {
            type_: Type::Primitive("u32".into()),
            value: Some("255".into()),
            default_unstable: None,
        },
    };
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Foo"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10)],
            }),
        ),
        make_impl(10, None, &[101], false, false),
        const_item,
    ];
    let c = make_crate(0, items);
    let split = split_impls(&c, &[Id(10)], false);
    assert_eq!(split.consts.len(), 1);
    assert_eq!(split.consts[0].name, "MAX");
    assert_eq!(split.consts[0].type_, "u32");
    assert!(split.methods.is_empty());
}

#[test]
fn build_item_detail_enum_non_exhaustive() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_with_attrs(
            1,
            "MyEnum",
            vec![Attribute::NonExhaustive],
            ItemEnum::Enum(Enum {
                generics: stub_generics(),
                has_stripped_variants: false,
                variants: vec![],
                impls: vec![],
            }),
        ),
    ];
    let c = make_crate(0, items);
    let e = entry(c);
    let d = build_item_detail(&e, "krate::MyEnum", false, false).expect("MyEnum exists");
    assert!(d.non_exhaustive, "should be marked non_exhaustive");
}

#[test]
fn build_item_detail_enum_exhaustive() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("MyEnum"),
            Visibility::Public,
            ItemEnum::Enum(Enum {
                generics: stub_generics(),
                has_stripped_variants: false,
                variants: vec![],
                impls: vec![],
            }),
        ),
    ];
    let c = make_crate(0, items);
    let e = entry(c);
    let d = build_item_detail(&e, "krate::MyEnum", false, false).expect("MyEnum exists");
    assert!(!d.non_exhaustive, "should not be marked non_exhaustive");
}

#[test]
fn split_impls_where_clause_goes_to_trait_impls() {
    use rustdoc_types::{
        GenericBound, GenericParamDef, GenericParamDefKind, TraitBoundModifier, WherePredicate,
    };
    let where_preds = vec![WherePredicate::BoundPredicate {
        type_: Type::Generic("T".into()),
        bounds: vec![GenericBound::TraitBound {
            trait_: rustdoc_types::Path {
                path: "core::marker::Send".into(),
                id: Id(0),
                args: None,
            },
            generic_params: vec![],
            modifier: TraitBoundModifier::None,
        }],
        generic_params: vec![],
    }];
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Foo"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10)],
            }),
        ),
        make_impl_with_where(
            10,
            Some("core::marker::Send"),
            &[],
            false,
            false,
            where_preds,
        ),
        item_bare(
            0,
            None,
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: Generics {
                    params: vec![GenericParamDef {
                        name: "T".into(),
                        kind: GenericParamDefKind::Type {
                            bounds: vec![],
                            default: None,
                            is_synthetic: false,
                        },
                    }],
                    where_predicates: vec![],
                },
                impls: vec![],
            }),
        ),
    ];
    let c = make_crate(0, items);
    let split = split_impls(&c, &[Id(10)], false);
    assert!(
        split.derives.is_empty(),
        "conditional Send must NOT be in derives"
    );
    assert_eq!(
        split.trait_impls.len(),
        1,
        "conditional Send must be in trait_impls"
    );
    assert!(
        split.trait_impls[0].contains("Send"),
        "label must contain trait name: {}",
        split.trait_impls[0]
    );
    assert!(
        split.trait_impls[0].contains("where"),
        "label must contain where clause: {}",
        split.trait_impls[0]
    );
}

#[test]
fn split_impls_unconditional_marker_stays_in_derives() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Foo"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10)],
            }),
        ),
        make_impl(10, Some("core::clone::Clone"), &[], false, false),
    ];
    let c = make_crate(0, items);
    let split = split_impls(&c, &[Id(10)], false);
    assert_eq!(
        split.derives,
        ["Clone"],
        "unconditional Clone must stay in derives"
    );
    assert!(split.trait_impls.is_empty());
}

#[test]
fn split_impls_unresolved_explicit_standard_auto_trait_is_not_a_derive() {
    // Dependency rustdoc commonly references core's auto traits by path while
    // omitting their definitions from the dependency crate index: such impls
    // are non-synthetic and empty.
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Bytes"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10), Id(11)],
            }),
        ),
        make_impl(10, Some("Send"), &[], false, false),
        make_impl(11, Some("Sync"), &[], false, false),
    ];
    let krate = make_crate(0, items);

    let split = split_impls(&krate, &[Id(10), Id(11)], false);

    assert_eq!(split.auto_traits, ["Send", "Sync"]);
    assert!(split.derives.is_empty());
    assert!(split.trait_impls.is_empty());
}

#[test]
fn build_item_detail_struct() {
    let items = vec![
        item_docs(0, "krate", "Root.", module(&[1, 2, 3])),
        item_docs(
            1,
            "Point",
            "A 2D point.",
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Plain {
                    fields: vec![Id(2), Id(3)],
                    has_stripped_fields: false,
                },
                generics: stub_generics(),
                impls: vec![],
            }),
        ),
        item_bare(
            2,
            Some("x"),
            Visibility::Public,
            ItemEnum::StructField(Type::Primitive("f32".into())),
        ),
        item_bare(
            3,
            Some("y"),
            Visibility::Public,
            ItemEnum::StructField(Type::Primitive("f32".into())),
        ),
    ];
    let c = make_crate(0, items);
    let e = entry(c);
    let d = build_item_detail(&e, "krate::Point", false, false).expect("Point exists");
    assert_eq!(d.path, "krate::Point");
    assert_eq!(d.kind, ItemKind::Struct);
    assert_eq!(d.docs.as_deref(), Some("A 2D point."));
    assert_eq!(d.fields.len(), 2);
    assert_eq!(d.fields[0].name, "x");
    assert!(!d.non_exhaustive);
    assert!(d.consts.is_empty());
}

#[test]
fn build_item_detail_enum() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1, 10, 11])),
        item_bare(
            1,
            Some("MyEnum"),
            Visibility::Public,
            ItemEnum::Enum(Enum {
                generics: stub_generics(),
                has_stripped_variants: false,
                variants: vec![Id(10), Id(11)],
                impls: vec![],
            }),
        ),
        item_bare(
            10,
            Some("A"),
            Visibility::Public,
            ItemEnum::Variant(Variant {
                kind: VariantKind::Plain,
                discriminant: None,
            }),
        ),
        item_bare(
            11,
            Some("B"),
            Visibility::Public,
            ItemEnum::Variant(Variant {
                kind: VariantKind::Plain,
                discriminant: None,
            }),
        ),
    ];
    let c = make_crate(0, items);
    let e = entry(c);
    let d = build_item_detail(&e, "krate::MyEnum", false, false).expect("MyEnum exists");
    assert_eq!(d.kind, ItemKind::Enum);
    assert_eq!(d.variants.len(), 2);
    assert_eq!(d.variants[0].name, "A");
    assert!(!d.non_exhaustive);
}

#[test]
fn build_item_detail_function() {
    let mut f = stub_fn_inner();
    f.sig.output = Some(Type::Primitive("u32".into()));
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(1, Some("answer"), Visibility::Public, ItemEnum::Function(f)),
    ];
    let c = make_crate(0, items);
    let e = entry(c);
    let d = build_item_detail(&e, "krate::answer", false, false).expect("answer exists");
    assert_eq!(d.kind, ItemKind::Function);
    assert_eq!(d.signature.as_deref(), Some("fn answer() -> u32"));
}

#[test]
fn build_item_detail_unknown_path_returns_none() {
    let c = make_crate(
        0,
        vec![item_bare(0, Some("krate"), Visibility::Public, module(&[]))],
    );
    let e = entry(c);
    assert!(build_item_detail(&e, "krate::DoesNotExist", false, false).is_none());
}

#[test]
fn finalized_public_method_path_resolves_to_the_public_path() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Widget"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10)],
            }),
        ),
        make_impl(10, None, &[11], false, false),
        item_bare(
            11,
            Some("run"),
            Visibility::Public,
            ItemEnum::Function(stub_fn_inner()),
        ),
    ];
    let c = make_crate_with_paths(
        0,
        items,
        HashMap::from([(
            Id(1),
            ItemSummary {
                crate_id: 0,
                path: vec!["krate".into(), "inner".into(), "Widget".into()],
                kind: rustdoc_types::ItemKind::Struct,
            },
        )]),
    );
    let e = entry(c);

    let detail = build_item_detail(&e, "krate::Widget::run", false, false)
        .expect("finalized public method path resolves");
    assert_eq!(detail.path, "krate::Widget::run");
    assert_eq!(detail.impl_constraint.as_deref(), Some("impl _"));
}

#[test]
fn direct_parent_method_path_does_not_resolve_a_private_method() {
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Widget"),
            Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![Id(10)],
            }),
        ),
        make_impl(10, None, &[11], false, false),
        item_bare(
            11,
            Some("hidden"),
            Visibility::Crate,
            ItemEnum::Function(stub_fn_inner()),
        ),
    ];

    assert!(matches!(
        resolve_item_detail(
            &entry(make_crate(0, items)),
            "krate::Widget::hidden",
            false,
            false,
        ),
        ItemDetailLookup::Missing
    ));
}

#[test]
fn build_item_detail_trait_with_methods() {
    use rustdoc_types::Trait;
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("MyTrait"),
            Visibility::Public,
            ItemEnum::Trait(Trait {
                is_auto: false,
                is_unsafe: false,
                is_dyn_compatible: true,
                generics: stub_generics(),
                bounds: vec![],
                items: vec![Id(10), Id(11)],
                implementations: vec![],
            }),
        ),
        item_bare(
            10,
            Some("required"),
            Visibility::Public,
            ItemEnum::Function({
                let mut f = stub_fn_inner();
                f.has_body = false;
                f
            }),
        ),
        item_bare(
            11,
            Some("provided"),
            Visibility::Public,
            ItemEnum::Function(stub_fn_inner()),
        ),
    ];
    let c = make_crate(0, items);
    let e = entry(c);
    let d = build_item_detail(&e, "krate::MyTrait", false, false).expect("MyTrait exists");
    assert_eq!(d.kind, ItemKind::Trait);
    assert_eq!(d.required_methods.len(), 1);
    assert_eq!(d.required_methods[0].name, "required");
    assert_eq!(d.provided_methods.len(), 1);
    assert_eq!(d.provided_methods[0].name, "provided");
    assert!(d.methods.is_empty());
}

#[test]
fn full_docs_returns_complete_text_and_clears_truncated() {
    let items = vec![
        item_docs(0, "krate", "Root.", module(&[1])),
        item_docs(
            1,
            "Thing",
            "First paragraph.\n\nSecond paragraph with more detail.",
            ItemEnum::Struct(Struct {
                kind: rustdoc_types::StructKind::Unit,
                generics: stub_generics(),
                impls: vec![],
            }),
        ),
    ];
    let c = make_crate(0, items);
    let e = entry(c);

    let truncated = build_item_detail(&e, "krate::Thing", false, false).unwrap();
    assert_eq!(truncated.docs.as_deref(), Some("First paragraph."));
    assert!(truncated.docs_truncated);

    let full = build_item_detail(&e, "krate::Thing", false, true).unwrap();
    assert_eq!(
        full.docs.as_deref(),
        Some("First paragraph.\n\nSecond paragraph with more detail.")
    );
    assert!(!full.docs_truncated);
}

#[test]
fn build_item_detail_trait_assoc_types() {
    use rustdoc_types::Trait;
    let items = vec![
        item_bare(0, Some("krate"), Visibility::Public, module(&[1])),
        item_bare(
            1,
            Some("Iterator"),
            Visibility::Public,
            ItemEnum::Trait(Trait {
                is_auto: false,
                is_unsafe: false,
                is_dyn_compatible: true,
                generics: stub_generics(),
                bounds: vec![],
                items: vec![Id(10), Id(11)],
                implementations: vec![],
            }),
        ),
        item_bare(
            10,
            Some("Item"),
            Visibility::Public,
            ItemEnum::AssocType {
                generics: stub_generics(),
                bounds: vec![],
                type_: None,
                default_unstable: None,
            },
        ),
        item_bare(
            11,
            Some("next"),
            Visibility::Public,
            ItemEnum::Function({
                let mut f = stub_fn_inner();
                f.has_body = false;
                f
            }),
        ),
    ];
    let c = make_crate(0, items);
    let e = entry(c);
    let d = build_item_detail(&e, "krate::Iterator", false, false).expect("Iterator exists");
    assert_eq!(d.assoc_types.len(), 1);
    assert_eq!(d.assoc_types[0].name, "Item");
    assert!(d.assoc_types[0].bounds.is_empty());
    assert!(d.assoc_types[0].default.is_none());
    assert_eq!(d.required_methods.len(), 1);
    assert_eq!(d.required_methods[0].name, "next");
}

#[tokio::test]
#[ignore = "network + filesystem"]
async fn get_item_reqwest_client_live() {
    let cache = crate::cache::CrateCache::new().unwrap();
    let entry = cache
        .get_resolved_with_report("reqwest", "latest")
        .await
        .unwrap()
        .entry;
    let d = build_item_detail(&entry, "reqwest::Client", false, false)
        .expect("reqwest::Client in surface");
    assert!(!d.methods.is_empty(), "Client should have inherent methods");
    assert!(d.blanket_impls.is_none(), "blanket_impls absent by default");
}
