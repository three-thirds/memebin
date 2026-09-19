//! Public types for the meme search crate.

use serde::{Deserialize, Serialize};

/// Fields required to rank/filter a meme without depending on Tauri.
pub trait Searchable: Clone {
    fn name(&self) -> &str;
    fn tags(&self) -> &[String];
    fn extension(&self) -> &str;
    fn favorite(&self) -> bool;
    fn use_count(&self) -> u64;
    fn last_used_at(&self) -> Option<&str>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit<T> {
    pub item: T,
    pub score: f64,
    /// Why this hit matched, e.g. `name:exact`, `tag:prefix`, `filter:fav`.
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    /// Drop hits below this score (after ranking). Ignored for empty-token browse.
    pub min_score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedQuery {
    pub tokens: Vec<String>,
    pub excluded_tokens: Vec<String>,
    pub tag_filters: Vec<String>,
    pub excluded_tags: Vec<String>,
    pub extensions: Vec<String>,
    pub favorites_only: bool,
}

impl ParsedQuery {
    pub fn has_text_tokens(&self) -> bool {
        !self.tokens.is_empty()
    }

    pub(crate) fn push_token(&mut self, value: String) {
        self.tokens.push(value);
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
