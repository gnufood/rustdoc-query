//! Private, versioned continuation tokens for `find_items`.
//!
//! Tokens are bound to query, filter, and resolved crate version to prevent replay.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};

use crate::contract::generated::{
    ApiError, FindRequest, InvalidRequestError, InvalidRequestErrorCode, RustdocItemKind,
};

// Bump when the payload shape changes, so mismatched cursors fail validation
// instead of decoding into the wrong shape.
const CURSOR_VERSION: u8 = 3;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct Context {
    crate_name: String,
    query: Option<String>,
    doc_query: Option<String>,
    kind: Option<String>,
    module: Option<String>,
    resolved_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum Position {
    Documentation { offset: usize },
    Memory { order: MemoryOrder },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum MemoryOrder {
    Alphabetical { path: String },
    Fuzzy { score: i64, path: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    v: u8,
    context: Context,
    position: Position,
}

pub(super) fn context(request: &FindRequest, resolved_version: &str) -> Context {
    Context {
        crate_name: request.crate_name.as_str().to_owned(),
        query: request
            .query
            .as_ref()
            .map(|query| normalize(query.as_str())),
        doc_query: request
            .doc_query
            .as_ref()
            .map(|query| normalize(query.as_str())),
        kind: request.kind.map(kind_name),
        module: request
            .module
            .as_ref()
            .map(|module| module.as_str().to_owned()),
        resolved_version: resolved_version.to_owned(),
    }
}

pub(super) fn decode(
    request: &FindRequest,
    resolved_version: &str,
) -> Result<Option<Position>, ApiError> {
    let Some(cursor) = request.cursor.as_ref() else {
        return Ok(None);
    };
    let bytes = URL_SAFE_NO_PAD
        .decode(cursor.as_str())
        .map_err(|_| invalid_cursor())?;
    let payload: Payload = serde_json::from_slice(&bytes).map_err(|_| invalid_cursor())?;
    if payload.v != CURSOR_VERSION || payload.context != context(request, resolved_version) {
        return Err(invalid_cursor());
    }
    match &payload.position {
        Position::Documentation { offset } if *offset > 0 => {}
        Position::Memory { .. } => {}
        Position::Documentation { .. } => return Err(invalid_cursor()),
    }
    Ok(Some(payload.position))
}

pub(super) fn type_mismatch() -> ApiError {
    invalid_cursor()
}

pub(super) fn encode(context: Context, position: Position) -> Result<String, ApiError> {
    let bytes = serde_json::to_vec(&Payload {
        v: CURSOR_VERSION,
        context,
        position,
    })
    .map_err(|_| invalid_cursor())?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

pub(super) fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn kind_name(kind: RustdocItemKind) -> String {
    kind.to_string()
}

fn invalid_cursor() -> ApiError {
    ApiError::InvalidRequestError(InvalidRequestError {
        code: InvalidRequestErrorCode::InvalidRequest,
        message: "`cursor` is malformed, stale, or does not match this find request".to_owned(),
        field: Some("cursor".to_owned()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(value: serde_json::Value) -> FindRequest {
        serde_json::from_value(value).expect("valid find request")
    }

    #[test]
    fn cursors_are_bound_to_the_normalized_request_and_version() {
        let original = request(serde_json::json!({
            "crate_name": "demo", "doc_query": "send   request", "kind": "function"
        }));
        let token = encode(
            context(&original, "1.0.0"),
            Position::Documentation { offset: 9 },
        )
        .expect("test cursor encodes");
        let resumed = request(serde_json::json!({
            "crate_name": "demo", "doc_query": "send request", "kind": "function", "cursor": token
        }));
        assert!(decode(&resumed, "1.0.0").is_ok());
        assert!(decode(&resumed, "2.0.0").is_err());
    }
}
