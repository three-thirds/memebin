//! Rank → sort → page, plus tag collection helpers.

use std::collections::HashSet;
use std::time::Instant;

use crate::engine::score_item;
use crate::index::InvertedIndex;
use crate::matcher::suggest_points;
use crate::parse::parse_query;
use crate::synonyms::SynonymMap;
use crate::types::{RankedSearch, SearchHit, SearchMetrics, SearchOptions, SearchSort, Searchable};

const DEFAULT_LIMIT: usize = 50;

#[derive(Clone)]
struct RankKey {
    score: f64,
    use_count: u64,
    last_used: String,
    name_key: String,
}

impl RankKey {
    fn from_hit<T: Searchable>(hit: &SearchHit<T>) -> Self {
        Self {
            score: hit.score,
            use_count: hit.item.use_count(),
            last_used: hit.item.last_used_at().unwrap_or("").to_string(),
            name_key: hit.item.name().to_lowercase(),
        }
    }
}

impl Ord for RankKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .partial_cmp(&self.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| other.use_count.cmp(&self.use_count))
            .then_with(|| other.last_used.cmp(&self.last_used))
            .then_with(|| self.name_key.cmp(&other.name_key))
    }
}

impl PartialOrd for RankKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RankKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for RankKey {}

pub fn rank_items<T: Searchable>(
    items: &[T],
    query: &str,
    opts: &SearchOptions,
) -> Vec<SearchHit<T>> {
    rank_items_with(items, query, opts, None, None).hits
}

pub fn rank_items_with<T: Searchable>(
    items: &[T],
    query: &str,
    opts: &SearchOptions,
    synonyms: Option<&SynonymMap>,
    index: Option<&InvertedIndex>,
) -> RankedSearch<T> {
    let started = Instant::now();

    let mut parsed = parse_query(query);
    if let Some(map) = synonyms {
        parsed = map.expand_parsed(&parsed);
    }

    let owned_index;
    let index = match index {
        Some(i) => i,
        None => {
            owned_index = InvertedIndex::build(items);
            &owned_index
        }
    };

    let limit = opts.limit.unwrap_or(DEFAULT_LIMIT).max(1);
    let offset = opts.offset.unwrap_or(0);
    let min_score = opts.min_score.unwrap_or(0.0);
    let enforce_min = parsed.has_text_tokens();
    let sort = opts.sort.unwrap_or_default();

    let candidate_ids: Option<HashSet<String>> = index.candidates(&parsed);
    let used_index = candidate_ids.is_some();

    let to_score: Vec<&T> = match &candidate_ids {
        None => items.iter().collect(),
        Some(ids) if ids.is_empty() => Vec::new(),
        Some(ids) => items.iter().filter(|item| ids.contains(item.id())).collect(),
    };

    let candidates = to_score.len();
    let scanned = candidates;

    let mut hits: Vec<SearchHit<T>> = to_score
        .into_iter()
        .filter_map(|item| {
            let (score, reasons, highlights) = score_item(item, &parsed)?;
            Some(SearchHit {
                item: item.clone(),
                score,
                reasons,
                highlights,
            })
        })
        .filter(|h| !enforce_min || h.score >= min_score)
        .collect();

    let matched = hits.len();

    match sort {
        SearchSort::Relevance => {
            hits.sort_by(|a, b| RankKey::from_hit(a).cmp(&RankKey::from_hit(b)));
        }
        SearchSort::Recent => {
            hits.sort_by(|a, b| {
                let la = a.item.last_used_at().unwrap_or("");
                let lb = b.item.last_used_at().unwrap_or("");
                lb.cmp(la)
                    .then_with(|| RankKey::from_hit(a).cmp(&RankKey::from_hit(b)))
            });
        }
        SearchSort::Name => {
            hits.sort_by(|a, b| {
                a.item
                    .name()
                    .to_lowercase()
                    .cmp(&b.item.name().to_lowercase())
                    .then_with(|| RankKey::from_hit(a).cmp(&RankKey::from_hit(b)))
            });
        }
    }

    let hits: Vec<_> = hits.into_iter().skip(offset).take(limit).collect();
    let returned = hits.len();
    let ms = started.elapsed().as_millis() as u64;

    RankedSearch {
        hits,
        metrics: SearchMetrics {
            library_size: items.len(),
            candidates,
            scanned,
            matched,
            returned,
            ms,
            used_index,
        },
    }
}

pub fn collect_tags<T: Searchable>(items: &[T]) -> Vec<String> {
    let mut tags: Vec<String> = items
        .iter()
        .flat_map(|m| m.tags().iter().cloned())
        .collect();
    tags.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    tags.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    tags
}

/// Prefix / fuzzy tag suggestions for picker chips.
pub fn suggest_tags<T: Searchable>(items: &[T], prefix: &str, limit: usize) -> Vec<String> {
    let prefix = prefix.trim().to_lowercase();
    let limit = limit.max(1);
    let catalog = collect_tags(items);

    if prefix.is_empty() {
        return catalog.into_iter().take(limit).collect();
    }

    let mut scored: Vec<(f64, String)> = catalog
        .into_iter()
        .filter_map(|tag| {
            let pts = suggest_points(&tag.to_lowercase(), &prefix);
            (pts > 0.0).then_some((pts, tag))
        })
        .collect();

    scored.sort_by(|a, b| {
        b.0.partial_cmp(&a.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.1.to_lowercase().cmp(&b.1.to_lowercase()))
    });

    scored.into_iter().take(limit).map(|(_, t)| t).collect()
}
