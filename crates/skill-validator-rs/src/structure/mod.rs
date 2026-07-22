//! Structure validation: the main offline entry point (Go `structure`
//! package). [`validate`] runs the directory-layout check, then frontmatter,
//! token, markdown, internal-link, and orphan checks in the same order the Go
//! tool does. [`check_structure`] validates the directory layout itself.

pub mod frontmatter;
pub mod fsutil;
pub mod internal_links;
pub mod markdown;
pub mod orphans;
pub mod tokens;

use std::path::Path;

use crate::skill::Skill;
use crate::types::{Level, MultiReport, Report, ResultContext, ValidationResult};
use crate::util::plural_s;

pub use frontmatter::check_frontmatter;
pub use internal_links::check_internal_links;
pub use markdown::check_markdown;
pub use orphans::{check_flat_orphan_files, check_orphan_files};
pub use tokens::{check_skill_ratio, check_tokens, count_tokens};

/// Recognised subdirectories defined by the skill spec (Go `recognizedDirs`),
/// kept in a fixed sorted order so the deep-nesting walk is deterministic.
pub(crate) const RECOGNIZED_DIRS: &[&str] = &["assets", "references", "scripts"];

/// Files commonly found in repos but not intended for agent consumption
/// (Go `knownExtraneousFiles`, lower-cased keys).
const KNOWN_EXTRANEOUS_FILES: &[&str] = &[
    "readme.md",
    "readme",
    "changelog.md",
    "changelog",
    "license",
    "license.md",
    "license.txt",
    "contributing.md",
    "code_of_conduct.md",
    "installation_guide.md",
    "quick_reference.md",
    "makefile",
    ".gitignore",
    "owners.yaml",
    "owners",
];

/// Options configuring which checks [`validate`] runs (Go `structure.Options`).
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Skip the orphan-file reachability checks.
    pub skip_orphans: bool,
    /// Do not warn about unrecognised frontmatter fields.
    pub allow_extra_frontmatter: bool,
    /// Treat root-level text files as standard content (flat skills).
    pub allow_flat_layouts: bool,
    /// Directory names accepted without warning and exempt from nesting checks.
    pub allow_dirs: Vec<String>,
}

fn is_recognized_dir(name: &str) -> bool {
    RECOGNIZED_DIRS.contains(&name)
}

fn is_known_extraneous(lower: &str) -> bool {
    KNOWN_EXTRANEOUS_FILES.contains(&lower)
}

/// Validate the directory layout of a skill package (Go `CheckStructure`):
/// require SKILL.md, flag unrecognised directories and extraneous root files,
/// and warn about deep nesting in recognised directories.
pub fn check_structure(dir: &Path, opts: &Options) -> Vec<ValidationResult> {
    let ctx = ResultContext::new("Structure");
    let mut results: Vec<ValidationResult> = Vec::new();

    // SKILL.md must exist.
    if !dir.join("SKILL.md").exists() {
        results.push(ctx.error_file("SKILL.md", "SKILL.md not found"));
        return results;
    }
    results.push(ctx.pass_file("SKILL.md", "SKILL.md found"));

    for entry in fsutil::read_dir_sorted(dir) {
        if entry.name.starts_with('.') {
            continue; // skip hidden files/dirs
        }
        if !entry.is_dir {
            if entry.name != "SKILL.md" && !opts.allow_flat_layouts {
                results.push(extraneous_file_result(&ctx, &entry.name));
            }
            continue;
        }
        if !is_recognized_dir(&entry.name) && !opts.allow_dirs.iter().any(|d| d == &entry.name) {
            results.push(unknown_dir_result(&ctx, dir, &entry.name, &entry.path));
        }
    }

    // Deep-nesting checks for recognised directories only.
    for dir_name in RECOGNIZED_DIRS {
        let subdir = dir.join(dir_name);
        if !subdir.exists() {
            continue;
        }
        results.extend(check_nesting(&ctx, &subdir, dir_name));
    }

    results
}

fn unknown_dir_result(
    ctx: &ResultContext,
    dir: &Path,
    name: &str,
    path: &Path,
) -> ValidationResult {
    let file_count = fsutil::read_dir_sorted(path)
        .iter()
        .filter(|se| !se.name.starts_with('.'))
        .count();
    if file_count == 0 {
        return ctx.warn(format!("unknown directory: {name}/"));
    }
    let hint = unknown_dir_hint(dir);
    ctx.warn(format!(
        "unknown directory: {name}/ (contains {file_count} file{plural}) — agents using the \
         standard skill structure won't discover these files{hint}",
        plural = plural_s(file_count),
    ))
}

fn extraneous_file_result(ctx: &ResultContext, name: &str) -> ValidationResult {
    let lower = name.to_lowercase();
    if lower == "agents.md" {
        return ctx.warn_file(
            name,
            format!(
                "{name} is for repo-level agent configuration, not skill content — move it \
                 outside the skill directory (e.g. to the repository root) where agents \
                 discover it automatically"
            ),
        );
    }
    if is_known_extraneous(&lower) {
        return ctx.warn_file(
            name,
            format!(
                "{name} is not needed in a skill — agents may load it into their context window, \
                 taking space from your actual task (Anthropic best practices: skills should only \
                 contain files that directly support agent functionality)"
            ),
        );
    }
    ctx.warn_file(
        name,
        format!(
            "unexpected file at root: {name} — if agents need this file, move it into \
             references/ or assets/ as appropriate; otherwise remove it to avoid unnecessary \
             context window usage"
        ),
    )
}

fn unknown_dir_hint(dir: &Path) -> String {
    let mut candidates: Vec<&str> = Vec::new();
    if !dir.join("references").exists() {
        candidates.push("references/");
    }
    if !dir.join("assets").exists() {
        candidates.push("assets/");
    }
    if candidates.is_empty() {
        return String::new();
    }
    format!("; should this be {}?", candidates.join(" or "))
}

fn check_nesting(ctx: &ResultContext, dir: &Path, prefix: &str) -> Vec<ValidationResult> {
    let mut results = Vec::new();
    for entry in fsutil::read_dir_sorted(dir) {
        if entry.name.starts_with('.') {
            continue;
        }
        if entry.is_dir {
            results.push(ctx.warn(format!("deep nesting detected: {}/{}/", prefix, entry.name)));
        }
    }
    results
}

/// Run all structure checks against the skill in `dir` (Go `structure.Validate`).
pub fn validate(dir: &Path, opts: &Options) -> Report {
    let mut report = Report::new(&dir.to_string_lossy());

    let struct_results = check_structure(dir, opts);
    let has_skill_md = struct_results
        .iter()
        .any(|r| r.level == Level::Pass && r.message == "SKILL.md found");
    report.results.extend(struct_results);

    if !has_skill_md {
        report.tally();
        return report;
    }

    let skill = match Skill::load(dir) {
        Ok(s) => s,
        Err(e) => {
            let ctx = ResultContext::with_file("Frontmatter", "SKILL.md");
            report.results.push(ctx.error(e));
            report.tally();
            return report;
        }
    };

    report.results.extend(check_frontmatter(&skill, opts));

    let (token_results, token_counts, other_counts) = check_tokens(dir, &skill.body, opts);
    report.results.extend(token_results);
    report.token_counts = token_counts;
    report.other_token_counts = other_counts;

    // Holistic structure check: is this actually a skill?
    let ratio = check_skill_ratio(&report.token_counts, &report.other_token_counts);
    report.results.extend(ratio);

    report.results.extend(check_markdown(dir, &skill.body));
    report
        .results
        .extend(check_internal_links(dir, &skill.body));

    if !opts.skip_orphans {
        report
            .results
            .extend(check_orphan_files(dir, &skill.body, opts));
        if opts.allow_flat_layouts {
            report
                .results
                .extend(check_flat_orphan_files(dir, &skill.body));
        }
    }

    report.tally();
    report
}

/// Validate several skill directories and aggregate the reports
/// (Go `structure.ValidateMulti`).
pub fn validate_multi(dirs: &[String], opts: &Options) -> MultiReport {
    let mut mr = MultiReport::default();
    for dir in dirs {
        let report = validate(Path::new(dir), opts);
        mr.errors += report.errors;
        mr.warnings += report.warnings;
        mr.skills.push(report);
    }
    mr
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &Path, rel: &str, contents: &str) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn missing_skill_md_is_error() {
        let tmp = TempDir::new().unwrap();
        let results = check_structure(tmp.path(), &Options::default());
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].level, Level::Error);
        assert_eq!(results[0].message, "SKILL.md not found");
    }

    #[test]
    fn recognized_dirs_and_extraneous_root_file() {
        let tmp = TempDir::new().unwrap();
        write(tmp.path(), "SKILL.md", "# Skill\n");
        write(tmp.path(), "references/guide.md", "# Guide\n");
        write(tmp.path(), "README.md", "# Readme\n");
        write(tmp.path(), "weird/note.txt", "hi\n");

        let results = check_structure(tmp.path(), &Options::default());
        assert!(results
            .iter()
            .any(|r| r.level == Level::Pass && r.message == "SKILL.md found"));
        // README.md is a known extraneous file.
        assert!(results
            .iter()
            .any(|r| r.file == "README.md" && r.level == Level::Warning));
        // "weird" is an unknown directory containing a file.
        assert!(results.iter().any(
            |r| r.level == Level::Warning && r.message.starts_with("unknown directory: weird/")
        ));
        // references/ is recognised, so no warning for it.
        assert!(!results
            .iter()
            .any(|r| r.message.contains("unknown directory: references/")));
    }

    #[test]
    fn allow_dirs_suppresses_unknown_dir_warning() {
        let tmp = TempDir::new().unwrap();
        write(tmp.path(), "SKILL.md", "# Skill\n");
        write(tmp.path(), "evals/case.md", "# Case\n");

        let opts = Options {
            allow_dirs: vec!["evals".to_string()],
            ..Options::default()
        };
        let results = check_structure(tmp.path(), &opts);
        assert!(!results
            .iter()
            .any(|r| r.message.contains("unknown directory")));
    }

    #[test]
    fn validate_wires_frontmatter_and_tokens() {
        // The skill directory basename must satisfy the name pattern and match
        // the frontmatter `name`, so use a dedicated child directory.
        let tmp = TempDir::new().unwrap();
        let skill_dir = tmp.path().join("my-skill");
        write(
            &skill_dir,
            "SKILL.md",
            "---\nname: my-skill\ndescription: A tiny valid skill for testing.\n---\n# Body\n\nUse the tool.\n",
        );

        let report = validate(&skill_dir, &Options::default());
        assert_eq!(report.errors, 0, "results: {:?}", report.results);
        assert!(report
            .token_counts
            .iter()
            .any(|c| c.file == "SKILL.md body"));
        assert!(report
            .results
            .iter()
            .any(|r| r.category == "Frontmatter" && r.message.contains("description: (")));
    }
}
