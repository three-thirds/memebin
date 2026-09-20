//! Case folding and CamelCase-aware word splitting.

pub fn fold_case(s: &str) -> String {
    s.to_lowercase()
}

/// Split into lowercase alphanumeric runs, breaking on CamelCase boundaries.
pub fn words_of(field: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut cur = String::new();

    for ch in field.chars() {
        if ch.is_alphanumeric() {
            if camel_boundary(&cur, ch) {
                words.push(std::mem::take(&mut cur).to_lowercase());
            }
            cur.push(ch);
        } else if !cur.is_empty() {
            words.push(std::mem::take(&mut cur).to_lowercase());
        }
    }

    if !cur.is_empty() {
        words.push(cur.to_lowercase());
    }
    words
}

fn camel_boundary(cur: &str, next: char) -> bool {
    !cur.is_empty()
        && next.is_uppercase()
        && cur.chars().last().is_some_and(|c| c.is_lowercase())
}
