//! Maps private pipeline failures onto the generated public error contract.
//!
//! Only schema-defined error codes, messages, and safe context cross the boundary.

use crate::contract::generated::{
    ApiError, CrateNotFoundError, CrateNotFoundErrorCode, ItemNotFoundError, ItemNotFoundErrorCode,
    ModuleNotFoundError, ModuleNotFoundErrorCode, OperationFailedError, OperationFailedErrorCode,
    SourceUnavailableError, SourceUnavailableErrorCode, UnsupportedRustdocFormatError,
    UnsupportedRustdocFormatErrorCode,
};
use crate::error::Error;

pub(crate) fn to_api_error(error: &Error) -> ApiError {
    match error {
        Error::NotFound { krate, version } => ApiError::CrateNotFoundError(CrateNotFoundError {
            code: CrateNotFoundErrorCode::CrateNotFound,
            crate_name: krate.clone(),
            resolved_version: version.clone(),
            message: error.to_string(),
        }),
        Error::UnsupportedFormat { found } => {
            ApiError::UnsupportedRustdocFormatError(UnsupportedRustdocFormatError {
                code: UnsupportedRustdocFormatErrorCode::UnsupportedRustdocFormat,
                found: u64::from(*found),
                message: error.to_string(),
            })
        }
        Error::Http(_) => ApiError::SourceUnavailableError(SourceUnavailableError {
            code: SourceUnavailableErrorCode::SourceUnavailable,
            message: error.to_string(),
            retryable: true,
        }),
        Error::Shared(cause) => to_api_error(cause),
        Error::OfflineCacheMiss { .. }
        | Error::Io(_)
        | Error::Json(_)
        | Error::Sidecar(_)
        | Error::Fts(_)
        | Error::NoCacheDir => operation_failed(&error.to_string()),
    }
}

/// Build the catch-all public failure for an infrastructure error.
pub(crate) fn operation_failed(message: &str) -> ApiError {
    ApiError::OperationFailedError(OperationFailedError {
        code: OperationFailedErrorCode::OperationFailed,
        message: message.to_owned(),
        retryable: false,
        diagnostic_id: None,
    })
}

pub(crate) fn module_not_found(crate_name: &str, resolved_version: &str, module: &str) -> ApiError {
    ApiError::ModuleNotFoundError(ModuleNotFoundError {
        code: ModuleNotFoundErrorCode::ModuleNotFound,
        crate_name: crate_name.to_owned(),
        resolved_version: resolved_version.to_owned(),
        module: module.to_owned(),
        message: format!("no public module `{module}` in `{crate_name}` {resolved_version}"),
    })
}

pub(crate) fn item_not_found(crate_name: &str, resolved_version: &str, path: &str) -> ApiError {
    ApiError::ItemNotFoundError(ItemNotFoundError {
        code: ItemNotFoundErrorCode::ItemNotFound,
        crate_name: crate_name.to_owned(),
        resolved_version: resolved_version.to_owned(),
        path: path.to_owned(),
        message: format!("no public item `{path}` in `{crate_name}` {resolved_version}"),
    })
}

#[cfg(test)]
#[path = "error/tests.rs"]
mod tests;
