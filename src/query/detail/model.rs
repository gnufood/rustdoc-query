#[derive(Debug, serde::Serialize)]
pub(crate) struct ItemDetail {
    pub path: String,
    pub kind: rustdoc_types::ItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    pub docs_truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impl_constraint: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<FieldRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<VariantRow>,
    #[serde(skip_serializing_if = "is_false")]
    pub non_exhaustive: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<MethodRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required_methods: Vec<MethodRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provided_methods: Vec<MethodRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assoc_types: Vec<AssocTypeRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consts: Vec<ConstRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trait_impls: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derives: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auto_traits: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blanket_impls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

// Serde's `skip_serializing_if` callback receives a reference.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct FieldRow {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct VariantRow {
    pub name: String,
    pub kind: rustdoc_types::VariantKind,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<FieldRow>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct MethodRow {
    pub name: String,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impl_constraint: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct ConstRow {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct AssocTypeRow {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bounds: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}
