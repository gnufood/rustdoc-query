//! Validates find requests and projects private search rows onto the contract.

use rustdoc_types::ItemKind;

use crate::cache::CrateEntry;
use crate::contract::generated::{
    ApiError, FindRequest, FindResult as ContractFindResult, FindRow as ContractFindRow,
    InvalidRequestError, InvalidRequestErrorCode,
};
use crate::query::{find_document_items, find_items, FindRow, SortKey};

use super::error;

mod cursor;

/// Returned when a documentation search matches nothing.
pub(crate) const DOC_SEARCH_HINT: &str =
    "No full-text matches — try different keywords, or use `query` for name-based search.";

pub(crate) async fn page(
    entry: &CrateEntry,
    request: &FindRequest,
    resolved_version: &str,
) -> Result<Page, ApiError> {
    let kind = request.kind.map(crate::service::kind::from_contract);
    let module = request.module.as_ref().map(|module| module.as_str());
    let with_summary = summaries(request);
    let page_size = page_size(request);
    let context = cursor::context(request, resolved_version);
    let position = cursor::decode(request, resolved_version)?;

    let Some(doc_query) = request.doc_query.as_ref() else {
        let found = find_items(
            entry,
            request.query.as_ref().map(|query| query.as_str()),
            kind,
            module,
            with_summary,
        );
        return memory_page(
            found.rows,
            found.order,
            found.hint,
            position,
            page_size,
            context,
        );
    };

    let public_module_scope = module.map(|path| {
        entry
            .surface
            .get_by_path(path)
            .filter(|entry| entry.kind == ItemKind::Module)
            .map(|entry| entry.path.join("::"))
    });
    if matches!(public_module_scope, Some(None)) {
        return Ok(Page {
            rows: Vec::new(),
            hint: Some(DOC_SEARCH_HINT.to_owned()),
            next: None,
            context,
        });
    }

    let offset = match position {
        None => None,
        Some(cursor::Position::Documentation { offset }) => Some(offset),
        Some(cursor::Position::Memory { .. }) => return Err(cursor::type_mismatch()),
    };
    let found = find_document_items(
        entry,
        &cursor::normalize(doc_query),
        request.kind.as_ref().map(ToString::to_string).as_deref(),
        public_module_scope
            .as_ref()
            .and_then(|scope| scope.as_deref()),
        offset,
        page_size,
        with_summary,
    )
    .await
    .map_err(|failure| error::to_api_error(&failure))?;
    let hint = found.rows.is_empty().then(|| DOC_SEARCH_HINT.to_owned());

    let next = if found.has_more {
        let Some(offset) = found.next_offset else {
            return Err(cursor::type_mismatch());
        };
        Some(cursor::Position::Documentation { offset })
    } else {
        None
    };
    Ok(Page {
        rows: found.rows,
        hint,
        next,
        context,
    })
}

pub(crate) fn validate(request: &FindRequest) -> Result<(), ApiError> {
    if request.query.is_some() && request.doc_query.is_some() {
        return Err(ApiError::InvalidRequestError(InvalidRequestError {
            code: InvalidRequestErrorCode::InvalidRequest,
            message: "`query` and `doc_query` are mutually exclusive; supply at most one"
                .to_owned(),
            field: None,
        }));
    }
    Ok(())
}

fn page_size(request: &FindRequest) -> usize {
    let value = request
        .page_size
        .as_ref()
        .map_or(u64::from(crate::contract::defaults::PAGE_SIZE), |size| {
            u64::from(size.get())
        });
    match usize::try_from(value) {
        Ok(value) => value,
        Err(_) => unreachable!("the schema-bounded page size must fit usize"),
    }
}

/// Applies the schema default when `with_summary` is omitted.
pub(crate) fn summaries(request: &FindRequest) -> bool {
    request
        .with_summary
        .as_ref()
        .map_or(crate::contract::defaults::INCLUDE_SUMMARY, |with_summary| {
            **with_summary
        })
}

#[derive(Debug)]
pub(crate) struct Page {
    rows: Vec<FindRow>,
    hint: Option<String>,
    next: Option<cursor::Position>,
    context: cursor::Context,
}

fn memory_page(
    rows: Vec<FindRow>,
    order: Vec<SortKey>,
    hint: Option<String>,
    cursor_position: Option<cursor::Position>,
    page_size: usize,
    context: cursor::Context,
) -> Result<Page, ApiError> {
    let start_after = match cursor_position {
        None => None,
        Some(cursor::Position::Memory { order }) => Some(order),
        Some(cursor::Position::Documentation { .. }) => return Err(cursor::type_mismatch()),
    };
    let mut selected: Vec<_> = rows
        .into_iter()
        .zip(order)
        .filter(|(_, key)| {
            start_after
                .as_ref()
                .is_none_or(|position| key_after(key, position))
        })
        .take(page_size.saturating_add(1))
        .collect();
    let has_more = selected.len() > page_size;
    if has_more {
        selected.pop();
    }
    let next = if has_more {
        selected.last().map(|(_, key)| cursor::Position::Memory {
            order: key_to_cursor(key),
        })
    } else {
        None
    };
    Ok(Page {
        rows: selected.into_iter().map(|(row, _)| row).collect(),
        hint,
        next,
        context,
    })
}

fn key_after(key: &SortKey, cursor: &cursor::MemoryOrder) -> bool {
    match (key, cursor) {
        (SortKey::Alphabetical { path }, cursor::MemoryOrder::Alphabetical { path: after }) => {
            path > after
        }
        (
            SortKey::Fuzzy { score, path },
            cursor::MemoryOrder::Fuzzy {
                score: after_score,
                path: after_path,
            },
        ) => score < after_score || (score == after_score && path > after_path),
        _ => false,
    }
}

fn key_to_cursor(key: &SortKey) -> cursor::MemoryOrder {
    match key {
        SortKey::Alphabetical { path } => cursor::MemoryOrder::Alphabetical { path: path.clone() },
        SortKey::Fuzzy { score, path } => cursor::MemoryOrder::Fuzzy {
            score: *score,
            path: path.clone(),
        },
    }
}

pub(crate) fn to_contract(
    page: Page,
    resolved_version: &str,
) -> Result<ContractFindResult, ApiError> {
    let rows: Vec<ContractFindRow> = page.rows.into_iter().map(to_row).collect();
    match page.next {
        None => Ok(ContractFindResult::CompletePage {
            has_more: false,
            rows,
            resolved_version: resolved_version.to_owned(),
            hint: page.hint,
        }),
        Some(position) => Ok(ContractFindResult::ContinuationPage {
            has_more: true,
            rows,
            next_cursor: crate::contract::generated::PageCursor::try_from(cursor::encode(
                page.context,
                position,
            )?)
            .map_err(|_| cursor::type_mismatch())?,
            resolved_version: resolved_version.to_owned(),
            hint: page.hint,
        }),
    }
}

fn to_row(row: FindRow) -> ContractFindRow {
    ContractFindRow {
        path: row.path,
        kind: crate::service::kind::to_contract(row.kind),
        summary: row.summary,
    }
}

#[cfg(test)]
#[path = "find/tests.rs"]
mod tests;
