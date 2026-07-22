//! Bridge onto `skill-validator-rs`, replacing the Go `validator_bridge.go`
//! which coupled to the Go validator's `orchestrate.RunContentAnalysis` and
//! `structure.Validate`. The two calls are mapped to
//! [`skill_validator_rs::analyze_content`] and [`skill_validator_rs::validate`].

use regex::Regex;
use skill_validator_rs::{analyze_content, validate, ContentReport, Level, Options, Report};
use std::path::Path;

/// Cached validator results for a single skill. Constructed once per score.
/// Either field may be `None`, matching the Go bridge where `Content` came
/// from a possibly-nil report and unit tests supplied a zero-valued bridge.
#[derive(Default)]
pub struct ValidatorBridge {
    /// Content metrics for SKILL.md (Go `b.Content`).
    pub content: Option<ContentReport>,
    /// Structural validation report (Go `b.Structure`).
    pub structure: Option<Report>,
}

impl ValidatorBridge {
    /// Build a bridge by running the validator over `skill_dir`.
    ///
    /// The Go bridge called `orchestrate.RunContentAnalysis(skillDir)` (which
    /// reads and analyses SKILL.md) and `structure.Validate(skillDir, opts)`
    /// with `SkipOrphans`, `AllowFlatLayouts` and `AllowExtraFrontmatter` all
    /// set. Here we read SKILL.md directly and feed it to `analyze_content`,
    /// and set the equivalently named `Options` fields.
    pub fn new(skill_dir: &Path) -> Self {
        let content = std::fs::read_to_string(skill_dir.join("SKILL.md"))
            .ok()
            .map(|c| analyze_content(&c));

        let opts = Options {
            skip_orphans: true,
            allow_flat_layouts: true,
            allow_extra_frontmatter: true,
            allow_dirs: Vec::new(),
        };
        let structure = Some(validate(skill_dir, &opts));

        ValidatorBridge { content, structure }
    }

    /// Token count for SKILL.md, or 0 if unavailable (Go `skillMDTokens`).
    /// The validator labels the body count `"SKILL.md body"`, so the Go
    /// prefix match on `"SKILL.md "` transfers directly.
    pub fn skill_md_tokens(&self) -> i32 {
        let Some(structure) = &self.structure else {
            return 0;
        };
        for tc in &structure.token_counts {
            if tc.file == "SKILL.md" || tc.file.starts_with("SKILL.md ") {
                return tc.tokens as i32;
            }
        }
        0
    }

    /// True if a warning-level structure result contains `substr` (Go
    /// `hasStructureWarning`).
    pub fn has_structure_warning(&self, substr: &str) -> bool {
        let Some(structure) = &self.structure else {
            return false;
        };
        structure
            .results
            .iter()
            .any(|r| r.level == Level::Warning && r.message.contains(substr))
    }

    /// Byte length of the description reported by the frontmatter check
    /// (`"description: (N chars)"`), or -1 if not found (Go `descriptionLen`).
    pub fn description_len(&self) -> i32 {
        let Some(structure) = &self.structure else {
            return -1;
        };
        let re = Regex::new(r"^description: \((\d+) chars\)").expect("valid regex");
        for r in &structure.results {
            if r.category != "Frontmatter" {
                continue;
            }
            if let Some(caps) = re.captures(&r.message) {
                if let Ok(n) = caps[1].parse::<i32>() {
                    return n;
                }
            }
        }
        -1
    }

    /// True if the internal-links check emitted a warning, covering `../`
    /// outside-code-block violations (Go `hasInternalLinkWarning`).
    pub fn has_internal_link_warning(&self) -> bool {
        let Some(structure) = &self.structure else {
            return false;
        };
        structure
            .results
            .iter()
            .any(|r| r.category == "Links" && r.level == Level::Warning)
    }
}
