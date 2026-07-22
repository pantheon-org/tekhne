//! D4 — Specification Compliance (max 17). Ported from `d4_specification.go`.

use super::agents::{find_agent_ref, find_harness_path};
use super::bridge::ValidatorBridge;
use super::helpers::{extract_frontmatter_field, remove_code_blocks};
use super::{warn_diag, Diagnostic};
use regex::Regex;
use std::path::Path;

/// Score the Specification Compliance dimension.
pub fn score(content: &str, skill_dir: &Path, b: &ValidatorBridge) -> (i32, Vec<Diagnostic>) {
    let mut score = 8;
    let mut diags = Vec::new();

    // Description length: prefer library result; fall back to custom parse.
    let mut desc_len = b.description_len();
    if desc_len < 0 {
        desc_len = extract_frontmatter_field(content, "description").len() as i32;
    }
    if desc_len > 100 {
        score += 2;
    }
    if desc_len > 200 {
        score += 1;
    }

    // Keyword-stuffing proxy via and/or count.
    let description = extract_frontmatter_field(content, "description");
    let and_or_re = Regex::new(r"(?i) and | or ").expect("valid regex");
    let and_or_count = and_or_re.find_iter(&description).count();
    if and_or_count > 3 {
        score -= 2;
    } else if and_or_count > 1 {
        score -= 1;
    }

    // Harness-specific paths.
    let harness = find_harness_path(content);
    if !harness.is_empty() {
        diags.push(warn_diag(
            "D4",
            &format!("harness-specific path found: {harness}"),
        ));
    } else {
        score += 1;
    }

    // Agent name references.
    let agent = find_agent_ref(content);
    if !agent.is_empty() {
        diags.push(warn_diag(
            "D4",
            &format!("agent-specific reference found: {agent}"),
        ));
    } else {
        score += 1;
    }

    let rel_path_re =
        Regex::new(r"(scripts|references|assets)/[a-zA-Z0-9_-]+").expect("valid regex");
    if rel_path_re.is_match(content) {
        score += 1;
    }

    let non_code = remove_code_blocks(content);

    // `../` violations: use library internal-link check as primary signal.
    if b.has_internal_link_warning() || non_code.contains("../") {
        score -= 2;
        diags.push(warn_diag(
            "D4",
            "../ reference outside code blocks (self-containment violation)",
        ));
    }

    let abs_path_re =
        Regex::new(r"skills/[a-z][a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+").expect("valid regex");
    if let Some(m) = abs_path_re.find(&non_code) {
        score -= 1;
        diags.push(warn_diag(
            "D4",
            &format!("absolute skill path outside code blocks: {}", m.as_str()),
        ));
    }

    let ctx_agents_re = Regex::new(r"\.(context|agents)/").expect("valid regex");
    if let Some(m) = ctx_agents_re.find(&non_code) {
        score -= 1;
        diags.push(warn_diag(
            "D4",
            &format!(
                ".context/ or .agents/ reference outside code blocks: {}",
                m.as_str()
            ),
        ));
    }

    let scripts_dir = skill_dir.join("scripts");
    let references_dir = skill_dir.join("references");
    score -= penalty_from_dir(&scripts_dir, &abs_path_re);
    score -= penalty_from_dir(&scripts_dir, &ctx_agents_re);
    score -= penalty_from_dir(&references_dir, &abs_path_re);
    score -= penalty_from_dir(&references_dir, &ctx_agents_re);

    score = score.clamp(0, 15);

    let mut bonus = 0;
    if scripts_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&scripts_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.ends_with(".py") || name.ends_with(".ts") || name.ends_with(".js") {
                    bonus += 1;
                    break;
                }
            }
        }
    }

    let mut last_h2 = "";
    for line in content.split('\n') {
        if let Some(rest) = line.strip_prefix("## ") {
            last_h2 = rest;
        }
    }
    let bullet_link_re = Regex::new(r"(?m)^- \[.+\]\(.+\)").expect("valid regex");
    if last_h2.trim() == "References" && bullet_link_re.is_match(content) {
        bonus += 1;
    }

    score += bonus;
    if score > 17 {
        score = 17;
    }
    (score, diags)
}

/// Number of files in `dir` matching `re`, capped at 2 (Go `penaltyFromDir`).
fn penalty_from_dir(dir: &Path, re: &Regex) -> i32 {
    if !dir.is_dir() {
        return 0;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut penalty = 0;
    for entry in entries.flatten() {
        if entry.path().is_dir() {
            continue;
        }
        let Ok(data) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        if re.is_match(&data) {
            penalty += 1;
            if penalty >= 2 {
                break;
            }
        }
    }
    penalty
}

#[cfg(test)]
mod tests {
    use super::super::bridge::ValidatorBridge;
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn nil_bridge() -> ValidatorBridge {
        ValidatorBridge::default()
    }

    #[test]
    fn good_description() {
        let desc = "Validates and sanitizes user inputs before processing to prevent injection attacks and data corruption in production systems";
        let content =
            format!("---\ndescription: {desc}\n---\n# Skill\nSee references/guide.md for details.");
        let (score, _) = score(&content, tempdir().unwrap().path(), &nil_bridge());
        assert!(score >= 10);
    }

    #[test]
    fn harness_path_warning() {
        let cases = [
            ("claude", "See .claude/settings.json for config."),
            ("cursor", "Edit .cursor/rules here."),
            ("continue", "Config at .continue/config.json."),
            ("windsurf", "See .windsurf/settings."),
            ("goose", "Config in .goose/profile."),
            ("agents", "Place files under .agents/skills/."),
            ("copilot", "Config at .copilot/instructions."),
            ("gemini", "See .gemini/settings."),
            ("firebender", "Edit .firebender/config."),
        ];
        for (name, snippet) in cases {
            let content =
                format!("---\ndescription: does something useful\n---\n# Skill\n{snippet}");
            let (_, diags) = score(&content, tempdir().unwrap().path(), &nil_bridge());
            assert!(
                diags
                    .iter()
                    .any(|d| d.dimension == "D4" && d.severity == "warning"),
                "expected harness warning for .{name}/"
            );
        }
    }

    #[test]
    fn agent_ref_warning() {
        let cases = [
            "This works with Claude Code.",
            "Use cursor agent for this.",
            "Requires GitHub Copilot.",
            "Works with opencode.",
            "Tested on Windsurf.",
            "Run via Gemini CLI.",
            "Compatible with Goose.",
            "Use with Codex.",
            "Requires Cline.",
            "Run aider to apply.",
        ];
        for snippet in cases {
            let content =
                format!("---\ndescription: does something useful\n---\n# Skill\n{snippet}");
            let (_, diags) = score(&content, tempdir().unwrap().path(), &nil_bridge());
            assert!(
                diags
                    .iter()
                    .any(|d| d.dimension == "D4" && d.severity == "warning"),
                "expected agent warning for {snippet:?}"
            );
        }
    }

    #[test]
    fn relative_path_violation() {
        let content = "---\ndescription: does something useful\n---\n# Skill\nSee ../other-skill/SKILL.md for more.";
        let (score, diags) = score(content, tempdir().unwrap().path(), &nil_bridge());
        assert!(diags
            .iter()
            .any(|d| d.dimension == "D4" && d.severity == "warning"));
        assert!(score <= 11);
    }

    #[test]
    fn penalty_from_dir_abs_path() {
        let dir = tempdir().unwrap();
        let scripts = dir.path().join("scripts");
        fs::create_dir_all(&scripts).unwrap();
        fs::write(
            scripts.join("run.sh"),
            "#!/bin/sh\ncd skills/ci-cd/my-skill && run",
        )
        .unwrap();
        let content = "---\ndescription: does something useful\n---\n# Skill\ncontent here";
        let (score1, _) = score(content, tempdir().unwrap().path(), &nil_bridge());
        let (score2, _) = score(content, dir.path(), &nil_bridge());
        assert!(score2 < score1);
    }
}
