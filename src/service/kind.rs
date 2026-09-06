//! Lossless conversion between rustdoc's private item kinds and the public contract.

use rustdoc_types::ItemKind;

use crate::contract::generated::RustdocItemKind;

pub(super) fn to_contract(kind: ItemKind) -> RustdocItemKind {
    match kind {
        ItemKind::Module => RustdocItemKind::Module,
        ItemKind::ExternCrate => RustdocItemKind::ExternCrate,
        ItemKind::Use => RustdocItemKind::Use,
        ItemKind::Struct => RustdocItemKind::Struct,
        ItemKind::StructField => RustdocItemKind::StructField,
        ItemKind::Union => RustdocItemKind::Union,
        ItemKind::Enum => RustdocItemKind::Enum,
        ItemKind::Variant => RustdocItemKind::Variant,
        ItemKind::Function => RustdocItemKind::Function,
        ItemKind::TypeAlias => RustdocItemKind::TypeAlias,
        ItemKind::Constant => RustdocItemKind::Constant,
        ItemKind::Trait => RustdocItemKind::Trait,
        ItemKind::TraitAlias => RustdocItemKind::TraitAlias,
        ItemKind::Impl => RustdocItemKind::Impl,
        ItemKind::Static => RustdocItemKind::Static,
        ItemKind::ExternType => RustdocItemKind::ExternType,
        ItemKind::Macro => RustdocItemKind::Macro,
        ItemKind::ProcAttribute => RustdocItemKind::ProcAttribute,
        ItemKind::ProcDerive => RustdocItemKind::ProcDerive,
        ItemKind::AssocConst => RustdocItemKind::AssocConst,
        ItemKind::AssocType => RustdocItemKind::AssocType,
        ItemKind::Primitive => RustdocItemKind::Primitive,
        ItemKind::Keyword => RustdocItemKind::Keyword,
        ItemKind::Attribute => RustdocItemKind::Attribute,
    }
}

pub(super) fn from_contract(kind: RustdocItemKind) -> ItemKind {
    match kind {
        RustdocItemKind::Module => ItemKind::Module,
        RustdocItemKind::ExternCrate => ItemKind::ExternCrate,
        RustdocItemKind::Use => ItemKind::Use,
        RustdocItemKind::Struct => ItemKind::Struct,
        RustdocItemKind::StructField => ItemKind::StructField,
        RustdocItemKind::Union => ItemKind::Union,
        RustdocItemKind::Enum => ItemKind::Enum,
        RustdocItemKind::Variant => ItemKind::Variant,
        RustdocItemKind::Function => ItemKind::Function,
        RustdocItemKind::TypeAlias => ItemKind::TypeAlias,
        RustdocItemKind::Constant => ItemKind::Constant,
        RustdocItemKind::Trait => ItemKind::Trait,
        RustdocItemKind::TraitAlias => ItemKind::TraitAlias,
        RustdocItemKind::Impl => ItemKind::Impl,
        RustdocItemKind::Static => ItemKind::Static,
        RustdocItemKind::ExternType => ItemKind::ExternType,
        RustdocItemKind::Macro => ItemKind::Macro,
        RustdocItemKind::ProcAttribute => ItemKind::ProcAttribute,
        RustdocItemKind::ProcDerive => ItemKind::ProcDerive,
        RustdocItemKind::AssocConst => ItemKind::AssocConst,
        RustdocItemKind::AssocType => ItemKind::AssocType,
        RustdocItemKind::Primitive => ItemKind::Primitive,
        RustdocItemKind::Keyword => ItemKind::Keyword,
        RustdocItemKind::Attribute => ItemKind::Attribute,
    }
}
