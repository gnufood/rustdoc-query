//! Public application service for the generated rustdoc-query contract.

use std::fmt::Display;

use serde::Serialize;
use serde_json::Value;

use crate::cache::CrateCache;
use crate::contract::generated::{
    ApiError, ErrorOutcome, ErrorOutcomeStatus, FindOutcome, FindRequest, FindSuccess,
    FindSuccessStatus, GetItemOutcome, GetItemRequest, GetItemSuccess, GetItemSuccessStatus,
    OutputFormat, OverviewOutcome, OverviewRequest, OverviewSuccess, OverviewSuccessStatus,
    ResponseMetadata,
};
use crate::query::{build_overview, resolve_item_detail, ItemDetailLookup};

mod detail;
mod error;
mod find;
mod format;
mod kind;
mod metadata;
mod overview;
mod version;

#[cfg(test)]
mod test_support;

/// Public application boundary for rustdoc-query operations and serialization.
#[derive(Debug)]
#[allow(
    clippy::module_name_repetitions,
    reason = "This is the public application-boundary type."
)]
pub struct RustdocQueryService {
    cache: CrateCache,
}

#[derive(Debug)]
pub(crate) struct JsonEncoding {
    pub(crate) value: Value,
    pub(crate) text: String,
}

impl RustdocQueryService {
    /// Opens the platform's rustdoc cache.
    ///
    /// Set `RUSTDOC_QUERY_CACHE_DIR` to place the cache in an explicit
    /// directory.
    ///
    /// # Errors
    ///
    /// Returns an API error when the cache directory cannot be initialized.
    pub fn new() -> Result<Self, ApiError> {
        Ok(Self {
            cache: CrateCache::new().map_err(|failure| error::to_api_error(&failure))?,
        })
    }

    /// Orientation view for a crate root or one of its public modules.
    pub async fn overview(&self, request: OverviewRequest) -> OverviewOutcome {
        match self.overview_success(request).await {
            Ok(success) => OverviewOutcome::OverviewSuccess(success),
            Err(error) => OverviewOutcome::ErrorOutcome(ErrorOutcome {
                status: ErrorOutcomeStatus::Error,
                error,
            }),
        }
    }

    async fn overview_success(
        &self,
        request: OverviewRequest,
    ) -> Result<OverviewSuccess, ApiError> {
        let crate_name = request.crate_name.as_str();
        let requested = version::requested(request.version.as_ref());
        let cached = self
            .cache
            .get_resolved_with_report(crate_name, requested)
            .await
            .map_err(|failure| error::to_api_error(&failure))?;

        let module = request.module.as_ref().map(|module| module.as_str());
        let rendered = build_overview(
            &cached.entry,
            module,
            request
                .full_docs
                .unwrap_or(crate::contract::defaults::OVERVIEW_REQUEST_FULL_DOCS),
        )
        .ok_or_else(|| {
            error::module_not_found(
                crate_name,
                &cached.resolved_version,
                module.unwrap_or(crate_name),
            )
        })?;

        Ok(OverviewSuccess {
            status: OverviewSuccessStatus::Ok,
            result: overview::to_contract(rendered, &cached.resolved_version),
            metadata: ResponseMetadata {
                cache: Some(metadata::cache_metadata(cached.report)),
                version: metadata::version_comparison(
                    requested,
                    &cached.resolved_version,
                    request.dependency_version.as_ref().map(|v| v.as_str()),
                ),
            },
        })
    }

    /// Search a crate's public surface by name, kind, module, or documentation.
    pub async fn find(&self, request: FindRequest) -> FindOutcome {
        match self.find_success(request).await {
            Ok(success) => FindOutcome::FindSuccess(success),
            Err(error) => FindOutcome::ErrorOutcome(ErrorOutcome {
                status: ErrorOutcomeStatus::Error,
                error,
            }),
        }
    }

    async fn find_success(&self, request: FindRequest) -> Result<FindSuccess, ApiError> {
        find::validate(&request)?;

        let requested = version::requested(request.version.as_ref());
        let cached = self
            .cache
            .get_resolved_with_report(request.crate_name.as_str(), requested)
            .await
            .map_err(|failure| error::to_api_error(&failure))?;

        let page = find::page(&cached.entry, &request, &cached.resolved_version).await?;

        Ok(FindSuccess {
            status: FindSuccessStatus::Ok,
            result: find::to_contract(page, &cached.resolved_version)?,
            metadata: ResponseMetadata {
                cache: Some(metadata::cache_metadata(cached.report)),
                version: metadata::version_comparison(
                    requested,
                    &cached.resolved_version,
                    request.dependency_version.as_ref().map(|v| v.as_str()),
                ),
            },
        })
    }

    /// Full public detail for one item or inherent method.
    pub async fn get_item(&self, request: GetItemRequest) -> GetItemOutcome {
        match self.get_item_success(request).await {
            Ok(success) => GetItemOutcome::GetItemSuccess(success),
            Err(error) => GetItemOutcome::ErrorOutcome(ErrorOutcome {
                status: ErrorOutcomeStatus::Error,
                error,
            }),
        }
    }

    async fn get_item_success(&self, request: GetItemRequest) -> Result<GetItemSuccess, ApiError> {
        let requested = version::requested(request.version.as_ref());
        let cached = self
            .cache
            .get_resolved_with_report(request.crate_name.as_str(), requested)
            .await
            .map_err(|failure| error::to_api_error(&failure))?;
        let detail = resolve_item_detail(
            &cached.entry,
            request.path.as_str(),
            request
                .include_impls
                .unwrap_or(crate::contract::defaults::GET_ITEM_REQUEST_INCLUDE_IMPLS),
            request
                .full_docs
                .unwrap_or(crate::contract::defaults::GET_ITEM_REQUEST_FULL_DOCS),
        );
        let detail = match detail {
            ItemDetailLookup::Found(detail) => *detail,
            ItemDetailLookup::Missing => {
                return Err(error::item_not_found(
                    request.crate_name.as_str(),
                    &cached.resolved_version,
                    request.path.as_str(),
                ))
            }
        };
        Ok(GetItemSuccess {
            status: GetItemSuccessStatus::Ok,
            result: detail::to_contract(detail, &cached.resolved_version),
            metadata: ResponseMetadata {
                cache: Some(metadata::cache_metadata(cached.report)),
                version: metadata::version_comparison(
                    requested,
                    &cached.resolved_version,
                    request.dependency_version.as_ref().map(|v| v.as_str()),
                ),
            },
        })
    }

    /// Resolves `output_format`, falling back to the schema default when omitted.
    #[must_use]
    pub fn output_format(requested: Option<OutputFormat>) -> OutputFormat {
        format::resolve(requested)
    }

    /// Serializes `value` as a JSON value.
    ///
    /// # Errors
    ///
    /// Returns an API error when JSON serialization fails.
    pub fn json_value<T: Serialize>(value: &T) -> Result<Value, ApiError> {
        serde_json::to_value(value).map_err(|error| serialization_error("JSON", error))
    }

    pub(crate) fn json<T: Serialize>(value: &T) -> Result<JsonEncoding, ApiError> {
        let value = Self::json_value(value)?;
        let text =
            serde_json::to_string(&value).map_err(|error| serialization_error("JSON", error))?;
        Ok(JsonEncoding { value, text })
    }

    /// Serializes `value` in TOON format.
    ///
    /// # Errors
    ///
    /// Returns an API error when TOON serialization fails.
    pub fn toon<T: Serialize>(value: &T) -> Result<String, ApiError> {
        toon_format::encode_default(value).map_err(|error| serialization_error("TOON", error))
    }
}

fn serialization_error(encoding: &str, error: impl Display) -> ApiError {
    error::operation_failed(&format!("{encoding} serialization failed: {error}"))
}
