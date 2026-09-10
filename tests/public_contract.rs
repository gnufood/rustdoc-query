#![allow(missing_docs)]

use std::future::Future;

use rustdoc_query::{
    contract::generated::{
        error::ConversionError, ApiError, AssociatedConst, AssociatedType, CacheMetadata,
        CacheMetadataStatus, CacheSource, CrateName, CrateNotFoundError, CrateNotFoundErrorCode,
        DependencyVersion, DocumentationQuery, ErrorOutcome, ErrorOutcomeStatus, Field,
        FindOutcome, FindRequest, FindRequestQuery, FindResult, FindRow, FindSuccess,
        FindSuccessStatus, GetItemOutcome, GetItemRequest, GetItemSuccess, GetItemSuccessStatus,
        IncludeSummary, InvalidRequestError, InvalidRequestErrorCode, ItemDetail,
        ItemNotFoundError, ItemNotFoundErrorCode, KindCount, Method, ModuleNotFoundError,
        ModuleNotFoundErrorCode, OperationFailedError, OperationFailedErrorCode, OutputFormat,
        Overview, OverviewItem, OverviewOutcome, OverviewRequest, OverviewSuccess,
        OverviewSuccessStatus, PageCursor, PageSize, PageSizeError, PublicItemPath,
        PublicModulePath, RequestedVersion, ResponseMetadata, RustdocItemKind,
        RustdocQueryPublicApiV2, SourceUnavailableError, SourceUnavailableErrorCode, Totals,
        UnsupportedRustdocFormatError, UnsupportedRustdocFormatErrorCode, Variant, VariantKind,
        VersionComparison, VersionComparisonStatus,
    },
    mcp::RustdocMcpServer,
    service::RustdocQueryService,
};

fn public_type<T: 'static>() {}

fn accepts_overview(_: impl Future<Output = OverviewOutcome>) {}

fn accepts_find(_: impl Future<Output = FindOutcome>) {}

fn accepts_get_item(_: impl Future<Output = GetItemOutcome>) {}

fn type_only<T>() -> T {
    unreachable!()
}

fn service_entry_points(service: &RustdocQueryService) {
    accepts_overview(service.overview(type_only()));
    accepts_find(service.find(type_only()));
    accepts_get_item(service.get_item(type_only()));
}

#[test]
fn approved_v1_public_contract_surface_compiles() {
    let _: fn() -> Result<RustdocQueryService, ApiError> = RustdocQueryService::new;
    let _: fn() -> Result<RustdocMcpServer, ApiError> = RustdocMcpServer::new;
    let _: fn(OutputFormat) -> Result<RustdocMcpServer, ApiError> =
        RustdocMcpServer::with_output_format;
    let _ = service_entry_points;

    public_type::<ConversionError>();
    public_type::<ApiError>();
    public_type::<AssociatedConst>();
    public_type::<AssociatedType>();
    public_type::<CacheMetadata>();
    public_type::<CacheMetadataStatus>();
    public_type::<CacheSource>();
    public_type::<CrateName>();
    public_type::<CrateNotFoundError>();
    public_type::<CrateNotFoundErrorCode>();
    public_type::<DependencyVersion>();
    public_type::<DocumentationQuery>();
    public_type::<ErrorOutcome>();
    public_type::<ErrorOutcomeStatus>();
    public_type::<Field>();
    public_type::<FindOutcome>();
    public_type::<FindRequest>();
    public_type::<FindRequestQuery>();
    public_type::<FindResult>();
    public_type::<FindRow>();
    public_type::<FindSuccess>();
    public_type::<FindSuccessStatus>();
    public_type::<GetItemOutcome>();
    public_type::<GetItemRequest>();
    public_type::<GetItemSuccess>();
    public_type::<GetItemSuccessStatus>();
    public_type::<IncludeSummary>();
    public_type::<InvalidRequestError>();
    public_type::<InvalidRequestErrorCode>();
    public_type::<ItemDetail>();
    public_type::<ItemNotFoundError>();
    public_type::<ItemNotFoundErrorCode>();
    public_type::<KindCount>();
    public_type::<Method>();
    public_type::<ModuleNotFoundError>();
    public_type::<ModuleNotFoundErrorCode>();
    public_type::<OperationFailedError>();
    public_type::<OperationFailedErrorCode>();
    public_type::<OutputFormat>();
    public_type::<Overview>();
    public_type::<OverviewItem>();
    public_type::<OverviewOutcome>();
    public_type::<OverviewRequest>();
    public_type::<OverviewSuccess>();
    public_type::<OverviewSuccessStatus>();
    public_type::<PageCursor>();
    public_type::<PageSize>();
    public_type::<PageSizeError>();
    public_type::<PublicItemPath>();
    public_type::<PublicModulePath>();
    public_type::<RequestedVersion>();
    public_type::<ResponseMetadata>();
    public_type::<RustdocItemKind>();
    public_type::<RustdocQueryPublicApiV2>();
    public_type::<SourceUnavailableError>();
    public_type::<SourceUnavailableErrorCode>();
    public_type::<Totals>();
    public_type::<UnsupportedRustdocFormatError>();
    public_type::<UnsupportedRustdocFormatErrorCode>();
    public_type::<Variant>();
    public_type::<VariantKind>();
    public_type::<VersionComparison>();
    public_type::<VersionComparisonStatus>();
}
