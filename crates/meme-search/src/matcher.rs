//! Field-level match grading via [`MatchKind`].

use strsim::{jaro_winkler, normalized_levenshtein};

use crate::normalize::{fold_case, words_of};

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
            Self::Fuzzy => 0.0, // filled by similarity
            Self::None => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FieldGrade {
    pub kind: MatchKind,
    pub points: f64,
}

impl FieldGrade {
    pub(crate) fn none() -> Self {
        Self {
            kind: MatchKind::None,
            points: 0.0,
        }
    }

    pub(crate) fn fixed(kind: MatchKind) -> Self {
        Self {
            kind,
            points: kind.base_points(),
        }
    }

    pub(crate) fn fuzzy(sim: f64) -> Self {
        Self {
            kind: MatchKind::Fuzzy,
            points: 20.0 + sim.clamp(0.0, 1.0) * 25.0,
        }
    }
}

pub(crate) fn grade_field(field: &str, token: &str) -> FieldGrade {
    let folded = fold_case(field);
    if folded == token {
        return FieldGrade::fixed(MatchKind::Exact);
    }

    let parts = words_of(&folded);
    if parts.iter().any(|w| w == token) {
        return FieldGrade::fixed(MatchKind::Word);
    }
    if folded.starts_with(token) || parts.iter().any(|w| w.starts_with(token)) {
        return FieldGrade::fixed(MatchKind::Prefix);
    }
    if folded.contains(token) || parts.iter().any(|w| w.contains(token)) {
        return FieldGrade::fixed(MatchKind::Substr);
    }

    let sim = max_similarity(&folded, &parts, token);
    if sim >= JARO_THRESHOLD || sim >= FUZZY_THRESHOLD {
        return FieldGrade::fuzzy(sim);
    }

    FieldGrade::none()
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
