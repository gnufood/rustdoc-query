use serde_json::json;

use crate::error::Result;

/// Migrate a document from `from` to [`rustdoc_types::FORMAT_VERSION`].
///
/// Applies only the required additive migration steps.
pub(crate) fn migrate(json: &[u8], from: u32) -> Result<Vec<u8>> {
    let mut value: serde_json::Value = serde_json::from_slice(json)?;

    if from < 54 {
        // fv54 replaced raw `Item::attrs` strings with typed `Attribute`s;
        // unrecognized attributes become `Attribute::Other`.
        if let Some(index) = value.get_mut("index").and_then(|v| v.as_object_mut()) {
            for item in index.values_mut() {
                if let Some(attrs) = item.get_mut("attrs").and_then(|a| a.as_array_mut()) {
                    for attr in attrs.iter_mut() {
                        if let Some(s) = attr.as_str().map(str::to_owned) {
                            *attr = attr_string_to_value(&s);
                        }
                    }
                }
            }
        }
    }
    if from < 57 {
        // `ExternalCrate::path` was introduced in fv57; older JSON legitimately
        // omits it.
        patch_external_crate_path(&mut value);
    }
    if from < 58 {
        // `Item::stability` was introduced in fv58.
        patch_missing_item_field(&mut value, "stability");
    }
    if from < 59 {
        // `Item::const_stability` was introduced in fv59.
        patch_missing_item_field(&mut value, "const_stability");
    }
    if from < 61 {
        patch_stability_representation(&mut value);
    }
    // `default_unstable` on `function`/`assoc_const`/`assoc_type` item kinds was
    // introduced in fv60; every version this function handles (50–59) needs it.
    patch_default_unstable(&mut value);
    value["format_version"] = json!(rustdoc_types::FORMAT_VERSION);

    Ok(serde_json::to_vec(&value)?)
}

/// Convert fv60's flattened stable representation into fv61's externally
/// tagged `StabilityLevel`. Unstable values are already represented as the
/// fv61 unit variant and need no rewrite.
pub(super) fn patch_stability_representation(value: &mut serde_json::Value) {
    let Some(index) = value.get_mut("index").and_then(|v| v.as_object_mut()) else {
        return;
    };
    for item in index.values_mut() {
        for field in ["stability", "const_stability"] {
            let Some(stability) = item.get_mut(field).and_then(|v| v.as_object_mut()) else {
                continue;
            };
            if stability.get("level") != Some(&json!("stable")) {
                continue;
            }
            let since = stability.remove("since").unwrap_or(serde_json::Value::Null);
            stability.insert("level".to_owned(), json!({ "stable": { "since": since } }));
        }
    }
}

/// Convert a raw fv50–53 attribute string to its fv54+ JSON representation.
///
/// We reconstruct the unit variants that the rest of the crate uses
/// (`NonExhaustive`). Everything else becomes `Attribute::Other(string)`,
/// which serialises as `{"other": "..."}`.
pub(super) fn attr_string_to_value(s: &str) -> serde_json::Value {
    match s {
        "#[non_exhaustive]" => json!("non_exhaustive"),
        _ => json!({ "other": s }),
    }
}

/// Introduced in fv57; older JSON legitimately omits it.
pub(super) fn patch_external_crate_path(value: &mut serde_json::Value) {
    if let Some(ext) = value
        .get_mut("external_crates")
        .and_then(|v| v.as_object_mut())
    {
        for ec in ext.values_mut() {
            if let Some(obj) = ec.as_object_mut() {
                obj.entry("path").or_insert_with(|| json!(""));
            }
        }
    }
}

/// Used for `Item::stability` (fv58) and `Item::const_stability` (fv59), both
/// of which are `Option` fields absent from older documents.
pub(super) fn patch_missing_item_field(value: &mut serde_json::Value, field: &str) {
    if let Some(index) = value.get_mut("index").and_then(|v| v.as_object_mut()) {
        for item in index.values_mut() {
            if let Some(obj) = item.as_object_mut() {
                obj.entry(field).or_insert(serde_json::Value::Null);
            }
        }
    }
}

/// Introduced in fv60 on the `function`/`assoc_const`/`assoc_type` `ItemEnum`
/// variants specifically (not on `Item` itself), so it lives under
/// `item.inner.<variant>` rather than at the top level.
pub(super) fn patch_default_unstable(value: &mut serde_json::Value) {
    if let Some(index) = value.get_mut("index").and_then(|v| v.as_object_mut()) {
        for item in index.values_mut() {
            let Some(inner) = item.get_mut("inner").and_then(|v| v.as_object_mut()) else {
                continue;
            };
            for variant in ["function", "assoc_const", "assoc_type"] {
                if let Some(payload) = inner.get_mut(variant).and_then(|v| v.as_object_mut()) {
                    payload
                        .entry("default_unstable")
                        .or_insert(serde_json::Value::Null);
                }
            }
        }
    }
}
