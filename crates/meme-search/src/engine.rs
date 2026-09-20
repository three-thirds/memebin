//! Token/item scoring and filter gates.

use crate::boost::{browse_total, filter_reason_labels, query_soft_boosts};
use crate::matcher::{grade_field, span_to_match, FieldGrade};
use crate::normalize::fold_case;
use crate::types::{MatchSpan, ParsedQuery, Searchable};

const NAME_WEIGHT: f64 = 1.0;
const ALIAS_WEIGHT: f64 = 0.95;
const TAG_WEIGHT: f64 = 0.85;
const EXCLUSION_FLOOR: f64 = 50.0;

/// Best score for a single token against name + aliases + tags.
pub fn score_token<T: Searchable>(item: &T, token: &str) -> (f64, Vec<String>, Vec<MatchSpan>) {
    let mut best = 0.0_f64;
    let mut reasons = Vec::new();
    let mut highlights = Vec::new();

    consider(
        &mut best,
        &mut reasons,
        &mut highlights,
        "name",
        item.name(),
        grade_field(item.name(), token),
        NAME_WEIGHT,
    );

    for alias in item.aliases() {
        consider(
            &mut best,
            &mut reasons,
            &mut highlights,
            "alias",
            alias,
            grade_field(alias, token),
            ALIAS_WEIGHT,
        );
    }

    for tag in item.tags() {
        let label = format!("tag:{tag}");
        consider(
            &mut best,
            &mut reasons,
            &mut highlights,
            &label,
            tag,
            grade_field(tag, token),
            TAG_WEIGHT,
        );
    }

    (best, reasons, highlights)
}

fn consider(
    best: &mut f64,
    reasons: &mut Vec<String>,
    highlights: &mut Vec<MatchSpan>,
    channel: &str,
    _field_text: &str,
    grade: FieldGrade,
    weight: f64,
) {
    let pts = grade.points * weight;
    let reason_channel = if channel.starts_with("tag:") {
        "tag"
    } else {
        channel
    };

    if pts > *best {
        *best = pts;
        reasons.clear();
        highlights.clear();
        if grade.points > 0.0 {
            reasons.push(format!("{reason_channel}:{}", grade.kind.as_label()));
            if let Some(span) = grade.span {
                highlights.push(span_to_match(channel, span));
            }
        }
    } else if (pts - *best).abs() < f64::EPSILON && grade.points > 0.0 {
        reasons.push(format!("{reason_channel}:{}", grade.kind.as_label()));
        if let Some(span) = grade.span {
            let h = span_to_match(channel, span);
            if !highlights.iter().any(|x| x == &h) {
                highlights.push(h);
            }
        }
    }
}

/// Score an item against a parsed query. Returns None if it fails AND/OR/filters/exclusions.
pub fn score_item<T: Searchable>(
    item: &T,
    parsed: &ParsedQuery,
) -> Option<(f64, Vec<String>, Vec<MatchSpan>)> {
    if !gates_ok(item, parsed) {
        return None;
    }

    let mut reasons = filter_reason_labels(parsed);
    let mut highlights = Vec::new();

    if !parsed.has_text_tokens() {
        let score = browse_total(item, &mut reasons);
        return Some((score, reasons, highlights));
    }

    let mut total = 0.0_f64;
    for group in &parsed.must_groups {
        if group.is_empty() {
            continue;
        }
        let mut group_best = 0.0_f64;
        let mut group_reasons = Vec::new();
        let mut group_highlights = Vec::new();

        for alt in group {
            let (pts, alt_reasons, alt_spans) = score_token(item, alt);
            if pts > group_best {
                group_best = pts;
                group_reasons = alt_reasons;
                group_highlights = alt_spans;
            }
        }

        if group_best <= 0.0 {
            return None;
        }
        total += group_best;
        reasons.extend(group_reasons);
        for h in group_highlights {
            if !highlights.iter().any(|x| x == &h) {
                highlights.push(h);
            }
        }
    }

    total += query_soft_boosts(item, parsed, &mut reasons);
    Some((total, reasons, highlights))
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
        let (pts, _, _) = score_token(item, ex);
        if pts >= EXCLUSION_FLOOR {
            return false;
        }
    }

    true
}
