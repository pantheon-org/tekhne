//! Core data types ported from the Go validator's `types` package: severity
//! levels, validation results, token counts, and the aggregated report. Only
//! the offline-deterministic surface is ported; the LLM scoring types
//! (`DimensionScore`, `Scored`) are intentionally omitted (A4a scope).

use serde::{Deserialize, Serialize};

/// Severity of a validation result. Discriminants match the Go `iota` order,
/// so `Level as u8` yields the same integer the Go tool serialises to JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    /// A check passed successfully.
    Pass = 0,
    /// An informational finding requiring no action.
    Info = 1,
    /// A non-blocking issue that should be reviewed.
    Warning = 2,
    /// A blocking issue that must be fixed.
    Error = 3,
}

impl Level {
    /// The integer discriminant the Go tool marshals (`Pass` = 0 .. `Error` = 3).
    pub fn as_int(self) -> u8 {
        self as u8
    }

    /// The lowercase name (Go `Level.String`).
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Pass => "pass",
            Level::Info => "info",
            Level::Warning => "warning",
            Level::Error => "error",
        }
    }
}

/// A single validation finding (Go `types.Result`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Severity of the finding.
    #[serde(rename = "Level", with = "level_int")]
    pub level: Level,
    /// The check category, e.g. "Structure", "Frontmatter", "Tokens".
    #[serde(rename = "Category")]
    pub category: String,
    /// The human-readable message (some are parsed by the auditor verbatim).
    #[serde(rename = "Message")]
    pub message: String,
    /// Path relative to the skill dir, e.g. "SKILL.md", "references/guide.md".
    #[serde(rename = "File")]
    pub file: String,
    /// Line number; 0 means no line info.
    #[serde(rename = "Line")]
    pub line: usize,
}

/// Token count for a single file (Go `types.TokenCount`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenCount {
    /// File label, e.g. "SKILL.md body" or "references/guide.md".
    #[serde(rename = "File")]
    pub file: String,
    /// Number of `o200k_base` token ids for the file's text.
    #[serde(rename = "Tokens")]
    pub tokens: usize,
}

/// Content quality metrics (Go `types.ContentReport`). Field names match the
/// Go JSON tags so golden reports deserialise directly.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContentReport {
    /// Whitespace-delimited word count over the raw SKILL.md content.
    #[serde(rename = "word_count")]
    pub word_count: usize,
    /// Number of fenced code blocks.
    #[serde(rename = "code_block_count")]
    pub code_block_count: usize,
    /// Ratio of code-block words to total words (4dp).
    #[serde(rename = "code_block_ratio")]
    pub code_block_ratio: f64,
    /// Code-fence language tags, in document order.
    #[serde(rename = "code_languages")]
    pub code_languages: Vec<String>,
    /// Number of prose sentences after stripping code.
    #[serde(rename = "sentence_count")]
    pub sentence_count: usize,
    /// Sentences beginning with an imperative verb.
    #[serde(rename = "imperative_count")]
    pub imperative_count: usize,
    /// Imperative sentences / total sentences (4dp).
    #[serde(rename = "imperative_ratio")]
    pub imperative_ratio: f64,
    /// Blended density metric (4dp).
    #[serde(rename = "information_density")]
    pub information_density: f64,
    /// Count of strong directive markers.
    #[serde(rename = "strong_markers")]
    pub strong_markers: usize,
    /// Count of weak/advisory markers.
    #[serde(rename = "weak_markers")]
    pub weak_markers: usize,
    /// strong / (strong + weak) (4dp).
    #[serde(rename = "instruction_specificity")]
    pub instruction_specificity: f64,
    /// Number of H2+ section headers.
    #[serde(rename = "section_count")]
    pub section_count: usize,
    /// Number of list items.
    #[serde(rename = "list_item_count")]
    pub list_item_count: usize,
}

/// Cross-language contamination metrics (Go `types.ContaminationReport`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContaminationReport {
    /// Multi-interface tools detected in the name/content.
    #[serde(rename = "multi_interface_tools")]
    pub multi_interface_tools: Vec<String>,
    /// Code-fence languages (document order).
    #[serde(rename = "code_languages")]
    pub code_languages: Vec<String>,
    /// Sorted language categories present in code blocks.
    #[serde(rename = "language_categories")]
    pub language_categories: Vec<String>,
    /// The dominant language category (tie-break by first appearance).
    #[serde(rename = "primary_category")]
    pub primary_category: String,
    /// Sorted application categories mismatched against the primary.
    #[serde(rename = "mismatched_categories")]
    pub mismatched_categories: Vec<String>,
    /// Per-category mismatch weights.
    #[serde(rename = "mismatch_weights")]
    pub mismatch_weights: std::collections::BTreeMap<String, f64>,
    /// Whether an application-to-application language mismatch exists.
    #[serde(rename = "language_mismatch")]
    pub language_mismatch: bool,
    /// Sorted technology-reference categories detected in prose.
    #[serde(rename = "tech_references")]
    pub tech_references: Vec<String>,
    /// Number of distinct scope categories.
    #[serde(rename = "scope_breadth")]
    pub scope_breadth: usize,
    /// Combined contamination score in [0, 1] (4dp).
    #[serde(rename = "contamination_score")]
    pub contamination_score: f64,
    /// "low", "medium", or "high".
    #[serde(rename = "contamination_level")]
    pub contamination_level: String,
}

/// Per-reference-file analysis (Go `types.ReferenceFileReport`).
#[derive(Debug, Clone, Default)]
pub struct ReferenceFileReport {
    /// Reference filename (basename under `references/`).
    pub file: String,
    /// Content analysis for the file, if content group ran.
    pub content_report: Option<ContentReport>,
    /// Contamination analysis for the file, if that group ran.
    pub contamination_report: Option<ContaminationReport>,
}

/// Aggregated validation output for a single skill (Go `types.Report`).
#[derive(Debug, Clone, Default)]
pub struct Report {
    /// The validated skill directory.
    pub skill_dir: String,
    /// All findings, in emission order.
    pub results: Vec<ValidationResult>,
    /// Token counts for standard files (SKILL.md body, references, assets).
    pub token_counts: Vec<TokenCount>,
    /// Token counts for non-standard ("other") files.
    pub other_token_counts: Vec<TokenCount>,
    /// Content analysis for SKILL.md, if run.
    pub content_report: Option<ContentReport>,
    /// Aggregate content analysis over reference files, if run.
    pub references_content_report: Option<ContentReport>,
    /// Contamination analysis for SKILL.md, if run.
    pub contamination_report: Option<ContaminationReport>,
    /// Aggregate contamination over reference files, if run.
    pub references_contamination_report: Option<ContaminationReport>,
    /// Per-reference-file reports.
    pub reference_reports: Vec<ReferenceFileReport>,
    /// Count of error-level results (populated by [`Report::tally`]).
    pub errors: usize,
    /// Count of warning-level results (populated by [`Report::tally`]).
    pub warnings: usize,
}

impl Report {
    /// Create an empty report for `dir`.
    pub fn new(dir: &str) -> Self {
        Report {
            skill_dir: dir.to_string(),
            ..Default::default()
        }
    }

    /// Recount errors and warnings from `results` (Go `Report.Tally`).
    pub fn tally(&mut self) {
        self.errors = 0;
        self.warnings = 0;
        for r in &self.results {
            match r.level {
                Level::Error => self.errors += 1,
                Level::Warning => self.warnings += 1,
                _ => {}
            }
        }
    }
}

/// Whether a directory is a single skill, a multi-skill parent, or neither
/// (Go `types.SkillMode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillMode {
    /// No SKILL.md was found.
    NoSkill,
    /// The directory itself contains a SKILL.md.
    SingleSkill,
    /// The directory contains subdirectories with SKILL.md files.
    MultiSkill,
}

/// Aggregated results across multiple skills (Go `types.MultiReport`).
#[derive(Debug, Default)]
pub struct MultiReport {
    /// One report per skill directory.
    pub skills: Vec<Report>,
    /// Total error count across all skills.
    pub errors: usize,
    /// Total warning count across all skills.
    pub warnings: usize,
}

/// A builder that stamps a shared `Category` and default `File` onto results
/// (Go `types.ResultContext`).
#[derive(Debug, Clone, Default)]
pub struct ResultContext {
    /// The category applied to every result built here.
    pub category: String,
    /// The default file; `*_file` builders override it.
    pub file: String,
}

impl ResultContext {
    /// A context with the given category and no default file.
    pub fn new(category: &str) -> Self {
        ResultContext {
            category: category.to_string(),
            file: String::new(),
        }
    }

    /// A context with the given category and default file.
    pub fn with_file(category: &str, file: &str) -> Self {
        ResultContext {
            category: category.to_string(),
            file: file.to_string(),
        }
    }

    fn build(&self, level: Level, file: &str, line: usize, msg: String) -> ValidationResult {
        let file = if file.is_empty() { &self.file } else { file };
        ValidationResult {
            level,
            category: self.category.clone(),
            message: msg,
            file: file.to_string(),
            line,
        }
    }

    /// Pass result using the default file.
    pub fn pass(&self, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Pass, "", 0, msg.into())
    }

    /// Info result using the default file.
    pub fn info(&self, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Info, "", 0, msg.into())
    }

    /// Warning result using the default file.
    pub fn warn(&self, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Warning, "", 0, msg.into())
    }

    /// Error result using the default file.
    pub fn error(&self, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Error, "", 0, msg.into())
    }

    /// Pass result with an explicit file.
    pub fn pass_file(&self, file: &str, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Pass, file, 0, msg.into())
    }

    /// Warning result with an explicit file.
    pub fn warn_file(&self, file: &str, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Warning, file, 0, msg.into())
    }

    /// Error result with an explicit file.
    pub fn error_file(&self, file: &str, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Error, file, 0, msg.into())
    }

    /// Error result with an explicit file and line number.
    pub fn error_at_line(&self, file: &str, line: usize, msg: impl Into<String>) -> ValidationResult {
        self.build(Level::Error, file, line, msg.into())
    }
}

// Serialise/deserialise `Level` as its integer discriminant, matching Go's
// default marshalling of the `int`-backed `Level` type.
mod level_int {
    use super::Level;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(level: &Level, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(level.as_int())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Level, D::Error> {
        let n = u8::deserialize(d)?;
        Ok(match n {
            0 => Level::Pass,
            1 => Level::Info,
            2 => Level::Warning,
            _ => Level::Error,
        })
    }
}
