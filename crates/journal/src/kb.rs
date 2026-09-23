//! Generate a knowledge-base browse index: a separate corpus of frontmatter-only
//! articles (not dated journal entries), grouped by their top-level directory
//! ("domain") rather than by date.
//!
//! Port of the journal CLI's `kb-index/` tree. Reuses this crate's existing
//! frontmatter line-scanning helpers (`scan::split_frontmatter`, `fm_scalar`,
//! `fm_sequence`) rather than a full YAML parser, matching how `scan.rs`
//! already treats journal entries. One known compatibility gap against the
//! TypeScript original: that implementation rejects an article whose
//! frontmatter carries any field outside `title`/`topic`/`created_at`/
//! `updated_at`/`tags` (a Zod `strictObject`); this scanner does not enforce
//! that, since the shared helpers extract named keys and ignore the rest.

use std::path::Path;

use serde::Serialize;

use crate::scan::{fm_scalar, fm_sequence, split_frontmatter};

/// Default directory scanned for knowledge-base articles.
pub const KB_INDEX_ROOT: &str = "docs/knowledge-base";
/// Default machine-readable index path (the source of truth).
pub const KB_INDEX_DATA_PATH: &str = "docs/knowledge-base-index.ndjson";
/// Default human-readable index path (rendered from the data).
pub const KB_INDEX_VIEW_PATH: &str = "docs/knowledge-base-index.md";

/// One record in the knowledge-base index NDJSON: one line equals one
/// article. Field order here is the serialisation order, kept fixed for
/// deterministic output. `created_at`/`updated_at`/`tags` are omitted (not
/// serialised as `null`) when absent, matching articles written before those
/// fields existed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KbRecord {
    pub file: String,
    pub domain: String,
    pub title: String,
    pub topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// An article that could not be scanned, kept rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KbScanError {
    /// Root-relative forward-slash path of the file.
    pub file: String,
    /// Human-readable failure description.
    pub error: String,
}

/// The result of a knowledge-base corpus scan.
#[derive(Debug, Clone, Default)]
pub struct KbScanResult {
    /// Records, sorted by domain then title (matching the rendered view).
    pub records: Vec<KbRecord>,
    /// Articles that failed to parse.
    pub errors: Vec<KbScanError>,
}

/// True if `s` looks like an ISO calendar date (`YYYY-MM-DD`); a coarse check
/// mirroring the regex the TypeScript schema uses, without pulling in a date
/// library for a single format check.
fn looks_like_iso_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[0..4].iter().all(u8::is_ascii_digit)
        && b[4] == b'-'
        && b[5..7].iter().all(u8::is_ascii_digit)
        && b[7] == b'-'
        && b[8..10].iter().all(u8::is_ascii_digit)
}

/// Recursively collect root-relative forward-slash paths of every `.md` file
/// under `root`, sorted.
fn list_markdown(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    walk(root, "", &mut out);
    out.sort();
    out
}

fn walk(dir: &Path, rel: &str, out: &mut Vec<String>) {
    let Ok(read) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let child_rel = if rel.is_empty() {
            name.to_string()
        } else {
            format!("{rel}/{name}")
        };
        let path = entry.path();
        if path.is_dir() {
            walk(&path, &child_rel, out);
        } else if name.ends_with(".md") {
            out.push(child_rel);
        }
    }
}

/// Extract an optional ISO-date field, erroring rather than silently accepting
/// a malformed value.
fn parse_optional_iso_date(block: &[&str], key: &str) -> Result<Option<String>, String> {
    match fm_scalar(block, key).as_str() {
        "" => Ok(None),
        s if looks_like_iso_date(s) => Ok(Some(s.to_string())),
        s => Err(format!(
            "frontmatter schema violation: {key} \"{s}\" must be an ISO date (YYYY-MM-DD)"
        )),
    }
}

/// Parse one article's frontmatter into a record, or an error. `file` is the
/// root-relative path; its first path segment becomes `domain`.
fn parse_article(file: &str, content: &str) -> Result<KbRecord, String> {
    if !content.starts_with("---") {
        return Err("missing frontmatter block".to_string());
    }
    let (block, _body) = split_frontmatter(content);
    let Some(block) = block else {
        return Err("unterminated frontmatter block".to_string());
    };

    let title = fm_scalar(&block, "title");
    if title.is_empty() {
        return Err("frontmatter schema violation: title is required".to_string());
    }
    let topic = fm_scalar(&block, "topic");
    if topic.is_empty() {
        return Err("frontmatter schema violation: topic is required".to_string());
    }

    let created_at = parse_optional_iso_date(&block, "created_at")?;
    let updated_at = parse_optional_iso_date(&block, "updated_at")?;
    let tags = fm_sequence(&block, "tags");
    let tags = if tags.is_empty() { None } else { Some(tags) };

    let domain = file.split('/').next().unwrap_or("").to_string();

    Ok(KbRecord {
        file: file.to_string(),
        domain,
        title,
        topic,
        created_at,
        updated_at,
        tags,
    })
}

/// Scan every article under `root`, parsing frontmatter into records. Records
/// are sorted by domain then title so the rendered view is stable across
/// regenerations. Unreadable or malformed files are collected in `errors`,
/// never silently dropped.
pub fn scan_kb_articles(root: &Path) -> KbScanResult {
    let mut result = KbScanResult::default();
    for rel in list_markdown(root) {
        let path = root.join(&rel);
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                result.errors.push(KbScanError {
                    file: rel,
                    error: format!("read failed: {e}"),
                });
                continue;
            }
        };
        match parse_article(&rel, &content) {
            Ok(record) => result.records.push(record),
            Err(error) => result.errors.push(KbScanError { file: rel, error }),
        }
    }
    result
        .records
        .sort_by(|a, b| a.domain.cmp(&b.domain).then_with(|| a.title.cmp(&b.title)));
    result
}

/// Serialise records to NDJSON: one compact JSON object per line, with a
/// trailing newline when non-empty.
pub fn records_to_ndjson(records: &[KbRecord]) -> String {
    if records.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for r in records {
        // `KbRecord` has no invalid states serde can fail on (no maps with
        // non-string keys, no NaN floats), so this cannot actually error;
        // `unwrap_or_default` keeps the function infallible without a panic
        // path, rather than threading a Result through every caller for a
        // case that cannot occur.
        let line = serde_json::to_string(r).unwrap_or_default();
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// "Aws" from "aws", "Ci Cd" is not attempted (single-word domains only, as
/// this repo's domains are); mirrors the TypeScript renderer's `titleCase`.
fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Work out what every generated link must be prefixed with so it resolves
/// from the index file that contains it. Article paths are recorded relative
/// to `root`, but the index may be written elsewhere (e.g. articles scanned
/// under `docs/knowledge-base` linked from `docs/knowledge-base-index.md`
/// need a `knowledge-base/` hop, since the index sits in `docs/`). Returns a
/// prefix ending in `/`, or empty when the index already sits in `root`.
pub fn compute_link_prefix(root: &Path, view_path: &Path) -> String {
    let view_dir = view_path.parent().unwrap_or_else(|| Path::new(""));
    let hop = pathdiff(root, view_dir);
    if hop.is_empty() {
        String::new()
    } else {
        format!("{hop}/")
    }
}

/// A minimal relative-path computation between two plain (non-canonicalised)
/// relative paths, sufficient for the repo-relative paths this command deals
/// with: strip the common prefix of components, then `..` out of whatever of
/// `from` remains before appending whatever of `to` remains.
fn pathdiff(to: &Path, from: &Path) -> String {
    let to_parts: Vec<_> = to.components().collect();
    let from_parts: Vec<_> = from.components().collect();
    let common = to_parts
        .iter()
        .zip(from_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut parts: Vec<String> = Vec::new();
    for _ in common..from_parts.len() {
        parts.push("..".to_string());
    }
    for part in &to_parts[common..] {
        parts.push(part.as_os_str().to_string_lossy().to_string());
    }
    parts.join("/")
}

/// Render the complete knowledge-base index document: header with article
/// count and last-updated date, then one table per domain, sorted
/// alphabetically. Ends with a single trailing newline.
pub fn render_kb_index(records: &[KbRecord], today: &str, link_prefix: &str) -> String {
    let count = records.len();
    let header = format!(
        "# Knowledge Base Index\n\n> Auto-generated. Do not edit. Regenerated by the hk pre-commit job.\n> {count} articles \u{b7} last updated {today}\n"
    );

    let mut domains: Vec<&str> = records.iter().map(|r| r.domain.as_str()).collect();
    domains.sort();
    domains.dedup();

    let sections: Vec<String> = domains
        .iter()
        .map(|domain| {
            let rows: Vec<String> = records
                .iter()
                .filter(|r| r.domain == *domain)
                .map(|r| format!("| [{}]({}{}) | {} |", r.title, link_prefix, r.file, r.topic))
                .collect();
            format!(
                "## {}\n\n| Article | Topic |\n| --- | --- |\n{}",
                title_case(domain),
                rows.join("\n")
            )
        })
        .collect();

    format!("{header}\n{}\n", sections.join("\n\n"))
}

/// Build the index artifacts (ndjson + markdown) from a scan root.
pub fn build_kb_index(
    root: &Path,
    view_path: &Path,
    today: &str,
) -> (KbScanResult, String, String) {
    let scan = scan_kb_articles(root);
    let ndjson = records_to_ndjson(&scan.records);
    let view = render_kb_index(&scan.records, today, &compute_link_prefix(root, view_path));
    (scan, ndjson, view)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, rel: &str, content: &str) {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn parses_required_and_optional_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(
            root,
            "aws/lambda-cold-starts.md",
            "---\ntitle: \"Lambda Cold Starts\"\ntopic: \"Why cold starts spike\"\ncreated_at: 2026-01-05\nupdated_at: 2026-02-01\ntags:\n  - aws-lambda\n  - performance\n---\n\nBody.\n",
        );
        let scan = scan_kb_articles(root);
        assert!(scan.errors.is_empty());
        assert_eq!(scan.records.len(), 1);
        let r = &scan.records[0];
        assert_eq!(r.file, "aws/lambda-cold-starts.md");
        assert_eq!(r.domain, "aws");
        assert_eq!(r.title, "Lambda Cold Starts");
        assert_eq!(r.topic, "Why cold starts spike");
        assert_eq!(r.created_at.as_deref(), Some("2026-01-05"));
        assert_eq!(r.updated_at.as_deref(), Some("2026-02-01"));
        assert_eq!(
            r.tags,
            Some(vec!["aws-lambda".to_string(), "performance".to_string()])
        );
    }

    #[test]
    fn optional_fields_absent_when_not_set() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(
            root,
            "jira/checklists.md",
            "---\ntitle: \"Jira Checklists\"\ntopic: \"How to write ADF task lists\"\n---\n\nBody.\n",
        );
        let scan = scan_kb_articles(root);
        assert!(scan.errors.is_empty());
        let r = &scan.records[0];
        assert_eq!(r.created_at, None);
        assert_eq!(r.updated_at, None);
        assert_eq!(r.tags, None);
        let ndjson = records_to_ndjson(&scan.records);
        assert!(!ndjson.contains("created_at"));
        assert!(!ndjson.contains("tags"));
    }

    #[test]
    fn missing_frontmatter_is_an_error_not_a_drop() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, "aws/no-frontmatter.md", "# Just a heading\n");
        let scan = scan_kb_articles(root);
        assert!(scan.records.is_empty());
        assert_eq!(scan.errors.len(), 1);
        assert!(scan.errors[0].error.contains("missing frontmatter"));
    }

    #[test]
    fn missing_required_field_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, "aws/no-topic.md", "---\ntitle: \"X\"\n---\nBody.\n");
        let scan = scan_kb_articles(root);
        assert_eq!(scan.errors.len(), 1);
        assert!(scan.errors[0].error.contains("topic is required"));
    }

    #[test]
    fn records_sort_by_domain_then_title() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, "jira/b.md", "---\ntitle: \"B\"\ntopic: \"t\"\n---\n");
        write(root, "aws/z.md", "---\ntitle: \"Z\"\ntopic: \"t\"\n---\n");
        write(root, "aws/a.md", "---\ntitle: \"A\"\ntopic: \"t\"\n---\n");
        let scan = scan_kb_articles(root);
        let titles: Vec<&str> = scan.records.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(titles, vec!["A", "Z", "B"]);
    }

    #[test]
    fn ndjson_is_compact_and_newline_terminated() {
        let records = vec![KbRecord {
            file: "aws/x.md".to_string(),
            domain: "aws".to_string(),
            title: "X".to_string(),
            topic: "T".to_string(),
            created_at: None,
            updated_at: None,
            tags: None,
        }];
        let out = records_to_ndjson(&records);
        assert!(out.ends_with('\n'));
        assert_eq!(out.lines().count(), 1);
        assert_eq!(
            out.trim(),
            r#"{"file":"aws/x.md","domain":"aws","title":"X","topic":"T"}"#
        );
    }

    #[test]
    fn empty_records_yield_empty_ndjson() {
        assert_eq!(records_to_ndjson(&[]), "");
    }

    #[test]
    fn link_prefix_hops_into_the_scan_root() {
        let prefix = compute_link_prefix(
            Path::new("docs/knowledge-base"),
            Path::new("docs/knowledge-base-index.md"),
        );
        assert_eq!(prefix, "knowledge-base/");
    }

    #[test]
    fn link_prefix_empty_when_index_sits_in_root() {
        let prefix = compute_link_prefix(Path::new("docs/kb"), Path::new("docs/kb/index.md"));
        assert_eq!(prefix, "");
    }

    #[test]
    fn render_includes_header_and_domain_tables() {
        let records = vec![
            KbRecord {
                file: "aws/a.md".to_string(),
                domain: "aws".to_string(),
                title: "A".to_string(),
                topic: "Topic A".to_string(),
                created_at: None,
                updated_at: None,
                tags: None,
            },
            KbRecord {
                file: "jira/b.md".to_string(),
                domain: "jira".to_string(),
                title: "B".to_string(),
                topic: "Topic B".to_string(),
                created_at: None,
                updated_at: None,
                tags: None,
            },
        ];
        let view = render_kb_index(&records, "2026-09-23", "knowledge-base/");
        assert!(view.contains("# Knowledge Base Index"));
        assert!(view.contains("2 articles \u{b7} last updated 2026-09-23"));
        assert!(view.contains("## Aws"));
        assert!(view.contains("## Jira"));
        assert!(view.contains("| [A](knowledge-base/aws/a.md) | Topic A |"));
        assert!(view.ends_with('\n') && !view.ends_with("\n\n"));
    }
}
