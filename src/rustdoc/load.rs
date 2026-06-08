//! Loads compressed rustdoc JSON into the native typed schema.
//!
//! Supported format versions are migrated before deserialization.

use crate::error::{Error, Result};
use serde::Deserialize;

mod patches;
use patches::migrate;
#[cfg(test)]
use patches::{
    attr_string_to_value, patch_default_unstable, patch_external_crate_path,
    patch_missing_item_field, patch_stability_representation,
};
#[cfg(test)]
use serde_json::json;

/// Minimal view used to read `format_version` without deserializing the whole
/// document. Every rustdoc JSON blob carries this field at the top level.
#[derive(Deserialize)]
struct VersionPeek {
    format_version: u32,
}

/// Accepts format versions 50 through [`rustdoc_types::FORMAT_VERSION`],
/// migrating older supported schemas before deserialization.
pub(crate) fn load_crate(zst: &[u8]) -> Result<rustdoc_types::Crate> {
    let json = zstd::stream::decode_all(zst)?;

    let VersionPeek { format_version } = serde_json::from_slice(&json)?;

    let native = rustdoc_types::FORMAT_VERSION;
    let json = if format_version == native {
        json
    } else if (50..native).contains(&format_version) {
        migrate(&json, format_version)?
    } else {
        return Err(Error::UnsupportedFormat {
            found: format_version,
        });
    };

    Ok(serde_json::from_slice(&json)?)
}

#[cfg(test)]
#[path = "load/tests.rs"]
mod tests;
