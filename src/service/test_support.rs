//! Deterministic prepared-entry fixtures for service-boundary tests.

use super::RustdocQueryService;
use crate::cache::CrateCache;

mod fixture;

pub(crate) async fn service() -> RustdocQueryService {
    let cache = CrateCache::new().expect("test cache initializes");
    cache
        .insert_for_test("krate", "1.0.0", fixture::entry())
        .await;
    RustdocQueryService { cache }
}
