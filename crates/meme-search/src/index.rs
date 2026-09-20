//! In-memory inverted index for candidate retrieval before scoring.

use std::collections::{HashMap, HashSet};

use strsim::{jaro_winkler, normalized_levenshtein};

use crate::normalize::{fold_case, words_of};
use crate::types::{ParsedQuery, Searchable};

const FUZZY_THRESHOLD: f64 = 0.72;
const JARO_THRESHOLD: f64 = 0.88;

/// Token → document-id postings built from name / aliases / tags.
#[derive(Debug, Clone, Default)]
pub struct InvertedIndex {
    postings: HashMap<String, Vec<String>>,
    tag_postings: HashMap<String, Vec<String>>,
    ext_postings: HashMap<String, Vec<String>>,
    favorite_ids: HashSet<String>,
    doc_ids: Vec<String>,
}

impl InvertedIndex {
    pub fn build<T: Searchable>(items: &[T]) -> Self {
        let mut index = Self::default();
        for item in items {
            index.insert(item);
        }
        index
    }

    pub fn len(&self) -> usize {
        self.doc_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.doc_ids.is_empty()
    }

    pub fn doc_ids(&self) -> &[String] {
        &self.doc_ids
    }

    fn insert<T: Searchable>(&mut self, item: &T) {
        let id = item.id().to_string();
        if self.doc_ids.iter().any(|d| d == &id) {
            self.remove(&id);
        }
        self.doc_ids.push(id.clone());

        if item.favorite() {
            self.favorite_ids.insert(id.clone());
        }

        let ext = fold_case(item.extension());
        if !ext.is_empty() {
            self.ext_postings.entry(ext).or_default().push(id.clone());
        }

        for tag in item.tags() {
            let folded = fold_case(tag);
            self.tag_postings
                .entry(folded.clone())
                .or_default()
                .push(id.clone());
            self.add_terms(&id, &folded);
            for w in words_of(tag) {
                self.add_term(&id, &w);
            }
        }

        self.add_terms(&id, &fold_case(item.name()));
        for w in words_of(item.name()) {
            self.add_term(&id, &w);
        }

        for alias in item.aliases() {
            let folded = fold_case(alias);
            self.add_terms(&id, &folded);
            for w in words_of(alias) {
                self.add_term(&id, &w);
            }
        }
    }

    /// Drop one document (for incremental updates). Rebuild is preferred after batch edits.
    pub fn remove(&mut self, id: &str) {
        self.doc_ids.retain(|d| d != id);
        self.favorite_ids.remove(id);
        for ids in self.postings.values_mut() {
            ids.retain(|d| d != id);
        }
        self.postings.retain(|_, ids| !ids.is_empty());
        for ids in self.tag_postings.values_mut() {
            ids.retain(|d| d != id);
        }
        self.tag_postings.retain(|_, ids| !ids.is_empty());
        for ids in self.ext_postings.values_mut() {
            ids.retain(|d| d != id);
        }
        self.ext_postings.retain(|_, ids| !ids.is_empty());
    }

    fn add_terms(&mut self, id: &str, text: &str) {
        if text.is_empty() {
            return;
        }
        self.add_term(id, text);
    }

    fn add_term(&mut self, id: &str, term: &str) {
        if term.is_empty() {
            return;
        }
        let entry = self.postings.entry(term.to_string()).or_default();
        if !entry.iter().any(|d| d == id) {
            entry.push(id.to_string());
        }
    }

    /// Candidate document ids for a parsed query.
    ///
    /// Returns `None` when the caller should score the full library (browse mode).
    /// Returns `Some(empty)` when the index proves no document can match.
    pub fn candidates(&self, parsed: &ParsedQuery) -> Option<HashSet<String>> {
        let mut set = if parsed.has_text_tokens() {
            let mut group_sets: Vec<HashSet<String>> = Vec::new();
            for group in &parsed.must_groups {
                if group.is_empty() {
                    continue;
                }
                let mut union = HashSet::new();
                for alt in group {
                    union.extend(self.docs_for_token(alt));
                }
                if union.is_empty() {
                    return Some(HashSet::new());
                }
                group_sets.push(union);
            }
            if group_sets.is_empty() {
                self.doc_ids.iter().cloned().collect()
            } else {
                let mut iter = group_sets.into_iter();
                let mut acc = iter.next().unwrap();
                for next in iter {
                    acc = acc.intersection(&next).cloned().collect();
                    if acc.is_empty() {
                        return Some(HashSet::new());
                    }
                }
                acc
            }
        } else {
            // Browse / filter-only: start from full library.
            return None;
        };

        if parsed.favorites_only {
            set.retain(|id| self.favorite_ids.contains(id));
        }

        if !parsed.extensions.is_empty() {
            let mut allowed = HashSet::new();
            for ext in &parsed.extensions {
                if let Some(ids) = self.ext_postings.get(ext) {
                    allowed.extend(ids.iter().cloned());
                }
            }
            set.retain(|id| allowed.contains(id));
        }

        for needed in &parsed.tag_filters {
            set.retain(|id| self.tag_matches(id, needed));
            if set.is_empty() {
                return Some(set);
            }
        }

        for banned in &parsed.excluded_tags {
            set.retain(|id| !self.tag_matches(id, banned));
        }

        Some(set)
    }

    fn tag_matches(&self, id: &str, needle: &str) -> bool {
        for (tag, ids) in &self.tag_postings {
            if tag.contains(needle) && ids.iter().any(|d| d == id) {
                return true;
            }
        }
        false
    }

    fn docs_for_token(&self, token: &str) -> HashSet<String> {
        let mut out = HashSet::new();
        if let Some(ids) = self.postings.get(token) {
            out.extend(ids.iter().cloned());
        }

        for (term, ids) in &self.postings {
            if term == token {
                continue;
            }
            if term.starts_with(token)
                || token.starts_with(term.as_str())
                || term.contains(token)
            {
                out.extend(ids.iter().cloned());
            }
        }

        if out.is_empty() {
            for (term, ids) in &self.postings {
                let sim = jaro_winkler(term, token).max(normalized_levenshtein(term, token));
                if sim >= JARO_THRESHOLD || sim >= FUZZY_THRESHOLD {
                    out.extend(ids.iter().cloned());
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_query;

    #[derive(Clone)]
    struct Doc {
        id: String,
        name: String,
        tags: Vec<String>,
    }

    impl Searchable for Doc {
        fn id(&self) -> &str {
            &self.id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn tags(&self) -> &[String] {
            &self.tags
        }
        fn extension(&self) -> &str {
            "gif"
        }
        fn favorite(&self) -> bool {
            false
        }
        fn use_count(&self) -> u64 {
            0
        }
        fn last_used_at(&self) -> Option<&str> {
            None
        }
    }

    #[test]
    fn index_finds_by_name_word() {
        let items = vec![
            Doc {
                id: "1".into(),
                name: "Funny Cat".into(),
                tags: vec![],
            },
            Doc {
                id: "2".into(),
                name: "Doggo".into(),
                tags: vec![],
            },
        ];
        let index = InvertedIndex::build(&items);
        let c = index.candidates(&parse_query("cat")).unwrap();
        assert!(c.contains("1"));
        assert!(!c.contains("2"));
    }

    #[test]
    fn and_intersects_groups() {
        let items = vec![
            Doc {
                id: "1".into(),
                name: "Cat Meme".into(),
                tags: vec![],
            },
            Doc {
                id: "2".into(),
                name: "Cat Photo".into(),
                tags: vec![],
            },
        ];
        let index = InvertedIndex::build(&items);
        let c = index.candidates(&parse_query("cat meme")).unwrap();
        assert_eq!(c.len(), 1);
        assert!(c.contains("1"));
    }
}
