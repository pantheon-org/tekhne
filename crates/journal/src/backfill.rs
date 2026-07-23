//! Backfill missing YAML frontmatter (`title`, `date`) across existing entries.
//!
//! Port of the journal CLI's `backfill/` tree. For every dated entry file it
//! ensures a `title:` and `date:` frontmatter key exist: a file with no
//! frontmatter gets a full block prepended (`title`, `date`, `tags: []`); a file
//! with frontmatter that lacks either key has the missing lines spliced in after
//! the opening fence. The `title` is taken from existing frontmatter or the body
//! H1; the `date` from the filename prefix. Existing keys are never overwritten.

use std::collections::BTreeSet;
use std::path::Path;

use common::Result;
use regex::Regex;

use crate::scan::{first_h1, is_dated, list_markdown, parse_frontmatter};

/// The outcome of a backfill pass.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BackfillReport {
    /// Number of dated files examined.
    pub processed: usize,
    /// Number of files a `title:` line was injected into.
    pub titles_injected: usize,
    /// Number of files a `date:` line was injected into.
    pub dates_injected: usize,
    /// Dated files that carry no frontmatter tags (for the caller to warn).
    pub missing_tags: Vec<String>,
    /// Read/write failures, kept rather than aborting the whole pass.
    pub warnings: Vec<String>,
}

/// Escape double quotes so a title is a valid double-quoted YAML scalar.
fn escape_quotes(s: &str) -> String {
    s.replace('"', "\\\"")
}

/// The frontmatter block lines and closing-fence line index, or `None` when
/// there is no `---` fenced block with a closing fence.
fn frontmatter_block(content: &str) -> Option<(Vec<&str>, usize)> {
    if !content.starts_with("---") {
        return None;
    }
    let lines: Vec<&str> = content.split('\n').collect();
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            return Some((lines[1..i].to_vec(), i));
        }
    }
    None
}

/// The top-level keys present in a frontmatter block.
fn block_keys(block: &[&str]) -> BTreeSet<String> {
    let key_re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*):").expect("valid key regex");
    let mut keys = BTreeSet::new();
    for line in block {
        if let Some(caps) = key_re.captures(line) {
            keys.insert(caps[1].to_string());
        }
    }
    keys
}

/// Inject `title:` and/or `date:` into a document's frontmatter, adding only the
/// keys that are absent. A document without frontmatter gets a fresh block.
pub fn inject_frontmatter(content: &str, title: &str, date: &str) -> String {
    let Some((block, _)) = frontmatter_block(content) else {
        // The blank line before the closing fence stops markdownlint (which does
        // not parse YAML) from reading `tags: []` + `---` as a setext heading.
        return format!(
            "---\ntitle: \"{}\"\ndate: {}\ntags: []\n\n---\n\n{}",
            escape_quotes(title),
            date,
            content
        );
    };
    let keys = block_keys(&block);
    let mut to_add: Vec<String> = Vec::new();
    if !keys.contains("title") {
        to_add.push(format!("title: \"{}\"", escape_quotes(title)));
    }
    if !keys.contains("date") {
        to_add.push(format!("date: {date}"));
    }
    if to_add.is_empty() {
        return content.to_string();
    }
    // Splice the new lines directly after the opening `---`.
    let mut lines: Vec<String> = content.split('\n').map(str::to_string).collect();
    for (offset, line) in to_add.into_iter().enumerate() {
        lines.insert(1 + offset, line);
    }
    lines.join("\n")
}

/// True when `content` has no top-level `title:` / `date:` line anywhere.
fn missing_key(content: &str, key: &str) -> bool {
    let re = Regex::new(&format!(r"(?m)^\s*{key}:")).expect("valid key regex");
    !re.is_match(content)
}

/// The `YYYY-MM-DD` date prefix of a dated file's basename.
fn filename_date(rel: &str) -> String {
    let base = rel.rsplit('/').next().unwrap_or(rel);
    base.chars().take(10).collect()
}

/// True when a file's basename is a `YYYY-MM-DD-` dated stem.
fn is_dated_file(rel: &str) -> bool {
    let base = rel.rsplit('/').next().unwrap_or(rel);
    is_dated(base)
}

/// Run the backfill over every dated entry under `root`. When `dry_run` is set,
/// nothing is written but the report still reflects what would change.
pub fn backfill(root: &Path, dry_run: bool) -> Result<BackfillReport> {
    let files = list_markdown(root);
    let mut report = BackfillReport::default();

    for rel in files.into_iter().filter(|f| is_dated_file(f)) {
        report.processed += 1;
        let content = match std::fs::read_to_string(root.join(&rel)) {
            Ok(c) => c,
            Err(e) => {
                report.warnings.push(format!("failed to read {rel}: {e}"));
                continue;
            }
        };

        let (fm, body) = parse_frontmatter(&content);
        if fm.tags.is_empty() {
            report.missing_tags.push(rel.clone());
        }

        let title_missing = missing_key(&content, "title");
        let date_missing = missing_key(&content, "date");
        if !title_missing && !date_missing {
            continue;
        }

        let title = if fm.title.is_empty() {
            first_h1(&body)
        } else {
            fm.title.clone()
        };
        let date = filename_date(&rel);
        let new_content = inject_frontmatter(&content, &title, &date);

        if title_missing {
            report.titles_injected += 1;
        }
        if date_missing {
            report.dates_injected += 1;
        }

        if new_content != content && !dry_run {
            if let Err(e) = std::fs::write(root.join(&rel), &new_content) {
                report.warnings.push(format!("failed to write {rel}: {e}"));
            }
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injects_full_block_when_no_frontmatter() {
        let out = inject_frontmatter("# Heading\n\nbody\n", "My Title", "2026-07-01");
        assert!(out.starts_with("---\ntitle: \"My Title\"\ndate: 2026-07-01\ntags: []\n\n---\n\n"));
        assert!(out.contains("# Heading"));
    }

    #[test]
    fn splices_missing_keys_into_existing_block() {
        let content = "---\ntags:\n  - x\n---\n\n# T\n";
        let out = inject_frontmatter(content, "The Title", "2026-07-02");
        assert!(out.contains("title: \"The Title\""));
        assert!(out.contains("date: 2026-07-02"));
        // Existing tags preserved.
        assert!(out.contains("tags:"));
        // New keys sit right after the opening fence.
        let lines: Vec<&str> = out.split('\n').collect();
        assert_eq!(lines[0], "---");
        assert!(lines[1].starts_with("title:") || lines[1].starts_with("date:"));
    }

    #[test]
    fn leaves_content_untouched_when_both_present() {
        let content = "---\ntitle: X\ndate: 2026-07-01\n---\n\n# T\n";
        assert_eq!(
            inject_frontmatter(content, "ignored", "2026-01-01"),
            content
        );
    }

    #[test]
    fn escapes_quotes_in_title() {
        let out = inject_frontmatter("body\n", "A \"quoted\" title", "2026-07-01");
        assert!(out.contains(r#"title: "A \"quoted\" title""#));
    }

    #[test]
    fn backfill_applies_and_reports() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("2026/07")).unwrap();
        // No frontmatter at all; H1 supplies the title.
        std::fs::write(
            root.join("2026/07/2026-07-01-alpha.md"),
            "# Alpha Entry\n\ntext\n",
        )
        .unwrap();
        // Frontmatter present but missing title/date; also no tags.
        std::fs::write(
            root.join("2026/07/2026-07-02-beta.md"),
            "---\nstatus: draft\n---\n\n# Beta\n",
        )
        .unwrap();

        let report = backfill(root, false).unwrap();
        assert_eq!(report.processed, 2);
        assert_eq!(report.titles_injected, 2);
        assert_eq!(report.dates_injected, 2);
        assert_eq!(report.missing_tags.len(), 2);
        assert!(report.warnings.is_empty());

        let alpha = std::fs::read_to_string(root.join("2026/07/2026-07-01-alpha.md")).unwrap();
        assert!(alpha.contains("title: \"Alpha Entry\""));
        assert!(alpha.contains("date: 2026-07-01"));

        let beta = std::fs::read_to_string(root.join("2026/07/2026-07-02-beta.md")).unwrap();
        assert!(beta.contains("date: 2026-07-02"));
        assert!(beta.contains("status: draft"));
    }

    #[test]
    fn dry_run_writes_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("2026/07")).unwrap();
        let path = root.join("2026/07/2026-07-01-alpha.md");
        std::fs::write(&path, "# Alpha\n").unwrap();

        let report = backfill(root, true).unwrap();
        assert_eq!(report.titles_injected, 1);
        // File is unchanged on disk.
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "# Alpha\n");
    }
}
