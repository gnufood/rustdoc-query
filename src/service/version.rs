//! Resolves the requested crate version against the schema default.

use crate::contract::generated::RequestedVersion;

/// Schema defaults are annotations, not automatically applied.
pub(crate) fn requested(version: Option<&RequestedVersion>) -> &str {
    version.map_or(crate::contract::defaults::REQUESTED_VERSION, |version| {
        version.as_str()
    })
}

#[cfg(test)]
#[path = "version/tests.rs"]
mod tests;
