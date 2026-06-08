use rustdoc_types::Crate;

use crate::rustdoc::Surface;
use crate::search::FtsDb;

use super::CacheLoadReport;

/// Always shared as `Arc<CrateEntry>` out of the cache.
#[derive(Debug)]
pub(crate) struct CrateEntry {
    pub(crate) krate: Crate,
    /// The public-surface index built over [`Self::krate`].
    pub(crate) surface: Surface,
    /// Dep crates fetched during cross-crate expansion, in the order they were
    /// stitched. `SurfaceEntry::dep_idx` is a 1-based index into this vec;
    /// `dep_idx == 0` means the item lives in `krate`.
    pub(crate) dep_crates: Vec<Crate>,
    /// `SQLite` FTS5-backed.
    pub(crate) fts: FtsDb,
    pub(crate) load_report: CacheLoadReport,
}

impl CrateEntry {
    pub(crate) fn new(krate: Crate, surface: Surface, dep_crates: Vec<Crate>, fts: FtsDb) -> Self {
        Self {
            krate,
            surface,
            dep_crates,
            fts,
            load_report: CacheLoadReport::memory(),
        }
    }

    /// Routes dep-expanded items to their source crate's JSON so ID lookups
    /// don't collide with the main crate's integer namespace.
    pub(crate) fn krate_for(&self, se: &crate::rustdoc::SurfaceEntry) -> &Crate {
        if se.dep_idx == 0 {
            &self.krate
        } else {
            self.dep_crates
                .get(usize::from(se.dep_idx - 1))
                .unwrap_or(&self.krate)
        }
    }
}
