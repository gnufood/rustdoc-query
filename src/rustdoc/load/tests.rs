use super::*;

fn zst(json: &str) -> Vec<u8> {
    zstd::stream::encode_all(json.as_bytes(), 0).unwrap()
}

#[test]
fn rejects_fv_below_floor() {
    let blob = zst(r#"{"format_version":49}"#);
    let err = load_crate(&blob).unwrap_err();
    assert!(
        matches!(err, Error::UnsupportedFormat { found: 49 }),
        "unexpected error: {err:?}"
    );
}

#[test]
fn rejects_format_above_native_ceiling() {
    let found = rustdoc_types::FORMAT_VERSION + 1;
    let blob = zst(&format!(r#"{{"format_version":{found}}}"#));
    let err = load_crate(&blob).unwrap_err();
    assert!(
        matches!(err, Error::UnsupportedFormat { found: actual } if actual == found),
        "unexpected error: {err:?}"
    );
}

#[test]
fn rejects_non_zstd_input() {
    let err = load_crate(b"not a zstd frame").unwrap_err();
    assert!(matches!(err, Error::Io(_)));
}

#[test]
fn attr_non_exhaustive_maps_correctly() {
    let v = attr_string_to_value("#[non_exhaustive]");
    assert_eq!(v, json!("non_exhaustive"));
}

#[test]
fn attr_unknown_becomes_other() {
    let v = attr_string_to_value("#[cold]");
    assert_eq!(v, json!({ "other": "#[cold]" }));
}

#[test]
fn external_crate_path_patch() {
    let mut value = json!({
        "external_crates": {
            "0": { "name": "std", "html_root_url": null },
            "1": { "name": "core", "html_root_url": null, "path": "/existing" }
        }
    });
    patch_external_crate_path(&mut value);
    assert_eq!(value["external_crates"]["0"]["path"], json!(""));
    assert_eq!(value["external_crates"]["1"]["path"], json!("/existing"));
}

#[test]
fn missing_item_field_patch() {
    let mut value = json!({
        "index": {
            "0": { "name": "foo" },
            "1": { "name": "bar", "stability": { "feature": "x" } }
        }
    });
    patch_missing_item_field(&mut value, "stability");
    assert_eq!(value["index"]["0"]["stability"], json!(null));
    assert_eq!(value["index"]["1"]["stability"], json!({ "feature": "x" }));
}

#[test]
fn stability_representation_migrates_from_flattened_stable_level() {
    let mut value = json!({
        "index": {
            "1": {
                "stability": { "feature": "api", "level": "stable", "since": "1.0.0" },
                "const_stability": { "feature": "const_api", "level": "unstable" }
            }
        }
    });

    patch_stability_representation(&mut value);

    assert_eq!(
        value["index"]["1"]["stability"],
        json!({ "feature": "api", "level": { "stable": { "since": "1.0.0" } } })
    );
    assert_eq!(
        value["index"]["1"]["const_stability"],
        json!({ "feature": "const_api", "level": "unstable" })
    );
}

#[test]
fn default_unstable_patch() {
    let mut value = json!({
        "index": {
            "0": { "inner": { "function": { "has_body": true } } },
            "1": { "inner": { "assoc_const": { "value": null } } },
            "2": { "inner": { "assoc_type": { "type_": null } } },
            "3": { "inner": { "module": { "is_crate": true } } }
        }
    });
    patch_default_unstable(&mut value);
    assert_eq!(
        value["index"]["0"]["inner"]["function"]["default_unstable"],
        json!(null)
    );
    assert_eq!(
        value["index"]["1"]["inner"]["assoc_const"]["default_unstable"],
        json!(null)
    );
    assert_eq!(
        value["index"]["2"]["inner"]["assoc_type"]["default_unstable"],
        json!(null)
    );
    assert_eq!(
        value["index"]["3"]["inner"]["module"].get("default_unstable"),
        None
    );
}

/// An fv59 document migrates successfully to the native schema, including a
/// `Function` whose `default_unstable` field is absent.
#[test]
fn loads_fv59_document_end_to_end() {
    let doc = json!({
        "root": 0,
        "crate_version": null,
        "includes_private": false,
        "index": {
            "0": {
                "id": 0, "crate_id": 0, "name": "root", "span": null,
                "visibility": "public", "docs": null, "links": {}, "attrs": [],
                "deprecation": null,
                "inner": { "module": { "is_crate": true, "items": [1], "is_stripped": false } }
            },
            "1": {
                "id": 1, "crate_id": 0, "name": "foo", "span": null,
                "visibility": "public", "docs": null, "links": {}, "attrs": [],
                "deprecation": null,
                "inner": { "function": {
                    "sig": { "inputs": [], "output": null, "is_c_variadic": false },
                    "generics": { "params": [], "where_predicates": [] },
                    "header": { "is_const": false, "is_unsafe": false, "is_async": false, "abi": "Rust" },
                    "has_body": true
                } }
            }
        },
        "paths": {},
        "external_crates": {},
        "target": { "triple": "x86_64-unknown-linux-gnu", "target_features": [] },
        "format_version": 59
    });
    let blob = zst(&doc.to_string());
    let krate = load_crate(&blob).expect("fv59 document should load");
    assert_eq!(
        krate.format_version,
        rustdoc_types::FORMAT_VERSION,
        "historical documents are normalized to the linked native schema"
    );
    let foo = &krate.index[&rustdoc_types::Id(1)];
    assert!(
        matches!(&foo.inner, rustdoc_types::ItemEnum::Function(f) if f.default_unstable.is_none())
    );
}
