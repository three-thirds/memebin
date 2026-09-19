//! Token/item scoring and filter gates.

use crate::boost::{browse_total, filter_reason_labels, query_soft_boosts};
use crate::matcher::{grade_field, FieldGrade};
use crate::normalize::fold_case;
use crate::types::{ParsedQuery, Searchable};

const NAME_WEIGHT: f64 = 1.0;
const TAG_WEIGHT: f64 = 0.85;
const EXCLUSION_FLOOR: f64 = 50.0;

/// Best score for a single token against name + tags (name weighted higher).
pub fn score_token<T: Searchable>(item: &T, token: &str) -> (f64, Vec<String>) {
    let mut best = 0.0_f64;
    let mut reasons = Vec::new();

    consider(
        &mut best,
        &mut reasons,
        "name",
        grade_field(item.name(), token),
        NAME_WEIGHT,
    );

    for tag in item.tags() {
        consider(
            &mut best,
            &mut reasons,
            "tag",
            grade_field(tag, token),
            TAG_WEIGHT,
        );
    }

    (best, reasons)
}

fn consider(
    best: &mut f64,
    reasons: &mut Vec<String>,
    channel: &str,
    grade: FieldGrade,
    weight: f64,
) {
    let pts = grade.points * weight;
    if pts > *best {
        *best = pts;
        reasons.clear();
        if grade.points > 0.0 {
            reasons.push(format!("{channel}:{}", grade.kind.as_label()));
        }
    } else if (pts - *best).abs() < f64::EPSILON && grade.points > 0.0 {
        reasons.push(format!("{channel}:{}", grade.kind.as_label()));
    }
}

/// Score an item against a parsed query. Returns None if it fails AND/filters/exclusions.
pub fn score_item<T: Searchable>(item: &T, parsed: &ParsedQuery) -> Option<(f64, Vec<String>)> {
    if !gates_ok(item, parsed) {
        return None;
    }

    let mut reasons = filter_reason_labels(parsed);

    if !parsed.has_text_tokens() {
        let score = browse_total(item, &mut reasons);
        return Some((score, reasons));
    }

    let mut total = 0.0_f64;
    for token in &parsed.tokens {
        let (pts, token_reasons) = score_token(item, token);
        if pts <= 0.0 {
            return None;
        }
        total += pts;
        reasons.extend(token_reasons);
    }

    total += query_soft_boosts(item, parsed, &mut reasons);
    Some((total, reasons))
}

fn gates_ok<T: Searchable>(item: &T, parsed: &ParsedQuery) -> bool {
    if parsed.favorites_only && !item.favorite() {
        return false;
    }

    if !parsed.extensions.is_empty() {
        let ext = fold_case(item.extension());
        if parsed.extensions.iter().all(|e| e != &ext) {
            return false;
        }
    }

    for needed in &parsed.tag_filters {
        if !item.tags().iter().any(|t| fold_case(t).contains(needed)) {
            return false;
        }
    }

    for banned in &parsed.excluded_tags {
        if item.tags().iter().any(|t| fold_case(t).contains(banned)) {
            return false;
        }
    }

    for ex in &parsed.excluded_tokens {
        let (pts, _) = score_token(item, ex);
        if pts >= EXCLUSION_FLOOR {
            return false;
        }
    }

    true
}
