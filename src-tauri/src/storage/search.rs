//! Ranked search wrappers over the `meme-search` crate for storage memes.

use super::Meme;
use meme_search::{collect_tags as collect_tags_inner, rank_items, Searchable};
use serde::{Deserialize, Serialize};

pub use meme_search::SearchOptions;

impl Searchable for Meme {
    fn name(&self) -> &str {
        &self.name
    }
    fn tags(&self) -> &[String] {
        &self.tags
    }
    fn extension(&self) -> &str {
        &self.extension
    }
    fn favorite(&self) -> bool {
        self.favorite
    }
    fn use_count(&self) -> u64 {
        self.use_count
    }
    fn last_used_at(&self) -> Option<&str> {
        self.last_used_at.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub meme: Meme,
    pub score: f64,
}

pub fn rank_memes(memes: &[Meme], query: &str, opts: &SearchOptions) -> Vec<SearchHit> {
    rank_items(memes, query, opts)
        .into_iter()
        .map(|h| SearchHit {
            meme: h.item,
            score: h.score,
        })
        .collect()
}

pub fn collect_tags(memes: &[Meme]) -> Vec<String> {
    collect_tags_inner(memes)
}
