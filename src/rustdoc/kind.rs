use rustdoc_types::ItemKind;

#[must_use]
pub(crate) fn label(kind: ItemKind) -> &'static str {
    match kind {
        ItemKind::Module => "module",
        ItemKind::ExternCrate => "extern_crate",
        ItemKind::Use => "use",
        ItemKind::Struct => "struct",
        ItemKind::StructField => "struct_field",
        ItemKind::Union => "union",
        ItemKind::Enum => "enum",
        ItemKind::Variant => "variant",
        ItemKind::Function => "function",
        ItemKind::TypeAlias => "type_alias",
        ItemKind::Constant => "constant",
        ItemKind::Trait => "trait",
        ItemKind::TraitAlias => "trait_alias",
        ItemKind::Impl => "impl",
        ItemKind::Static => "static",
        ItemKind::ExternType => "extern_type",
        ItemKind::Macro => "macro",
        ItemKind::ProcAttribute => "proc_attribute",
        ItemKind::ProcDerive => "proc_derive",
        ItemKind::AssocConst => "assoc_const",
        ItemKind::AssocType => "assoc_type",
        ItemKind::Primitive => "primitive",
        ItemKind::Keyword => "keyword",
        ItemKind::Attribute => "attribute",
    }
}

#[must_use]
pub(crate) fn parse_label(value: &str) -> Option<ItemKind> {
    match value {
        "module" => Some(ItemKind::Module),
        "extern_crate" => Some(ItemKind::ExternCrate),
        "use" => Some(ItemKind::Use),
        "struct" => Some(ItemKind::Struct),
        "struct_field" => Some(ItemKind::StructField),
        "union" => Some(ItemKind::Union),
        "enum" => Some(ItemKind::Enum),
        "variant" => Some(ItemKind::Variant),
        "function" => Some(ItemKind::Function),
        "type_alias" => Some(ItemKind::TypeAlias),
        "constant" => Some(ItemKind::Constant),
        "trait" => Some(ItemKind::Trait),
        "trait_alias" => Some(ItemKind::TraitAlias),
        "impl" => Some(ItemKind::Impl),
        "static" => Some(ItemKind::Static),
        "extern_type" => Some(ItemKind::ExternType),
        "macro" => Some(ItemKind::Macro),
        "proc_attribute" => Some(ItemKind::ProcAttribute),
        "proc_derive" => Some(ItemKind::ProcDerive),
        "assoc_const" => Some(ItemKind::AssocConst),
        "assoc_type" => Some(ItemKind::AssocType),
        "primitive" => Some(ItemKind::Primitive),
        "keyword" => Some(ItemKind::Keyword),
        "attribute" => Some(ItemKind::Attribute),
        _ => None,
    }
}
