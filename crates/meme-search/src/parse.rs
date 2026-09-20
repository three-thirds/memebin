//! Lex + classify a raw query string into [`ParsedQuery`].

use crate::types::ParsedQuery;

/// Parse a query string.
///
/// - Whitespace tokens → AND
/// - `cat OR dog` / `cat | dog` → OR within a group (also `(cat | dog)`)
/// - `"funny cat"` → phrase token
/// - `-dog` → must not match
/// - `tag:foo` / `#foo` → require tag
/// - `-tag:bar` / `-#bar` → exclude tag
/// - `ext:gif` → extension filter
/// - `fav:1|true|yes` / `is:fav` → favorites only
pub fn parse_query(input: &str) -> ParsedQuery {
    let mut parsed = ParsedQuery::default();
    let mut pending_or = false;

    for raw in lex_quoted(input) {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }

        let lower_raw = trimmed.to_lowercase();
        if lower_raw == "or" || lower_raw == "|" {
            pending_or = true;
            continue;
        }

        absorb(&mut parsed, trimmed.to_string(), &mut pending_or);
    }

    parsed
}

fn absorb(into: &mut ParsedQuery, raw: String, pending_or: &mut bool) {
    let (negated, body) = match raw.strip_prefix('-') {
        Some(rest) if !rest.is_empty() => (true, rest),
        _ => (false, raw.as_str()),
    };
    let body = body.trim_matches(|c| c == '(' || c == ')');
    if body.is_empty() {
        *pending_or = false;
        return;
    }
    let lower = body.to_lowercase();

    if let Some(rest) = take_prefix(&lower, "tag:") {
        if negated {
            into.push_excluded_tag(rest);
        } else {
            into.push_tag(rest);
        }
        *pending_or = false;
        return;
    }
    if let Some(rest) = take_prefix(&lower, "#") {
        if negated {
            into.push_excluded_tag(rest);
        } else {
            into.push_tag(rest);
        }
        *pending_or = false;
        return;
    }
    if let Some(rest) = take_prefix(&lower, "ext:") {
        if !negated {
            into.push_ext(rest);
        }
        *pending_or = false;
        return;
    }
    if let Some(rest) = take_prefix(&lower, "fav:") {
        if !negated && matches!(rest.as_str(), "1" | "true" | "yes") {
            into.set_favorites_only();
        }
        *pending_or = false;
        return;
    }
    if matches!(
        lower.as_str(),
        "is:fav" | "is:favorite" | "is:favourite"
    ) {
        if !negated {
            into.set_favorites_only();
        }
        *pending_or = false;
        return;
    }

    if negated {
        into.push_excluded_token(lower);
        *pending_or = false;
        return;
    }

    let use_or = *pending_or;
    into.push_text(lower, use_or);
    *pending_or = false;
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
            '|' if !in_quote => {
                if !buf.is_empty() {
                    out.push(std::mem::take(&mut buf));
                }
                out.push("|".into());
            }
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

    #[test]
    fn parse_or_groups() {
        let q = parse_query("cat OR dog meme");
        assert_eq!(
            q.must_groups,
            vec![
                vec!["cat".to_string(), "dog".to_string()],
                vec!["meme".to_string()]
            ]
        );
    }

    #[test]
    fn parse_pipe_and_parens() {
        let q = parse_query("(cat | dog) funny");
        assert_eq!(
            q.must_groups,
            vec![
                vec!["cat".to_string(), "dog".to_string()],
                vec!["funny".to_string()]
            ]
        );
    }
}
