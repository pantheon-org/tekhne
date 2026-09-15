//! The ADR record: YAML frontmatter plus a Branch ADR body.
//!
//! Frontmatter is the single source of truth for every piece of metadata about
//! a decision: its status, author, branch, tags, supersede links and the full
//! history of status transitions. Nothing about a record is stored anywhere
//! else, so there is no second file to fall out of step with the prose.
//!
//! Every function here is pure with respect to the filesystem and an injectable
//! `today`, so behaviour is deterministic in tests.

use std::fmt;
use std::path::{Path, PathBuf};

use common::{Error, Result};
use serde::{Deserialize, Serialize};

/// The default ADR directory, relative to the repository root.
pub const DEFAULT_DIR: &str = "docs/adr";

/// The environment variable that overrides the ADR directory.
pub const DIR_ENV: &str = "ADR_DIR";

/// The frontmatter delimiter.
const FENCE: &str = "---";

/// Where a decision sits in its lifecycle.
///
/// Status lives in frontmatter and moves only through [`Record::transition`],
/// which appends to the history at the same time. There is no `rejected`
/// state: a decision that was considered and declined is `deprecated`, or stays
/// `proposed` with the reasons written into the prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// The record exists and is being written. Assigned at creation.
    Proposed,
    /// The record passed the completeness gate and is waiting on a human.
    ReviewRequested,
    /// The decision is in force and its prose is frozen.
    Accepted,
    /// The decision no longer applies, and no single ADR replaced it.
    Deprecated,
    /// A later ADR replaces this one; `superseded_by` names it.
    Superseded,
}

impl Status {
    /// Every status, in lifecycle order, for help text and validation messages.
    pub const ALL: [Status; 5] = [
        Status::Proposed,
        Status::ReviewRequested,
        Status::Accepted,
        Status::Deprecated,
        Status::Superseded,
    ];

    /// The kebab-case wire form, as written to frontmatter.
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Proposed => "proposed",
            Status::ReviewRequested => "review-requested",
            Status::Accepted => "accepted",
            Status::Deprecated => "deprecated",
            Status::Superseded => "superseded",
        }
    }

    /// Parse a status, naming every valid value when the input is not one.
    pub fn parse(s: &str) -> Result<Status> {
        Status::ALL
            .into_iter()
            .find(|status| status.as_str() == s)
            .ok_or_else(|| {
                let valid: Vec<&str> = Status::ALL.iter().map(|s| s.as_str()).collect();
                Error::Config(format!(
                    "invalid status {s:?}: must be one of {}",
                    valid.join(", ")
                ))
            })
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The kind of change a decision accompanies, taken from the branch prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BranchType {
    /// A new capability.
    Feat,
    /// A correction to existing behaviour.
    Fix,
    /// Documentation only.
    Docs,
    /// Maintenance with no behaviour change.
    Chore,
}

impl BranchType {
    /// Every branch type, for help text and validation messages.
    pub const ALL: [BranchType; 4] = [
        BranchType::Feat,
        BranchType::Fix,
        BranchType::Docs,
        BranchType::Chore,
    ];

    /// The lowercase wire form, as written to frontmatter.
    pub fn as_str(self) -> &'static str {
        match self {
            BranchType::Feat => "feat",
            BranchType::Fix => "fix",
            BranchType::Docs => "docs",
            BranchType::Chore => "chore",
        }
    }

    /// Parse a branch type, naming every valid value when the input is not one.
    pub fn parse(s: &str) -> Result<BranchType> {
        BranchType::ALL
            .into_iter()
            .find(|t| t.as_str() == s)
            .ok_or_else(|| {
                let valid: Vec<&str> = BranchType::ALL.iter().map(|t| t.as_str()).collect();
                Error::Config(format!(
                    "invalid type {s:?}: must be one of {}",
                    valid.join(", ")
                ))
            })
    }
}

impl fmt::Display for BranchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One recorded status change, appended whenever the status actually moves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
    /// The status before the change.
    pub from: Status,
    /// The status after the change.
    pub to: Status,
    /// The UTC instant of the change, as RFC 3339.
    pub at: String,
}

/// The frontmatter block of an ADR: every field the tooling reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frontmatter {
    /// The human-readable decision title.
    pub title: String,
    /// Always `decision`, so ADRs sort alongside other context typologies.
    #[serde(rename = "type")]
    pub doc_type: String,
    /// The kind of change this decision accompanies.
    pub branch_type: BranchType,
    /// Where the decision sits in its lifecycle.
    pub status: Status,
    /// The creation date as `YYYY-MM-DD`.
    pub date: String,
    /// Who wrote the record.
    pub author: String,
    /// The branch the decision was made on, when there was one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The pull request number, once one is open.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pr: Option<u32>,
    /// Free-form tags for filtering.
    #[serde(default)]
    pub tags: Vec<String>,
    /// The slug of an earlier decision this one replaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    /// The slug of a later decision that replaced this one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_by: Option<String>,
    /// Paths or links to related records and documents.
    #[serde(default)]
    pub related: Vec<String>,
    /// The files the branch touched, as a snapshot taken at creation.
    ///
    /// This is history rather than live state, so it does not go stale: it
    /// records what the decision changed at the time it was made, and lets
    /// `draft` surface earlier decisions that touched the same files.
    #[serde(default)]
    pub changed_files: Vec<String>,
    /// Every status change, oldest first.
    #[serde(default)]
    pub history: Vec<Transition>,
}

/// A complete ADR: its frontmatter, its prose, and where it lives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// The slug, which is the file stem and the record's identity.
    pub slug: String,
    /// Everything the tooling reads.
    pub meta: Frontmatter,
    /// The markdown body, starting at the `# ` heading.
    pub body: String,
}

impl Record {
    /// Build a new record from the Branch ADR template.
    pub fn new(
        slug: &str,
        title: &str,
        branch_type: BranchType,
        author: &str,
        today: &str,
        branch: Option<&str>,
    ) -> Record {
        Record {
            slug: slug.to_string(),
            meta: Frontmatter {
                title: title.to_string(),
                doc_type: "decision".to_string(),
                branch_type,
                status: Status::Proposed,
                date: today.to_string(),
                author: author.to_string(),
                branch: branch.map(str::to_string),
                pr: None,
                tags: Vec::new(),
                supersedes: None,
                superseded_by: None,
                related: Vec::new(),
                changed_files: Vec::new(),
                history: Vec::new(),
            },
            body: render_template(title),
        }
    }

    /// Move the record to `next`, appending a history entry.
    ///
    /// A transition to the status the record already holds is a no-op, so the
    /// history carries no noise.
    pub fn transition(&mut self, next: Status, at: &str) {
        if self.meta.status == next {
            return;
        }
        self.meta.history.push(Transition {
            from: self.meta.status,
            to: next,
            at: at.to_string(),
        });
        self.meta.status = next;
    }

    /// The file name for this record within an ADR directory.
    pub fn file_name(&self) -> String {
        format!("{}.md", self.slug)
    }

    /// The full path for this record under `dir`.
    pub fn path_in(&self, dir: &Path) -> PathBuf {
        dir.join(self.file_name())
    }

    /// Render the complete file: frontmatter fence, metadata, fence, body.
    pub fn render(&self) -> Result<String> {
        let yaml = serde_yaml_ng::to_string(&self.meta)
            .map_err(|e| Error::Config(format!("serialise frontmatter: {e}")))?;
        Ok(format!("{FENCE}\n{yaml}{FENCE}\n\n{}", self.body))
    }

    /// Parse a record from file content, taking its slug from the file stem.
    pub fn parse(slug: &str, content: &str) -> Result<Record> {
        let (yaml, body) = split_frontmatter(content).ok_or_else(|| {
            Error::Config(format!(
                "{slug}: no YAML frontmatter found (expected a leading `---` block)"
            ))
        })?;
        let meta: Frontmatter = serde_yaml_ng::from_str(yaml)
            .map_err(|e| Error::Config(format!("{slug}: invalid frontmatter: {e}")))?;
        Ok(Record {
            slug: slug.to_string(),
            meta,
            body: body.to_string(),
        })
    }
}

/// Split `content` into its frontmatter YAML and the body that follows it.
///
/// Returns `None` when the content does not open with a `---` fence, so a file
/// without frontmatter is reported as such rather than silently parsed as an
/// empty record.
fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let rest = content
        .strip_prefix(FENCE)
        .and_then(|r| r.strip_prefix('\n'))?;
    let end = find_closing_fence(rest)?;
    let yaml = &rest[..end];
    let after = &rest[end + FENCE.len()..];
    Some((yaml, after.trim_start_matches('\n')))
}

/// The byte offset of the closing `---` fence, which must start its own line.
fn find_closing_fence(rest: &str) -> Option<usize> {
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == FENCE {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

/// Derive a slug and branch type from a git branch name.
///
/// `feat/adopt-otel` gives `("adopt-otel", Feat)`. A branch with no recognised
/// prefix keeps its whole name as the slug and is typed `chore`.
pub fn from_branch(branch: &str) -> (String, BranchType) {
    for t in BranchType::ALL {
        let prefix = format!("{}/", t.as_str());
        if let Some(rest) = branch.strip_prefix(&prefix) {
            return (slugify(rest), t);
        }
        if branch == t.as_str() {
            return (slugify(branch), t);
        }
    }
    (slugify(branch), BranchType::Chore)
}

/// Convert free text into a kebab-case slug: lowercase, non-alphanumeric runs
/// collapsed to single hyphens, with no leading or trailing hyphen.
///
/// Forward slashes collapse like any other separator, so a branch name that
/// kept a prefix does not turn into a nested path.
pub fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut prev_hyphen = true; // suppress a leading hyphen
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            prev_hyphen = false;
        } else if !prev_hyphen {
            slug.push('-');
            prev_hyphen = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

/// Turn a slug back into a title-ish string, for a record created without one.
pub fn title_from_slug(slug: &str) -> String {
    let mut out = String::with_capacity(slug.len());
    for (i, word) in slug.split('-').filter(|w| !w.is_empty()).enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let mut chars = word.chars();
        match chars.next() {
            Some(first) if i == 0 => {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
            Some(first) => {
                out.push(first);
                out.push_str(chars.as_str());
            }
            None => {}
        }
    }
    out
}

/// Resolve the ADR directory from an explicit flag, the [`DIR_ENV`] variable,
/// or the [`DEFAULT_DIR`] default, in that order of precedence.
pub fn resolve_dir(flag: Option<&str>, env_value: Option<&str>) -> PathBuf {
    if let Some(dir) = flag.filter(|d| !d.is_empty()) {
        return PathBuf::from(dir);
    }
    if let Some(dir) = env_value.filter(|d| !d.is_empty()) {
        return PathBuf::from(dir);
    }
    PathBuf::from(DEFAULT_DIR)
}

/// The Branch ADR body template, with one HTML-comment placeholder per section.
///
/// The headings are load-bearing: [`crate::completeness`] scores sections by
/// name, so renaming one drops its weight and caps the record's score.
fn render_template(title: &str) -> String {
    format!(
        "# {title}\n\
         \n\
         ## Problem Statement\n\
         \n\
         ### Context\n\
         <!-- Describe the situation requiring this decision -->\n\
         \n\
         ### Goals\n\
         <!-- What this decision must achieve -->\n\
         \n\
         ### Non-Goals\n\
         <!-- Explicitly out of scope -->\n\
         \n\
         ## Decision Record\n\
         \n\
         ### Options Considered\n\
         <!-- List alternatives evaluated -->\n\
         \n\
         ### Chosen Solution\n\
         <!-- What was decided and implemented -->\n\
         \n\
         ### Rationale\n\
         <!-- Why this option over alternatives -->\n\
         \n\
         ## Implementation\n\
         \n\
         ### Key Changes\n\
         <!-- Files/modules changed -->\n\
         \n\
         ### Testing Strategy\n\
         <!-- How correctness is verified -->\n\
         \n\
         ## Challenges & Solutions\n\
         <!-- Technical or process obstacles encountered -->\n\
         \n\
         ## Impact Assessment\n\
         \n\
         - **Performance**: <!-- impact or \"none\" -->\n\
         - **Security**: <!-- impact or \"none\" -->\n\
         - **Maintenance**: <!-- impact or \"none\" -->\n\
         \n\
         ## Risks & Pitfalls\n\
         \n\
         - **Risk**: <!-- describe -->  **Mitigation**: <!-- describe or \"accepted\" -->\n\
         \n\
         ## Outcome & Lessons\n\
         <!-- Fill in post-merge: results, metrics, lessons learned -->\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Record {
        Record::new(
            "adopt-otel",
            "Adopt OpenTelemetry for tracing",
            BranchType::Feat,
            "Test User",
            "2026-09-15",
            Some("feat/adopt-otel"),
        )
    }

    #[test]
    fn status_round_trips_through_its_wire_form() {
        for status in Status::ALL {
            assert_eq!(Status::parse(status.as_str()).unwrap(), status);
        }
    }

    #[test]
    fn invalid_status_names_every_valid_value() {
        let err = Status::parse("rejected").unwrap_err().to_string();
        assert!(err.contains("rejected"), "{err}");
        assert!(err.contains("review-requested"), "{err}");
        assert!(err.contains("superseded"), "{err}");
    }

    #[test]
    fn branch_type_round_trips_and_rejects_others() {
        for t in BranchType::ALL {
            assert_eq!(BranchType::parse(t.as_str()).unwrap(), t);
        }
        assert!(BranchType::parse("refactor").is_err());
    }

    #[test]
    fn branch_prefix_gives_slug_and_type() {
        assert_eq!(
            from_branch("feat/adopt-otel"),
            ("adopt-otel".to_string(), BranchType::Feat)
        );
        assert_eq!(
            from_branch("fix/leaky-socket"),
            ("leaky-socket".to_string(), BranchType::Fix)
        );
    }

    #[test]
    fn unprefixed_branch_is_a_chore() {
        assert_eq!(
            from_branch("spike-tracing"),
            ("spike-tracing".to_string(), BranchType::Chore)
        );
    }

    #[test]
    fn nested_branch_slug_has_no_separators() {
        let (slug, t) = from_branch("feat/team/adopt-otel");
        assert_eq!(slug, "team-adopt-otel");
        assert_eq!(t, BranchType::Feat);
    }

    #[test]
    fn slugify_collapses_runs_and_trims() {
        assert_eq!(slugify("Adopt  OpenTelemetry!"), "adopt-opentelemetry");
        assert_eq!(slugify("--leading and trailing--"), "leading-and-trailing");
    }

    #[test]
    fn title_from_slug_capitalises_only_the_first_word() {
        assert_eq!(title_from_slug("adopt-otel-tracing"), "Adopt otel tracing");
    }

    #[test]
    fn a_new_record_is_proposed_with_no_history() {
        let r = sample();
        assert_eq!(r.meta.status, Status::Proposed);
        assert!(r.meta.history.is_empty());
        assert_eq!(r.file_name(), "adopt-otel.md");
    }

    #[test]
    fn render_then_parse_round_trips() {
        let original = sample();
        let text = original.render().unwrap();
        let parsed = Record::parse("adopt-otel", &text).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn rendered_file_opens_with_a_frontmatter_fence() {
        let text = sample().render().unwrap();
        assert!(text.starts_with("---\n"), "{text}");
        assert!(
            text.contains("\n---\n\n# Adopt OpenTelemetry for tracing"),
            "{text}"
        );
    }

    #[test]
    fn body_carries_no_metadata_block() {
        let r = sample();
        assert!(!r.body.contains("## Meta"), "{}", r.body);
        assert!(!r.body.contains("**Status**"), "{}", r.body);
    }

    #[test]
    fn template_has_every_scored_heading() {
        let r = sample();
        for heading in [
            "### Context",
            "### Goals",
            "### Non-Goals",
            "### Options Considered",
            "### Chosen Solution",
            "### Rationale",
            "### Key Changes",
            "### Testing Strategy",
            "## Challenges & Solutions",
            "## Impact Assessment",
            "## Risks & Pitfalls",
            "## Outcome & Lessons",
        ] {
            assert!(r.body.contains(heading), "missing {heading}");
        }
    }

    #[test]
    fn transition_records_history_and_ignores_a_repeat() {
        let mut r = sample();
        r.transition(Status::ReviewRequested, "2026-09-15T10:00:00Z");
        r.transition(Status::ReviewRequested, "2026-09-15T10:05:00Z");
        r.transition(Status::Accepted, "2026-09-15T11:00:00Z");

        assert_eq!(r.meta.status, Status::Accepted);
        assert_eq!(r.meta.history.len(), 2);
        assert_eq!(r.meta.history[0].from, Status::Proposed);
        assert_eq!(r.meta.history[0].to, Status::ReviewRequested);
        assert_eq!(r.meta.history[1].to, Status::Accepted);
    }

    #[test]
    fn parse_rejects_a_file_with_no_frontmatter() {
        let err = Record::parse("x", "# Just a heading\n")
            .unwrap_err()
            .to_string();
        assert!(err.contains("frontmatter"), "{err}");
    }

    #[test]
    fn parse_rejects_an_unterminated_fence() {
        let err = Record::parse("x", "---\ntitle: nope\n")
            .unwrap_err()
            .to_string();
        assert!(err.contains("frontmatter"), "{err}");
    }

    #[test]
    fn parse_reports_the_slug_on_bad_yaml() {
        let err = Record::parse("adopt-otel", "---\ntitle: [unclosed\n---\n\nbody\n")
            .unwrap_err()
            .to_string();
        assert!(err.contains("adopt-otel"), "{err}");
    }

    #[test]
    fn resolve_dir_prefers_flag_then_env_then_default() {
        assert_eq!(resolve_dir(Some("a"), Some("b")), PathBuf::from("a"));
        assert_eq!(resolve_dir(None, Some("b")), PathBuf::from("b"));
        assert_eq!(resolve_dir(Some(""), Some("b")), PathBuf::from("b"));
        assert_eq!(resolve_dir(None, None), PathBuf::from(DEFAULT_DIR));
    }
}
