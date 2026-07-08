use rustdoc_types::{Crate, ItemEnum};
use sha2::{Digest, Sha256};

use crate::rustdoc::label as kind_label;
use crate::rustdoc::{render_fn_sig, Surface, SurfaceResolution};

#[derive(Debug)]
pub(super) struct IndexRow {
    /// Fully qualified path, retrieved but not tokenized.
    pub(super) path: String,
    pub(super) kind: &'static str,
    /// Final path segment — the item's own identity, tokenized for search.
    pub(super) name: String,
    pub(super) inherent_method: bool,
    pub(super) docs: String,
    pub(super) sig: String,
}

/// Entries carry the index of the crate that owns them, so documentation and
/// signatures are read from the defining crate rather than the root crate.
pub(super) fn collect_rows(
    krate: &Crate,
    surface: &Surface,
    dep_crates: &[Crate],
) -> Vec<IndexRow> {
    surface
        .iter()
        .map(|se| {
            let owner = if se.dep_idx == 0 {
                krate
            } else {
                dep_crates.get(usize::from(se.dep_idx) - 1).unwrap_or(krate)
            };
            let name = se.path.last().map_or("", String::as_str);
            let docs = owner
                .index
                .get(&se.id)
                .and_then(|i| i.docs.clone())
                .unwrap_or_default();
            let sig = owner
                .index
                .get(&se.id)
                .and_then(|item| match &item.inner {
                    ItemEnum::Function(f) => Some(render_fn_sig(owner, name, f)),
                    _ => None,
                })
                .unwrap_or_default();
            IndexRow {
                path: se.path.join("::"),
                kind: kind_label(se.kind),
                name: name.to_owned(),
                inherent_method: matches!(se.resolution, SurfaceResolution::InherentMethod { .. }),
                docs,
                sig,
            }
        })
        .collect()
}

/// Content hash of everything indexed, used to invalidate a stale sidecar.
///
/// Field lengths are hashed alongside their bytes so that moving text between
/// adjacent fields cannot produce a collision.
pub(super) fn fingerprint(rows: &[IndexRow]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"rustdoc-query-fts\0");
    for row in rows {
        for value in [
            row.path.as_bytes(),
            row.kind.as_bytes(),
            row.name.as_bytes(),
            &[u8::from(row.inherent_method)],
            row.docs.as_bytes(),
            row.sig.as_bytes(),
        ] {
            digest.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_le_bytes());
            digest.update(value);
        }
    }
    format!("{:x}", digest.finalize())
}
