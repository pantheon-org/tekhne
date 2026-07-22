//! Compliance validation for journal entries, ported from
//! `scripts/validate-journal-entry.sh`.
//!
//! The checks run in the same order the shell script uses and stop at the first
//! failure: filename date and lowercase slug, a single H1, the required
//! sections, code fences carrying a language, headings free of trailing
//! punctuation, well-formed tags, an H1 ending in the long date, and the
//! `## Proposed Ticket Description` section when the frontmatter names a
//! `refinement_ticket`. Two checks the shell script implies but does not run
//! are added here at the task's request: frontmatter must be present, and the
//! frontmatter `date` must match the filename date (triple-sync).
//!
//! Files under a `.templates` directory, or not inside a `YYYY/` directory, are
//! skipped, exactly as the shell script skips them.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use regex::Regex;

use crate::date::Date;

/// The result of validating a file that is subject to the checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The file passed every applicable check.
    Validated,
    /// The file is a template or lives outside a `YYYY/` tree, so it is not a
    /// journal entry and is not checked.
    Skipped,
}

/// A specific compliance failure. Variants mirror the shell script's numbered
/// exit paths, plus the two added triple-sync checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Violation {
    /// The path does not point at an existing file.
    NotFound(PathBuf),
    /// The filename does not start with `YYYY-MM-DD`.
    BadFilenameDate(String),
    /// The filename slug (after the date) contains uppercase letters.
    UppercaseSlug(String),
    /// No YAML frontmatter block was found at the top of the file.
    MissingFrontmatter,
    /// The document does not contain exactly one H1.
    H1Count(usize),
    /// A required section heading is absent.
    MissingSection(&'static str),
    /// One or more opening code fences omit a language specifier.
    BareCodeFence(Vec<usize>),
    /// One or more headings end with trailing punctuation.
    HeadingPunctuation(Vec<usize>),
    /// The `## Tags` section is present but empty.
    EmptyTags,
    /// The `## Tags` section contains no recognisable tag tokens.
    NoTagTokens,
    /// One or more tags break the lowercase, hyphen-separated convention.
    InvalidTags(Vec<String>),
    /// The H1 does not end with the expected long-form date.
    H1DateMismatch {
        /// The `Month D, YYYY` string the H1 was expected to end with.
        expected: String,
    },
    /// The frontmatter `date` does not match the filename date.
    DateDesync {
        /// The `date` value found in the frontmatter.
        frontmatter: String,
        /// The date parsed from the filename.
        filename: String,
    },
    /// `refinement_ticket` is set but the proposed-description section is
    /// missing.
    MissingProposedTicketDescription(String),
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::NotFound(p) => write!(f, "file not found: {}", p.display()),
            Violation::BadFilenameDate(name) => {
                write!(f, "filename must start with YYYY-MM-DD (got: {name})")
            }
            Violation::UppercaseSlug(name) => {
                write!(f, "filename slug must be lowercase-only (in: {name})")
            }
            Violation::MissingFrontmatter => {
                write!(f, "missing YAML frontmatter block at top of file")
            }
            Violation::H1Count(n) => write!(f, "expected exactly one H1 (found {n})"),
            Violation::MissingSection(s) => write!(f, "missing {s} section"),
            Violation::BareCodeFence(lines) => write!(
                f,
                "found opening code fence(s) without a language specifier at line(s): {}",
                join_lines(lines)
            ),
            Violation::HeadingPunctuation(lines) => write!(
                f,
                "found heading(s) with trailing punctuation at line(s): {}",
                join_lines(lines)
            ),
            Violation::EmptyTags => write!(f, "## Tags section is empty"),
            Violation::NoTagTokens => write!(f, "no tag tokens detected in ## Tags section"),
            Violation::InvalidTags(tags) => write!(
                f,
                "found tag(s) that break the lowercase, hyphen-separated convention: {}",
                tags.join(" ")
            ),
            Violation::H1DateMismatch { expected } => write!(
                f,
                "H1 must end with the full date formatted as 'Month D, YYYY' (expected ending: {expected})"
            ),
            Violation::DateDesync {
                frontmatter,
                filename,
            } => write!(
                f,
                "frontmatter date ({frontmatter}) does not match filename date ({filename})"
            ),
            Violation::MissingProposedTicketDescription(ticket) => write!(
                f,
                "refinement_ticket ({ticket}) is set but there is no '## Proposed Ticket Description' section"
            ),
        }
    }
}

fn join_lines(lines: &[usize]) -> String {
    lines
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

const REQUIRED_SECTIONS: [&str; 3] = ["## Session Overview", "## Compliance", "## Tags"];

/// Validate the file at `path`, returning the outcome or the first violation.
pub fn validate(path: &Path) -> Result<Outcome, Violation> {
    if !path.is_file() {
        return Err(Violation::NotFound(path.to_path_buf()));
    }

    let path_str = path.to_string_lossy().replace('\\', "/");
    if path_str.contains("/.templates/") || path_str.starts_with(".templates/") {
        return Ok(Outcome::Skipped);
    }
    if !year_dir_re().is_match(&path_str) {
        return Ok(Outcome::Skipped);
    }

    let basename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string();

    let file_date = match filename_date_re().captures(&basename) {
        Some(c) => c.get(1).unwrap().as_str().to_string(),
        None => return Err(Violation::BadFilenameDate(basename)),
    };

    let slug = basename
        .strip_prefix(&format!("{file_date}-"))
        .unwrap_or(&basename);
    if slug.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(Violation::UppercaseSlug(basename));
    }

    let content =
        std::fs::read_to_string(path).map_err(|_| Violation::NotFound(path.to_path_buf()))?;

    if !has_frontmatter(&content) {
        return Err(Violation::MissingFrontmatter);
    }

    let scan = Scan::new(&content);

    let h1_lines: Vec<&Line> = scan
        .outside
        .iter()
        .filter(|l| l.text.starts_with("# "))
        .collect();
    if h1_lines.len() != 1 {
        return Err(Violation::H1Count(h1_lines.len()));
    }
    let h1_text = h1_lines[0].text.trim_start_matches("# ").trim();

    for title in REQUIRED_SECTIONS {
        if !scan.outside.iter().any(|l| l.text == title) {
            return Err(Violation::MissingSection(title));
        }
    }

    if !scan.bad_fences.is_empty() {
        return Err(Violation::BareCodeFence(scan.bad_fences.clone()));
    }

    let bad_headings = scan.heading_punctuation_lines();
    if !bad_headings.is_empty() {
        return Err(Violation::HeadingPunctuation(bad_headings));
    }

    scan.validate_tags()?;

    let expected = long_date(&file_date);
    if !h1_text.ends_with(&expected) {
        return Err(Violation::H1DateMismatch { expected });
    }

    if let Some(fm_date) = frontmatter_field(&content, "date") {
        if !fm_date.is_empty() && fm_date != file_date {
            return Err(Violation::DateDesync {
                frontmatter: fm_date,
                filename: file_date,
            });
        }
    }

    if let Some(ticket) = frontmatter_field(&content, "refinement_ticket") {
        if !ticket.is_empty()
            && !scan
                .outside
                .iter()
                .any(|l| l.text == "## Proposed Ticket Description")
        {
            return Err(Violation::MissingProposedTicketDescription(ticket));
        }
    }

    Ok(Outcome::Validated)
}

/// One source line with its 1-based number, kept for error reporting.
struct Line {
    number: usize,
    text: String,
}

/// A single pass over the document that records the lines outside fenced code
/// blocks and the line numbers of opening fences that omit a language.
struct Scan {
    outside: Vec<Line>,
    bad_fences: Vec<usize>,
}

impl Scan {
    fn new(content: &str) -> Self {
        let mut outside = Vec::new();
        let mut bad_fences = Vec::new();
        let mut in_fence = false;
        for (idx, raw) in content.lines().enumerate() {
            let number = idx + 1;
            let ticks = raw.chars().take_while(|c| *c == '`').count();
            if ticks >= 3 {
                if !in_fence {
                    if raw[ticks..].trim().is_empty() {
                        bad_fences.push(number);
                    }
                    in_fence = true;
                } else {
                    in_fence = false;
                }
                continue;
            }
            if !in_fence {
                outside.push(Line {
                    number,
                    text: raw.to_string(),
                });
            }
        }
        Self {
            outside,
            bad_fences,
        }
    }

    /// Line numbers of headings ending in `. : ; ! ?`.
    fn heading_punctuation_lines(&self) -> Vec<usize> {
        self.outside
            .iter()
            .filter(|l| is_heading(&l.text) && ends_with_punctuation(l.text.trim_end()))
            .map(|l| l.number)
            .collect()
    }

    /// Port of the shell tag checks: the `## Tags` block must be non-empty,
    /// yield tokens, and every token must follow the tag convention.
    fn validate_tags(&self) -> Result<(), Violation> {
        let Some(start) = self.outside.iter().position(|l| l.text == "## Tags") else {
            return Ok(());
        };
        let block: Vec<&str> = self.outside[start + 1..]
            .iter()
            .take_while(|l| !l.text.starts_with("## "))
            .map(|l| l.text.as_str())
            .filter(|t| !t.trim().is_empty())
            .collect();
        if block.is_empty() {
            return Err(Violation::EmptyTags);
        }

        let joined = block.join("\n");
        let tokens: Vec<String> = tag_token_re()
            .find_iter(&joined)
            .map(|m| m.as_str().to_string())
            .collect();
        if tokens.is_empty() {
            return Err(Violation::NoTagTokens);
        }

        let mut invalid = Vec::new();
        for token in &tokens {
            let cleaned = token.trim().trim_start_matches('#').to_ascii_lowercase();
            if !tag_shape_re().is_match(&cleaned) {
                invalid.push(token.clone());
            }
        }
        if !invalid.is_empty() {
            return Err(Violation::InvalidTags(invalid));
        }
        Ok(())
    }
}

/// True when the trimmed line is a markdown heading (`#` to `######` plus space).
fn is_heading(line: &str) -> bool {
    let hashes = line.chars().take_while(|c| *c == '#').count();
    (1..=6).contains(&hashes) && line[hashes..].starts_with(' ')
}

fn ends_with_punctuation(line: &str) -> bool {
    matches!(
        line.chars().last(),
        Some('.') | Some(':') | Some(';') | Some('!') | Some('?')
    )
}

/// True when `content` opens with a `---` fenced YAML frontmatter block.
fn has_frontmatter(content: &str) -> bool {
    let mut lines = content.lines();
    if lines.next().map(str::trim) != Some("---") {
        return false;
    }
    lines.any(|l| l.trim() == "---")
}

/// Extract the first top-level frontmatter field value, mirroring the shell
/// `frontmatter_field` helper (strips one layer of surrounding quotes).
fn frontmatter_field(content: &str, field: &str) -> Option<String> {
    let mut lines = content.lines();
    if lines.next().map(str::trim) != Some("---") {
        return None;
    }
    let prefix = format!("{field}:");
    for line in lines {
        if line.trim() == "---" {
            break;
        }
        if let Some(rest) = line.strip_prefix(&prefix) {
            let value = rest.trim().trim_matches('"').trim().to_string();
            return Some(value);
        }
    }
    None
}

/// Format an ISO `YYYY-MM-DD` string as `Month D, YYYY`.
fn long_date(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    let year = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
    let month = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let day = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    Date::new(year, month, day).long()
}

fn year_dir_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(^|/)[0-9]{4}/").unwrap())
}

fn filename_date_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([0-9]{4}-[0-9]{2}-[0-9]{2})").unwrap())
}

fn tag_token_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"#?[A-Za-z0-9][A-Za-z0-9_/ -]*").unwrap())
}

fn tag_shape_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^([a-z0-9]+(-[a-z0-9]+)*)(/([a-z0-9]+(-[a-z0-9]+)*))*$").unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    fn write(dir: &Path, rel: &str, content: &str) -> PathBuf {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        path
    }

    const GOOD: &str = "---\ntitle: \"Example\"\ndate: 2026-07-22\ntags:\n  - journal\n---\n\n<!-- markdownlint-disable MD025 -->\n\n# Example - July 22, 2026\n\n## Session Overview\n\nText.\n\n## Compliance\n\nText.\n\n## Tags\n\n`journal` | `example`\n";

    #[test]
    fn good_entry_validates() {
        let tmp = tempfile::tempdir().unwrap();
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", GOOD);
        assert_eq!(validate(&path), Ok(Outcome::Validated));
    }

    #[test]
    fn outside_year_dir_is_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let path = write(tmp.path(), "notes/2026-07-22-example.md", GOOD);
        assert_eq!(validate(&path), Ok(Outcome::Skipped));
    }

    #[test]
    fn uppercase_slug_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let path = write(tmp.path(), "2026/07/2026-07-22-Example.md", GOOD);
        assert!(matches!(validate(&path), Err(Violation::UppercaseSlug(_))));
    }

    #[test]
    fn missing_h1_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace("# Example - July 22, 2026", "Example without heading");
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert!(matches!(validate(&path), Err(Violation::H1Count(0))));
    }

    #[test]
    fn missing_required_section_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace("## Compliance\n\nText.\n\n", "");
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert_eq!(
            validate(&path),
            Err(Violation::MissingSection("## Compliance"))
        );
    }

    #[test]
    fn bare_code_fence_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace("Text.\n\n## Compliance", "```\nbare\n```\n\n## Compliance");
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert!(matches!(validate(&path), Err(Violation::BareCodeFence(_))));
    }

    #[test]
    fn heading_trailing_punctuation_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        // Keep the required sections intact and add a separate offending
        // heading; the required-section check runs first otherwise.
        let body = GOOD.replace("## Compliance\n", "## Notes:\n\nText.\n\n## Compliance\n");
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert!(matches!(
            validate(&path),
            Err(Violation::HeadingPunctuation(_))
        ));
    }

    #[test]
    fn invalid_tag_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace("`journal` | `example`", "`Journal_Bad`");
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert!(matches!(validate(&path), Err(Violation::InvalidTags(_))));
    }

    #[test]
    fn h1_date_mismatch_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace("# Example - July 22, 2026", "# Example - July 23, 2026");
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert!(matches!(
            validate(&path),
            Err(Violation::H1DateMismatch { .. })
        ));
    }

    #[test]
    fn frontmatter_date_desync_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace("date: 2026-07-22", "date: 2026-07-20");
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert!(matches!(validate(&path), Err(Violation::DateDesync { .. })));
    }

    #[test]
    fn missing_proposed_ticket_description_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace(
            "date: 2026-07-22\n",
            "date: 2026-07-22\nrefinement_ticket: TICKET-1\n",
        );
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert!(matches!(
            validate(&path),
            Err(Violation::MissingProposedTicketDescription(_))
        ));
    }

    #[test]
    fn missing_frontmatter_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replacen(
            "---\ntitle: \"Example\"\ndate: 2026-07-22\ntags:\n  - journal\n---\n",
            "",
            1,
        );
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert_eq!(validate(&path), Err(Violation::MissingFrontmatter));
    }

    #[test]
    fn fenced_heading_does_not_count_as_h1() {
        let tmp = tempfile::tempdir().unwrap();
        let body = GOOD.replace(
            "## Tags",
            "## Extra\n\n```markdown\n# not a real h1\n```\n\n## Tags",
        );
        let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
        assert_eq!(validate(&path), Ok(Outcome::Validated));
    }
}
