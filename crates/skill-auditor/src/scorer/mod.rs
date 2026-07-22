//! Deterministic 9-dimension skill scorer, ported from the Go
//! `tools/skill-auditor/scorer` package. No LLM or network access is involved;
//! every dimension is a pure function of on-disk content and the
//! `skill-validator-rs` analysis.

mod agents;
mod bridge;
mod d1_knowledge_delta;
mod d2_mindset_procedures;
mod d3_anti_pattern;
mod d4_specification;
mod d5_progressive_disclosure;
mod d6_freedom_calibration;
mod d7_pattern_recognition;
mod d8_practical_usability;
mod d9_eval_validation;
mod grades;
mod helpers;

pub use bridge::ValidatorBridge;
pub use grades::{grade, grade_rank};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

/// A single error or warning produced during scoring (Go `Diagnostic`).
/// `severity` is internal state, not serialised, matching the Go tag layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The dimension code, e.g. "D1".
    pub dimension: String,
    /// The human-readable message.
    pub message: String,
    /// "error" or "warning"; not serialised.
    #[serde(skip)]
    pub severity: String,
}

impl Diagnostic {
    /// The severity, "error" or "warning" (Go `Diagnostic.Severity`).
    pub fn severity(&self) -> &str {
        &self.severity
    }
}

fn err_diag(dimension: &str, message: &str) -> Diagnostic {
    Diagnostic {
        dimension: dimension.to_string(),
        message: message.to_string(),
        severity: "error".to_string(),
    }
}

fn warn_diag(dimension: &str, message: &str) -> Diagnostic {
    Diagnostic {
        dimension: dimension.to_string(),
        message: message.to_string(),
        severity: "warning".to_string(),
    }
}

/// The output of scoring a skill (Go `Result`). The JSON shape (field order,
/// camelCase names, sorted `dimensions` keys, omitted empty detail arrays)
/// matches the Go `--json` and stored `audit.json` format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result {
    /// The skill path or canonical key.
    pub skill: String,
    /// Score date (YYYY-MM-DD).
    pub date: String,
    /// Per-dimension scores keyed by camelCase name (sorted).
    pub dimensions: BTreeMap<String, i32>,
    /// Sum of all dimension scores.
    pub total: i32,
    /// Maximum possible total (140).
    #[serde(rename = "maxTotal")]
    pub max_total: i32,
    /// Letter grade for `total`.
    pub grade: String,
    /// Total SKILL.md line count (including blank lines).
    pub lines: i32,
    /// Whether a `references/` directory with markdown files exists.
    #[serde(rename = "hasReferences")]
    pub has_references: bool,
    /// Count of markdown files under `references/`.
    #[serde(rename = "referenceCount")]
    pub reference_count: i32,
    /// Whether `## References` is the last H2 with a bullet link.
    #[serde(rename = "referenceSectionCompliant")]
    pub reference_section_compliant: bool,
    /// Number of error diagnostics.
    pub errors: i32,
    /// Number of warning diagnostics.
    pub warnings: i32,
    /// Error diagnostics (omitted when empty).
    #[serde(
        rename = "errorDetails",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_details: Vec<Diagnostic>,
    /// Warning diagnostics (omitted when empty).
    #[serde(
        rename = "warningDetails",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub warning_details: Vec<Diagnostic>,
}

/// Evaluate a skill at `skill_path`, stamping today's date (Go `Score`).
pub fn score(skill_path: &Path) -> io::Result<Result> {
    score_with_date(skill_path, &today())
}

/// Evaluate a skill at `skill_path` with an explicit date. Making the date a
/// parameter keeps scoring deterministic; the later parity chunk pins it.
pub fn score_with_date(skill_path: &Path, date: &str) -> io::Result<Result> {
    let content = std::fs::read_to_string(skill_path)?;
    let skill_dir = skill_path.parent().unwrap_or_else(|| Path::new("."));
    let evals_dir = skill_dir.join("evals");
    Ok(score_from_content(skill_path, &content, &evals_dir, date))
}

/// Score a skill from pre-loaded content, an evals directory, and a date (Go
/// `ScoreFromContent`, with `date` injected instead of `time.Now()`).
pub fn score_from_content(
    skill_path: &Path,
    content: &str,
    evals_dir: &Path,
    date: &str,
) -> Result {
    let skill_dir = skill_path.parent().unwrap_or_else(|| Path::new("."));
    let bridge = ValidatorBridge::new(skill_dir);

    let (d1, diag1) = d1_knowledge_delta::score(content, skill_dir);
    let d2 = d2_mindset_procedures::score(content, &bridge);
    let (d3, diag3) = d3_anti_pattern::score(content, skill_dir, &bridge);
    let (d4, diag4) = d4_specification::score(content, skill_dir, &bridge);
    let (d5, lines, ref_count, has_refs) =
        d5_progressive_disclosure::score_with_meta(content, skill_dir, &bridge);
    let d6 = d6_freedom_calibration::score(&bridge);
    let (d7, diag7) = d7_pattern_recognition::score(&bridge);
    let d8 = d8_practical_usability::score(content, &bridge);
    let (d9, diag9) = d9_eval_validation::score(evals_dir);

    let total = d1 + d2 + d3 + d4 + d5 + d6 + d7 + d8 + d9;

    let mut error_details = Vec::new();
    let mut warning_details = Vec::new();
    for d in diag1
        .into_iter()
        .chain(diag3)
        .chain(diag4)
        .chain(diag7)
        .chain(diag9)
    {
        if d.severity == "error" {
            error_details.push(d);
        } else {
            warning_details.push(d);
        }
    }
    if !has_refs {
        warning_details.push(warn_diag(
            "D5",
            "no references/ directory (progressive disclosure missing)",
        ));
    }

    let dimensions = BTreeMap::from([
        ("knowledgeDelta".to_string(), d1),
        ("mindsetProcedures".to_string(), d2),
        ("antiPatternQuality".to_string(), d3),
        ("specificationCompliance".to_string(), d4),
        ("progressiveDisclosure".to_string(), d5),
        ("freedomCalibration".to_string(), d6),
        ("patternRecognition".to_string(), d7),
        ("practicalUsability".to_string(), d8),
        ("evalValidation".to_string(), d9),
    ]);

    Result {
        skill: skill_path.to_string_lossy().into_owned(),
        date: date.to_string(),
        dimensions,
        total,
        max_total: 140,
        grade: grade(total),
        lines,
        has_references: has_refs,
        reference_count: ref_count,
        reference_section_compliant: d5_progressive_disclosure::is_reference_section_compliant(
            content,
        ),
        errors: error_details.len() as i32,
        warnings: warning_details.len() as i32,
        error_details,
        warning_details,
    }
}

/// Today's date as YYYY-MM-DD (UTC). This is the only non-deterministic input
/// and is confined to the public `score` entry so tests can pin it.
fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    civil_from_days(secs.div_euclid(86_400))
}

/// Convert a count of days since the Unix epoch to a YYYY-MM-DD string using
/// Howard Hinnant's civil-from-days algorithm.
fn civil_from_days(days: i64) -> String {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{year:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("testdata")
            .join("fixtures")
            .join(name)
            .join("SKILL.md")
    }

    #[test]
    fn err_and_warn_diag() {
        let e = err_diag("D1", "something failed");
        assert_eq!(e.dimension, "D1");
        assert_eq!(e.message, "something failed");
        assert_eq!(e.severity, "error");
        let w = warn_diag("D4", "missing ref");
        assert_eq!(w.dimension, "D4");
        assert_eq!(w.message, "missing ref");
        assert_eq!(w.severity, "warning");
    }

    #[test]
    fn score_minimal() {
        let result = score(&fixture("skill-minimal")).expect("score");
        assert!(
            result.total < 80,
            "total {} grade {}",
            result.total,
            result.grade
        );
        assert_eq!(result.grade, "F");
    }

    #[test]
    fn score_full_a_grade() {
        let result = score(&fixture("skill-full")).expect("score");
        assert!(
            result.total >= 126,
            "total {} grade {} dims {:?}",
            result.total,
            result.grade,
            result.dimensions
        );
        assert!(grade_rank(&result.grade).unwrap() >= grade_rank("A").unwrap());
    }

    #[test]
    fn civil_from_days_epoch() {
        assert_eq!(civil_from_days(0), "1970-01-01");
        assert_eq!(civil_from_days(19_925), "2024-07-21");
    }
}
