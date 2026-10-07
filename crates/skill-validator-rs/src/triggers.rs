//! Trigger-phrase drift: which trigger phrases a skill's `description` carried
//! at a base commit and no longer carries now.
//!
//! A rewrite can silently drop a phrase that auto-invocation relied on while
//! every quality score stays flat. This compares the base and head listing
//! text and names each phrase that vanished. It is advisory: a drop is often a
//! deliberate consolidation of near-synonyms, so the caller never fails on it.
//!
//! Tekhne descriptions have no quoted-phrase convention. Triggers are the
//! comma-separated clauses after a `Use when` marker and the lists after a
//! `Keywords:` or `Triggers:` marker, so those are the phrases. A base phrase counts as
//! preserved when its normalised text still appears anywhere in the head
//! listing text, which keeps a reflowed or reordered clause from being
//! reported as a drop.

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;
use serde::Serialize;
use serde_yaml_ng::Value;

use crate::skill::Skill;

/// Phrases shorter than this (after normalising) are noise.
const MIN_PHRASE_CHARS: usize = 3;

/// The drift report for one skill.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TriggerReport {
    /// The head skill directory, as supplied.
    pub skill_dir: String,
    /// True when there is no base `SKILL.md` to compare against (a new skill).
    pub new_skill: bool,
    /// Trigger phrases found at the base.
    pub base_phrases: Vec<String>,
    /// Trigger phrases found at the head.
    pub head_phrases: Vec<String>,
    /// Base phrases that no longer appear in the head listing text.
    pub dropped: Vec<String>,
}

/// Compare `head_dir` with `base_dir`. A base directory without a readable
/// `SKILL.md` makes the skill new, which reports no drift.
pub fn compare(head_dir: &Path, base_dir: &Path) -> Result<TriggerReport, String> {
    let head = Skill::load(head_dir)?;
    let head_text = listing_text(&head);
    let mut report = TriggerReport {
        skill_dir: head_dir.to_string_lossy().into_owned(),
        head_phrases: phrases(&head_text),
        ..TriggerReport::default()
    };

    let base = match Skill::load(base_dir) {
        Ok(b) => b,
        Err(_) => {
            report.new_skill = true;
            return Ok(report);
        }
    };
    let base_text = listing_text(&base);
    report.base_phrases = phrases(&base_text);
    report.dropped = dropped(&report.base_phrases, &head_text);
    Ok(report)
}

/// Base phrases whose normalised text is absent from the head listing text.
pub fn dropped(base_phrases: &[String], head_text: &str) -> Vec<String> {
    let head = normalize(head_text);
    base_phrases
        .iter()
        .filter(|p| !head.contains(p.as_str()))
        .cloned()
        .collect()
}

/// The text Claude Code lists for a skill: `description` plus `when_to_use`.
pub fn listing_text(skill: &Skill) -> String {
    let mut text = skill.frontmatter.description.clone();
    if let Some(Value::String(when)) = skill.raw_frontmatter.get("when_to_use") {
        text.push(' ');
        text.push_str(when);
    }
    text
}

fn marker_use_when() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\buse\s+(?:this\s+skill\s+)?(?:proactively\s+)?when\b:?").unwrap()
    })
}

/// Clause boundaries: commas, semicolons, newlines, and a period that ends a
/// sentence (so `Commander.js` and `biome.json` stay whole).
fn clause_split() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)[,;\n]|\.(?:\s|$)|\b(?:keywords|triggers?)\s*:").unwrap())
}

fn marker_keywords() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)\bkeywords\s*:").unwrap())
}

/// `Triggers:` or `Trigger:`, the third marker tekhne descriptions use.
fn marker_triggers() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)\btriggers?\s*:").unwrap())
}

/// Extract the sorted, de-duplicated, normalised trigger phrases from listing
/// text. A description with none of the three markers has no trigger phrases.
pub fn phrases(text: &str) -> Vec<String> {
    let mut segments: Vec<&str> = Vec::new();
    if let Some(m) = marker_use_when().find(text) {
        segments.push(&text[m.end()..]);
    }
    if let Some(m) = marker_keywords().find(text) {
        segments.push(&text[m.end()..]);
    }
    if let Some(m) = marker_triggers().find(text) {
        segments.push(&text[m.end()..]);
    }

    let mut out = BTreeSet::new();
    for seg in segments {
        for raw in clause_split().split(seg) {
            let p = clean(&normalize(raw));
            if p.chars().count() >= MIN_PHRASE_CHARS {
                out.insert(p);
            }
        }
    }
    out.into_iter().collect()
}

/// Lowercase, drop quote and emphasis characters, and collapse whitespace. Both
/// sides of a comparison go through this, so quoting changes never register.
fn normalize(s: &str) -> String {
    s.to_lowercase()
        .replace(
            [
                '\'', '"', '`', '*', '\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}',
            ],
            "",
        )
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Strip list glue and edge punctuation from one normalised clause.
fn clean(s: &str) -> String {
    let mut p = s.trim_matches(|c: char| !c.is_alphanumeric()).to_string();
    for glue in ["and ", "or ", "when ", "users ask to ", "asked to ", "to "] {
        if let Some(rest) = p.strip_prefix(glue) {
            p = rest.trim().to_string();
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write(dir: &Path, description: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!("---\nname: s\ndescription: |-\n  {description}\n---\n\nBody\n"),
        )
        .unwrap();
    }

    #[test]
    fn extracts_use_when_and_keywords() {
        let p = phrases(
            "Does a thing. Use when: building CLI tools, parsing arguments, or \
             migrating from yargs.\n\nKeywords: Commander.js, subcommands",
        );
        assert!(p.contains(&"building cli tools".to_string()));
        assert!(p.contains(&"parsing arguments".to_string()));
        assert!(p.contains(&"migrating from yargs".to_string()));
        assert!(p.contains(&"subcommands".to_string()));
        assert!(p.contains(&"commander.js".to_string()));
    }

    #[test]
    fn quotes_and_inline_keywords_do_not_leak_into_phrases() {
        let p = phrases(
            "X. Use when asked to 'ready up' a ticket, \"write up PROJ-1\". Keywords: Jira",
        );
        assert!(p.contains(&"ready up a ticket".to_string()), "{p:?}");
        assert!(p.contains(&"write up proj-1".to_string()), "{p:?}");
        assert!(p.contains(&"jira".to_string()), "{p:?}");
        assert!(p.iter().all(|x| !x.contains("keywords")));
        let head = "Use when asked to \"ready up\" a ticket, write up PROJ-1, Jira";
        assert!(dropped(&p, head).is_empty());
    }

    #[test]
    fn triggers_marker_alone_yields_phrases() {
        let p = phrases(
            "Search Semantic Scholar for papers. Preferred over the other. Triggers: search papers, \
             paper by DOI, citation analysis.",
        );
        assert_eq!(
            p,
            vec!["citation analysis", "paper by doi", "search papers"]
        );
        let q = phrases("Does a thing. Trigger: first thing, second thing");
        assert_eq!(q, vec!["first thing", "second thing"]);
    }

    #[test]
    fn triggers_after_use_when_is_not_glued_to_the_first_phrase() {
        let p = phrases(
            "Find papers. Use when discovering papers. Triggers: search papers, find author.",
        );
        assert!(p.contains(&"search papers".to_string()), "{p:?}");
        assert!(p.contains(&"find author".to_string()), "{p:?}");
        assert!(p.iter().all(|x| !x.contains("triggers")), "{p:?}");
    }

    #[test]
    fn triggers_marker_is_case_insensitive_and_needs_a_colon() {
        assert!(phrases("TRIGGERS: alpha thing").contains(&"alpha thing".to_string()));
        // Prose that merely uses the word is not a marker.
        assert!(phrases("It triggers on every commit, always").is_empty());
    }

    #[test]
    fn a_dropped_triggers_phrase_is_reported() {
        let base = phrases("X. Triggers: search papers, paper by doi, citation analysis");
        let head = "X. Triggers: search papers, citation analysis";
        assert_eq!(dropped(&base, head), vec!["paper by doi"]);
    }

    #[test]
    fn no_marker_means_no_phrases() {
        assert!(phrases("Complete guidance for things.").is_empty());
    }

    #[test]
    fn proactively_and_this_skill_markers() {
        let p = phrases("Guidance; use proactively when setting up Bun projects, writing tests");
        assert!(p.contains(&"setting up bun projects".to_string()));
        let q = phrases("Guidance. Use this skill when configuring biome.json");
        assert!(q.contains(&"configuring biome.json".to_string()));
    }

    #[test]
    fn reordered_or_reflowed_phrase_is_preserved() {
        let base = phrases("X. Use when running lint, fixing formatter drift");
        let head = "Y. Use when fixing formatter drift or running   LINT commands";
        assert!(dropped(&base, head).is_empty());
    }

    #[test]
    fn dropped_phrase_is_reported() {
        let base = phrases("X. Use when running lint, fixing formatter drift");
        let head = "X. Use when running lint";
        assert_eq!(dropped(&base, head), vec!["fixing formatter drift"]);
    }

    #[test]
    fn compare_reports_drop_rename_and_no_change() {
        let tmp = tempfile::tempdir().unwrap();
        let (base, head) = (tmp.path().join("base"), tmp.path().join("head"));
        write(&base, "X. Use when running lint, fixing formatter drift");

        write(&head, "X. Use when running lint");
        let r = compare(&head, &base).unwrap();
        assert!(!r.new_skill);
        assert_eq!(r.dropped, vec!["fixing formatter drift"]);

        write(&head, "X. Use when running lint, fixing formatter drift");
        assert!(compare(&head, &base).unwrap().dropped.is_empty());
    }

    #[test]
    fn missing_base_is_a_new_skill() {
        let tmp = tempfile::tempdir().unwrap();
        let head = tmp.path().join("head");
        write(&head, "X. Use when running lint");
        let r = compare(&head, &tmp.path().join("nope")).unwrap();
        assert!(r.new_skill);
        assert!(r.dropped.is_empty());
    }

    #[test]
    fn when_to_use_counts_as_listing_text() {
        let tmp = tempfile::tempdir().unwrap();
        let (base, head) = (tmp.path().join("base"), tmp.path().join("head"));
        write(&base, "X. Use when running lint, auditing configs");
        fs::create_dir_all(&head).unwrap();
        fs::write(
            head.join("SKILL.md"),
            "---\nname: s\ndescription: X. Use when running lint\nwhen_to_use: auditing configs\n---\n\nB\n",
        )
        .unwrap();
        assert!(compare(&head, &base).unwrap().dropped.is_empty());
    }
}
