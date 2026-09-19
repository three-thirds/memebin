//! Lex + classify a raw query string into [`ParsedQuery`].

use crate::types::ParsedQuery;

/// Parse a query string.
///
/// - Whitespace tokens → AND
/// - `"funny cat"` → phrase token
/// - `-dog` → must not match
/// - `tag:foo` / `#foo` → require tag
/// - `-tag:bar` / `-#bar` → exclude tag
/// - `ext:gif` → extension filter
/// - `fav:1|true|yes` / `is:fav` → favorites only
pub fn parse_query(input: &str) -> ParsedQuery {
    let mut parsed = ParsedQuery::default();
    for raw in lex_quoted(input) {
        absorb(&mut parsed, raw);
    }
    parsed
}

fn absorb(into: &mut ParsedQuery, raw: String) {
    let (negated, body) = match raw.strip_prefix('-') {
        Some(rest) if !rest.is_empty() => (true, rest),
        _ => (false, raw.as_str()),
    };
    if body.is_empty() {
        return;
    }
    let lower = body.to_lowercase();

    if let Some(rest) = take_prefix(&lower, "tag:") {
        if negated {
            into.push_excluded_tag(rest);
        } else {
            into.push_tag(rest);
        }
        return;
    }
    if let Some(rest) = take_prefix(&lower, "#") {
        if negated {
            into.push_excluded_tag(rest);
        } else {
            into.push_tag(rest);
        }
        return;
    }
    if let Some(rest) = take_prefix(&lower, "ext:") {
        if !negated {
            into.push_ext(rest);
        }
        return;
    }
    if let Some(rest) = take_prefix(&lower, "fav:") {
        if !negated && matches!(rest.as_str(), "1" | "true" | "yes") {
            into.set_favorites_only();
        }
        return;
    }
    if matches!(
        lower.as_str(),
        "is:fav" | "is:favorite" | "is:favourite"
    ) {
        if !negated {
            into.set_favorites_only();
        }
        return;
    }

    if negated {
        into.push_excluded_token(lower);
    } else {
        into.push_token(lower);
    }
}

fn take_prefix(s: &str, prefix: &str) -> Option<String> {
    s.strip_prefix(prefix)
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(str::to_string)
}

fn lex_quoted(query: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_quote = false;

    for ch in query.chars() {
        match ch {
            '"' => in_quote = !in_quote,
            c if c.is_whitespace() && !in_quote => {
                if !buf.is_empty() {
                    out.push(std::mem::take(&mut buf));
                }
            }
            c => buf.push(c),
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_keeps_phrases() {
        assert_eq!(lex_quoted(r#"a "b c" d"#), vec!["a", "b c", "d"]);
    }
}
