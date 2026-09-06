use super::resolve;
use crate::contract::generated::{FindRequest, GetItemRequest, OutputFormat, OverviewRequest};

fn default_format() -> OutputFormat {
    OutputFormat::default()
}

#[test]
fn an_omitted_format_resolves_to_json() {
    assert_eq!(resolve(None), default_format());
}

#[test]
fn an_explicit_format_is_used_verbatim() {
    assert_eq!(resolve(Some(OutputFormat::Toon)), OutputFormat::Toon);
    assert_eq!(resolve(Some(OutputFormat::Json)), default_format());
}

#[test]
fn every_request_omitting_the_field_resolves_to_json() {
    let overview: OverviewRequest =
        serde_json::from_value(serde_json::json!({ "crate_name": "reqwest" }))
            .expect("overview request is valid");
    let find: FindRequest = serde_json::from_value(serde_json::json!({ "crate_name": "reqwest" }))
        .expect("find request is valid");
    let get_item: GetItemRequest = serde_json::from_value(
        serde_json::json!({ "crate_name": "reqwest", "path": "reqwest::Client" }),
    )
    .expect("get_item request is valid");

    assert_eq!(resolve(overview.output_format), default_format());
    assert_eq!(resolve(find.output_format), default_format());
    assert_eq!(resolve(get_item.output_format), default_format());
}

#[test]
fn a_requested_toon_format_survives_deserialization() {
    let request: OverviewRequest = serde_json::from_value(
        serde_json::json!({ "crate_name": "reqwest", "output_format": "toon" }),
    )
    .expect("overview request is valid");

    assert_eq!(resolve(request.output_format), OutputFormat::Toon);
}
