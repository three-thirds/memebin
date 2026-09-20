//! Ranked multi-token meme search (no Tauri dependency).
//!
//! Public surface is stable for storage wrappers; internals live in submodules.

mod boost;
mod engine;
mod index;
mod matcher;
mod normalize;
mod parse;
mod pipeline;
mod synonyms;
mod types;

pub use engine::{score_item, score_token};
pub use index::InvertedIndex;
pub use parse::parse_query;
pub use pipeline::{collect_tags, rank_items, rank_items_with, suggest_tags};
pub use synonyms::SynonymMap;
pub use types::{
    MatchSpan, ParsedQuery, RankedSearch, SearchHit, SearchMetrics, SearchOptions, SearchSort,
    Searchable,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Stub {
        id: String,
        name: String,
        tags: Vec<String>,
        aliases: Vec<String>,
        extension: String,
        favorite: bool,
        use_count: u64,
        last_used_at: Option<String>,
    }

    impl Searchable for Stub {
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

    fn stub(name: &str, tags: &[&str], favorite: bool, use_count: u64) -> Stub {
        Stub {
            id: name.to_lowercase().replace(' ', "-"),
            name: name.into(),
            tags: tags.iter().map(|t| (*t).to_string()).collect(),
            aliases: vec![],
            extension: "gif".into(),
            favorite,
            use_count,
            last_used_at: if use_count > 0 {
                Some("2024-06-01T00:00:00Z".into())
            } else {
                None
            },
        }
    }

    #[test]
    fn parse_tokens_filters_hash_and_negation() {
        let q = parse_query(r#"cat "funny dog" #lol -sad -tag:nsfw ext:GIF is:fav"#);
        assert_eq!(
            q.must_groups,
            vec![
                vec!["cat".to_string()],
                vec!["funny dog".to_string()]
            ]
        );
        assert_eq!(q.tag_filters, vec!["lol"]);
        assert_eq!(q.excluded_tokens, vec!["sad"]);
        assert_eq!(q.excluded_tags, vec!["nsfw"]);
        assert_eq!(q.extensions, vec!["gif"]);
        assert!(q.favorites_only);
    }

    #[test]
    fn and_requires_all_tokens() {
        let m = stub("Funny Cat", &["animals"], false, 0);
        assert!(score_item(&m, &parse_query("cat dog")).is_none());
        assert!(score_item(&m, &parse_query("cat funny")).is_some());
    }

    #[test]
    fn or_group_matches_either() {
        let cat = stub("Funny Cat", &[], false, 0);
        let dog = stub("Funny Dog", &[], false, 0);
        let bird = stub("Funny Bird", &[], false, 0);
        let hits = rank_items(&[cat, dog, bird], "cat OR dog", &SearchOptions::default());
        assert_eq!(hits.len(), 2);
        let names: Vec<_> = hits.iter().map(|h| h.item.name.as_str()).collect();
        assert!(names.contains(&"Funny Cat"));
        assert!(names.contains(&"Funny Dog"));
        assert!(!names.contains(&"Funny Bird"));
    }

    #[test]
    fn or_and_combo() {
        let a = stub("Cat Meme", &[], false, 0);
        let b = stub("Dog Meme", &[], false, 0);
        let c = stub("Cat Photo", &[], false, 0);
        let hits = rank_items(&[a, b, c], "(cat | dog) meme", &SearchOptions::default());
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn alias_matches() {
        let mut m = stub("Distracted Boyfriend", &[], false, 0);
        m.aliases = vec!["db".into()];
        let (score, reasons, highlights) = score_item(&m, &parse_query("db")).unwrap();
        assert!(score > 0.0);
        assert!(reasons.iter().any(|r| r.starts_with("alias:")));
        assert!(highlights.iter().any(|h| h.field == "alias"));
    }

    #[test]
    fn highlights_on_name() {
        let m = stub("Funny Cat", &[], false, 0);
        let (_, _, highlights) = score_item(&m, &parse_query("cat")).unwrap();
        assert!(!highlights.is_empty());
        assert_eq!(highlights[0].field, "name");
        assert!(highlights[0].end > highlights[0].start);
    }

    #[test]
    fn synonyms_expand_query() {
        let map = SynonymMap::from_json_str(r#"{"lol":["lul"]}"#).unwrap();
        let mut m = stub("Reaction", &["lul"], false, 0);
        let ranked =
            rank_items_with(&[m.clone()], "lol", &SearchOptions::default(), Some(&map), None);
        assert_eq!(ranked.hits.len(), 1);
        assert!(ranked.metrics.used_index);

        m.tags = vec!["other".into()];
        let miss = rank_items(&[m], "lol", &SearchOptions::default());
        assert!(miss.is_empty());
    }

    #[test]
    fn index_narrows_candidates() {
        let items: Vec<_> = (0..50)
            .map(|i| stub(&format!("Item{i}"), &["noise"], false, 0))
            .chain(std::iter::once(stub("Target Cat", &[], false, 0)))
            .collect();
        let ranked = rank_items_with(&items, "cat", &SearchOptions::default(), None, None);
        assert_eq!(ranked.hits.len(), 1);
        assert!(ranked.metrics.candidates < ranked.metrics.library_size);
        assert!(ranked.metrics.used_index);
        assert_eq!(ranked.hits[0].item.name, "Target Cat");
    }

    #[test]
    fn browse_skips_index_filter() {
        let items = vec![stub("A", &[], false, 1), stub("B", &[], true, 5)];
        let ranked = rank_items_with(&items, "", &SearchOptions::default(), None, None);
        assert!(!ranked.metrics.used_index);
        assert_eq!(ranked.metrics.scanned, 2);
        assert!(ranked.metrics.ms < 5_000);
    }

    #[test]
    fn sort_by_name() {
        let b = stub("Beta", &["x"], false, 5);
        let a = stub("Alpha", &["x"], false, 10);
        let hits = rank_items(
            &[b, a],
            "x",
            &SearchOptions {
                sort: Some(SearchSort::Name),
                ..Default::default()
            },
        );
        assert_eq!(hits[0].item.name, "Alpha");
        assert_eq!(hits[1].item.name, "Beta");
    }

    #[test]
    fn negation_excludes_matches() {
        let cat = stub("Funny Cat", &["animals"], false, 0);
        let dog = stub("Funny Dog", &["animals"], false, 0);
        let hits = rank_items(&[cat, dog], "funny -dog", &SearchOptions::default());
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].item.name, "Funny Cat");
    }

    #[test]
    fn exact_outranks_fuzzy() {
        let exact = stub("cat", &[], false, 0);
        let fuzzy = stub("cot", &[], false, 0);
        let hits = rank_items(&[fuzzy, exact], "cat", &SearchOptions::default());
        assert_eq!(hits[0].item.name, "cat");
        assert!(hits[0].score > hits.get(1).map(|h| h.score).unwrap_or(0.0));
    }

    #[test]
    fn name_outranks_tag_for_same_token() {
        let by_name = stub("lol", &["other"], false, 0);
        let by_tag = stub("something", &["lol"], false, 0);
        let hits = rank_items(&[by_tag, by_name], "lol", &SearchOptions::default());
        assert_eq!(hits[0].item.name, "lol");
    }

    #[test]
    fn camel_case_word_match() {
        let m = stub("FunnyCat", &[], false, 0);
        let (score, reasons, _) = score_item(&m, &parse_query("cat")).unwrap();
        assert!(score > 0.0);
        assert!(reasons.iter().any(|r| r.starts_with("name:")));
    }

    #[test]
    fn phrase_token_matches_name() {
        let m = stub("Funny Cat Dance", &[], false, 0);
        assert!(score_item(&m, &parse_query(r#""funny cat""#)).is_some());
    }

    #[test]
    fn fav_and_tag_and_ext_filters() {
        let a = stub("A", &["lol"], true, 0);
        let mut b = stub("B", &["lol"], false, 0);
        b.extension = "png".into();
        let c = stub("C", &["sad"], true, 0);

        let hits = rank_items(&[a, b, c], "is:fav #lol ext:gif", &SearchOptions::default());
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].item.name, "A");
    }

    #[test]
    fn empty_query_orders_by_usage() {
        let low = stub("Low", &[], false, 1);
        let high = stub("High", &[], true, 10);
        let hits = rank_items(
            &[low, high],
            "",
            &SearchOptions {
                limit: Some(10),
                ..Default::default()
            },
        );
        assert_eq!(hits[0].item.name, "High");
    }

    #[test]
    fn offset_and_limit() {
        let items: Vec<_> = (0..10)
            .map(|i| stub(&format!("M{i}"), &[], false, i as u64))
            .collect();
        let hits = rank_items(
            &items,
            "",
            &SearchOptions {
                limit: Some(3),
                offset: Some(2),
                min_score: None,
                sort: None,
            },
        );
        assert_eq!(hits.len(), 3);
    }

    #[test]
    fn suggest_tags_prefix() {
        let items = vec![
            stub("A", &["animals", "lol"], false, 0),
            stub("B", &["angry"], false, 0),
        ];
        let tags = suggest_tags(&items, "an", 10);
        assert!(tags.iter().any(|t| t.eq_ignore_ascii_case("animals")));
        assert!(tags.iter().any(|t| t.eq_ignore_ascii_case("angry")));
        assert!(!tags.iter().any(|t| t.eq_ignore_ascii_case("lol")));
    }

    #[test]
    fn collect_tags_unique_sorted() {
        let items = vec![
            stub("A", &["Zebra", "cat"], false, 0),
            stub("B", &["cat", "bird"], false, 0),
        ];
        let tags = collect_tags(&items);
        assert_eq!(
            tags.iter().map(|t| t.to_lowercase()).collect::<Vec<_>>(),
            vec!["bird", "cat", "zebra"]
        );
    }
}
