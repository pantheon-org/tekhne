//! Tessl registry compliance checks for a single skill.
//!
//! Ports `tessl-compliance-check.sh`: three groups over the skill's own
//! Markdown files.
//! 1. Agent-agnostic: no `claude|cursor|openai|copilot|gemini|chatgpt`
//!    references outside `BAD` examples, and no agent-specific `allowed-tools`.
//! 2. Performance metrics: at least one metrics section AND at least one
//!    quantified claim.
//! 3. Cross-platform: platform-specific paths / package commands. This group is
//!    advisory only (warnings) and never fails the run, matching the shell.
//!
//! Overall compliance requires groups 1 and 2 to pass. The check reads only the
//! skill's own files, so it needs no bundled data.

use std::path::{Path, PathBuf};

use regex::Regex;

/// A Markdown file discovered under the skill directory.
struct MdFile {
    /// Path as displayed (the skill path joined with the relative path).
    display: String,
    content: String,
}

/// The result of a tessl-compliance run over one skill.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TesslReport {
    /// Files carrying agent-specific references outside BAD examples.
    pub agent_reference_files: Vec<String>,
    /// True if `allowed-tools` names an agent-specific tool.
    pub agent_specific_tools: bool,
    /// A metrics section (success metrics / outcomes / effectiveness / …) exists.
    pub has_metrics_section: bool,
    /// A quantified performance claim exists.
    pub has_quantified_claim: bool,
    /// Files with hard-coded platform-specific paths (warning only).
    pub platform_path_files: Vec<String>,
    /// Files with platform package commands lacking OS alternatives (warning).
    pub platform_command_files_without_alternatives: Vec<String>,
}

impl TesslReport {
    /// Agent-agnostic group passes when no offending references or tools exist.
    pub fn agent_agnostic_pass(&self) -> bool {
        self.agent_reference_files.is_empty() && !self.agent_specific_tools
    }

    /// Performance group passes when a metrics section and a quantified claim
    /// both exist.
    pub fn performance_pass(&self) -> bool {
        self.has_metrics_section && self.has_quantified_claim
    }

    /// Overall compliance: agent-agnostic and performance groups pass
    /// (cross-platform is advisory only).
    pub fn compliant(&self) -> bool {
        self.agent_agnostic_pass() && self.performance_pass()
    }
}

/// Compiled patterns shared across the checks.
struct Patterns {
    agent: Regex,
    bad_example: Regex,
    agent_tool: Regex,
    metrics: Regex,
    quantified: Regex,
    platform_path: Regex,
    platform_command: Regex,
    alternatives: Regex,
}

impl Patterns {
    fn new() -> Self {
        let re = |p: &str| Regex::new(p).expect("static regex is valid");
        Patterns {
            agent: re(r"(?i)claude|cursor|openai|copilot|gemini|chatgpt"),
            // A "BAD example" line: an `❌ … BAD` marker or a `BAD:` label.
            bad_example: re(r"❌.*BAD|BAD:"),
            agent_tool: re(r"(claude|cursor|openai|copilot)-"),
            metrics: re(r"(?i)success metrics|expected outcomes|effectiveness|performance"),
            quantified: re(
                r"[0-9]+(%|x|×|times|\s(seconds|minutes|hours)|reduction|improvement|faster|slower)",
            ),
            platform_path: re(r"/usr/local|C:\\|/home/[^/]+|~/\."),
            platform_command: re(r"brew install|apt install|yum install|choco install"),
            alternatives: re(r"macOS|Ubuntu|Windows|Linux"),
        }
    }
}

/// Run the tessl-compliance checks over the skill directory `skill_path`.
pub fn check(skill_path: &Path) -> TesslReport {
    let pats = Patterns::new();
    let files = collect_md_files(skill_path);
    let mut report = TesslReport::default();

    agent_agnostic(&files, skill_path, &pats, &mut report);
    performance(&files, &pats, &mut report);
    cross_platform(&files, &pats, &mut report);

    report
}

/// Recursively collect `*.md` files under `dir`, with display paths prefixed by
/// `dir` (mirroring the shell's `find "$SKILL_PATH"`).
fn collect_md_files(dir: &Path) -> Vec<MdFile> {
    let mut out = Vec::new();
    collect(dir, &mut out);
    out.sort_by(|a, b| a.display.cmp(&b.display));
    out
}

fn collect(dir: &Path, out: &mut Vec<MdFile>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        match entry.file_type() {
            Ok(t) if t.is_dir() => collect(&path, out),
            Ok(t) if t.is_file() && path.extension().is_some_and(|e| e == "md") => {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    out.push(MdFile {
                        display: path.to_string_lossy().replace('\\', "/"),
                        content,
                    });
                }
            }
            _ => {}
        }
    }
}

fn agent_agnostic(files: &[MdFile], skill_path: &Path, pats: &Patterns, report: &mut TesslReport) {
    for f in files {
        // Compliance docs teach agent-agnostic patterns using bad examples.
        if f.display.contains("tessl-compliance") {
            continue;
        }
        // Flag when an agent term appears on a line that is not a BAD example.
        let offends = f
            .content
            .lines()
            .filter(|l| pats.agent.is_match(l))
            .any(|l| !pats.bad_example.is_match(l));
        if offends {
            report.agent_reference_files.push(f.display.clone());
        }
    }

    // Agent-specific tools in SKILL.md `allowed-tools:`.
    let skill_md = skill_path.join("SKILL.md");
    if let Ok(content) = std::fs::read_to_string(skill_md) {
        report.agent_specific_tools = content
            .lines()
            .filter(|l| l.starts_with("allowed-tools:"))
            .any(|l| pats.agent_tool.is_match(l));
    }
}

fn performance(files: &[MdFile], pats: &Patterns, report: &mut TesslReport) {
    report.has_metrics_section = files.iter().any(|f| pats.metrics.is_match(&f.content));
    report.has_quantified_claim = files.iter().any(|f| pats.quantified.is_match(&f.content));
}

fn cross_platform(files: &[MdFile], pats: &Patterns, report: &mut TesslReport) {
    for f in files {
        if pats.platform_path.is_match(&f.content) {
            report.platform_path_files.push(f.display.clone());
        }
        if pats.platform_command.is_match(&f.content) && !pats.alternatives.is_match(&f.content) {
            report
                .platform_command_files_without_alternatives
                .push(f.display.clone());
        }
    }
}

/// Resolve `<skills_dir>/<skill_name>` (the shell's `SKILL_PATH`).
pub fn skill_path(skills_dir: &str, skill_name: &str) -> PathBuf {
    Path::new(skills_dir).join(skill_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn skill_with(files: &[(&str, &str)]) -> tempfile::TempDir {
        let tmp = tempdir().unwrap();
        for (rel, body) in files {
            let p = tmp.path().join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, body).unwrap();
        }
        tmp
    }

    #[test]
    fn clean_skill_is_compliant() {
        let tmp = skill_with(&[(
            "SKILL.md",
            "# Skill\n\n## Success Metrics\n\nReduces review time by 40% for teams.\n",
        )]);
        let r = check(tmp.path());
        assert!(r.agent_agnostic_pass());
        assert!(r.performance_pass());
        assert!(r.compliant());
    }

    #[test]
    fn agent_reference_outside_bad_example_fails() {
        let tmp = skill_with(&[(
            "SKILL.md",
            "# Skill\n\nUse Claude to do the thing.\n\n## Performance\n\n50% faster.\n",
        )]);
        let r = check(tmp.path());
        assert!(!r.agent_agnostic_pass());
        assert_eq!(r.agent_reference_files.len(), 1);
        assert!(!r.compliant());
    }

    #[test]
    fn agent_reference_only_in_bad_example_passes() {
        let tmp = skill_with(&[(
            "SKILL.md",
            "# Skill\n\n❌ BAD: mentions Claude directly\n\n## Effectiveness\n\n2x improvement.\n",
        )]);
        let r = check(tmp.path());
        assert!(r.agent_agnostic_pass(), "{:?}", r.agent_reference_files);
    }

    #[test]
    fn tessl_compliance_doc_is_skipped() {
        let tmp = skill_with(&[
            (
                "SKILL.md",
                "# Skill\n\n## Metrics\n\nperformance improved 30%.\n",
            ),
            (
                "references/tessl-compliance-framework.md",
                "Avoid naming Cursor or Copilot in prose.\n",
            ),
        ]);
        let r = check(tmp.path());
        assert!(r.agent_agnostic_pass());
    }

    #[test]
    fn missing_metrics_fails_performance() {
        let tmp = skill_with(&[("SKILL.md", "# Skill\n\nJust prose, no numbers.\n")]);
        let r = check(tmp.path());
        assert!(!r.performance_pass());
        assert!(!r.has_metrics_section);
        assert!(!r.has_quantified_claim);
    }

    #[test]
    fn agent_specific_allowed_tools_fails() {
        let tmp = skill_with(&[(
            "SKILL.md",
            "---\nallowed-tools: claude-search, other\n---\n# Skill\n\n## Performance\n\n10% faster.\n",
        )]);
        let r = check(tmp.path());
        assert!(r.agent_specific_tools);
        assert!(!r.agent_agnostic_pass());
    }

    #[test]
    fn platform_paths_warn_but_do_not_fail() {
        let tmp = skill_with(&[(
            "SKILL.md",
            "# Skill\n\nInstall to /usr/local/bin.\n\n## Metrics\n\n25% reduction.\n",
        )]);
        let r = check(tmp.path());
        assert_eq!(r.platform_path_files.len(), 1);
        // Cross-platform is advisory: overall compliance is unaffected.
        assert!(r.compliant());
    }
}
