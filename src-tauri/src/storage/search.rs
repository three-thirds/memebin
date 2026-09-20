//! Ranked search wrappers over the `meme-search` crate for storage memes.

use super::bindings::{load_bindings, normalize_trigger};
use super::Meme;
use meme_search::{
    collect_tags as collect_tags_inner, rank_items_with, suggest_tags as suggest_tags_inner,
    InvertedIndex, MatchSpan, SearchMetrics, Searchable, SynonymMap,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

pub use meme_search::{SearchOptions, SearchSort};

/// Process-wide inverted index; cleared on library mutations.
#[derive(Default)]
pub struct SearchIndexCache {
    inner: Mutex<Option<InvertedIndex>>,
}

impl SearchIndexCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn invalidate(&self) {
        if let Ok(mut guard) = self.inner.lock() {
            *guard = None;
        }
    }

    pub fn get_or_build(&self, memes: &[Meme]) -> InvertedIndex {
        if let Ok(guard) = self.inner.lock() {
            if let Some(index) = guard.as_ref() {
                return index.clone();
            }
        }
        let built = InvertedIndex::build(memes);
        if let Ok(mut guard) = self.inner.lock() {
            *guard = Some(built.clone());
        }
        built
    }
}

impl Searchable for Meme {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn tags(&self) -> &[String] {
        &self.tags
    }
    fn aliases(&self) -> &[String] {
        &self.aliases
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
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(default)]
    pub highlights: Vec<MatchSpan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRankedResult {
    pub hits: Vec<SearchHit>,
    pub metrics: SearchMetrics,
}

/// Load optional `search_synonyms.json` from app data (missing file → empty map).
pub fn load_synonyms(path: &Path) -> SynonymMap {
    if !path.exists() {
        return SynonymMap::new();
    }
    match fs::read_to_string(path) {
        Ok(raw) if raw.trim().is_empty() => SynonymMap::new(),
        Ok(raw) => SynonymMap::from_json_str(&raw).unwrap_or_else(|_| SynonymMap::new()),
        Err(_) => SynonymMap::new(),
    }
}

/// Merge binding triggers into each meme's aliases for scoring (does not write disk).
pub fn enrich_aliases_from_bindings(memes: &mut [Meme], bindings_path: &Path) {
    let Ok(bindings) = load_bindings(bindings_path) else {
        return;
    };
    let mut by_meme: HashMap<String, Vec<String>> = HashMap::new();
    for b in bindings {
        by_meme
            .entry(b.meme_id)
            .or_default()
            .push(normalize_trigger(&b.trigger));
    }
    for meme in memes.iter_mut() {
        if let Some(triggers) = by_meme.get(&meme.id) {
            for t in triggers {
                if t.is_empty() {
                    continue;
                }
                if !meme.aliases.iter().any(|a| a.eq_ignore_ascii_case(t)) {
                    meme.aliases.push(t.clone());
                }
            }
        }
    }
}

pub fn rank_memes(
    memes: &[Meme],
    query: &str,
    opts: &SearchOptions,
    synonyms: Option<&SynonymMap>,
    index: Option<&InvertedIndex>,
) -> SearchRankedResult {
    let ranked = rank_items_with(memes, query, opts, synonyms, index);
    SearchRankedResult {
        hits: ranked
            .hits
            .into_iter()
            .map(|h| SearchHit {
                meme: h.item,
                score: h.score,
                reasons: h.reasons,
                highlights: h.highlights,
            })
            .collect(),
        metrics: ranked.metrics,
    }
}

pub fn collect_tags(memes: &[Meme]) -> Vec<String> {
    collect_tags_inner(memes)
}

pub fn suggest_tags_for(memes: &[Meme], prefix: &str, limit: usize) -> Vec<String> {
    suggest_tags_inner(memes, prefix, limit)
}
