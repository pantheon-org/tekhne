//! Skill artifact-convention checks, ported from the shell
//! `validate-skill-artifacts.sh`.
//!
//! Two entry points mirror the script's two modes:
//! - [`check_files`]: validate an explicit list of changed files (the
//!   `hk` pre-commit invocation, `validate artifacts <files...>`).
//! - [`check_tree`]: scan the whole `skills/` tree (the no-argument mode).
//!
//! Every finding is an error (the shell only ever raised errors), category
//! `Artifacts`. YAML and JSON validity are checked in-process with
//! `serde_yaml_ng` / `serde_json` rather than shelling out to `yq` / `jq`; the
//! optional `sh -n` and `py_compile` syntax checks are still delegated to those
//! tools when present, matching the shell's `command -v` gating.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::types::{Level, ValidationResult};

const CATEGORY: &str = "Artifacts";

/// Allowed immediate subdirectories of a skill directory.
const ALLOWED_SKILL_DIRS: &[&str] = &[
    "scripts",
    "references",
    "assets",
    "evals",
    "schemas",
    "templates",
];

/// Allowed immediate subdirectories of a skill's `assets/` directory.
const ALLOWED_ASSETS_DIRS: &[&str] = &["templates", "schemas", "requirements", "examples"];

/// The result of an artifact-convention run.
#[derive(Debug, Default)]
pub struct ArtifactReport {
    /// All findings, in emission order.
    pub results: Vec<ValidationResult>,
}

impl ArtifactReport {
    fn error(&mut self, file: &str, message: String) {
        self.results.push(ValidationResult {
            level: Level::Error,
            category: CATEGORY.to_string(),
            message,
            file: file.to_string(),
            line: 0,
        });
    }

    /// Number of error-level findings.
    pub fn errors(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.level == Level::Error)
            .count()
    }
}

/// Validate an explicit list of files (hook mode). For each file the relevant
/// per-file check runs; when a file is a `skills/<domain>/<skill>/SKILL.md`, the
/// enclosing skill directory is also validated.
pub fn check_files(files: &[PathBuf]) -> ArtifactReport {
    let mut report = ArtifactReport::default();
    for file in files {
        if !file.is_file() {
            continue;
        }
        let rel = file.to_string_lossy().replace('\\', "/");
        check_file(&rel, file, &mut report);
        if is_skill_md(&rel) {
            if let Some(dir) = file.parent() {
                check_skill_dir(dir, &mut report);
            }
        }
    }
    report
}

/// Scan the whole `skills/` tree under `root` (no-argument mode): every
/// template/schema/script file is checked, and every `skills/<domain>/<skill>`
/// directory containing a SKILL.md is validated.
pub fn check_tree(root: &Path) -> ArtifactReport {
    let mut report = ArtifactReport::default();

    let mut files = Vec::new();
    collect_files(root, &mut files);
    for file in &files {
        let rel = file.to_string_lossy().replace('\\', "/");
        if rel.contains("/assets/templates/")
            || rel.contains("/assets/schemas/")
            || rel.contains("/scripts/")
        {
            check_file(&rel, file, &mut report);
        }
    }

    // skills/<domain>/<skill> directories (exactly two levels under the root).
    if let Ok(domains) = std::fs::read_dir(root) {
        for domain in domains.flatten().filter(is_dir) {
            if let Ok(skills) = std::fs::read_dir(domain.path()) {
                for skill in skills.flatten().filter(is_dir) {
                    if skill.path().join("SKILL.md").is_file() {
                        check_skill_dir(&skill.path(), &mut report);
                    }
                }
            }
        }
    }

    report
}

fn is_dir(entry: &std::fs::DirEntry) -> bool {
    entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        match entry.file_type() {
            Ok(t) if t.is_dir() => collect_files(&path, out),
            Ok(t) if t.is_file() => out.push(path),
            _ => {}
        }
    }
}

/// A `skills/<domain>/<skill>/SKILL.md` path (exactly four components).
fn is_skill_md(rel: &str) -> bool {
    let parts: Vec<&str> = rel.split('/').collect();
    parts.len() == 4 && parts[0] == "skills" && parts[3] == "SKILL.md"
}

/// Dispatch a single file to its convention check by location.
fn check_file(rel: &str, path: &Path, report: &mut ArtifactReport) {
    if rel.contains("/assets/templates/") {
        check_templates_file(rel, path, report);
    } else if rel.contains("/assets/schemas/") {
        check_schemas_file(rel, path, report);
    } else if rel.contains("/scripts/") {
        check_scripts_file(rel, path, report);
    }
}

fn check_templates_file(rel: &str, path: &Path, report: &mut ArtifactReport) {
    if rel.ends_with("/.gitkeep") {
        return;
    }
    if !(rel.ends_with(".yaml") || rel.ends_with(".yml")) {
        return;
    }
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    // Skip files that carry templating placeholders (they are not valid YAML by
    // design): `{{ ... }}`, `{% ... %}`, or `[snake_case]` tokens.
    if text.contains("{{") || text.contains("{%") || has_bracket_token(&text) {
        return;
    }
    if serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&text).is_err() {
        report.error(rel, format!("{rel} is not valid YAML."));
    }
}

/// True when the text contains a `[lowercase_or_underscore]` placeholder token.
fn has_bracket_token(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            let mut j = i + 1;
            while j < bytes.len() && (bytes[j].is_ascii_lowercase() || bytes[j] == b'_') {
                j += 1;
            }
            if j > i + 1 && j < bytes.len() && bytes[j] == b']' {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn check_schemas_file(rel: &str, path: &Path, report: &mut ArtifactReport) {
    if !rel.ends_with(".schema.json") {
        report.error(
            rel,
            format!("{rel} is in assets/schemas/ but does not use the .schema.json extension."),
        );
        return;
    }
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    if serde_json::from_str::<serde_json::Value>(&text).is_err() {
        report.error(rel, format!("{rel} is not valid JSON."));
        return;
    }
    if !declares_schema_url(&text) {
        report.error(
            rel,
            format!(r#"{rel} does not declare a JSON Schema "$schema" URL from json-schema.org."#),
        );
    }
}

/// True when the JSON text declares `"$schema": "http(s)://json-schema.org/..."`.
fn declares_schema_url(text: &str) -> bool {
    let Some(idx) = text.find("\"$schema\"") else {
        return false;
    };
    let rest = &text[idx + "\"$schema\"".len()..];
    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix(':') else {
        return false;
    };
    let rest = rest.trim_start();
    rest.starts_with("\"http://json-schema.org/") || rest.starts_with("\"https://json-schema.org/")
}

fn check_scripts_file(rel: &str, path: &Path, report: &mut ArtifactReport) {
    if rel.contains("/__pycache__/") || rel.ends_with("/.gitkeep") {
        return;
    }
    if rel.ends_with(".sh") {
        check_shell_script(rel, path, report);
    } else if rel.ends_with(".py") {
        check_python_script(rel, path, report);
    } else if rel.ends_with(".ts") {
        check_ts_script(rel, path, report);
    } else if rel.ends_with(".js") {
        check_js_script(rel, path, report);
    } else {
        report.error(
            rel,
            format!(
                "{rel} is in scripts/ but is not a recognised script type (.sh, .py, .ts, .js)."
            ),
        );
    }
}

fn first_line(path: &Path) -> String {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|t| t.lines().next().map(str::to_string))
        .unwrap_or_default()
}

fn check_shell_script(rel: &str, path: &Path, report: &mut ArtifactReport) {
    let shebang = first_line(path);
    let portable_msg = format!(
        "{rel} must start with portable shebang: #!/usr/bin/env sh (or add '# shell: bash' to allow bash)"
    );
    if shebang == "#!/usr/bin/env sh" {
        // Best-effort POSIX syntax check when `sh` is available.
        if let Some(false) = run_syntax_check("sh", &["-n"], path) {
            report.error(
                rel,
                format!("{rel} failed POSIX shell syntax check (sh -n)."),
            );
        }
    } else if shebang == "#!/usr/bin/env bash" {
        let allows_bash = std::fs::read_to_string(path)
            .map(|t| t.lines().any(|l| l.starts_with("# shell: bash")))
            .unwrap_or(false);
        if !allows_bash {
            report.error(rel, portable_msg);
        }
    } else {
        report.error(rel, portable_msg);
    }
}

fn check_python_script(rel: &str, path: &Path, report: &mut ArtifactReport) {
    let shebang = first_line(path);
    if shebang != "#!/usr/bin/env python3" && shebang != "#!/usr/bin/env python" {
        report.error(
            rel,
            format!("{rel} must start with shebang: #!/usr/bin/env python3"),
        );
        return;
    }
    if let Some(false) = run_python_syntax_check(path) {
        report.error(rel, format!("{rel} failed Python syntax check."));
    }
}

fn check_ts_script(rel: &str, path: &Path, report: &mut ArtifactReport) {
    let shebang = first_line(path);
    let ok = matches!(
        shebang.as_str(),
        "#!/usr/bin/env bun" | "#!/usr/bin/env -S bun" | "#!/usr/bin/env -S bun run"
    );
    if !ok {
        report.error(
            rel,
            format!("{rel} must start with shebang: #!/usr/bin/env bun"),
        );
    }
}

fn check_js_script(rel: &str, path: &Path, report: &mut ArtifactReport) {
    let shebang = first_line(path);
    if shebang != "#!/usr/bin/env node" && shebang != "#!/usr/bin/env bun" {
        report.error(
            rel,
            format!("{rel} must start with shebang: #!/usr/bin/env node (or #!/usr/bin/env bun)"),
        );
    }
}

/// Run `<tool> <args...> <path>` when `tool` resolves, returning `Some(true)` on
/// success, `Some(false)` on failure, or `None` when the tool is unavailable
/// (mirrors the shell's `command -v` gating so absent tools never fail a run).
fn run_syntax_check(tool: &str, args: &[&str], path: &Path) -> Option<bool> {
    let output = Command::new(tool).args(args).arg(path).output().ok()?;
    Some(output.status.success())
}

/// Best-effort Python syntax check via `ast.parse`. Unlike the shell's
/// `py_compile`, this writes no bytecode, so it never litters `__pycache__/`
/// directories into the tree. Returns `None` when `python3` is unavailable.
fn run_python_syntax_check(path: &Path) -> Option<bool> {
    let script = "import ast,sys; ast.parse(open(sys.argv[1]).read())";
    let output = Command::new("python3")
        .args(["-c", script])
        .arg(path)
        .output()
        .ok()?;
    Some(output.status.success())
}

/// Validate a single skill directory's structure, SKILL.md size, name match and
/// self-containment.
fn check_skill_dir(dir: &Path, report: &mut ArtifactReport) {
    let dir_rel = dir.to_string_lossy().replace('\\', "/");
    let skill_name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

    check_subdirs(dir, &dir_rel, report);
    check_assets(dir, &dir_rel, report);
    check_skill_md(dir, &dir_rel, &skill_name, report);
}

/// Immediate subdirectories of the skill dir must be in the allowed set.
fn check_subdirs(dir: &Path, dir_rel: &str, report: &mut ArtifactReport) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten().filter(is_dir) {
        let name = entry.file_name().to_string_lossy().into_owned();
        // Dot-directories (e.g. `.tessl-plugin`) are skipped: the shell's `*/`
        // glob does not match hidden entries, so they were never flagged.
        if name.starts_with('.') {
            continue;
        }
        if !ALLOWED_SKILL_DIRS.contains(&name.as_str()) {
            report.error(
                dir_rel,
                format!(
                    "{dir_rel}: non-standard directory '{name}' (allowed: scripts/, references/, assets/, evals/, schemas/, templates/)"
                ),
            );
        }
    }
}

/// Validate `assets/` subdirectory names and reject loose YAML placed directly
/// in `assets/` (it must live under `assets/templates/`).
fn check_assets(dir: &Path, dir_rel: &str, report: &mut ArtifactReport) {
    let assets = dir.join("assets");
    let Ok(entries) = std::fs::read_dir(&assets) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        // Hidden entries are skipped by the shell's `*/` and `*.yaml` globs.
        if name.starts_with('.') {
            continue;
        }
        if is_dir(&entry) {
            if !ALLOWED_ASSETS_DIRS.contains(&name.as_str()) {
                report.error(
                    dir_rel,
                    format!(
                        "{dir_rel}: non-standard assets/ subdirectory '{name}' (allowed: templates/, schemas/, requirements/, examples/)"
                    ),
                );
            }
        } else if name.ends_with(".yaml") || name.ends_with(".yml") {
            let f = format!("{dir_rel}/assets/{name}");
            report.error(
                &f,
                format!(
                    "{f}: YAML files must be placed in assets/templates/, not directly in assets/"
                ),
            );
        }
    }
}

/// Validate SKILL.md size, frontmatter name match, and self-containment.
fn check_skill_md(dir: &Path, dir_rel: &str, skill_name: &str, report: &mut ArtifactReport) {
    let Ok(content) = std::fs::read_to_string(dir.join("SKILL.md")) else {
        return;
    };
    let md_rel = format!("{dir_rel}/SKILL.md");

    let line_count = content.lines().count();
    if line_count > 500 {
        report.error(
            &md_rel,
            format!(
                "{md_rel}: {line_count} lines exceeds 500-line limit - move detail to references/"
            ),
        );
    }

    if let Some(name_val) = frontmatter_name(&content) {
        if !name_val.is_empty() && name_val != skill_name {
            report.error(
                &md_rel,
                format!(
                    "{md_rel}: frontmatter name '{name_val}' does not match directory name '{skill_name}'"
                ),
            );
        }
    }

    if contains_parent_ref_outside_code(&content) {
        report.error(
            &md_rel,
            format!("{md_rel}: contains '../' path reference outside code blocks (skills must be self-contained)"),
        );
    }
}

/// Extract the first `name:` frontmatter value, stripped of surrounding quotes
/// and whitespace (ports the shell `grep | sed | tr` pipeline).
fn frontmatter_name(content: &str) -> Option<String> {
    let line = content.lines().find(|l| l.starts_with("name:"))?;
    let value = line["name:".len()..].trim();
    let cleaned: String = value
        .chars()
        .filter(|c| *c != '"' && *c != '\'' && *c != ' ')
        .collect();
    Some(cleaned)
}

/// True when `../` appears outside fenced code blocks. Lines from a ```` ``` ````
/// fence to the next fence (inclusive) are removed first, matching the shell's
/// `sed '/^```/,/^```/d'`.
fn contains_parent_ref_outside_code(content: &str) -> bool {
    let mut in_fence = false;
    for line in content.lines() {
        if line.starts_with("```") {
            // Both the opening and closing fence lines are dropped.
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if line.contains("../") {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    #[test]
    fn schema_extension_and_url_enforced() {
        let dir = tempdir().unwrap();
        let bad_ext = dir.path().join("skills/d/s/assets/schemas/thing.json");
        write(
            &bad_ext,
            r#"{"$schema":"https://json-schema.org/draft/2020-12/schema"}"#,
        );
        let mut report = ArtifactReport::default();
        check_schemas_file(
            "skills/d/s/assets/schemas/thing.json",
            &bad_ext,
            &mut report,
        );
        assert_eq!(report.errors(), 1);
        assert!(report.results[0].message.contains(".schema.json extension"));
    }

    #[test]
    fn schema_missing_url_flagged() {
        let dir = tempdir().unwrap();
        let f = dir.path().join("x.schema.json");
        write(&f, r#"{"type":"object"}"#);
        let mut report = ArtifactReport::default();
        check_schemas_file("skills/d/s/assets/schemas/x.schema.json", &f, &mut report);
        assert_eq!(report.errors(), 1);
        assert!(report.results[0].message.contains("$schema"));
    }

    #[test]
    fn valid_schema_passes() {
        let dir = tempdir().unwrap();
        let f = dir.path().join("x.schema.json");
        write(
            &f,
            r#"{"$schema": "https://json-schema.org/draft-07/schema#","type":"object"}"#,
        );
        let mut report = ArtifactReport::default();
        check_schemas_file("skills/d/s/assets/schemas/x.schema.json", &f, &mut report);
        assert_eq!(report.errors(), 0);
    }

    #[test]
    fn unknown_script_type_flagged() {
        let dir = tempdir().unwrap();
        let f = dir.path().join("tool.rb");
        write(&f, "puts 1\n");
        let mut report = ArtifactReport::default();
        check_scripts_file("skills/d/s/scripts/tool.rb", &f, &mut report);
        assert_eq!(report.errors(), 1);
        assert!(report.results[0].message.contains("recognised script type"));
    }

    #[test]
    fn bad_shell_shebang_flagged() {
        let dir = tempdir().unwrap();
        let f = dir.path().join("go.sh");
        write(&f, "#!/bin/bash\necho hi\n");
        let mut report = ArtifactReport::default();
        check_scripts_file("skills/d/s/scripts/go.sh", &f, &mut report);
        assert_eq!(report.errors(), 1);
        assert!(report.results[0].message.contains("portable shebang"));
    }

    #[test]
    fn bash_shebang_with_marker_allowed() {
        let dir = tempdir().unwrap();
        let f = dir.path().join("go.sh");
        write(&f, "#!/usr/bin/env bash\n# shell: bash\necho hi\n");
        let mut report = ArtifactReport::default();
        check_scripts_file("skills/d/s/scripts/go.sh", &f, &mut report);
        assert_eq!(report.errors(), 0);
    }

    #[test]
    fn template_placeholder_yaml_skipped() {
        let dir = tempdir().unwrap();
        let f = dir.path().join("tmpl.yaml");
        write(&f, "name: {{ value }}\n:::not valid yaml:::\n");
        let mut report = ArtifactReport::default();
        check_templates_file("skills/d/s/assets/templates/tmpl.yaml", &f, &mut report);
        assert_eq!(report.errors(), 0);
    }

    #[test]
    fn invalid_template_yaml_flagged() {
        let dir = tempdir().unwrap();
        let f = dir.path().join("tmpl.yaml");
        write(&f, "key: value\n\tbad: indent\n  - nope\n");
        let mut report = ArtifactReport::default();
        check_templates_file("skills/d/s/assets/templates/tmpl.yaml", &f, &mut report);
        assert_eq!(report.errors(), 1);
    }

    #[test]
    fn non_standard_skill_subdir_flagged() {
        let dir = tempdir().unwrap();
        let skill = dir.path().join("skills/d/my-skill");
        write(&skill.join("SKILL.md"), "---\nname: my-skill\n---\nBody\n");
        fs::create_dir_all(skill.join("weird")).unwrap();
        let mut report = ArtifactReport::default();
        check_skill_dir(&skill, &mut report);
        assert_eq!(report.errors(), 1);
        assert!(report.results[0]
            .message
            .contains("non-standard directory 'weird'"));
    }

    #[test]
    fn dot_directories_are_not_flagged() {
        // `.tessl-plugin` (and any hidden dir) is skipped, matching the shell's
        // `*/` glob which never matched dot-directories.
        let dir = tempdir().unwrap();
        let skill = dir.path().join("skills/d/my-skill");
        write(&skill.join("SKILL.md"), "---\nname: my-skill\n---\nBody\n");
        fs::create_dir_all(skill.join(".tessl-plugin")).unwrap();
        write(&skill.join(".tessl-plugin/plugin.json"), "{}\n");
        let mut report = ArtifactReport::default();
        check_skill_dir(&skill, &mut report);
        assert_eq!(report.errors(), 0, "{:?}", report.results);
    }

    #[test]
    fn name_mismatch_flagged() {
        let dir = tempdir().unwrap();
        let skill = dir.path().join("skills/d/my-skill");
        write(
            &skill.join("SKILL.md"),
            "---\nname: wrong-name\n---\nBody\n",
        );
        let mut report = ArtifactReport::default();
        check_skill_dir(&skill, &mut report);
        assert_eq!(report.errors(), 1);
        assert!(report.results[0]
            .message
            .contains("does not match directory name"));
    }

    #[test]
    fn parent_ref_outside_code_flagged() {
        let content = "---\nname: s\n---\nSee ../other/file for details.\n";
        assert!(contains_parent_ref_outside_code(content));
    }

    #[test]
    fn parent_ref_inside_code_ignored() {
        let content = "---\nname: s\n---\n```sh\ncat ../foo\n```\nClean prose.\n";
        assert!(!contains_parent_ref_outside_code(content));
    }

    #[test]
    fn loose_yaml_in_assets_flagged() {
        let dir = tempdir().unwrap();
        let skill = dir.path().join("skills/d/my-skill");
        write(&skill.join("SKILL.md"), "---\nname: my-skill\n---\nBody\n");
        write(&skill.join("assets/loose.yaml"), "key: value\n");
        let mut report = ArtifactReport::default();
        check_skill_dir(&skill, &mut report);
        assert!(report
            .results
            .iter()
            .any(|r| r.message.contains("must be placed in assets/templates/")));
    }
}
