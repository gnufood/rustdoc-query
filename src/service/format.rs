//! Resolves request output formats against the generated schema default.

use crate::contract::generated::OutputFormat;

pub(crate) fn resolve(requested: Option<OutputFormat>) -> OutputFormat {
    requested.unwrap_or_default()
}

#[cfg(test)]
#[path = "format/tests.rs"]
mod tests;
