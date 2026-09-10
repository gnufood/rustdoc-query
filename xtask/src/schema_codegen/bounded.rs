//! Compiles canonical bounded integer definitions into checked public scalars.

use std::fmt::Write;

use serde_json::Value;

#[derive(Debug)]
pub(super) struct Spec {
    pub(super) name: String,
    description: String,
    minimum: u64,
    maximum: u64,
    storage: &'static str,
}

pub(super) fn collect(schema: &Value) -> Result<Vec<Spec>, String> {
    let definitions = schema
        .get("$defs")
        .and_then(Value::as_object)
        .ok_or_else(|| "schema must declare $defs".to_owned())?;

    let specs = definitions
        .iter()
        .map(|(name, definition)| bounded_integer(name, definition).transpose())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(specs.into_iter().flatten().collect())
}

fn bounded_integer(name: &str, definition: &Value) -> Option<Result<Spec, String>> {
    if definition.get("type").and_then(Value::as_str) != Some("integer") {
        return None;
    }
    let (Some(minimum), Some(maximum)) = (
        definition.get("minimum").and_then(Value::as_u64),
        definition.get("maximum").and_then(Value::as_u64),
    ) else {
        return None;
    };
    Some(
        (minimum <= maximum)
            .then(|| Spec {
                name: name.to_owned(),
                description: definition
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("Schema-bounded integer value.")
                    .to_owned(),
                minimum,
                maximum,
                storage: storage_for(maximum),
            })
            .ok_or_else(|| format!("/$defs/{name} minimum must not exceed maximum")),
    )
}

fn storage_for(maximum: u64) -> &'static str {
    if u8::try_from(maximum).is_ok() {
        "u8"
    } else if u16::try_from(maximum).is_ok() {
        "u16"
    } else if u32::try_from(maximum).is_ok() {
        "u32"
    } else {
        "u64"
    }
}

pub(super) fn source(specs: &[Spec]) -> Result<String, String> {
    let mut source = String::new();
    for spec in specs {
        let error = format!("{}Error", spec.name);
        write!(
            source,
            r#"
#[derive(Clone, Debug, PartialEq, Eq)]
/// Error returned when a value is outside the schema-defined range.
pub struct {error} {{
    value: ::std::primitive::u64,
}}

impl ::std::fmt::Display for {error} {{
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{
        write!(formatter, "{name} must be between {minimum} and {maximum}; got {{}}", self.value)
    }}
}}

impl ::std::error::Error for {error} {{}}

#[derive(::serde::Serialize, Clone, Debug, rmcp::schemars::JsonSchema)]
#[serde(transparent)]
/// {description}
pub struct {name}(#[schemars(range(min = {minimum}, max = {maximum}))] ::std::primitive::{storage});

impl {name} {{
    /// Smallest value accepted by this schema type.
    pub const MINIMUM: ::std::primitive::u64 = {minimum};
    /// Largest value accepted by this schema type.
    pub const MAXIMUM: ::std::primitive::u64 = {maximum};

    /// Creates a value within the schema-defined range.
    ///
    /// # Errors
    ///
    /// Returns `{error}` when `value` is outside that range.
    pub fn new(value: ::std::primitive::u64) -> ::std::result::Result<Self, {error}> {{
        if (Self::MINIMUM..=Self::MAXIMUM).contains(&value) {{
            match <::std::primitive::{storage}>::try_from(value) {{
                Ok(value) => Ok(Self(value)),
                Err(_) => Err({error} {{ value }}),
            }}
        }} else {{
            Err({error} {{ value }})
        }}
    }}

    #[must_use]
    /// Returns the validated integer value.
    pub fn get(&self) -> ::std::primitive::{storage} {{
        self.0
    }}
}}

impl ::std::convert::TryFrom<::std::primitive::u64> for {name} {{
    type Error = {error};

    fn try_from(value: ::std::primitive::u64) -> ::std::result::Result<Self, Self::Error> {{
        Self::new(value)
    }}
}}

impl ::std::convert::From<{name}> for ::std::primitive::u64 {{
    fn from(value: {name}) -> Self {{
        value.0.into()
    }}
}}

impl<'de> ::serde::Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {{
        let value = <::std::primitive::u64 as ::serde::Deserialize>::deserialize(deserializer)?;
        Self::new(value).map_err(<D::Error as ::serde::de::Error>::custom)
    }}
}}
"#,
            name = spec.name,
            description = spec.description,
            minimum = spec.minimum,
            maximum = spec.maximum,
            storage = spec.storage,
        )
        .map_err(|_| "writing to String failed".to_owned())?;
    }
    Ok(source)
}
