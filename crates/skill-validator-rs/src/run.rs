//! Command runners: translate parsed CLI arguments into library calls and
//! build a [`CliReport`]. `validate structure` runs the structure group;
//! `check` additionally runs the content and contamination groups (per-file
//! when requested); `analyze content` runs the content group alone.

use std::path::Path;

use skill_validator_rs::structure::fsutil::read_dir_sorted;
use skill_validator_rs::{analyze_contamination, analyze_content, validate, Options, Skill};

use crate::model::{CliReport, ReferenceReport};

/// Run `validate structure <dir>`: the structure group only.
pub fn validate_structure(dir: &str, opts: &Options) -> CliReport {
    let report = validate(Path::new(dir), opts);
    CliReport {
        skill_dir: dir.to_string(),
        results: report.results,
        token_counts: report.token_counts,
        other_token_counts: report.other_token_counts,
        errors: report.errors,
        warnings: report.warnings,
        ..CliReport::default()
    }
}

/// Run `check <dir>`: the structure group plus content and contamination
/// analysis of SKILL.md, and (when `per_file`) of each reference file.
pub fn check(dir: &str, opts: &Options, per_file: bool) -> CliReport {
    let mut cli = validate_structure(dir, opts);

    // Content and contamination are metrics groups: they enrich the report but
    // never add findings, so the exit code stays driven by the structure group.
    let path = Path::new(dir);
    if let Ok(skill) = Skill::load(path) {
        let content = analyze_content(&skill.raw_content);
        let contamination = analyze_contamination(
            &skill_name(path),
            &skill.raw_content,
            &content.code_languages,
        );
        cli.content = Some(content);
        cli.contamination = Some(contamination);
    }

    if per_file {
        cli.reference_reports = analyze_reference_files(path);
    }

    cli
}

/// Run `analyze content <dir>`: the content group over SKILL.md only.
pub fn analyze_content_cmd(dir: &str) -> Result<CliReport, String> {
    let skill = Skill::load(Path::new(dir))?;
    Ok(CliReport {
        skill_dir: dir.to_string(),
        content: Some(analyze_content(&skill.raw_content)),
        ..CliReport::default()
    })
}

/// The skill name used for contamination (the directory basename), matching the
/// Go tool's use of the skill identity for multi-interface-tool detection.
fn skill_name(dir: &Path) -> String {
    dir.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Analyse every readable file directly under `references/` (sorted, hidden and
/// nested entries skipped), mirroring the token counter's reference walk.
fn analyze_reference_files(dir: &Path) -> Vec<ReferenceReport> {
    let refs_dir = dir.join("references");
    let name = skill_name(dir);
    let mut reports = Vec::new();
    for entry in read_dir_sorted(&refs_dir) {
        if entry.is_dir || entry.name.starts_with('.') {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&entry.path) else {
            continue;
        };
        let content = analyze_content(&text);
        let contamination = analyze_contamination(&name, &text, &content.code_languages);
        reports.push(ReferenceReport {
            file: format!("references/{}", entry.name),
            content: Some(content),
            contamination: Some(contamination),
        });
    }
    reports
}
