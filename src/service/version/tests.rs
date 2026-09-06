use super::requested;
use crate::contract::generated::{FindRequest, GetItemRequest, OverviewRequest};

#[test]
fn an_omitted_version_resolves_to_latest() {
    let overview: OverviewRequest =
        serde_json::from_value(serde_json::json!({ "crate_name": "reqwest" }))
            .expect("overview request is valid");
    let find: FindRequest = serde_json::from_value(serde_json::json!({ "crate_name": "reqwest" }))
        .expect("find request is valid");
    let get_item: GetItemRequest = serde_json::from_value(
        serde_json::json!({ "crate_name": "reqwest", "path": "reqwest::Client" }),
    )
    .expect("get_item request is valid");

    assert_eq!(
        requested(overview.version.as_ref()),
        crate::contract::defaults::REQUESTED_VERSION
    );
    assert_eq!(
        requested(find.version.as_ref()),
        crate::contract::defaults::REQUESTED_VERSION
    );
    assert_eq!(
        requested(get_item.version.as_ref()),
        crate::contract::defaults::REQUESTED_VERSION
    );
}

#[test]
fn an_explicit_version_is_used_verbatim() {
    let request: FindRequest =
        serde_json::from_value(serde_json::json!({ "crate_name": "reqwest", "version": "0.13.4" }))
            .expect("request is valid");

    assert_eq!(requested(request.version.as_ref()), "0.13.4");
}
