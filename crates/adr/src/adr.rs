//! Core ADR logic: numbering, rendering against the house template, listing,
//! and superseding. Every function is pure with respect to the filesystem and
//! an injectable `today`, so the behaviour is deterministic in tests.

use std::path::{Path, PathBuf};

use common::{Error, Result};

/// The default ADR directory, relative to the repository root.
pub const DEFAULT_DIR: &str = "docs/adr";

/// The environment variable that overrides the ADR directory.
pub const DIR_ENV: &str = "ADR_DIR";

/// A parsed ADR discovered on disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdrEntry {
    /// The zero-padded number parsed from the file name.
    pub number: u32,
    /// The title parsed from the `# ADR-NNNN: <title>` heading.
    pub title: String,
    /// The value of the `**Status:**` line, or `Unknown` when absent.
    pub status: String,
    /// The absolute path to the ADR file.
    pub path: PathBuf,
}

/// The inputs needed to render a single ADR document.
#[derive(Debug, Clone)]
pub struct AdrDraft {
    /// The ADR number (rendered zero-padded to four digits).
    pub number: u32,
    /// The human-readable title.
    pub title: String,
    /// The decision date as `YYYY-MM-DD`.
    pub date: String,
    /// The `**Status:**` value.
    pub status: String,
    /// When set, the number of the ADR this one supersedes.
    pub supersedes: Option<u32>,
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

/// Convert a free-text title into a kebab-case slug: lowercase, non-alphanumeric
/// runs collapsed to single hyphens, with no leading or trailing hyphen.
pub fn slugify(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());
    let mut prev_hyphen = true; // suppress a leading hyphen
    for ch in title.chars() {
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

/// The next ADR number for `dir`: one greater than the highest existing
/// `NNNN-*.md` file, or 1 when the directory is empty or absent.
pub fn next_number(dir: &Path) -> u32 {
    list(dir)
        .unwrap_or_default()
        .iter()
        .map(|e| e.number)
        .max()
        .map_or(1, |n| n + 1)
}

/// The four-digit, zero-padded file-name stem for an ADR, e.g. `0007-use-zustand`.
pub fn file_stem(number: u32, title: &str) -> String {
    format!("{number:04}-{}", slugify(title))
}

/// Render an ADR document from `draft`, following the house ADR template
/// exactly (heading, metadata block, and the Context through References
/// sections).
pub fn render(draft: &AdrDraft) -> String {
    let AdrDraft {
        number,
        title,
        date,
        status,
        supersedes,
    } = draft;

    let technical_story = match supersedes {
        Some(old) => format!("Supersedes ADR-{old:04}"),
        None => "{Link to issue/ticket}".to_string(),
    };

    let context_lead = match supersedes {
        Some(old) => format!(
            "This decision supersedes ADR-{old:04}.\n\nWhat is the issue we're facing in a given context?"
        ),
        None => "What is the issue we're facing in a given context?".to_string(),
    };

    format!(
        "# ADR-{number:04}: {title}\n\
         \n\
         **Status:** {status}\n\
         **Date:** {date}\n\
         **Deciders:** {{List of people involved}}\n\
         **Technical Story:** {technical_story}\n\
         \n\
         ## Context\n\
         \n\
         {context_lead}\n\
         - What are the current constraints?\n\
         - What are the requirements?\n\
         - What forces are at play (technical, business, political)?\n\
         \n\
         ## Decision\n\
         \n\
         What is the change we're proposing/making?\n\
         - Be specific and concrete\n\
         - Use active voice: \"We will...\"\n\
         \n\
         ## Consequences\n\
         \n\
         What becomes easier or more difficult to do because of this change?\n\
         \n\
         ### Positive\n\
         - Benefit 1\n\
         \n\
         ### Negative\n\
         - Drawback 1\n\
         \n\
         ### Neutral\n\
         - Implication 1\n\
         \n\
         ## Alternatives Considered\n\
         \n\
         What other options did we evaluate?\n\
         \n\
         ### Alternative 1: {{Name}}\n\
         - Description\n\
         - Pros\n\
         - Cons\n\
         - Why rejected\n\
         \n\
         ## Compliance\n\
         \n\
         How will we ensure this decision is followed?\n\
         - Code review checks\n\
         - Automated tests\n\
         - Linting rules\n\
         - Documentation updates\n\
         \n\
         ## References\n\
         \n\
         - [Link to relevant docs]\n\
         - [Link to related ADRs]\n"
    )
}

/// Create the next-numbered ADR in `dir` from the house template and return its
/// path. The directory is created if missing. `today` is the date to stamp.
pub fn create_new(dir: &Path, title: &str, today: &str) -> Result<PathBuf> {
    std::fs::create_dir_all(dir).map_err(|e| Error::io(dir, e))?;
    let number = next_number(dir);
    let draft = AdrDraft {
        number,
        title: title.to_string(),
        date: today.to_string(),
        status: "Proposed".to_string(),
        supersedes: None,
    };
    write_draft(dir, &draft)
}

/// Supersede ADR `old_number` with a newly-numbered ADR titled `new_title`.
///
/// Sets the old ADR's `**Status:**` to `Superseded by ADR-<new>` and creates
/// the new ADR (status `Accepted`) referencing the old one. Returns
/// `(old_path, new_path)`.
pub fn supersede(
    dir: &Path,
    old_number: u32,
    new_title: &str,
    today: &str,
) -> Result<(PathBuf, PathBuf)> {
    let entries = list(dir)?;
    let old = entries
        .iter()
        .find(|e| e.number == old_number)
        .ok_or_else(|| {
            Error::Config(format!(
                "no ADR numbered {old_number:04} in {}",
                dir.display()
            ))
        })?;
    let old_path = old.path.clone();

    let new_number = next_number(dir);
    let draft = AdrDraft {
        number: new_number,
        title: new_title.to_string(),
        date: today.to_string(),
        status: "Accepted".to_string(),
        supersedes: Some(old_number),
    };
    let new_path = write_draft(dir, &draft)?;

    let updated = set_status(&old_path, &format!("Superseded by ADR-{new_number:04}"))?;
    std::fs::write(&old_path, updated).map_err(|e| Error::io(&old_path, e))?;

    Ok((old_path, new_path))
}

/// List every ADR in `dir`, sorted by number ascending. Returns an empty list
/// when the directory does not exist.
pub fn list(dir: &Path) -> Result<Vec<AdrEntry>> {
    let read = match std::fs::read_dir(dir) {
        Ok(read) => read,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(Error::io(dir, e)),
    };

    let mut entries = Vec::new();
    for entry in read {
        let entry = entry.map_err(|e| Error::io(dir, e))?;
        let path = entry.path();
        let Some(number) = number_from_path(&path) else {
            continue;
        };
        let body = std::fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
        entries.push(AdrEntry {
            number,
            title: parse_title(&body).unwrap_or_else(|| "(untitled)".to_string()),
            status: parse_status(&body).unwrap_or_else(|| "Unknown".to_string()),
            path,
        });
    }
    entries.sort_by_key(|e| e.number);
    Ok(entries)
}

/// Write `draft` into `dir` and return the written path.
fn write_draft(dir: &Path, draft: &AdrDraft) -> Result<PathBuf> {
    let path = dir.join(format!("{}.md", file_stem(draft.number, &draft.title)));
    std::fs::write(&path, render(draft)).map_err(|e| Error::io(&path, e))?;
    Ok(path)
}

/// Parse the leading `NNNN` number from a `NNNN-*.md` file path.
fn number_from_path(path: &Path) -> Option<u32> {
    if path.extension().and_then(|e| e.to_str()) != Some("md") {
        return None;
    }
    let stem = path.file_stem().and_then(|s| s.to_str())?;
    let digits: String = stem.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    // Require the digits to be followed by a hyphen (or be the whole stem),
    // so files like `README.md` or `notes.md` are ignored.
    let rest = &stem[digits.len()..];
    if !rest.is_empty() && !rest.starts_with('-') {
        return None;
    }
    digits.parse().ok()
}

/// Extract the title from the first `# ADR-NNNN: <title>` heading.
fn parse_title(body: &str) -> Option<String> {
    body.lines().find_map(|line| {
        let heading = line.strip_prefix("# ")?;
        let (_, title) = heading.split_once(": ")?;
        Some(title.trim().to_string())
    })
}

/// Extract the value of the first `**Status:**` line.
fn parse_status(body: &str) -> Option<String> {
    body.lines().find_map(|line| {
        line.trim()
            .strip_prefix("**Status:**")
            .map(|rest| rest.trim().to_string())
    })
}

/// Return `body` with the first `**Status:**` line rewritten to `new_status`.
fn set_status(path: &Path, new_status: &str) -> Result<String> {
    let body = std::fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
    let mut replaced = false;
    let mut out = String::with_capacity(body.len());
    for line in body.lines() {
        if !replaced && line.trim_start().starts_with("**Status:**") {
            out.push_str(&format!("**Status:** {new_status}"));
            replaced = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    if !replaced {
        return Err(Error::Config(format!(
            "no **Status:** line found in {}",
            path.display()
        )));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn slugify_kebabs_and_trims() {
        assert_eq!(slugify("Use Zustand for State!"), "use-zustand-for-state");
        assert_eq!(slugify("  Adopt  Rust   "), "adopt-rust");
        assert_eq!(slugify("API/v2 & gRPC"), "api-v2-grpc");
    }

    #[test]
    fn resolve_dir_precedence() {
        assert_eq!(
            resolve_dir(Some("flag/dir"), Some("env/dir")),
            PathBuf::from("flag/dir")
        );
        assert_eq!(resolve_dir(None, Some("env/dir")), PathBuf::from("env/dir"));
        assert_eq!(resolve_dir(None, None), PathBuf::from(DEFAULT_DIR));
        assert_eq!(
            resolve_dir(Some(""), Some("env/dir")),
            PathBuf::from("env/dir")
        );
    }

    #[test]
    fn next_number_starts_at_one() {
        let dir = tmp();
        assert_eq!(next_number(dir.path()), 1);
    }

    #[test]
    fn create_new_numbers_sequentially() {
        let dir = tmp();
        let first = create_new(dir.path(), "First decision", "2026-07-22").unwrap();
        let second = create_new(dir.path(), "Second decision", "2026-07-22").unwrap();
        assert!(first.ends_with("0001-first-decision.md"));
        assert!(second.ends_with("0002-second-decision.md"));
    }

    #[test]
    fn create_new_respects_gaps_by_max_plus_one() {
        let dir = tmp();
        std::fs::write(
            dir.path().join("0005-existing.md"),
            "# ADR-0005: Existing\n",
        )
        .unwrap();
        let next = create_new(dir.path(), "New one", "2026-07-22").unwrap();
        assert!(next.ends_with("0006-new-one.md"));
    }

    #[test]
    fn render_matches_house_template_fields() {
        let draft = AdrDraft {
            number: 7,
            title: "Enable Strict Mode".to_string(),
            date: "2026-07-22".to_string(),
            status: "Proposed".to_string(),
            supersedes: None,
        };
        let doc = render(&draft);
        assert!(doc.starts_with("# ADR-0007: Enable Strict Mode\n"));
        assert!(doc.contains("**Status:** Proposed"));
        assert!(doc.contains("**Date:** 2026-07-22"));
        assert!(doc.contains("**Deciders:**"));
        assert!(doc.contains("**Technical Story:**"));
        for heading in [
            "## Context",
            "## Decision",
            "## Consequences",
            "### Positive",
            "### Negative",
            "### Neutral",
            "## Alternatives Considered",
            "## Compliance",
        ] {
            assert!(doc.contains(heading), "missing {heading}");
        }
    }

    #[test]
    fn list_parses_number_title_and_status() {
        let dir = tmp();
        create_new(dir.path(), "Only decision", "2026-07-22").unwrap();
        let entries = list(dir.path()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].number, 1);
        assert_eq!(entries[0].title, "Only decision");
        assert_eq!(entries[0].status, "Proposed");
    }

    #[test]
    fn list_ignores_non_adr_files() {
        let dir = tmp();
        std::fs::write(dir.path().join("README.md"), "# Readme\n").unwrap();
        std::fs::write(
            dir.path().join("0001-real.md"),
            "# ADR-0001: Real\n\n**Status:** Accepted\n",
        )
        .unwrap();
        let entries = list(dir.path()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].number, 1);
    }

    #[test]
    fn supersede_updates_old_and_links_new() {
        let dir = tmp();
        create_new(dir.path(), "Old approach", "2026-07-22").unwrap();
        let (old_path, new_path) = supersede(dir.path(), 1, "New approach", "2026-07-23").unwrap();

        let old_body = std::fs::read_to_string(&old_path).unwrap();
        assert!(old_body.contains("**Status:** Superseded by ADR-0002"));

        let new_body = std::fs::read_to_string(&new_path).unwrap();
        assert!(new_path.ends_with("0002-new-approach.md"));
        assert!(new_body.contains("**Status:** Accepted"));
        assert!(new_body.contains("Supersedes ADR-0001"));
        assert!(new_body.contains("This decision supersedes ADR-0001."));
    }

    #[test]
    fn supersede_unknown_number_errors() {
        let dir = tmp();
        let err = supersede(dir.path(), 99, "Nope", "2026-07-22").unwrap_err();
        assert!(err.to_string().contains("no ADR numbered 0099"));
    }
}
