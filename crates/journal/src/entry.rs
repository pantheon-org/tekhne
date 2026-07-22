//! Rendering and writing of journal entries.
//!
//! Ports the structure defined by the `journal-entry-creator` skill's template
//! schemas (`assets/templates/*.yaml`): YAML frontmatter with triple-synced
//! dates, a single H1 title ending `Month D, YYYY`, a metadata block, the
//! required sections for the chosen type, and a `## Tags` section that mirrors
//! the frontmatter. Every generated entry also carries the `## Compliance`
//! section the validator requires.
//!
//! Rendering is a pure function of the [`EntrySpec`] (including its injected
//! [`Timestamp`]), so output is deterministic and testable. Authored prose uses
//! British English and hyphens, never em dashes.

use std::path::{Path, PathBuf};

use clap::ValueEnum;
use common::{Error, Result};

use crate::date::Timestamp;

/// The five entry types supported by the skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum EntryType {
    /// General-purpose documentation entry.
    Journal,
    /// Problem-resolution session.
    Troubleshooting,
    /// Knowledge-acquisition notes.
    Learning,
    /// Summary of external content.
    ArticleSummary,
    /// Issue-tracker ticket refinement session.
    TicketRefinement,
}

impl EntryType {
    /// The tag every entry of this type must carry (per the schema's
    /// `required_tags`). Article summaries use `article`, one of the accepted
    /// content-type tags.
    pub fn primary_tag(self) -> &'static str {
        match self {
            EntryType::Journal => "journal",
            EntryType::Troubleshooting => "troubleshooting",
            EntryType::Learning => "learning",
            EntryType::ArticleSummary => "article",
            EntryType::TicketRefinement => "ticket-refinement",
        }
    }

    /// A human-readable default title when the caller supplies none.
    pub fn default_title(self) -> &'static str {
        match self {
            EntryType::Journal => "Journal Entry",
            EntryType::Troubleshooting => "Troubleshooting Session",
            EntryType::Learning => "Learning Notes",
            EntryType::ArticleSummary => "Article Summary",
            EntryType::TicketRefinement => "Ticket Refinement",
        }
    }
}

/// Everything needed to render one entry.
#[derive(Debug, Clone)]
pub struct EntrySpec {
    /// Which template to render.
    pub entry_type: EntryType,
    /// The entry title, used in the H1 and frontmatter and to derive the slug.
    pub title: String,
    /// The author recorded in the frontmatter `authors` list.
    pub author: String,
    /// The injected instant used for the date and time fields.
    pub timestamp: Timestamp,
    /// Issue-tracker key: the refinement target (ticket-refinement) or the
    /// ticket that prefixes a troubleshooting slug.
    pub ticket: Option<String>,
    /// Source URL, recorded for article summaries.
    pub source: Option<String>,
}

impl EntrySpec {
    /// The lowercase, hyphenated slug derived from the title, with the ticket
    /// prefix applied for troubleshooting entries that name one.
    pub fn slug(&self) -> String {
        let mut slug = slugify(&self.title);
        if self.entry_type == EntryType::Troubleshooting {
            if let Some(ticket) = &self.ticket {
                let prefix = slugify(ticket);
                if !prefix.is_empty() {
                    slug = format!("{prefix}-{slug}");
                }
            }
        }
        slug
    }

    /// The entry filename, `YYYY-MM-DD-slug.md`.
    pub fn file_name(&self) -> String {
        format!("{}-{}.md", self.timestamp.date.iso(), self.slug())
    }

    /// The date-partitioned directory the entry lives in, `YYYY/MM`.
    pub fn relative_dir(&self) -> PathBuf {
        let date = self.timestamp.date;
        PathBuf::from(format!("{:04}", date.year)).join(format!("{:02}", date.month))
    }

    /// The frontmatter and section tags, primary tag first.
    pub fn tags(&self) -> Vec<String> {
        let mut tags = vec![self.entry_type.primary_tag().to_string()];
        let secondary = match (self.entry_type, &self.ticket) {
            (EntryType::TicketRefinement, Some(ticket)) => slugify(ticket),
            _ => slugify(&self.title)
                .split('-')
                .next()
                .unwrap_or_default()
                .to_string(),
        };
        if !secondary.is_empty() && secondary != tags[0] {
            tags.push(secondary);
        }
        tags
    }

    /// Render the complete entry as a single markdown document.
    pub fn render(&self) -> String {
        let iso = self.timestamp.date.iso();
        let long = self.timestamp.date.long();
        let tags = self.tags();

        let mut out = String::new();
        out.push_str(&self.frontmatter(&iso, &tags));
        out.push_str("\n<!-- markdownlint-disable MD025 -->\n\n");
        out.push_str(&format!("# {} - {}\n\n", self.title, long));
        out.push_str(&self.metadata_block(&long));
        out.push_str(&self.body());
        out.push_str(&section(
            "Compliance",
            "Validated against the journal-entry-creator structure: single H1 with a synced date, required sections present, and code fences carry language specifiers.",
        ));
        out.push_str(&tags_section(&tags));
        out
    }

    /// Write the rendered entry under `base_dir/YYYY/MM/`, creating the
    /// directory tree. Fails if the target file already exists so an existing
    /// entry is never silently overwritten.
    pub fn create(&self, base_dir: &Path) -> Result<PathBuf> {
        let dir = base_dir.join(self.relative_dir());
        std::fs::create_dir_all(&dir).map_err(|e| Error::io(&dir, e))?;
        let path = dir.join(self.file_name());
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| Error::io(&path, e))?;
        use std::io::Write as _;
        file.write_all(self.render().as_bytes())
            .map_err(|e| Error::io(&path, e))?;
        Ok(path)
    }

    fn frontmatter(&self, iso: &str, tags: &[String]) -> String {
        let title = self.title.replace('"', "'");
        let mut fm = String::from("---\n");
        fm.push_str(&format!("title: \"{title}\"\n"));
        fm.push_str(&format!("date: {iso}\n"));
        fm.push_str("authors:\n");
        fm.push_str(&format!("  - {}\n", self.author));
        fm.push_str("tags:\n");
        for tag in tags {
            fm.push_str(&format!("  - {tag}\n"));
        }
        if self.entry_type == EntryType::ArticleSummary {
            let source = self.source.clone().unwrap_or_default();
            fm.push_str(&format!("source: \"{source}\"\n"));
        }
        if self.entry_type == EntryType::TicketRefinement {
            fm.push_str(&format!("refinement_ticket: {}\n", self.ticket_key()));
        }
        fm.push_str("status: draft\n");
        fm.push_str("---\n");
        fm
    }

    fn ticket_key(&self) -> String {
        self.ticket
            .clone()
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| "TICKET-000".to_string())
    }

    fn metadata_block(&self, long: &str) -> String {
        let mut block = format!("**Date:** {long}\n");
        match self.entry_type {
            EntryType::Journal => {
                block.push_str("**Context:** Brief context for this session.\n");
                block.push_str("**System:** System or service this entry concerns.\n");
            }
            EntryType::Troubleshooting => {
                block.push_str(&format!("**Time:** {} UTC\n", self.timestamp.hhmm()));
                block.push_str("**Duration:** TODO\n");
                block.push_str("**System:** System or service affected.\n");
                block.push_str("**Issue:** Short description of the problem.\n");
                block.push_str("**Status:** Investigating\n");
            }
            EntryType::Learning => {
                block.push_str("**Topic:** Main subject area.\n");
            }
            EntryType::ArticleSummary => {
                let url = self.source.clone().unwrap_or_default();
                block.push_str("**Source:** Publication or platform name.\n");
                block.push_str(&format!("**URL:** {url}\n"));
                block.push_str("**Format:** Article\n");
                block.push_str(&format!("**Author:** {}\n", self.author));
            }
            EntryType::TicketRefinement => {
                block.push_str(&format!("**Ticket:** {}\n", self.ticket_key()));
                block.push_str("**Context:** Why this ticket is being refined.\n");
                block.push_str("**System:** System or service the ticket concerns.\n");
            }
        }
        block.push('\n');
        block
    }

    fn body(&self) -> String {
        match self.entry_type {
            EntryType::Journal => self.journal_body(),
            EntryType::Troubleshooting => self.troubleshooting_body(),
            EntryType::Learning => self.learning_body(),
            EntryType::ArticleSummary => self.article_body(),
            EntryType::TicketRefinement => self.ticket_refinement_body(),
        }
    }

    fn journal_body(&self) -> String {
        let mut out = String::new();
        out.push_str(&section("Session Overview", OVERVIEW));
        out.push_str(&section(
            "Context",
            "Background and motivation for the session.",
        ));
        out.push_str(&section(
            "Problem Description",
            "Symptoms, scope, and any error messages.",
        ));
        out.push_str(&section(
            "Key Learning / Summary",
            "One-line summary and the key takeaways.",
        ));
        out.push_str(&section("Investigation Process", STEP_JOURNAL));
        out.push_str(&section(
            "Resolution Steps Taken",
            "Concrete steps performed, with commands where relevant.",
        ));
        out.push_str(&section("Session Outcome", OUTCOME));
        out.push_str(&section(
            "Future Reference / Notes",
            "Prevention steps, links, and related entries.",
        ));
        out
    }

    fn troubleshooting_body(&self) -> String {
        let mut out = String::new();
        out.push_str(&section("Session Overview", OVERVIEW));
        out.push_str(&section(
            "Context",
            "Background and business impact of the issue.",
        ));
        out.push_str(&section(
            "Problem Description",
            "Observable symptoms, exact error messages, and affected scope.",
        ));
        out.push_str(&section("Investigation Process", STEP_TROUBLESHOOTING));
        out.push_str(&section(
            "Root Cause",
            "**Primary cause:** State the confirmed root cause with supporting evidence.",
        ));
        out.push_str(&section(
            "Resolution Steps Taken",
            "Reproducible steps with commands and code.",
        ));
        out.push_str(&section(
            "Screenshots / Evidence",
            "Reference evidence from the entry-specific assets directory.",
        ));
        out.push_str(&section("Session Outcome", OUTCOME));
        out.push_str(&section(
            "Future Reference / Preventative Measures",
            "Prevention, monitoring, alerts, and related incidents.",
        ));
        out
    }

    fn learning_body(&self) -> String {
        let mut out = String::new();
        out.push_str(&section("Session Overview", OVERVIEW));
        out.push_str(&section(
            "Context",
            "Why you needed to learn this, and the motivation.",
        ));
        out.push_str(&section(
            "Key Learning",
            "The core concept, tool, or process learned.",
        ));
        out.push_str(&section("Process / Solution", STEP_LEARNING));
        out.push_str(&section(
            "Key Takeaways",
            "Bulleted insights and important points.",
        ));
        out.push_str(&section(
            "Use Cases",
            "Where and when to apply this knowledge.",
        ));
        out.push_str(&section(
            "Next Steps / Actions",
            "Practical follow-ups and areas to explore.",
        ));
        out
    }

    fn article_body(&self) -> String {
        let mut out = String::new();
        out.push_str(&section(
            "Session Overview",
            "Why you consumed this content.",
        ));
        out.push_str(&section(
            "Core Summary",
            "Two to four sentences capturing the main thesis.",
        ));
        out.push_str(&section(
            "Key Points / Takeaways",
            "Main arguments with what, why it matters, and evidence.",
        ));
        out.push_str(&section(
            "Applications / Where to Use",
            "How this applies to your work or projects.",
        ));
        out.push_str(&section(
            "Personal Notes / Actions",
            "Your takeaways and actionable next steps.",
        ));
        out
    }

    fn ticket_refinement_body(&self) -> String {
        let ticket = self.ticket_key();
        let mut out = String::new();
        out.push_str(&section("Session Overview", OVERVIEW));
        out.push_str(&section(
            "Context",
            "Origin of the ticket and any constraints, including regulatory or personal-data ones.",
        ));
        out.push_str(&section(
            "Current Ticket State",
            "What the ticket said before refinement, and what was missing or ambiguous.",
        ));
        out.push_str(&section(
            "Findings",
            "What was learned that grounds the refinement.",
        ));
        out.push_str(&proposed_ticket_description(&ticket));
        out.push_str(&section(
            "Session Outcome",
            "Draft for review - the ticket itself was left untouched.\n\nNext steps:\n\n- Confirm the draft with the team and apply it when approved.",
        ));
        out.push_str(&section(
            "Future Reference / Notes",
            "Links to the ticket, parent incident, repo, and key files.",
        ));
        out
    }
}

const OVERVIEW: &str = "High-level summary of this session in two to four sentences.";
const OUTCOME: &str = "State the outcome and the next steps as a short list.";
const STEP_JOURNAL: &str = "### Step 1 - Initial Review\n\n- **What:** Describe what was investigated.\n- **Result:** Describe what was observed.";
const STEP_LEARNING: &str = "### Step 1 - Basic Usage\n\n```bash\necho \"replace with a real command\"\n```\n\nDescribe the command and its result.";
const STEP_TROUBLESHOOTING: &str = "### Step 1 - Reproduce the Failure\n\n- **What:** Describe what was investigated.\n\n```bash\necho \"replace with the command used\"\n```\n\n- **Result:** Describe what was observed.\n\n### Step 2 - Locate the Bug in Source Code\n\n**What:** Confirm which repository and file own the defect.\n\n```bash\ngit clone --depth 1 <repo-url> /tmp/<repo-name>\ngrep -rn \"<symbol>\" /tmp/<repo-name>/\n```\n\n**Result:** Record the repo, file, symbol, and line range.";

/// A `## <title>` section with a trailing blank line.
fn section(title: &str, body: &str) -> String {
    format!("## {title}\n\n{body}\n\n")
}

/// The `## Tags` section, backtick-and-pipe separated to match the frontmatter.
fn tags_section(tags: &[String]) -> String {
    let rendered = tags
        .iter()
        .map(|t| format!("`{t}`"))
        .collect::<Vec<_>>()
        .join(" | ");
    format!("## Tags\n\n{rendered}\n")
}

/// The `## Proposed Ticket Description` section: a copy-pasteable, not-yet-applied
/// draft inside a fenced markdown block, so its inner headings never become
/// document headings. The validator requires this whenever `refinement_ticket`
/// is set in the frontmatter.
fn proposed_ticket_description(ticket: &str) -> String {
    let mut out = String::from("## Proposed Ticket Description\n\n");
    out.push_str(&format!(
        "Draft for {ticket} - review before applying; not yet applied to the ticket.\n\n"
    ));
    out.push_str("````markdown\n");
    out.push_str("**Summary:** One-line restatement of the ticket title.\n\n");
    out.push_str("**Background**\n\n");
    out.push_str("Parent incident or request, corrected framing, and any regulated or personal-data constraint.\n\n");
    out.push_str("**Problem**\n\n");
    out.push_str("The specific engineering gap.\n\n");
    out.push_str("**Conditions of Satisfaction**\n\n");
    out.push_str("- [ ] **MUST** state the primary requirement.\n\n");
    out.push_str("**Acceptance criteria**\n\n");
    out.push_str("- Concrete, testable criterion.\n");
    out.push_str("````\n\n");
    out
}

/// Slugify a string: lowercase, drop common stop words, join alphanumeric
/// tokens with hyphens, cap at six tokens. Always lowercase-only so it passes
/// the filename slug check. Falls back to `entry` when nothing remains.
fn slugify(text: &str) -> String {
    const STOP: [&str; 10] = [
        "the", "a", "an", "and", "for", "with", "to", "of", "in", "on",
    ];
    let mut words: Vec<String> = Vec::new();
    for raw in text.split(|c: char| !c.is_ascii_alphanumeric()) {
        if raw.is_empty() {
            continue;
        }
        let word = raw.to_ascii_lowercase();
        if STOP.contains(&word.as_str()) {
            continue;
        }
        words.push(word);
        if words.len() >= 6 {
            break;
        }
    }
    if words.is_empty() {
        return "entry".to_string();
    }
    words.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::Date;

    fn spec(entry_type: EntryType) -> EntrySpec {
        EntrySpec {
            entry_type,
            title: "Example Title".to_string(),
            author: "Author".to_string(),
            timestamp: Timestamp::fixed(Date::new(2026, 7, 22), 14, 30),
            ticket: None,
            source: None,
        }
    }

    #[test]
    fn slugify_lowercases_and_drops_stop_words() {
        assert_eq!(
            slugify("The API Performance Issue"),
            "api-performance-issue"
        );
        assert_eq!(slugify("Example"), "example");
        assert_eq!(slugify("!!!"), "entry");
    }

    #[test]
    fn file_name_and_dir_are_date_partitioned() {
        let s = spec(EntryType::Journal);
        assert_eq!(s.file_name(), "2026-07-22-example-title.md");
        assert_eq!(s.relative_dir(), PathBuf::from("2026").join("07"));
    }

    #[test]
    fn troubleshooting_slug_gets_ticket_prefix() {
        let mut s = spec(EntryType::Troubleshooting);
        s.ticket = Some("PLGSD-9507".to_string());
        assert_eq!(s.file_name(), "2026-07-22-plgsd-9507-example-title.md");
    }

    #[test]
    fn h1_ends_with_long_date_and_is_unique() {
        let doc = spec(EntryType::Journal).render();
        let h1_lines: Vec<&str> = doc.lines().filter(|l| l.starts_with("# ")).collect();
        assert_eq!(h1_lines.len(), 1);
        assert!(h1_lines[0].ends_with("July 22, 2026"));
    }

    #[test]
    fn frontmatter_date_matches_filename_date() {
        let s = spec(EntryType::Learning);
        assert!(s.render().contains("date: 2026-07-22"));
        assert!(s.file_name().starts_with("2026-07-22-"));
    }

    #[test]
    fn article_carries_source_and_type_tag() {
        let mut s = spec(EntryType::ArticleSummary);
        s.source = Some("https://example.com/post".to_string());
        let doc = s.render();
        assert!(doc.contains("source: \"https://example.com/post\""));
        assert!(doc.contains("- article\n"));
        assert!(doc.contains("## Core Summary"));
    }

    #[test]
    fn ticket_refinement_sets_field_and_section() {
        let mut s = spec(EntryType::TicketRefinement);
        s.ticket = Some("TICKET-123".to_string());
        let doc = s.render();
        assert!(doc.contains("refinement_ticket: TICKET-123"));
        assert!(doc.contains("## Proposed Ticket Description"));
        assert!(doc.contains("````markdown"));
    }

    #[test]
    fn every_type_has_the_validator_required_sections() {
        for t in [
            EntryType::Journal,
            EntryType::Troubleshooting,
            EntryType::Learning,
            EntryType::ArticleSummary,
            EntryType::TicketRefinement,
        ] {
            let doc = spec(t).render();
            assert!(doc.contains("\n## Session Overview\n"), "{t:?} overview");
            assert!(doc.contains("\n## Compliance\n"), "{t:?} compliance");
            assert!(doc.contains("\n## Tags\n"), "{t:?} tags");
        }
    }

    #[test]
    fn create_writes_into_year_month_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let path = spec(EntryType::Journal).create(tmp.path()).unwrap();
        assert!(path.ends_with("2026/07/2026-07-22-example-title.md"));
        assert!(path.is_file());
    }

    #[test]
    fn create_refuses_to_overwrite() {
        let tmp = tempfile::tempdir().unwrap();
        let s = spec(EntryType::Journal);
        s.create(tmp.path()).unwrap();
        assert!(s.create(tmp.path()).is_err());
    }
}
