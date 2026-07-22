//! Shared CLI data types: the aggregated per-run report the command runners
//! build and the renderers consume, plus the output-format enum and the
//! exit-code helper. Kept binary-local (not part of the library surface).

use clap::ValueEnum;
use serde::Serialize;

use skill_validator_rs::{ContaminationReport, ContentReport, TokenCount, ValidationResult};

/// The report shape produced by every command and handed to the renderers.
///
/// `results`/`token_counts`/`other_token_counts` come from the structure
/// validator; `content`/`contamination`/`reference_reports` are populated only
/// by `check` and `analyze content`. Empty collections and absent reports are
/// omitted from JSON so each command yields a focused document.
#[derive(Debug, Default, Serialize)]
pub struct CliReport {
    /// The validated skill directory, exactly as supplied on the command line.
    pub skill_dir: String,
    /// All findings, in emission order (Level/Category/Message/File/Line).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<ValidationResult>,
    /// Token counts for standard files (SKILL.md body, references, assets).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub token_counts: Vec<TokenCount>,
    /// Token counts for non-standard ("other") files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub other_token_counts: Vec<TokenCount>,
    /// Content analysis for SKILL.md (check / analyze content only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentReport>,
    /// Contamination analysis for SKILL.md (check only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contamination: Option<ContaminationReport>,
    /// Per-reference-file analysis (check --per-file only).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reference_reports: Vec<ReferenceReport>,
    /// Count of error-level results.
    pub errors: usize,
    /// Count of warning-level results.
    pub warnings: usize,
}

/// Per-reference-file analysis emitted under `check --per-file`.
#[derive(Debug, Serialize)]
pub struct ReferenceReport {
    /// Reference path relative to the skill directory, e.g. `references/x.md`.
    pub file: String,
    /// Content analysis for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentReport>,
    /// Contamination analysis for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contamination: Option<ContaminationReport>,
}

/// How the report is rendered to stdout (`-o/--output`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Human-readable text (the default).
    Text,
    /// Stable, machine-parseable JSON.
    Json,
    /// A Markdown report with a results table.
    Markdown,
}

/// Process exit code, matching the Go tool's `cmd/exitcode.go` contract:
/// `0` clean, `1` any error (or, under `--strict`, any warning), `2` warnings
/// only. CLI usage errors (`3`) are handled at parse time, not here.
pub fn exit_code(errors: usize, warnings: usize, strict: bool) -> i32 {
    if errors > 0 {
        1
    } else if warnings > 0 {
        if strict {
            1
        } else {
            2
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::exit_code;

    #[test]
    fn clean_is_zero() {
        assert_eq!(exit_code(0, 0, false), 0);
        assert_eq!(exit_code(0, 0, true), 0);
    }

    #[test]
    fn error_is_one() {
        assert_eq!(exit_code(1, 0, false), 1);
        assert_eq!(exit_code(2, 5, false), 1);
    }

    #[test]
    fn warning_is_two_unless_strict() {
        assert_eq!(exit_code(0, 1, false), 2);
        assert_eq!(exit_code(0, 3, true), 1);
    }
}
