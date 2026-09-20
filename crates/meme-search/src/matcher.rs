//! Field-level match grading via [`MatchKind`].

use strsim::{jaro_winkler, normalized_levenshtein};

use crate::normalize::{fold_case, words_of};
use crate::types::MatchSpan;

const FUZZY_THRESHOLD: f64 = 0.72;
const JARO_THRESHOLD: f64 = 0.88;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MatchKind {
    Exact,
    Word,
    Prefix,
    Substr,
    Fuzzy,
    None,
}

impl MatchKind {
    pub(crate) fn as_label(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Word => "word",
            Self::Prefix => "prefix",
            Self::Substr => "substr",
            Self::Fuzzy => "fuzzy",
            Self::None => "none",
        }
    }

    pub(crate) fn base_points(self) -> f64 {
        match self {
            Self::Exact => 100.0,
            Self::Word => 95.0,
            Self::Prefix => 80.0,
            Self::Substr => 50.0,
            Self::Fuzzy => 0.0,
            Self::None => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FieldGrade {
    pub kind: MatchKind,
    pub points: f64,
    /// Character range in the original field when known.
    pub span: Option<(usize, usize)>,
}

impl FieldGrade {
    pub(crate) fn none() -> Self {
        Self {
            kind: MatchKind::None,
            points: 0.0,
            span: None,
        }
    }

    pub(crate) fn scored(kind: MatchKind, points: f64, span: Option<(usize, usize)>) -> Self {
        Self { kind, points, span }
    }
}

pub(crate) fn grade_field(field: &str, token: &str) -> FieldGrade {
    let folded = fold_case(field);
    if folded == token {
        return FieldGrade::scored(MatchKind::Exact, MatchKind::Exact.base_points(), Some((0, field.chars().count())));
    }

    let parts = words_of(&folded);
    if let Some(span) = find_token_span(field, token) {
        // Prefer exact word / prefix / substr classification using folded text.
        if parts.iter().any(|w| w == token) {
            return FieldGrade::scored(MatchKind::Word, MatchKind::Word.base_points(), Some(span));
        }
        if folded.starts_with(token) || parts.iter().any(|w| w.starts_with(token)) {
            return FieldGrade::scored(MatchKind::Prefix, MatchKind::Prefix.base_points(), Some(span));
        }
        if folded.contains(token) || parts.iter().any(|w| w.contains(token)) {
            return FieldGrade::scored(MatchKind::Substr, MatchKind::Substr.base_points(), Some(span));
        }
    } else {
        if parts.iter().any(|w| w == token) {
            return FieldGrade::scored(MatchKind::Word, MatchKind::Word.base_points(), None);
        }
        if folded.starts_with(token) || parts.iter().any(|w| w.starts_with(token)) {
            return FieldGrade::scored(MatchKind::Prefix, MatchKind::Prefix.base_points(), None);
        }
        if folded.contains(token) || parts.iter().any(|w| w.contains(token)) {
            return FieldGrade::scored(MatchKind::Substr, MatchKind::Substr.base_points(), None);
        }
    }

    let sim = max_similarity(&folded, &parts, token);
    if sim >= JARO_THRESHOLD || sim >= FUZZY_THRESHOLD {
        return FieldGrade::scored(
            MatchKind::Fuzzy,
            20.0 + sim.clamp(0.0, 1.0) * 25.0,
            find_token_span(field, token),
        );
    }

    FieldGrade::none()
}

/// Case-insensitive locate of `token` in `field` as char indices.
pub(crate) fn find_token_span(field: &str, token: &str) -> Option<(usize, usize)> {
    if token.is_empty() {
        return None;
    }
    let field_lower: Vec<char> = field.to_lowercase().chars().collect();
    let tok: Vec<char> = token.chars().collect();
    if tok.len() > field_lower.len() {
        return None;
    }
    'outer: for start in 0..=(field_lower.len() - tok.len()) {
        for (i, tc) in tok.iter().enumerate() {
            if field_lower[start + i] != *tc {
                continue 'outer;
            }
        }
        return Some((start, start + tok.len()));
    }
    None
}

pub(crate) fn span_to_match(field_label: &str, span: (usize, usize)) -> MatchSpan {
    MatchSpan {
        field: field_label.to_string(),
        start: span.0,
        end: span.1,
    }
}

fn max_similarity(field: &str, words: &[String], token: &str) -> f64 {
    let mut best = 0.0_f64;
    for cand in std::iter::once(field).chain(words.iter().map(String::as_str)) {
        best = best
            .max(jaro_winkler(cand, token))
            .max(normalized_levenshtein(cand, token));
    }
    best
}

pub(crate) fn suggest_points(field: &str, token: &str) -> f64 {
    grade_field(field, token).points
}
