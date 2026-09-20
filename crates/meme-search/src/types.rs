//! Public types for the meme search crate.

use serde::{Deserialize, Serialize};

/// Fields required to rank/filter a meme without depending on Tauri.
pub trait Searchable: Clone {
    /// Stable document id used by the inverted index.
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn tags(&self) -> &[String];
    fn extension(&self) -> &str;
    fn favorite(&self) -> bool;
    fn use_count(&self) -> u64;
    fn last_used_at(&self) -> Option<&str>;

    /// Optional short names / triggers included in scoring.
    fn aliases(&self) -> &[String] {
        &[]
    }
}

/// Timing / fan-out stats for one ranked search.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchMetrics {
    /// Total items in the library slice passed to rank.
    pub library_size: usize,
    /// Items selected by the inverted index (or full library on browse / fallback).
    pub candidates: usize,
    /// Items actually scored.
    pub scanned: usize,
    /// Hits that passed filters/min_score before pagination.
    pub matched: usize,
    /// Hits returned after offset/limit.
    pub returned: usize,
    /// Wall time in milliseconds.
    pub ms: u64,
    /// True when candidate retrieval used the inverted index (text query).
    pub used_index: bool,
}

/// Ranked hits plus metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedSearch<T> {
    pub hits: Vec<SearchHit<T>>,
    pub metrics: SearchMetrics,
}

/// UTF-8 character range on a matched field (for UI underline).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MatchSpan {
    /// `name`, `alias`, or `tag:<value>`
    pub field: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit<T> {
    pub item: T,
    pub score: f64,
    /// Why this hit matched, e.g. `name:exact`, `tag:prefix`, `filter:fav`.
    #[serde(default)]
    pub reasons: Vec<String>,
    /// Character spans for UI highlighting.
    #[serde(default)]
    pub highlights: Vec<MatchSpan>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchSort {
    #[default]
    Relevance,
    Recent,
    Name,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    /// Drop hits below this score (after ranking). Ignored for empty-token browse.
    pub min_score: Option<f64>,
    /// Post-filter ordering. Default: relevance.
    pub sort: Option<SearchSort>,
}

/// AND of OR-groups: every group must match; within a group any alternative may match.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedQuery {
    pub must_groups: Vec<Vec<String>>,
    pub excluded_tokens: Vec<String>,
    pub tag_filters: Vec<String>,
    pub excluded_tags: Vec<String>,
    pub extensions: Vec<String>,
    pub favorites_only: bool,
}

impl ParsedQuery {
    pub fn has_text_tokens(&self) -> bool {
        self.must_groups.iter().any(|g| !g.is_empty())
    }

    pub(crate) fn push_text(&mut self, value: String, or_with_previous: bool) {
        if or_with_previous {
            if let Some(last) = self.must_groups.last_mut() {
                last.push(value);
                return;
            }
        }
        self.must_groups.push(vec![value]);
    }

    pub(crate) fn push_excluded_token(&mut self, value: String) {
        self.excluded_tokens.push(value);
    }

    pub(crate) fn push_tag(&mut self, value: String) {
        self.tag_filters.push(value);
    }

    pub(crate) fn push_excluded_tag(&mut self, value: String) {
        self.excluded_tags.push(value);
    }

    pub(crate) fn push_ext(&mut self, value: String) {
        self.extensions
            .push(value.trim_start_matches('.').to_string());
    }

    pub(crate) fn set_favorites_only(&mut self) {
        self.favorites_only = true;
    }
}
