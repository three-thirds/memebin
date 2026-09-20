//! Local synonym / typo expansion map.

use std::collections::HashMap;

use crate::types::ParsedQuery;

/// Maps a canonical (or typed) token → alternate spellings.
/// Keys and values are matched case-insensitively after fold.
#[derive(Debug, Clone, Default)]
pub struct SynonymMap {
    map: HashMap<String, Vec<String>>,
}

impl SynonymMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build from `{"lol": ["lul", "lmao"], ...}` style JSON object.
    pub fn from_json_str(json: &str) -> Result<Self, String> {
        let raw: HashMap<String, Vec<String>> = serde_json::from_str(json)
            .map_err(|e| format!("invalid synonym map JSON: {e}"))?;
        Ok(Self::from_map(raw))
    }

    pub fn from_map(raw: HashMap<String, Vec<String>>) -> Self {
        let mut map = HashMap::new();
        for (k, values) in raw {
            let key = k.to_lowercase();
            let alts: Vec<String> = values
                .into_iter()
                .map(|v| v.to_lowercase())
                .filter(|v| !v.is_empty() && v != &key)
                .collect();
            if !alts.is_empty() {
                map.insert(key, alts);
            }
        }
        Self { map }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Expand each OR-group alternative with synonyms (as additional OR options).
    pub fn expand_parsed(&self, parsed: &ParsedQuery) -> ParsedQuery {
        if self.map.is_empty() {
            return parsed.clone();
        }
        let mut out = parsed.clone();
        out.must_groups = parsed
            .must_groups
            .iter()
            .map(|group| {
                let mut expanded = Vec::new();
                for tok in group {
                    if !expanded.iter().any(|e: &String| e == tok) {
                        expanded.push(tok.clone());
                    }
                    if let Some(alts) = self.map.get(tok) {
                        for a in alts {
                            if !expanded.iter().any(|e| e == a) {
                                expanded.push(a.clone());
                            }
                        }
                    }
                    // Also: if token is a synonym value, include the key.
                    for (key, alts) in &self.map {
                        if alts.iter().any(|a| a == tok) && !expanded.iter().any(|e| e == key) {
                            expanded.push(key.clone());
                        }
                    }
                }
                expanded
            })
            .collect();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_query;

    #[test]
    fn expands_or_alternatives() {
        let map = SynonymMap::from_json_str(r#"{"lol":["lul","lmao"]}"#).unwrap();
        let parsed = parse_query("lol cat");
        let expanded = map.expand_parsed(&parsed);
        assert!(expanded.must_groups[0].contains(&"lol".to_string()));
        assert!(expanded.must_groups[0].contains(&"lul".to_string()));
        assert_eq!(expanded.must_groups[1], vec!["cat".to_string()]);
    }
}
