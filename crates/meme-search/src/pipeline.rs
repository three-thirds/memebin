//! Rank → sort → page, plus tag collection helpers.

use crate::engine::score_item;
use crate::matcher::suggest_points;
use crate::parse::parse_query;
use crate::types::{SearchHit, SearchOptions, Searchable};

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
    let parsed = parse_query(query);
    let limit = opts.limit.unwrap_or(DEFAULT_LIMIT).max(1);
    let offset = opts.offset.unwrap_or(0);
    let min_score = opts.min_score.unwrap_or(0.0);
    let enforce_min = parsed.has_text_tokens();

    let mut hits: Vec<SearchHit<T>> = items
        .iter()
        .filter_map(|item| {
            let (score, reasons) = score_item(item, &parsed)?;
            Some(SearchHit {
                item: item.clone(),
                score,
                reasons,
            })
        })
        .filter(|h| !enforce_min || h.score >= min_score)
        .collect();

    hits.sort_by(|a, b| RankKey::from_hit(a).cmp(&RankKey::from_hit(b)));
    hits.into_iter().skip(offset).take(limit).collect()
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
