//! Projects the private overview result onto the generated contract type.

use crate::contract::generated::{
    KindCount as ContractKindCount, Overview as ContractOverview,
    OverviewItem as ContractOverviewItem, Totals as ContractTotals,
};
use crate::query::{KindCount, Overview, OverviewItem, Totals};

/// `resolved_version` is authoritative over the crate's self-reported version.
pub(crate) fn to_contract(overview: Overview, resolved_version: &str) -> ContractOverview {
    ContractOverview {
        totals: map_totals(&overview.totals),
        path: overview.path,
        resolved_version: resolved_version.to_owned(),
        docs: overview.docs,
        docs_truncated: overview.docs_truncated,
        items: map_items(overview.items),
    }
}

fn map_items(items: Vec<OverviewItem>) -> Vec<ContractOverviewItem> {
    items.into_iter().map(map_item).collect()
}

fn map_item(item: OverviewItem) -> ContractOverviewItem {
    ContractOverviewItem {
        name: item.name,
        path: item.path,
        kind: crate::service::kind::to_contract(item.kind),
        summary: item.summary,
        signature: item.signature,
        deprecated: item.deprecated,
        reexport: item.reexport,
    }
}

fn map_totals(totals: &Totals) -> ContractTotals {
    ContractTotals {
        direct: map_counts(&totals.direct),
        subtree: map_counts(&totals.subtree),
    }
}

fn map_counts(counts: &[KindCount]) -> Vec<ContractKindCount> {
    counts
        .iter()
        .map(|count| ContractKindCount {
            kind: crate::service::kind::to_contract(count.kind),
            count: widen(count.count),
        })
        .collect()
}

/// Saturating widen, so a count can never silently wrap into a wrong total.
fn widen(count: usize) -> u64 {
    u64::try_from(count).unwrap_or(u64::MAX)
}

#[cfg(test)]
#[path = "overview/tests.rs"]
mod tests;
