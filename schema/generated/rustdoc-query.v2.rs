#![allow(
    clippy::derivable_impls,
    clippy::enum_variant_names,
    clippy::large_enum_variant
)]
#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Typed failure returned in place of a result."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Typed failure returned in place of a result.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"InvalidRequestError\","]
#[doc = "      \"description\": \"The request violates the contract.\","]
#[doc = "      \"$ref\": \"#/$defs/InvalidRequestError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CrateNotFoundError\","]
#[doc = "      \"description\": \"No matching published crate exists.\","]
#[doc = "      \"$ref\": \"#/$defs/CrateNotFoundError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ModuleNotFoundError\","]
#[doc = "      \"description\": \"The requested public module is absent.\","]
#[doc = "      \"$ref\": \"#/$defs/ModuleNotFoundError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ItemNotFoundError\","]
#[doc = "      \"description\": \"The requested public item is absent.\","]
#[doc = "      \"$ref\": \"#/$defs/ItemNotFoundError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"UnsupportedRustdocFormatError\","]
#[doc = "      \"description\": \"The rustdoc format is unsupported.\","]
#[doc = "      \"$ref\": \"#/$defs/UnsupportedRustdocFormatError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"SourceUnavailableError\","]
#[doc = "      \"description\": \"Rustdoc data is temporarily unavailable.\","]
#[doc = "      \"$ref\": \"#/$defs/SourceUnavailableError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OperationFailedError\","]
#[doc = "      \"description\": \"The operation failed without a more specific classification.\","]
#[doc = "      \"$ref\": \"#/$defs/OperationFailedError\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(untagged)]
pub enum ApiError {
    #[doc = "The request violates the contract."]
    InvalidRequestError(InvalidRequestError),
    #[doc = "No matching published crate exists."]
    CrateNotFoundError(CrateNotFoundError),
    #[doc = "The requested public module is absent."]
    ModuleNotFoundError(ModuleNotFoundError),
    #[doc = "The requested public item is absent."]
    ItemNotFoundError(ItemNotFoundError),
    #[doc = "The rustdoc format is unsupported."]
    UnsupportedRustdocFormatError(UnsupportedRustdocFormatError),
    #[doc = "Rustdoc data is temporarily unavailable."]
    SourceUnavailableError(SourceUnavailableError),
    #[doc = "The operation failed without a more specific classification."]
    OperationFailedError(OperationFailedError),
}
impl ::std::convert::From<InvalidRequestError> for ApiError {
    fn from(value: InvalidRequestError) -> Self {
        Self::InvalidRequestError(value)
    }
}
impl ::std::convert::From<CrateNotFoundError> for ApiError {
    fn from(value: CrateNotFoundError) -> Self {
        Self::CrateNotFoundError(value)
    }
}
impl ::std::convert::From<ModuleNotFoundError> for ApiError {
    fn from(value: ModuleNotFoundError) -> Self {
        Self::ModuleNotFoundError(value)
    }
}
impl ::std::convert::From<ItemNotFoundError> for ApiError {
    fn from(value: ItemNotFoundError) -> Self {
        Self::ItemNotFoundError(value)
    }
}
impl ::std::convert::From<UnsupportedRustdocFormatError> for ApiError {
    fn from(value: UnsupportedRustdocFormatError) -> Self {
        Self::UnsupportedRustdocFormatError(value)
    }
}
impl ::std::convert::From<SourceUnavailableError> for ApiError {
    fn from(value: SourceUnavailableError) -> Self {
        Self::SourceUnavailableError(value)
    }
}
impl ::std::convert::From<OperationFailedError> for ApiError {
    fn from(value: OperationFailedError) -> Self {
        Self::OperationFailedError(value)
    }
}
#[doc = "A trait associated constant."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A trait associated constant.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Associated constant name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"Rendered constant type.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct AssociatedConst {
    #[doc = "Associated constant name."]
    pub name: ::std::string::String,
    #[doc = "Rendered constant type."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A trait associated type."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A trait associated type.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bounds\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bounds\": {"]
#[doc = "      \"description\": \"Rendered bounds. Empty when unbounded.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"Rendered default type. Absent when the trait supplies none.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Associated type name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct AssociatedType {
    #[doc = "Rendered bounds. Empty when unbounded."]
    pub bounds: ::std::string::String,
    #[doc = "Rendered default type. Absent when the trait supplies none."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Associated type name."]
    pub name: ::std::string::String,
}
#[doc = "Cache provenance and timing for the queried crate."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Cache provenance and timing for the queried crate.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"load_duration_ms\","]
#[doc = "    \"source\","]
#[doc = "    \"stale\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"age_ms\": {"]
#[doc = "      \"description\": \"Age of the durable source when known.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"load_duration_ms\": {"]
#[doc = "      \"description\": \"Elapsed time spent preparing the queried crate.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"description\": \"Cache tier that supplied the rustdoc data.\","]
#[doc = "      \"$ref\": \"#/$defs/CacheSource\""]
#[doc = "    },"]
#[doc = "    \"stale\": {"]
#[doc = "      \"description\": \"Whether the returned data is known to be stale.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Whether this request found usable data at the reported cache tier.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"Usable data was already present at this cache tier.\","]
#[doc = "          \"const\": \"hit\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"The request had to populate data beyond this cache tier.\","]
#[doc = "          \"const\": \"miss\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct CacheMetadata {
    #[doc = "Age of the durable source when known."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub age_ms: ::std::option::Option<u64>,
    #[doc = "Elapsed time spent preparing the queried crate."]
    pub load_duration_ms: u64,
    #[doc = "Cache tier that supplied the rustdoc data."]
    pub source: CacheSource,
    #[doc = "Whether the returned data is known to be stale."]
    pub stale: bool,
    #[doc = "Whether this request found usable data at the reported cache tier."]
    pub status: CacheMetadataStatus,
}
#[doc = "Whether this request found usable data at the reported cache tier."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Whether this request found usable data at the reported cache tier.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Usable data was already present at this cache tier.\","]
#[doc = "      \"const\": \"hit\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"The request had to populate data beyond this cache tier.\","]
#[doc = "      \"const\": \"miss\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum CacheMetadataStatus {
    #[doc = "Usable data was already present at this cache tier."]
    #[serde(rename = "hit")]
    Hit,
    #[doc = "The request had to populate data beyond this cache tier."]
    #[serde(rename = "miss")]
    Miss,
}
impl ::std::fmt::Display for CacheMetadataStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Hit => f.write_str("hit"),
            Self::Miss => f.write_str("miss"),
        }
    }
}
impl ::std::str::FromStr for CacheMetadataStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "hit" => Ok(Self::Hit),
            "miss" => Ok(Self::Miss),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CacheMetadataStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CacheMetadataStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CacheMetadataStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The cache tier or source that supplied the rustdoc data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The cache tier or source that supplied the rustdoc data.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The in-memory cache.\","]
#[doc = "      \"const\": \"memory\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A decoded local cache sidecar.\","]
#[doc = "      \"const\": \"sidecar\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A compressed local rustdoc blob.\","]
#[doc = "      \"const\": \"blob\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A freshly downloaded rustdoc artifact.\","]
#[doc = "      \"const\": \"network\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum CacheSource {
    #[doc = "The in-memory cache."]
    #[serde(rename = "memory")]
    Memory,
    #[doc = "A decoded local cache sidecar."]
    #[serde(rename = "sidecar")]
    Sidecar,
    #[doc = "A compressed local rustdoc blob."]
    #[serde(rename = "blob")]
    Blob,
    #[doc = "A freshly downloaded rustdoc artifact."]
    #[serde(rename = "network")]
    Network,
}
impl ::std::fmt::Display for CacheSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Memory => f.write_str("memory"),
            Self::Sidecar => f.write_str("sidecar"),
            Self::Blob => f.write_str("blob"),
            Self::Network => f.write_str("network"),
        }
    }
}
impl ::std::str::FromStr for CacheSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "memory" => Ok(Self::Memory),
            "sidecar" => Ok(Self::Sidecar),
            "blob" => Ok(Self::Blob),
            "network" => Ok(Self::Network),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CacheSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CacheSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CacheSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Crate name as published on crates.io."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Crate name as published on crates.io.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct CrateName(::std::string::String);
impl ::std::ops::Deref for CrateName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrateName> for ::std::string::String {
    fn from(value: CrateName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrateName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrateName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrateName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrateName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrateName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "No published crate matches the requested name and version."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"No published crate matches the requested name and version.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"crate_name\","]
#[doc = "    \"message\","]
#[doc = "    \"resolved_version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable machine-readable error code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"No matching published crate was found.\","]
#[doc = "          \"const\": \"crate_not_found\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"crate_name\": {"]
#[doc = "      \"description\": \"Requested crate name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation of the lookup failure.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resolved_version\": {"]
#[doc = "      \"description\": \"Version that could not be resolved.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct CrateNotFoundError {
    #[doc = "Stable machine-readable error code."]
    pub code: CrateNotFoundErrorCode,
    #[doc = "Requested crate name."]
    pub crate_name: ::std::string::String,
    #[doc = "Human-readable explanation of the lookup failure."]
    pub message: ::std::string::String,
    #[doc = "Version that could not be resolved."]
    pub resolved_version: ::std::string::String,
}
#[doc = "Stable machine-readable error code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable machine-readable error code.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"No matching published crate was found.\","]
#[doc = "      \"const\": \"crate_not_found\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum CrateNotFoundErrorCode {
    #[doc = "No matching published crate was found."]
    #[serde(rename = "crate_not_found")]
    CrateNotFound,
}
impl ::std::fmt::Display for CrateNotFoundErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CrateNotFound => f.write_str("crate_not_found"),
        }
    }
}
impl ::std::str::FromStr for CrateNotFoundErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crate_not_found" => Ok(Self::CrateNotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CrateNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrateNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrateNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The caller's resolved dependency version."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The caller's resolved dependency version.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct DependencyVersion(::std::string::String);
impl ::std::ops::Deref for DependencyVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DependencyVersion> for ::std::string::String {
    fn from(value: DependencyVersion) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DependencyVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DependencyVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DependencyVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DependencyVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DependencyVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A non-empty full-text query over documentation and signatures."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A non-empty full-text query over documentation and signatures.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct DocumentationQuery(::std::string::String);
impl ::std::ops::Deref for DocumentationQuery {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DocumentationQuery> for ::std::string::String {
    fn from(value: DocumentationQuery) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DocumentationQuery {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DocumentationQuery {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DocumentationQuery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DocumentationQuery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DocumentationQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A failed operation carrying a typed error."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A failed operation carrying a typed error.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"error\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"description\": \"Typed error explaining the failure.\","]
#[doc = "      \"$ref\": \"#/$defs/ApiError\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Outcome status discriminator.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The operation failed.\","]
#[doc = "          \"const\": \"error\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ErrorOutcome {
    #[doc = "Typed error explaining the failure."]
    pub error: ApiError,
    #[doc = "Outcome status discriminator."]
    pub status: ErrorOutcomeStatus,
}
#[doc = "Outcome status discriminator."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Outcome status discriminator.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The operation failed.\","]
#[doc = "      \"const\": \"error\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum ErrorOutcomeStatus {
    #[doc = "The operation failed."]
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for ErrorOutcomeStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for ErrorOutcomeStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ErrorOutcomeStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ErrorOutcomeStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ErrorOutcomeStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A named field with its rendered type."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A named field with its rendered type.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Field name. Tuple fields are named by position.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"Rendered Rust field type.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct Field {
    #[doc = "Field name. Tuple fields are named by position."]
    pub name: ::std::string::String,
    #[doc = "Rendered Rust field type."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Result of a search request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Result of a search request.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"FindSuccess\","]
#[doc = "      \"description\": \"A completed search.\","]
#[doc = "      \"$ref\": \"#/$defs/FindSuccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ErrorOutcome\","]
#[doc = "      \"description\": \"A failed search operation.\","]
#[doc = "      \"$ref\": \"#/$defs/ErrorOutcome\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(untagged)]
pub enum FindOutcome {
    #[doc = "A completed search."]
    FindSuccess(FindSuccess),
    #[doc = "A failed search operation."]
    ErrorOutcome(ErrorOutcome),
}
impl ::std::convert::From<FindSuccess> for FindOutcome {
    fn from(value: FindSuccess) -> Self {
        Self::FindSuccess(value)
    }
}
impl ::std::convert::From<ErrorOutcome> for FindOutcome {
    fn from(value: ErrorOutcome) -> Self {
        Self::ErrorOutcome(value)
    }
}
#[doc = "Request to search a crate's public surface by name, kind, module, or documentation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request to search a crate's public surface by name, kind, module, or documentation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"crate_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"crate_name\": {"]
#[doc = "      \"description\": \"Crate to search.\","]
#[doc = "      \"$ref\": \"#/$defs/CrateName\""]
#[doc = "    },"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"Continue a previous search. A cursor is valid only for the query, filters, and resolved version that produced it.\","]
#[doc = "      \"$ref\": \"#/$defs/PageCursor\""]
#[doc = "    },"]
#[doc = "    \"dependency_version\": {"]
#[doc = "      \"description\": \"Version the caller depends on, reported back as a comparison against the resolved version.\","]
#[doc = "      \"$ref\": \"#/$defs/DependencyVersion\""]
#[doc = "    },"]
#[doc = "    \"doc_query\": {"]
#[doc = "      \"description\": \"Documentation query. Mutually exclusive with `query`.\","]
#[doc = "      \"$ref\": \"#/$defs/DocumentationQuery\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Restrict results to one item kind.\","]
#[doc = "      \"$ref\": \"#/$defs/RustdocItemKind\""]
#[doc = "    },"]
#[doc = "    \"module\": {"]
#[doc = "      \"description\": \"Restrict results to one module and its descendants.\","]
#[doc = "      \"$ref\": \"#/$defs/PublicModulePath\""]
#[doc = "    },"]
#[doc = "    \"output_format\": {"]
#[doc = "      \"description\": \"Requested response encoding. Defaults to JSON.\","]
#[doc = "      \"$ref\": \"#/$defs/OutputFormat\""]
#[doc = "    },"]
#[doc = "    \"page_size\": {"]
#[doc = "      \"description\": \"Maximum results per page.\","]
#[doc = "      \"$ref\": \"#/$defs/PageSize\""]
#[doc = "    },"]
#[doc = "    \"query\": {"]
#[doc = "      \"description\": \"Fuzzy item-name query. Mutually exclusive with `doc_query`.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"Version to search.\","]
#[doc = "      \"$ref\": \"#/$defs/RequestedVersion\""]
#[doc = "    },"]
#[doc = "    \"with_summary\": {"]
#[doc = "      \"description\": \"Include a documentation summary on each result.\","]
#[doc = "      \"$ref\": \"#/$defs/IncludeSummary\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"dependentSchemas\": {"]
#[doc = "    \"doc_query\": {"]
#[doc = "      \"not\": {"]
#[doc = "        \"required\": ["]
#[doc = "          \"query\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"query\": {"]
#[doc = "      \"not\": {"]
#[doc = "        \"required\": ["]
#[doc = "          \"doc_query\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct FindRequest {
    #[doc = "Crate to search."]
    pub crate_name: CrateName,
    #[doc = "Continue a previous search. A cursor is valid only for the query, filters, and resolved version that produced it."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub cursor: ::std::option::Option<PageCursor>,
    #[doc = "Version the caller depends on, reported back as a comparison against the resolved version."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub dependency_version: ::std::option::Option<DependencyVersion>,
    #[doc = "Documentation query. Mutually exclusive with `query`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub doc_query: ::std::option::Option<DocumentationQuery>,
    #[doc = "Restrict results to one item kind."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub kind: ::std::option::Option<RustdocItemKind>,
    #[doc = "Restrict results to one module and its descendants."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub module: ::std::option::Option<PublicModulePath>,
    #[doc = "Requested response encoding. Defaults to JSON."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub output_format: ::std::option::Option<OutputFormat>,
    #[doc = "Maximum results per page."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub page_size: ::std::option::Option<PageSize>,
    #[doc = "Fuzzy item-name query. Mutually exclusive with `doc_query`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub query: ::std::option::Option<FindRequestQuery>,
    #[doc = "Version to search."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub version: ::std::option::Option<RequestedVersion>,
    #[doc = "Include a documentation summary on each result."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub with_summary: ::std::option::Option<IncludeSummary>,
}
#[doc = "Fuzzy item-name query. Mutually exclusive with `doc_query`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Fuzzy item-name query. Mutually exclusive with `doc_query`.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct FindRequestQuery(::std::string::String);
impl ::std::ops::Deref for FindRequestQuery {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FindRequestQuery> for ::std::string::String {
    fn from(value: FindRequestQuery) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FindRequestQuery {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FindRequestQuery {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FindRequestQuery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FindRequestQuery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FindRequestQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "One page of search results."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"One page of search results.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"CompletePage\","]
#[doc = "      \"description\": \"Final page. No further results exist.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"has_more\","]
#[doc = "        \"resolved_version\","]
#[doc = "        \"rows\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"has_more\": {"]
#[doc = "          \"description\": \"False because this is the final page.\","]
#[doc = "          \"enum\": ["]
#[doc = "            false"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"hint\": {"]
#[doc = "          \"description\": \"Guidance for refining a query that matched poorly.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"resolved_version\": {"]
#[doc = "          \"description\": \"Version actually queried.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"rows\": {"]
#[doc = "          \"description\": \"Rows in this page.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/FindRow\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ContinuationPage\","]
#[doc = "      \"description\": \"Partial page. Further results are reachable with `next_cursor`.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"has_more\","]
#[doc = "        \"next_cursor\","]
#[doc = "        \"resolved_version\","]
#[doc = "        \"rows\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"has_more\": {"]
#[doc = "          \"description\": \"True because another page is available.\","]
#[doc = "          \"enum\": ["]
#[doc = "            true"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"hint\": {"]
#[doc = "          \"description\": \"Guidance for refining a query that matched poorly.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"next_cursor\": {"]
#[doc = "          \"description\": \"Pass as cursor to retrieve the next page.\","]
#[doc = "          \"$ref\": \"#/$defs/PageCursor\""]
#[doc = "        },"]
#[doc = "        \"resolved_version\": {"]
#[doc = "          \"description\": \"Version actually queried.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"rows\": {"]
#[doc = "          \"description\": \"Rows in this page.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/FindRow\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(untagged, deny_unknown_fields)]
pub enum FindResult {
    #[doc = "Final page. No further results exist."]
    CompletePage {
        #[doc = "False because this is the final page."]
        has_more: bool,
        #[doc = "Guidance for refining a query that matched poorly."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        #[serde(deserialize_with = "super::strict_option::deserialize")]
        hint: ::std::option::Option<::std::string::String>,
        #[doc = "Version actually queried."]
        resolved_version: ::std::string::String,
        #[doc = "Rows in this page."]
        rows: ::std::vec::Vec<FindRow>,
    },
    #[doc = "Partial page. Further results are reachable with `next_cursor`."]
    ContinuationPage {
        #[doc = "True because another page is available."]
        has_more: bool,
        #[doc = "Guidance for refining a query that matched poorly."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        #[serde(deserialize_with = "super::strict_option::deserialize")]
        hint: ::std::option::Option<::std::string::String>,
        #[doc = "Pass as cursor to retrieve the next page."]
        next_cursor: PageCursor,
        #[doc = "Version actually queried."]
        resolved_version: ::std::string::String,
        #[doc = "Rows in this page."]
        rows: ::std::vec::Vec<FindRow>,
    },
}
#[doc = "A single search result."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A single search result.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Rustdoc kind of the matched item.\","]
#[doc = "      \"$ref\": \"#/$defs/RustdocItemKind\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Full path accepted by `get_item`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"First paragraph of the item documentation. Absent unless requested.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct FindRow {
    #[doc = "Rustdoc kind of the matched item."]
    pub kind: RustdocItemKind,
    #[doc = "Full path accepted by `get_item`."]
    pub path: ::std::string::String,
    #[doc = "First paragraph of the item documentation. Absent unless requested."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[doc = "A completed search."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A completed search.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"metadata\","]
#[doc = "    \"result\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Provenance for the completed operation.\","]
#[doc = "      \"$ref\": \"#/$defs/ResponseMetadata\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The requested search page.\","]
#[doc = "      \"$ref\": \"#/$defs/FindResult\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Outcome status discriminator.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The operation completed successfully.\","]
#[doc = "          \"const\": \"ok\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct FindSuccess {
    #[doc = "Provenance for the completed operation."]
    pub metadata: ResponseMetadata,
    #[doc = "The requested search page."]
    pub result: FindResult,
    #[doc = "Outcome status discriminator."]
    pub status: FindSuccessStatus,
}
#[doc = "Outcome status discriminator."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Outcome status discriminator.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The operation completed successfully.\","]
#[doc = "      \"const\": \"ok\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum FindSuccessStatus {
    #[doc = "The operation completed successfully."]
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for FindSuccessStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for FindSuccessStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FindSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FindSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FindSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Result of an item lookup request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Result of an item lookup request.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"GetItemSuccess\","]
#[doc = "      \"description\": \"A completed item lookup.\","]
#[doc = "      \"$ref\": \"#/$defs/GetItemSuccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ErrorOutcome\","]
#[doc = "      \"description\": \"A failed item lookup.\","]
#[doc = "      \"$ref\": \"#/$defs/ErrorOutcome\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(untagged)]
pub enum GetItemOutcome {
    #[doc = "A completed item lookup."]
    GetItemSuccess(GetItemSuccess),
    #[doc = "A failed item lookup."]
    ErrorOutcome(ErrorOutcome),
}
impl ::std::convert::From<GetItemSuccess> for GetItemOutcome {
    fn from(value: GetItemSuccess) -> Self {
        Self::GetItemSuccess(value)
    }
}
impl ::std::convert::From<ErrorOutcome> for GetItemOutcome {
    fn from(value: ErrorOutcome) -> Self {
        Self::ErrorOutcome(value)
    }
}
#[doc = "Request for full detail on one public item."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request for full detail on one public item.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"crate_name\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"crate_name\": {"]
#[doc = "      \"description\": \"Crate owning the item.\","]
#[doc = "      \"$ref\": \"#/$defs/CrateName\""]
#[doc = "    },"]
#[doc = "    \"dependency_version\": {"]
#[doc = "      \"description\": \"Version the caller depends on, reported back as a comparison against the resolved version.\","]
#[doc = "      \"$ref\": \"#/$defs/DependencyVersion\""]
#[doc = "    },"]
#[doc = "    \"full_docs\": {"]
#[doc = "      \"description\": \"Include full item documentation rather than its first paragraph.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"include_impls\": {"]
#[doc = "      \"description\": \"Include blanket implementations.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"output_format\": {"]
#[doc = "      \"description\": \"Requested response encoding. Defaults to JSON.\","]
#[doc = "      \"$ref\": \"#/$defs/OutputFormat\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Item to retrieve.\","]
#[doc = "      \"$ref\": \"#/$defs/PublicItemPath\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"Version to query.\","]
#[doc = "      \"$ref\": \"#/$defs/RequestedVersion\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct GetItemRequest {
    #[doc = "Crate owning the item."]
    pub crate_name: CrateName,
    #[doc = "Version the caller depends on, reported back as a comparison against the resolved version."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub dependency_version: ::std::option::Option<DependencyVersion>,
    #[doc = "Include full item documentation rather than its first paragraph."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub full_docs: ::std::option::Option<bool>,
    #[doc = "Include blanket implementations."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub include_impls: ::std::option::Option<bool>,
    #[doc = "Requested response encoding. Defaults to JSON."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub output_format: ::std::option::Option<OutputFormat>,
    #[doc = "Item to retrieve."]
    pub path: PublicItemPath,
    #[doc = "Version to query."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub version: ::std::option::Option<RequestedVersion>,
}
#[doc = "A completed item lookup."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A completed item lookup.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"metadata\","]
#[doc = "    \"result\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Provenance for the completed operation.\","]
#[doc = "      \"$ref\": \"#/$defs/ResponseMetadata\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"Full detail for the requested item.\","]
#[doc = "      \"$ref\": \"#/$defs/ItemDetail\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Outcome status discriminator.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The operation completed successfully.\","]
#[doc = "          \"const\": \"ok\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct GetItemSuccess {
    #[doc = "Provenance for the completed operation."]
    pub metadata: ResponseMetadata,
    #[doc = "Full detail for the requested item."]
    pub result: ItemDetail,
    #[doc = "Outcome status discriminator."]
    pub status: GetItemSuccessStatus,
}
#[doc = "Outcome status discriminator."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Outcome status discriminator.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The operation completed successfully.\","]
#[doc = "      \"const\": \"ok\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum GetItemSuccessStatus {
    #[doc = "The operation completed successfully."]
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for GetItemSuccessStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for GetItemSuccessStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetItemSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GetItemSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetItemSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Include a concise documentation summary for each result."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Include a concise documentation summary for each result.\","]
#[doc = "  \"type\": \"boolean\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct IncludeSummary(pub bool);
impl ::std::ops::Deref for IncludeSummary {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<IncludeSummary> for bool {
    fn from(value: IncludeSummary) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for IncludeSummary {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for IncludeSummary {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for IncludeSummary {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for IncludeSummary {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for IncludeSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "The request violated the contract. Retrying without changing it will fail again."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The request violated the contract. Retrying without changing it will fail again.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable machine-readable error code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The request violates the published contract.\","]
#[doc = "          \"const\": \"invalid_request\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"field\": {"]
#[doc = "      \"description\": \"Request property that was rejected. Absent when no single property is at fault.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation of the rejected request.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct InvalidRequestError {
    #[doc = "Stable machine-readable error code."]
    pub code: InvalidRequestErrorCode,
    #[doc = "Request property that was rejected. Absent when no single property is at fault."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub field: ::std::option::Option<::std::string::String>,
    #[doc = "Human-readable explanation of the rejected request."]
    pub message: ::std::string::String,
}
#[doc = "Stable machine-readable error code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable machine-readable error code.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The request violates the published contract.\","]
#[doc = "      \"const\": \"invalid_request\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum InvalidRequestErrorCode {
    #[doc = "The request violates the published contract."]
    #[serde(rename = "invalid_request")]
    InvalidRequest,
}
impl ::std::fmt::Display for InvalidRequestErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InvalidRequest => f.write_str("invalid_request"),
        }
    }
}
impl ::std::str::FromStr for InvalidRequestErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "invalid_request" => Ok(Self::InvalidRequest),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InvalidRequestErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InvalidRequestErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InvalidRequestErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Full detail for one public item."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Full detail for one public item.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"assoc_types\","]
#[doc = "    \"auto_traits\","]
#[doc = "    \"consts\","]
#[doc = "    \"derives\","]
#[doc = "    \"docs_truncated\","]
#[doc = "    \"fields\","]
#[doc = "    \"kind\","]
#[doc = "    \"methods\","]
#[doc = "    \"non_exhaustive\","]
#[doc = "    \"path\","]
#[doc = "    \"provided_methods\","]
#[doc = "    \"required_methods\","]
#[doc = "    \"resolved_version\","]
#[doc = "    \"trait_impls\","]
#[doc = "    \"variants\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"assoc_types\": {"]
#[doc = "      \"description\": \"Associated types of a trait.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AssociatedType\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"auto_traits\": {"]
#[doc = "      \"description\": \"Compiler-inferred marker traits.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"blanket_impls\": {"]
#[doc = "      \"description\": \"Implementations supplied by generic impls. Absent unless `include_impls` was requested.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"consts\": {"]
#[doc = "      \"description\": \"Associated constants of a trait.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AssociatedConst\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"derives\": {"]
#[doc = "      \"description\": \"Traits implemented through a derive attribute.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"docs\": {"]
#[doc = "      \"description\": \"Item documentation. Absent when undocumented.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"docs_truncated\": {"]
#[doc = "      \"description\": \"Whether docs was shortened. Request `full_docs` to receive it whole.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"fields\": {"]
#[doc = "      \"description\": \"Public fields of a struct or union. Empty for other kinds.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Field\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"impl_constraint\": {"]
#[doc = "      \"description\": \"Implementation context for an inherent method. Uses a concrete target or generic implementation header.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Rustdoc kind of the returned item.\","]
#[doc = "      \"$ref\": \"#/$defs/RustdocItemKind\""]
#[doc = "    },"]
#[doc = "    \"methods\": {"]
#[doc = "      \"description\": \"Inherent methods. Empty for a trait.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Method\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"non_exhaustive\": {"]
#[doc = "      \"description\": \"Whether the item cannot be exhaustively matched or constructed by downstream crates.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Canonical path of the returned item.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"provided_methods\": {"]
#[doc = "      \"description\": \"Trait methods with a default body.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Method\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"required_methods\": {"]
#[doc = "      \"description\": \"Trait methods an implementor must define.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Method\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"resolved_version\": {"]
#[doc = "      \"description\": \"Version actually queried.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"description\": \"Rendered signature. Present only for kinds that have one.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"trait_impls\": {"]
#[doc = "      \"description\": \"Rendered trait implementations, excluding auto traits and blanket implementations.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"variants\": {"]
#[doc = "      \"description\": \"Variants of an enum. Empty for other kinds.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Variant\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ItemDetail {
    #[doc = "Associated types of a trait."]
    pub assoc_types: ::std::vec::Vec<AssociatedType>,
    #[doc = "Compiler-inferred marker traits."]
    pub auto_traits: ::std::vec::Vec<::std::string::String>,
    #[doc = "Implementations supplied by generic impls. Absent unless `include_impls` was requested."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub blanket_impls: ::std::vec::Vec<::std::string::String>,
    #[doc = "Associated constants of a trait."]
    pub consts: ::std::vec::Vec<AssociatedConst>,
    #[doc = "Traits implemented through a derive attribute."]
    pub derives: ::std::vec::Vec<::std::string::String>,
    #[doc = "Item documentation. Absent when undocumented."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub docs: ::std::option::Option<::std::string::String>,
    #[doc = "Whether docs was shortened. Request `full_docs` to receive it whole."]
    pub docs_truncated: bool,
    #[doc = "Public fields of a struct or union. Empty for other kinds."]
    pub fields: ::std::vec::Vec<Field>,
    #[doc = "Implementation context for an inherent method. Uses a concrete target or generic implementation header."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub impl_constraint: ::std::option::Option<::std::string::String>,
    #[doc = "Rustdoc kind of the returned item."]
    pub kind: RustdocItemKind,
    #[doc = "Inherent methods. Empty for a trait."]
    pub methods: ::std::vec::Vec<Method>,
    #[doc = "Whether the item cannot be exhaustively matched or constructed by downstream crates."]
    pub non_exhaustive: bool,
    #[doc = "Canonical path of the returned item."]
    pub path: ::std::string::String,
    #[doc = "Trait methods with a default body."]
    pub provided_methods: ::std::vec::Vec<Method>,
    #[doc = "Trait methods an implementor must define."]
    pub required_methods: ::std::vec::Vec<Method>,
    #[doc = "Version actually queried."]
    pub resolved_version: ::std::string::String,
    #[doc = "Rendered signature. Present only for kinds that have one."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub signature: ::std::option::Option<::std::string::String>,
    #[doc = "Rendered trait implementations, excluding auto traits and blanket implementations."]
    pub trait_impls: ::std::vec::Vec<::std::string::String>,
    #[doc = "Variants of an enum. Empty for other kinds."]
    pub variants: ::std::vec::Vec<Variant>,
}
#[doc = "The crate resolved but exposes no such public item."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The crate resolved but exposes no such public item.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"crate_name\","]
#[doc = "    \"message\","]
#[doc = "    \"path\","]
#[doc = "    \"resolved_version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable machine-readable error code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The requested public item is absent.\","]
#[doc = "          \"const\": \"item_not_found\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"crate_name\": {"]
#[doc = "      \"description\": \"Requested crate name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation of the lookup failure.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Requested public item path.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resolved_version\": {"]
#[doc = "      \"description\": \"Version that was queried.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ItemNotFoundError {
    #[doc = "Stable machine-readable error code."]
    pub code: ItemNotFoundErrorCode,
    #[doc = "Requested crate name."]
    pub crate_name: ::std::string::String,
    #[doc = "Human-readable explanation of the lookup failure."]
    pub message: ::std::string::String,
    #[doc = "Requested public item path."]
    pub path: ::std::string::String,
    #[doc = "Version that was queried."]
    pub resolved_version: ::std::string::String,
}
#[doc = "Stable machine-readable error code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable machine-readable error code.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The requested public item is absent.\","]
#[doc = "      \"const\": \"item_not_found\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum ItemNotFoundErrorCode {
    #[doc = "The requested public item is absent."]
    #[serde(rename = "item_not_found")]
    ItemNotFound,
}
impl ::std::fmt::Display for ItemNotFoundErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemNotFound => f.write_str("item_not_found"),
        }
    }
}
impl ::std::str::FromStr for ItemNotFoundErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item_not_found" => Ok(Self::ItemNotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ItemNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ItemNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ItemNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Number of public items of one kind."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Number of public items of one kind.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"count\","]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"description\": \"Number of public items of this kind.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Rustdoc kind being counted.\","]
#[doc = "      \"$ref\": \"#/$defs/RustdocItemKind\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct KindCount {
    #[doc = "Number of public items of this kind."]
    pub count: u64,
    #[doc = "Rustdoc kind being counted."]
    pub kind: RustdocItemKind,
}
#[doc = "A method with its rendered signature."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A method with its rendered signature.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"signature\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"impl_constraint\": {"]
#[doc = "      \"description\": \"Implementation context distinguishing otherwise identical inherent methods. Uses a concrete target or generic implementation header.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Method name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"description\": \"Rendered method signature.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct Method {
    #[doc = "Implementation context distinguishing otherwise identical inherent methods. Uses a concrete target or generic implementation header."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub impl_constraint: ::std::option::Option<::std::string::String>,
    #[doc = "Method name."]
    pub name: ::std::string::String,
    #[doc = "Rendered method signature."]
    pub signature: ::std::string::String,
}
#[doc = "The crate resolved but exposes no such public module."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The crate resolved but exposes no such public module.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"crate_name\","]
#[doc = "    \"message\","]
#[doc = "    \"module\","]
#[doc = "    \"resolved_version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable machine-readable error code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The requested public module is absent.\","]
#[doc = "          \"const\": \"module_not_found\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"crate_name\": {"]
#[doc = "      \"description\": \"Requested crate name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation of the lookup failure.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"module\": {"]
#[doc = "      \"description\": \"Requested public module path.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resolved_version\": {"]
#[doc = "      \"description\": \"Version that was queried.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ModuleNotFoundError {
    #[doc = "Stable machine-readable error code."]
    pub code: ModuleNotFoundErrorCode,
    #[doc = "Requested crate name."]
    pub crate_name: ::std::string::String,
    #[doc = "Human-readable explanation of the lookup failure."]
    pub message: ::std::string::String,
    #[doc = "Requested public module path."]
    pub module: ::std::string::String,
    #[doc = "Version that was queried."]
    pub resolved_version: ::std::string::String,
}
#[doc = "Stable machine-readable error code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable machine-readable error code.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The requested public module is absent.\","]
#[doc = "      \"const\": \"module_not_found\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum ModuleNotFoundErrorCode {
    #[doc = "The requested public module is absent."]
    #[serde(rename = "module_not_found")]
    ModuleNotFound,
}
impl ::std::fmt::Display for ModuleNotFoundErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ModuleNotFound => f.write_str("module_not_found"),
        }
    }
}
impl ::std::str::FromStr for ModuleNotFoundErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "module_not_found" => Ok(Self::ModuleNotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModuleNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ModuleNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModuleNotFoundErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The operation failed for a reason no other error classifies."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The operation failed for a reason no other error classifies.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\","]
#[doc = "    \"retryable\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable machine-readable error code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"No more specific error classifies the failure.\","]
#[doc = "          \"const\": \"operation_failed\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"diagnostic_id\": {"]
#[doc = "      \"description\": \"Correlates this failure with a server log entry.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation of the failure.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"retryable\": {"]
#[doc = "      \"description\": \"Whether an identical request may succeed later.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct OperationFailedError {
    #[doc = "Stable machine-readable error code."]
    pub code: OperationFailedErrorCode,
    #[doc = "Correlates this failure with a server log entry."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub diagnostic_id: ::std::option::Option<::std::string::String>,
    #[doc = "Human-readable explanation of the failure."]
    pub message: ::std::string::String,
    #[doc = "Whether an identical request may succeed later."]
    pub retryable: bool,
}
#[doc = "Stable machine-readable error code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable machine-readable error code.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"No more specific error classifies the failure.\","]
#[doc = "      \"const\": \"operation_failed\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum OperationFailedErrorCode {
    #[doc = "No more specific error classifies the failure."]
    #[serde(rename = "operation_failed")]
    OperationFailed,
}
impl ::std::fmt::Display for OperationFailedErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OperationFailed => f.write_str("operation_failed"),
        }
    }
}
impl ::std::str::FromStr for OperationFailedErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "operation_failed" => Ok(Self::OperationFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OperationFailedErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OperationFailedErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OperationFailedErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Encoding of the canonical generated outcome selected by a library or MCP consumer."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Encoding of the canonical generated outcome selected by a library or MCP consumer.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Emit compact TOON text.\","]
#[doc = "      \"const\": \"toon\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Emit the canonical JSON outcome.\","]
#[doc = "      \"const\": \"json\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum OutputFormat {
    #[doc = "Emit compact TOON text."]
    #[serde(rename = "toon")]
    Toon,
    #[doc = "Emit the canonical JSON outcome."]
    #[serde(rename = "json")]
    Json,
}
impl ::std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Toon => f.write_str("toon"),
            Self::Json => f.write_str("json"),
        }
    }
}
impl ::std::str::FromStr for OutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "toon" => Ok(Self::Toon),
            "json" => Ok(Self::Json),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Orientation view of a crate root or public module."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Orientation view of a crate root or public module.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"docs_truncated\","]
#[doc = "    \"items\","]
#[doc = "    \"path\","]
#[doc = "    \"resolved_version\","]
#[doc = "    \"totals\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"docs\": {"]
#[doc = "      \"description\": \"Module documentation. Absent when undocumented.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"docs_truncated\": {"]
#[doc = "      \"description\": \"Whether docs was shortened. Request `full_docs` to receive it whole.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"items\": {"]
#[doc = "      \"description\": \"Items declared directly in this module.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OverviewItem\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Path of the described module.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resolved_version\": {"]
#[doc = "      \"description\": \"Version actually queried.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"totals\": {"]
#[doc = "      \"description\": \"Direct and descendant public item counts.\","]
#[doc = "      \"$ref\": \"#/$defs/Totals\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct Overview {
    #[doc = "Module documentation. Absent when undocumented."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub docs: ::std::option::Option<::std::string::String>,
    #[doc = "Whether docs was shortened. Request `full_docs` to receive it whole."]
    pub docs_truncated: bool,
    #[doc = "Items declared directly in this module."]
    pub items: ::std::vec::Vec<OverviewItem>,
    #[doc = "Path of the described module."]
    pub path: ::std::string::String,
    #[doc = "Version actually queried."]
    pub resolved_version: ::std::string::String,
    #[doc = "Direct and descendant public item counts."]
    pub totals: Totals,
}
#[doc = "A public item listed in an overview."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A public item listed in an overview.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"deprecated\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"path\","]
#[doc = "    \"reexport\","]
#[doc = "    \"summary\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"deprecated\": {"]
#[doc = "      \"description\": \"Whether the item is marked deprecated.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Rustdoc kind of this item.\","]
#[doc = "      \"$ref\": \"#/$defs/RustdocItemKind\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Item name within the described module.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Full path accepted by `get_item`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reexport\": {"]
#[doc = "      \"description\": \"Whether the item is defined outside the module that exposes it.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"description\": \"Rendered signature. Present only for kinds that have one.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"First paragraph of the item documentation. Empty when undocumented.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct OverviewItem {
    #[doc = "Whether the item is marked deprecated."]
    pub deprecated: bool,
    #[doc = "Rustdoc kind of this item."]
    pub kind: RustdocItemKind,
    #[doc = "Item name within the described module."]
    pub name: ::std::string::String,
    #[doc = "Full path accepted by `get_item`."]
    pub path: ::std::string::String,
    #[doc = "Whether the item is defined outside the module that exposes it."]
    pub reexport: bool,
    #[doc = "Rendered signature. Present only for kinds that have one."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub signature: ::std::option::Option<::std::string::String>,
    #[doc = "First paragraph of the item documentation. Empty when undocumented."]
    pub summary: ::std::string::String,
}
#[doc = "Result of an overview request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Result of an overview request.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"OverviewSuccess\","]
#[doc = "      \"description\": \"A completed overview.\","]
#[doc = "      \"$ref\": \"#/$defs/OverviewSuccess\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ErrorOutcome\","]
#[doc = "      \"description\": \"A failed overview operation.\","]
#[doc = "      \"$ref\": \"#/$defs/ErrorOutcome\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(untagged)]
pub enum OverviewOutcome {
    #[doc = "A completed overview."]
    OverviewSuccess(OverviewSuccess),
    #[doc = "A failed overview operation."]
    ErrorOutcome(ErrorOutcome),
}
impl ::std::convert::From<OverviewSuccess> for OverviewOutcome {
    fn from(value: OverviewSuccess) -> Self {
        Self::OverviewSuccess(value)
    }
}
impl ::std::convert::From<ErrorOutcome> for OverviewOutcome {
    fn from(value: ErrorOutcome) -> Self {
        Self::ErrorOutcome(value)
    }
}
#[doc = "Request for an orientation view of a crate root or public module."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request for an orientation view of a crate root or public module.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"crate_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"crate_name\": {"]
#[doc = "      \"description\": \"Crate to query.\","]
#[doc = "      \"$ref\": \"#/$defs/CrateName\""]
#[doc = "    },"]
#[doc = "    \"dependency_version\": {"]
#[doc = "      \"description\": \"Version the caller depends on, reported back as a comparison against the resolved version.\","]
#[doc = "      \"$ref\": \"#/$defs/DependencyVersion\""]
#[doc = "    },"]
#[doc = "    \"full_docs\": {"]
#[doc = "      \"description\": \"Include full module documentation rather than its first paragraph.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"module\": {"]
#[doc = "      \"description\": \"Module to describe. Defaults to the crate root.\","]
#[doc = "      \"$ref\": \"#/$defs/PublicModulePath\""]
#[doc = "    },"]
#[doc = "    \"output_format\": {"]
#[doc = "      \"description\": \"Requested response encoding. Defaults to JSON.\","]
#[doc = "      \"$ref\": \"#/$defs/OutputFormat\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"Version to query.\","]
#[doc = "      \"$ref\": \"#/$defs/RequestedVersion\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct OverviewRequest {
    #[doc = "Crate to query."]
    pub crate_name: CrateName,
    #[doc = "Version the caller depends on, reported back as a comparison against the resolved version."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub dependency_version: ::std::option::Option<DependencyVersion>,
    #[doc = "Include full module documentation rather than its first paragraph."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub full_docs: ::std::option::Option<bool>,
    #[doc = "Module to describe. Defaults to the crate root."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub module: ::std::option::Option<PublicModulePath>,
    #[doc = "Requested response encoding. Defaults to JSON."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub output_format: ::std::option::Option<OutputFormat>,
    #[doc = "Version to query."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub version: ::std::option::Option<RequestedVersion>,
}
#[doc = "A completed overview."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A completed overview.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"metadata\","]
#[doc = "    \"result\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Provenance for the completed operation.\","]
#[doc = "      \"$ref\": \"#/$defs/ResponseMetadata\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The requested overview.\","]
#[doc = "      \"$ref\": \"#/$defs/Overview\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Outcome status discriminator.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The operation completed successfully.\","]
#[doc = "          \"const\": \"ok\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct OverviewSuccess {
    #[doc = "Provenance for the completed operation."]
    pub metadata: ResponseMetadata,
    #[doc = "The requested overview."]
    pub result: Overview,
    #[doc = "Outcome status discriminator."]
    pub status: OverviewSuccessStatus,
}
#[doc = "Outcome status discriminator."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Outcome status discriminator.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The operation completed successfully.\","]
#[doc = "      \"const\": \"ok\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum OverviewSuccessStatus {
    #[doc = "The operation completed successfully."]
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for OverviewSuccessStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for OverviewSuccessStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OverviewSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OverviewSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OverviewSuccessStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Opaque continuation cursor returned by an earlier find_items page."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Opaque continuation cursor returned by an earlier find_items page.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1024,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct PageCursor(::std::string::String);
impl ::std::ops::Deref for PageCursor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PageCursor> for ::std::string::String {
    fn from(value: PageCursor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PageCursor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1024usize {
            return Err("longer than 1024 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PageCursor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PageCursor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PageCursor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PageCursor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A public Rust item path, optionally ending in an inherent method name."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A public Rust item path, optionally ending in an inherent method name.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct PublicItemPath(::std::string::String);
impl ::std::ops::Deref for PublicItemPath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicItemPath> for ::std::string::String {
    fn from(value: PublicItemPath) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PublicItemPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PublicItemPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PublicItemPath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicItemPath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicItemPath {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A public Rust module path."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A public Rust module path.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct PublicModulePath(::std::string::String);
impl ::std::ops::Deref for PublicModulePath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicModulePath> for ::std::string::String {
    fn from(value: PublicModulePath) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PublicModulePath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PublicModulePath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PublicModulePath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicModulePath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicModulePath {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A crates.io version or the literal latest."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A crates.io version or the literal latest.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct RequestedVersion(::std::string::String);
impl ::std::ops::Deref for RequestedVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RequestedVersion> for ::std::string::String {
    fn from(value: RequestedVersion) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RequestedVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RequestedVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RequestedVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequestedVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RequestedVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Provenance accompanying a successful outcome."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Provenance accompanying a successful outcome.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cache\": {"]
#[doc = "      \"description\": \"Cache provenance when available.\","]
#[doc = "      \"$ref\": \"#/$defs/CacheMetadata\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"Requested and resolved version provenance.\","]
#[doc = "      \"$ref\": \"#/$defs/VersionComparison\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ResponseMetadata {
    #[doc = "Cache provenance when available."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub cache: ::std::option::Option<CacheMetadata>,
    #[doc = "Requested and resolved version provenance."]
    pub version: VersionComparison,
}
#[doc = "Exact rustdoc item kind. Values preserve rustdoc JSON distinctions and are never normalized into a lossy category."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Exact rustdoc item kind. Values preserve rustdoc JSON distinctions and are never normalized into a lossy category.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"A Rust module.\","]
#[doc = "      \"const\": \"module\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An extern crate declaration.\","]
#[doc = "      \"const\": \"extern_crate\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A use declaration or re-export.\","]
#[doc = "      \"const\": \"use\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A struct.\","]
#[doc = "      \"const\": \"struct\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A struct field.\","]
#[doc = "      \"const\": \"struct_field\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A union.\","]
#[doc = "      \"const\": \"union\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An enum.\","]
#[doc = "      \"const\": \"enum\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An enum variant.\","]
#[doc = "      \"const\": \"variant\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A function.\","]
#[doc = "      \"const\": \"function\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A type alias.\","]
#[doc = "      \"const\": \"type_alias\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A constant.\","]
#[doc = "      \"const\": \"constant\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A trait.\","]
#[doc = "      \"const\": \"trait\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A trait alias.\","]
#[doc = "      \"const\": \"trait_alias\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An implementation block.\","]
#[doc = "      \"const\": \"impl\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A static item.\","]
#[doc = "      \"const\": \"static\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An extern type declaration.\","]
#[doc = "      \"const\": \"extern_type\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A declarative macro.\","]
#[doc = "      \"const\": \"macro\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An attribute procedural macro.\","]
#[doc = "      \"const\": \"proc_attribute\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A derive procedural macro.\","]
#[doc = "      \"const\": \"proc_derive\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An associated constant.\","]
#[doc = "      \"const\": \"assoc_const\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"An associated type.\","]
#[doc = "      \"const\": \"assoc_type\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A primitive type.\","]
#[doc = "      \"const\": \"primitive\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A Rust keyword.\","]
#[doc = "      \"const\": \"keyword\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A built-in attribute.\","]
#[doc = "      \"const\": \"attribute\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum RustdocItemKind {
    #[doc = "A Rust module."]
    #[serde(rename = "module")]
    Module,
    #[doc = "An extern crate declaration."]
    #[serde(rename = "extern_crate")]
    ExternCrate,
    #[doc = "A use declaration or re-export."]
    #[serde(rename = "use")]
    Use,
    #[doc = "A struct."]
    #[serde(rename = "struct")]
    Struct,
    #[doc = "A struct field."]
    #[serde(rename = "struct_field")]
    StructField,
    #[doc = "A union."]
    #[serde(rename = "union")]
    Union,
    #[doc = "An enum."]
    #[serde(rename = "enum")]
    Enum,
    #[doc = "An enum variant."]
    #[serde(rename = "variant")]
    Variant,
    #[doc = "A function."]
    #[serde(rename = "function")]
    Function,
    #[doc = "A type alias."]
    #[serde(rename = "type_alias")]
    TypeAlias,
    #[doc = "A constant."]
    #[serde(rename = "constant")]
    Constant,
    #[doc = "A trait."]
    #[serde(rename = "trait")]
    Trait,
    #[doc = "A trait alias."]
    #[serde(rename = "trait_alias")]
    TraitAlias,
    #[doc = "An implementation block."]
    #[serde(rename = "impl")]
    Impl,
    #[doc = "A static item."]
    #[serde(rename = "static")]
    Static,
    #[doc = "An extern type declaration."]
    #[serde(rename = "extern_type")]
    ExternType,
    #[doc = "A declarative macro."]
    #[serde(rename = "macro")]
    Macro,
    #[doc = "An attribute procedural macro."]
    #[serde(rename = "proc_attribute")]
    ProcAttribute,
    #[doc = "A derive procedural macro."]
    #[serde(rename = "proc_derive")]
    ProcDerive,
    #[doc = "An associated constant."]
    #[serde(rename = "assoc_const")]
    AssocConst,
    #[doc = "An associated type."]
    #[serde(rename = "assoc_type")]
    AssocType,
    #[doc = "A primitive type."]
    #[serde(rename = "primitive")]
    Primitive,
    #[doc = "A Rust keyword."]
    #[serde(rename = "keyword")]
    Keyword,
    #[doc = "A built-in attribute."]
    #[serde(rename = "attribute")]
    Attribute,
}
impl ::std::fmt::Display for RustdocItemKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Module => f.write_str("module"),
            Self::ExternCrate => f.write_str("extern_crate"),
            Self::Use => f.write_str("use"),
            Self::Struct => f.write_str("struct"),
            Self::StructField => f.write_str("struct_field"),
            Self::Union => f.write_str("union"),
            Self::Enum => f.write_str("enum"),
            Self::Variant => f.write_str("variant"),
            Self::Function => f.write_str("function"),
            Self::TypeAlias => f.write_str("type_alias"),
            Self::Constant => f.write_str("constant"),
            Self::Trait => f.write_str("trait"),
            Self::TraitAlias => f.write_str("trait_alias"),
            Self::Impl => f.write_str("impl"),
            Self::Static => f.write_str("static"),
            Self::ExternType => f.write_str("extern_type"),
            Self::Macro => f.write_str("macro"),
            Self::ProcAttribute => f.write_str("proc_attribute"),
            Self::ProcDerive => f.write_str("proc_derive"),
            Self::AssocConst => f.write_str("assoc_const"),
            Self::AssocType => f.write_str("assoc_type"),
            Self::Primitive => f.write_str("primitive"),
            Self::Keyword => f.write_str("keyword"),
            Self::Attribute => f.write_str("attribute"),
        }
    }
}
impl ::std::str::FromStr for RustdocItemKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "module" => Ok(Self::Module),
            "extern_crate" => Ok(Self::ExternCrate),
            "use" => Ok(Self::Use),
            "struct" => Ok(Self::Struct),
            "struct_field" => Ok(Self::StructField),
            "union" => Ok(Self::Union),
            "enum" => Ok(Self::Enum),
            "variant" => Ok(Self::Variant),
            "function" => Ok(Self::Function),
            "type_alias" => Ok(Self::TypeAlias),
            "constant" => Ok(Self::Constant),
            "trait" => Ok(Self::Trait),
            "trait_alias" => Ok(Self::TraitAlias),
            "impl" => Ok(Self::Impl),
            "static" => Ok(Self::Static),
            "extern_type" => Ok(Self::ExternType),
            "macro" => Ok(Self::Macro),
            "proc_attribute" => Ok(Self::ProcAttribute),
            "proc_derive" => Ok(Self::ProcDerive),
            "assoc_const" => Ok(Self::AssocConst),
            "assoc_type" => Ok(Self::AssocType),
            "primitive" => Ok(Self::Primitive),
            "keyword" => Ok(Self::Keyword),
            "attribute" => Ok(Self::Attribute),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RustdocItemKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RustdocItemKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RustdocItemKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Schema-first contract for the rustdoc-query library and MCP adapter. The root is a registry of named contract types; consumers select a named definition with $ref."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"urn:rustdoc-query:schema:v2\","]
#[doc = "  \"title\": \"rustdoc-query public API v2\","]
#[doc = "  \"description\": \"Schema-first contract for the rustdoc-query library and MCP adapter. The root is a registry of named contract types; consumers select a named definition with $ref.\","]
#[doc = "  \"x-mcp-tools\": ["]
#[doc = "    {"]
#[doc = "      \"name\": \"crate_overview\","]
#[doc = "      \"outcome\": \"OverviewOutcome\","]
#[doc = "      \"request\": \"OverviewRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"name\": \"find_items\","]
#[doc = "      \"outcome\": \"FindOutcome\","]
#[doc = "      \"request\": \"FindRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"name\": \"get_item\","]
#[doc = "      \"outcome\": \"GetItemOutcome\","]
#[doc = "      \"request\": \"GetItemRequest\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct RustdocQueryPublicApiV2(pub ::serde_json::Value);
impl ::std::ops::Deref for RustdocQueryPublicApiV2 {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<RustdocQueryPublicApiV2> for ::serde_json::Value {
    fn from(value: RustdocQueryPublicApiV2) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for RustdocQueryPublicApiV2 {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = "Rustdoc data could not be retrieved. Always safe to retry."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Rustdoc data could not be retrieved. Always safe to retry.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\","]
#[doc = "    \"retryable\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable machine-readable error code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"Rustdoc data could not be retrieved.\","]
#[doc = "          \"const\": \"source_unavailable\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation of the unavailable source.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"retryable\": {"]
#[doc = "      \"description\": \"Always true because retrying may succeed later.\","]
#[doc = "      \"type\": \"boolean\","]
#[doc = "      \"enum\": ["]
#[doc = "        true"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct SourceUnavailableError {
    #[doc = "Stable machine-readable error code."]
    pub code: SourceUnavailableErrorCode,
    #[doc = "Human-readable explanation of the unavailable source."]
    pub message: ::std::string::String,
    #[doc = "Always true because retrying may succeed later."]
    pub retryable: bool,
}
#[doc = "Stable machine-readable error code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable machine-readable error code.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Rustdoc data could not be retrieved.\","]
#[doc = "      \"const\": \"source_unavailable\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum SourceUnavailableErrorCode {
    #[doc = "Rustdoc data could not be retrieved."]
    #[serde(rename = "source_unavailable")]
    SourceUnavailable,
}
impl ::std::fmt::Display for SourceUnavailableErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SourceUnavailable => f.write_str("source_unavailable"),
        }
    }
}
impl ::std::str::FromStr for SourceUnavailableErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "source_unavailable" => Ok(Self::SourceUnavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SourceUnavailableErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SourceUnavailableErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SourceUnavailableErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Public item counts for a module."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Public item counts for a module.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"direct\","]
#[doc = "    \"subtree\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"direct\": {"]
#[doc = "      \"description\": \"Items declared in this module only.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/KindCount\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"subtree\": {"]
#[doc = "      \"description\": \"Items in this module and all descendant modules.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/KindCount\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct Totals {
    #[doc = "Items declared in this module only."]
    pub direct: ::std::vec::Vec<KindCount>,
    #[doc = "Items in this module and all descendant modules."]
    pub subtree: ::std::vec::Vec<KindCount>,
}
#[doc = "The crate's rustdoc data uses a format version this contract cannot read."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The crate's rustdoc data uses a format version this contract cannot read.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"found\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable machine-readable error code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The rustdoc format is outside the supported range.\","]
#[doc = "          \"const\": \"unsupported_rustdoc_format\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"found\": {"]
#[doc = "      \"description\": \"Format version encountered.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation of the format mismatch.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct UnsupportedRustdocFormatError {
    #[doc = "Stable machine-readable error code."]
    pub code: UnsupportedRustdocFormatErrorCode,
    #[doc = "Format version encountered."]
    pub found: u64,
    #[doc = "Human-readable explanation of the format mismatch."]
    pub message: ::std::string::String,
}
#[doc = "Stable machine-readable error code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable machine-readable error code.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The rustdoc format is outside the supported range.\","]
#[doc = "      \"const\": \"unsupported_rustdoc_format\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum UnsupportedRustdocFormatErrorCode {
    #[doc = "The rustdoc format is outside the supported range."]
    #[serde(rename = "unsupported_rustdoc_format")]
    UnsupportedRustdocFormat,
}
impl ::std::fmt::Display for UnsupportedRustdocFormatErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UnsupportedRustdocFormat => f.write_str("unsupported_rustdoc_format"),
        }
    }
}
impl ::std::str::FromStr for UnsupportedRustdocFormatErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unsupported_rustdoc_format" => Ok(Self::UnsupportedRustdocFormat),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UnsupportedRustdocFormatErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UnsupportedRustdocFormatErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UnsupportedRustdocFormatErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An enum variant with its field shape."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An enum variant with its field shape.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fields\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"description\": \"Empty for a unit variant. Tuple variant fields are named by position.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Field\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Field shape of this variant.\","]
#[doc = "      \"$ref\": \"#/$defs/VariantKind\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Variant name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    #[doc = "Empty for a unit variant. Tuple variant fields are named by position."]
    pub fields: ::std::vec::Vec<Field>,
    #[doc = "Field shape of this variant."]
    pub kind: VariantKind,
    #[doc = "Variant name."]
    pub name: ::std::string::String,
}
#[doc = "The field shape of an enum variant. This is distinct from RustdocItemKind."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The field shape of an enum variant. This is distinct from RustdocItemKind.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"A variant with no fields.\","]
#[doc = "      \"const\": \"unit\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A variant with positional fields.\","]
#[doc = "      \"const\": \"tuple\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A variant with named fields.\","]
#[doc = "      \"const\": \"struct\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum VariantKind {
    #[doc = "A variant with no fields."]
    #[serde(rename = "unit")]
    Unit,
    #[doc = "A variant with positional fields."]
    #[serde(rename = "tuple")]
    Tuple,
    #[doc = "A variant with named fields."]
    #[serde(rename = "struct")]
    Struct,
}
impl ::std::fmt::Display for VariantKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unit => f.write_str("unit"),
            Self::Tuple => f.write_str("tuple"),
            Self::Struct => f.write_str("struct"),
        }
    }
}
impl ::std::str::FromStr for VariantKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unit" => Ok(Self::Unit),
            "tuple" => Ok(Self::Tuple),
            "struct" => Ok(Self::Struct),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VariantKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VariantKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VariantKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Agreement between the version queried and the version the caller depends on."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Agreement between the version queried and the version the caller depends on.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\","]
#[doc = "    \"requested_version\","]
#[doc = "    \"resolved_version\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dependency_version\": {"]
#[doc = "      \"description\": \"Version echoed back from the request. Absent when the caller supplied none.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Explanation of the comparison outcome.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"requested_version\": {"]
#[doc = "      \"description\": \"Version as requested, before resolution.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resolved_version\": {"]
#[doc = "      \"description\": \"Version actually queried.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Comparison outcome. Versions agree only on exact equality; semver matching is not used.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"The supplied dependency version exactly matches the resolved version.\","]
#[doc = "          \"const\": \"matched\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"The supplied dependency version differs from the resolved version.\","]
#[doc = "          \"const\": \"mismatch\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"No dependency version was supplied.\","]
#[doc = "          \"const\": \"not_compared\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct VersionComparison {
    #[doc = "Version echoed back from the request. Absent when the caller supplied none."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(deserialize_with = "super::strict_option::deserialize")]
    pub dependency_version: ::std::option::Option<::std::string::String>,
    #[doc = "Explanation of the comparison outcome."]
    pub message: ::std::string::String,
    #[doc = "Version as requested, before resolution."]
    pub requested_version: ::std::string::String,
    #[doc = "Version actually queried."]
    pub resolved_version: ::std::string::String,
    #[doc = "Comparison outcome. Versions agree only on exact equality; semver matching is not used."]
    pub status: VersionComparisonStatus,
}
#[doc = "Comparison outcome. Versions agree only on exact equality; semver matching is not used."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Comparison outcome. Versions agree only on exact equality; semver matching is not used.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The supplied dependency version exactly matches the resolved version.\","]
#[doc = "      \"const\": \"matched\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"The supplied dependency version differs from the resolved version.\","]
#[doc = "      \"const\": \"mismatch\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"No dependency version was supplied.\","]
#[doc = "      \"const\": \"not_compared\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rmcp :: schemars :: JsonSchema,
)]
pub enum VersionComparisonStatus {
    #[doc = "The supplied dependency version exactly matches the resolved version."]
    #[serde(rename = "matched")]
    Matched,
    #[doc = "The supplied dependency version differs from the resolved version."]
    #[serde(rename = "mismatch")]
    Mismatch,
    #[doc = "No dependency version was supplied."]
    #[serde(rename = "not_compared")]
    NotCompared,
}
impl ::std::fmt::Display for VersionComparisonStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Matched => f.write_str("matched"),
            Self::Mismatch => f.write_str("mismatch"),
            Self::NotCompared => f.write_str("not_compared"),
        }
    }
}
impl ::std::str::FromStr for VersionComparisonStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "matched" => Ok(Self::Matched),
            "mismatch" => Ok(Self::Mismatch),
            "not_compared" => Ok(Self::NotCompared),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VersionComparisonStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VersionComparisonStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VersionComparisonStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
#[doc = " Error returned when a value is outside the schema-defined range."]
pub struct PageSizeError {
    value: ::std::primitive::u64,
}
impl ::std::fmt::Display for PageSizeError {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            formatter,
            "PageSize must be between 1 and 200; got {}",
            self.value
        )
    }
}
impl ::std::error::Error for PageSizeError {}
#[derive(:: serde :: Serialize, Clone, Debug, rmcp :: schemars :: JsonSchema)]
#[serde(transparent)]
#[doc = " Maximum rows to return in this page."]
pub struct PageSize(#[schemars(range(min = 1, max = 200))] ::std::primitive::u8);
impl PageSize {
    #[doc = " Smallest value accepted by this schema type."]
    pub const MINIMUM: ::std::primitive::u64 = 1;
    #[doc = " Largest value accepted by this schema type."]
    pub const MAXIMUM: ::std::primitive::u64 = 200;
    #[doc = " Creates a value within the schema-defined range."]
    #[doc = ""]
    #[doc = " # Errors"]
    #[doc = ""]
    #[doc = " Returns `PageSizeError` when `value` is outside that range."]
    pub fn new(value: ::std::primitive::u64) -> ::std::result::Result<Self, PageSizeError> {
        if (Self::MINIMUM..=Self::MAXIMUM).contains(&value) {
            match <::std::primitive::u8>::try_from(value) {
                Ok(value) => Ok(Self(value)),
                Err(_) => Err(PageSizeError { value }),
            }
        } else {
            Err(PageSizeError { value })
        }
    }
    #[must_use]
    #[doc = " Returns the validated integer value."]
    pub fn get(&self) -> ::std::primitive::u8 {
        self.0
    }
}
impl ::std::convert::TryFrom<::std::primitive::u64> for PageSize {
    type Error = PageSizeError;
    fn try_from(value: ::std::primitive::u64) -> ::std::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl ::std::convert::From<PageSize> for ::std::primitive::u64 {
    fn from(value: PageSize) -> Self {
        value.0.into()
    }
}
impl<'de> ::serde::Deserialize<'de> for PageSize {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let value = <::std::primitive::u64 as ::serde::Deserialize>::deserialize(deserializer)?;
        Self::new(value).map_err(<D::Error as ::serde::de::Error>::custom)
    }
}
impl ::std::default::Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}
