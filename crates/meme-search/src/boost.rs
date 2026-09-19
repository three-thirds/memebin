//! Soft score additives (usage, recency, favorites, name coverage).

use crate::normalize::fold_case;
use crate::types::{ParsedQuery, Searchable};

pub(crate) fn popularity(use_count: u64) -> f64 {
    ((use_count + 1) as f64).ln() * 2.5
}

pub(crate) fn recency(last_used_at: Option<&str>) -> f64 {
    match last_used_at {
        Some(ts) if !ts.is_empty() => {
            let freshness = ts.len().min(20) as f64 * 0.05;
            3.0 + freshness
        }
        _ => 0.0,
    }
}

/// Browse-mode score when the query has no text tokens.
pub(crate) fn browse_total<T: Searchable>(item: &T, reasons: &mut Vec<String>) -> f64 {
    let mut total = popularity(item.use_count()) + recency(item.last_used_at());
    if item.favorite() {
        total += 12.0;
        reasons.push("boost:favorite".into());
    }
    reasons.push("browse".into());
    total
}

/// Extra points after all text tokens already matched.
pub(crate) fn query_soft_boosts<T: Searchable>(
    item: &T,
    parsed: &ParsedQuery,
    reasons: &mut Vec<String>,
) -> f64 {
    let mut extra = 0.0_f64;

    if item.favorite() {
        extra += 8.0;
        reasons.push("boost:favorite".into());
    }
    extra += popularity(item.use_count()) * 0.15;
    extra += recency(item.last_used_at()) * 0.1;

    let name = fold_case(item.name());
    if parsed.tokens.len() > 1 && parsed.tokens.iter().all(|t| name.contains(t)) {
        extra += 15.0;
        reasons.push("boost:name-coverage".into());
    }

    extra
}

pub(crate) fn filter_reason_labels(parsed: &ParsedQuery) -> Vec<String> {
    let mut reasons = Vec::new();
    if parsed.favorites_only {
        reasons.push("filter:fav".into());
    }
    for t in &parsed.tag_filters {
        reasons.push(format!("filter:tag:{t}"));
    }
    for e in &parsed.extensions {
        reasons.push(format!("filter:ext:{e}"));
    }
    reasons
}
