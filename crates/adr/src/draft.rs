//! Aggregating context so a record can be filled in rather than guessed at.
//!
//! `draft` writes nothing. It reads the branch's commits and diff, works out
//! which sections of the record are still placeholders, and points at earlier
//! decisions that touched the same files. `--bootstrap` turns the same
//! information into a numbered question set for a record that is still blank.

use std::path::Path;
use std::process::Command;

use serde::Serialize;

use crate::git::BranchContext;
use crate::record::Record;

/// An earlier decision that touched some of the same files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Related {
    /// The earlier record's slug.
    pub slug: String,
    /// Its title, so the reference is legible without opening it.
    pub title: String,
    /// How many files the two decisions have in common.
    pub shared_files: usize,
}

/// Everything `draft` gathered, for `--json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Draft {
    /// The record being drafted.
    pub slug: String,
    /// The branch it belongs to, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The kind of change.
    pub branch_type: &'static str,
    /// The base branch the diff was taken against.
    pub base: String,
    /// Commit subjects and dates, newest first.
    pub commits: Vec<CommitRow>,
    /// Files touched.
    pub files_changed: usize,
    /// Lines added.
    pub insertions: usize,
    /// Lines removed.
    pub deletions: usize,
    /// Headings whose body is still a placeholder.
    pub unfilled_sections: Vec<String>,
    /// How many placeholder comments remain in total.
    pub placeholders: usize,
    /// Earlier decisions over the same files, most overlap first.
    pub related: Vec<Related>,
}

/// One commit, flattened for JSON output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommitRow {
    /// The commit subject line.
    pub subject: String,
    /// The author date.
    pub date: String,
}

/// The questions `--bootstrap` asks, in the order a record is written.
const QUESTIONS: [(&str, &str); 8] = [
    (
        "Problem Statement / Context",
        "Why does this decision exist? What problem does it solve?",
    ),
    ("Goals", "What must this decision achieve?"),
    ("Non-Goals", "What is explicitly out of scope?"),
    ("Options Considered", "What alternatives were evaluated?"),
    ("Chosen Solution", "What was decided, and what does it do?"),
    ("Rationale", "Why this option over the alternatives?"),
    (
        "Risks & Pitfalls",
        "What could go wrong? What mitigations are in place?",
    ),
    (
        "Impact Assessment",
        "What are the performance, security and maintenance impacts?",
    ),
];

/// Build the draft for `record` from its branch context and the collection.
pub fn build(record: &Record, ctx: &BranchContext, all: &[Record]) -> Draft {
    Draft {
        slug: record.slug.clone(),
        branch: ctx.branch.clone().or_else(|| record.meta.branch.clone()),
        branch_type: record.meta.branch_type.as_str(),
        base: ctx.base.clone(),
        commits: ctx
            .commits
            .iter()
            .map(|c| CommitRow {
                subject: c.subject.clone(),
                date: c.date.clone(),
            })
            .collect(),
        files_changed: ctx.diffstat.files,
        insertions: ctx.diffstat.insertions,
        deletions: ctx.diffstat.deletions,
        unfilled_sections: unfilled_sections(&record.body),
        placeholders: count_placeholders(&record.body),
        related: related(record, all),
    }
}

/// The headings whose body still contains a placeholder comment.
///
/// A heading whose prose has been written but which still carries a trailing
/// comment counts as unfilled, because the comment is what the score penalises.
pub fn unfilled_sections(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    let mut flagged = false;

    for line in body.lines() {
        if let Some(name) = heading_name(line) {
            if flagged {
                if let Some(prev) = current.take() {
                    out.push(prev);
                }
            }
            current = Some(name);
            flagged = false;
        } else if line.contains("<!--") {
            flagged = true;
        }
    }
    if flagged {
        if let Some(last) = current {
            out.push(last);
        }
    }
    out.sort();
    out.dedup();
    out
}

/// The text of a `##` or `###` heading, if `line` is one.
fn heading_name(line: &str) -> Option<String> {
    for prefix in ["### ", "## "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

/// Earlier decisions that touched the same files, most overlap first.
///
/// The record being drafted is excluded, as are records with no file overlap.
pub fn related(record: &Record, all: &[Record]) -> Vec<Related> {
    let mut out: Vec<Related> = all
        .iter()
        .filter(|other| other.slug != record.slug)
        .filter_map(|other| {
            let shared = other
                .meta
                .changed_files
                .iter()
                .filter(|f| record.meta.changed_files.contains(f))
                .count();
            (shared > 0).then(|| Related {
                slug: other.slug.clone(),
                title: other.meta.title.clone(),
                shared_files: shared,
            })
        })
        .collect();
    out.sort_by(|a, b| {
        b.shared_files
            .cmp(&a.shared_files)
            .then_with(|| a.slug.cmp(&b.slug))
    });
    out
}

/// The number of placeholder comments in `body`.
fn count_placeholders(body: &str) -> usize {
    body.matches("<!--").count()
}

/// Review comments on the current branch's pull request, via the `gh` CLI.
///
/// Returns `None` when `gh` is absent, unauthenticated, or there is no PR, so
/// `--pr` degrades to the ordinary draft rather than failing the command.
pub fn pr_comments(dir: &Path) -> Option<String> {
    let out = Command::new("gh")
        .args(["pr", "view", "--json", "comments,reviews"])
        .current_dir(dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;

    let mut lines = Vec::new();
    for key in ["comments", "reviews"] {
        for item in json
            .get(key)
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
        {
            let author = item
                .get("author")
                .and_then(|a| a.get("login"))
                .and_then(|l| l.as_str())
                .unwrap_or("unknown");
            let body = item
                .get("body")
                .and_then(|b| b.as_str())
                .unwrap_or("")
                .trim();
            if !body.is_empty() {
                lines.push(format!("- **{author}**: {body}"));
            }
        }
    }
    (!lines.is_empty()).then(|| lines.join("\n"))
}

/// Render the human-readable draft.
pub fn render(draft: &Draft, pr_feedback: Option<&str>) -> String {
    let mut out = String::new();

    out.push_str("## Branch Context\n");
    out.push_str(&format!(
        "Branch:   {}\n",
        draft.branch.as_deref().unwrap_or("(none)")
    ));
    out.push_str(&format!("Type:     {}\n", draft.branch_type));
    out.push_str(&format!("Base:     {}\n", draft.base));
    out.push_str(&format!("Commits:  {}\n", draft.commits.len()));
    for c in &draft.commits {
        out.push_str(&format!("  - {:?} ({})\n", c.subject, c.date));
    }

    out.push_str(&format!(
        "\n## Changes\nFiles changed: {}  (+{} / -{})\n",
        draft.files_changed, draft.insertions, draft.deletions
    ));

    if !draft.related.is_empty() {
        out.push_str("\n## Earlier decisions over the same files\n");
        for r in &draft.related {
            out.push_str(&format!(
                "  - {} ({}, {} shared file{})\n",
                r.slug,
                r.title,
                r.shared_files,
                if r.shared_files == 1 { "" } else { "s" }
            ));
        }
    }

    if let Some(feedback) = pr_feedback {
        out.push_str("\n## Reviewer Feedback\n");
        out.push_str(feedback);
        out.push('\n');
    }

    if draft.unfilled_sections.is_empty() {
        out.push_str("\n## Suggested sections to address\n  (none: every section is filled in)\n");
    } else {
        out.push_str("\n## Suggested sections to address\n");
        for section in &draft.unfilled_sections {
            out.push_str(&format!("  - {section}\n"));
        }
    }
    out
}

/// Render the `--bootstrap` question set.
pub fn render_bootstrap(draft: &Draft) -> String {
    let mut out = format!("## Bootstrap Questions for {}\n", draft.slug);
    out.push_str(&format!(
        "({} unfilled placeholder{} remain; answer these to write the first pass)\n\n",
        draft.placeholders,
        if draft.placeholders == 1 { "" } else { "s" }
    ));

    for (i, (section, question)) in QUESTIONS.iter().enumerate() {
        out.push_str(&format!("{}. **{section}**: {question}\n", i + 1));
        if *section == "Problem Statement / Context" && draft.files_changed > 0 {
            out.push_str(&format!(
                "   Context: {} file{} changed (+{}/-{}) on this branch.\n",
                draft.files_changed,
                if draft.files_changed == 1 { "" } else { "s" },
                draft.insertions,
                draft.deletions
            ));
        }
        out.push('\n');
    }

    if !draft.commits.is_empty() {
        out.push_str(&format!("Branch commits ({}):\n", draft.commits.len()));
        for c in &draft.commits {
            out.push_str(&format!("  - {:?} ({})\n", c.subject, c.date));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::{Commit, Diffstat};
    use crate::record::{BranchType, Record};

    fn record(slug: &str) -> Record {
        Record::new(
            slug,
            slug,
            BranchType::Feat,
            "Test User",
            "2026-09-15",
            None,
        )
    }

    fn ctx() -> BranchContext {
        BranchContext {
            branch: Some("feat/adopt-otel".to_string()),
            base: "main".to_string(),
            commits: vec![Commit {
                subject: "feat: add otel".to_string(),
                date: "2026-09-15".to_string(),
            }],
            diffstat: Diffstat {
                files: 3,
                insertions: 120,
                deletions: 4,
                by_dir: vec![("src".to_string(), 3, 120)],
            },
        }
    }

    #[test]
    fn an_untouched_template_reports_every_section_unfilled() {
        let r = record("adopt-otel");
        let sections = unfilled_sections(&r.body);

        assert!(sections.contains(&"Context".to_string()), "{sections:?}");
        assert!(
            sections.contains(&"Chosen Solution".to_string()),
            "{sections:?}"
        );
        assert!(
            sections.contains(&"Outcome & Lessons".to_string()),
            "{sections:?}"
        );
    }

    #[test]
    fn a_filled_section_drops_out_of_the_list() {
        let mut r = record("adopt-otel");
        r.body = r.body.replace(
            "### Rationale\n<!-- Why this option over alternatives -->",
            "### Rationale\nBecause it is vendor neutral.",
        );
        assert!(!unfilled_sections(&r.body).contains(&"Rationale".to_string()));
    }

    #[test]
    fn sections_are_sorted_and_deduplicated() {
        let body = "## B\n<!-- x -->\n## A\n<!-- y -->\n## A\n<!-- z -->\n";
        assert_eq!(
            unfilled_sections(body),
            vec!["A".to_string(), "B".to_string()]
        );
    }

    #[test]
    fn a_body_with_no_placeholders_has_no_unfilled_sections() {
        assert!(unfilled_sections("## A\nreal prose\n").is_empty());
    }

    #[test]
    fn related_records_rank_by_shared_files() {
        let mut current = record("adopt-otel");
        current.meta.changed_files = vec!["src/a.rs".into(), "src/b.rs".into()];

        let mut one = record("earlier-one");
        one.meta.title = "Earlier one".into();
        one.meta.changed_files = vec!["src/a.rs".into()];

        let mut two = record("earlier-two");
        two.meta.title = "Earlier two".into();
        two.meta.changed_files = vec!["src/a.rs".into(), "src/b.rs".into()];

        let mut unrelated = record("unrelated");
        unrelated.meta.changed_files = vec!["docs/x.md".into()];

        let found = related(&current, &[one, two, unrelated, current.clone()]);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].slug, "earlier-two");
        assert_eq!(found[0].shared_files, 2);
        assert_eq!(found[1].slug, "earlier-one");
    }

    #[test]
    fn a_record_is_never_related_to_itself() {
        let mut current = record("adopt-otel");
        current.meta.changed_files = vec!["src/a.rs".into()];
        assert!(related(&current, &[current.clone()]).is_empty());
    }

    #[test]
    fn build_carries_the_branch_context_through() {
        let r = record("adopt-otel");
        let d = build(&r, &ctx(), &[]);

        assert_eq!(d.slug, "adopt-otel");
        assert_eq!(d.branch.as_deref(), Some("feat/adopt-otel"));
        assert_eq!(d.base, "main");
        assert_eq!(d.files_changed, 3);
        assert_eq!(d.insertions, 120);
        assert_eq!(d.commits.len(), 1);
        assert!(d.placeholders > 0);
    }

    #[test]
    fn build_falls_back_to_the_recorded_branch_outside_a_repo() {
        let mut r = record("adopt-otel");
        r.meta.branch = Some("feat/recorded".to_string());
        let d = build(&r, &BranchContext::default(), &[]);
        assert_eq!(d.branch.as_deref(), Some("feat/recorded"));
    }

    #[test]
    fn the_human_draft_names_every_heading() {
        let out = render(&build(&record("adopt-otel"), &ctx(), &[]), None);
        assert!(out.contains("## Branch Context"), "{out}");
        assert!(out.contains("## Changes"), "{out}");
        assert!(out.contains("## Suggested sections to address"), "{out}");
        assert!(out.contains("Files changed: 3  (+120 / -4)"), "{out}");
    }

    #[test]
    fn reviewer_feedback_appears_only_when_supplied() {
        let draft = build(&record("adopt-otel"), &ctx(), &[]);
        assert!(!render(&draft, None).contains("Reviewer Feedback"));

        let with = render(&draft, Some("- **alice**: looks good"));
        assert!(with.contains("## Reviewer Feedback"), "{with}");
        assert!(with.contains("alice"), "{with}");
    }

    #[test]
    fn a_complete_record_says_there_is_nothing_to_address() {
        let mut r = record("done");
        r.body = "## Context\nreal prose\n".to_string();
        let out = render(&build(&r, &ctx(), &[]), None);
        assert!(out.contains("every section is filled in"), "{out}");
    }

    #[test]
    fn bootstrap_numbers_eight_questions_and_counts_placeholders() {
        let out = render_bootstrap(&build(&record("adopt-otel"), &ctx(), &[]));

        assert!(
            out.contains("## Bootstrap Questions for adopt-otel"),
            "{out}"
        );
        assert!(out.contains("1. **Problem Statement / Context**"), "{out}");
        assert!(out.contains("8. **Impact Assessment**"), "{out}");
        assert!(out.contains("unfilled placeholders remain"), "{out}");
        assert!(out.contains("3 files changed (+120/-4)"), "{out}");
    }

    #[test]
    fn bootstrap_omits_the_diff_hint_when_nothing_changed() {
        let r = record("adopt-otel");
        let out = render_bootstrap(&build(&r, &BranchContext::default(), &[]));
        assert!(!out.contains("files changed ("), "{out}");
    }

    #[test]
    fn related_decisions_appear_in_the_human_draft() {
        let mut current = record("adopt-otel");
        current.meta.changed_files = vec!["src/a.rs".into()];
        let mut earlier = record("earlier");
        earlier.meta.title = "Earlier decision".into();
        earlier.meta.changed_files = vec!["src/a.rs".into()];

        let draft = build(&current, &ctx(), &[earlier]);
        let out = render(&draft, None);
        assert!(
            out.contains("Earlier decisions over the same files"),
            "{out}"
        );
        assert!(out.contains("1 shared file)"), "{out}");
    }
}
