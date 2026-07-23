//! Scan a journal corpus into logical entries.
//!
//! Port of the journal CLI's `scan-entries.ts`, `parse-frontmatter.ts`,
//! `extract-summary.ts`, `derive-type.ts`, and `derive-tickets.ts`. Only the
//! year directories (`20NN/`) are walked, which naturally excludes
//! `node_modules`, `.git`, `docs`, and the like. A dated entry file and a
//! sibling spike folder of the same slug collapse into one logical entry so
//! their tags are not double-counted.
//!
//! Frontmatter is parsed with a focused, dependency-light parser (matching how
//! `validate.rs` hand-parses frontmatter) rather than a full YAML dependency. It
//! understands the two shapes the corpus uses: a block sequence (`tags:` then
//! `  - item` lines) and an inline flow sequence (`tags: [a, b]`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use regex::Regex;

use crate::taxonomy::{normalise_tag, Taxonomy};

/// One logical journal entry. A nested spike folder collapses to a single
/// entry: `file` is the entry point, `files` lists every member. Tags are
/// normalised; `entry_type` and `tickets` are derived from them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Repo-relative forward-slash path of the entry's primary file.
    pub file: String,
    /// Every file grouped under this logical entry, sorted.
    pub files: Vec<String>,
    /// The `YYYY-MM-DD-slug` stem of the entry.
    pub slug: String,
    /// `YYYY-MM-DD` taken from the slug prefix.
    pub date: String,
    /// Frontmatter `title`, falling back to the body H1, else empty.
    pub title: String,
    /// Sorted, de-duplicated, alias-normalised tags.
    pub tags: Vec<String>,
    /// Derived entry type (a `type` facet tag, else `general`).
    pub entry_type: String,
    /// Ticket keys among the tags (matching the taxonomy `ticketPattern`).
    pub tickets: Vec<String>,
    /// Frontmatter `authors`, else empty.
    pub authors: Vec<String>,
    /// Frontmatter `status`, else empty.
    pub status: String,
    /// First paragraph under `## Session Overview`, else empty.
    pub summary: String,
}

/// A file that could not be read, kept rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanError {
    /// Repo-relative forward-slash path of the file.
    pub file: String,
    /// Human-readable failure description.
    pub error: String,
}

/// The result of a corpus scan.
#[derive(Debug, Clone, Default)]
pub struct ScanResult {
    /// Logical entries, newest first (date descending, then slug ascending).
    pub entries: Vec<Entry>,
    /// Files that could not be read.
    pub errors: Vec<ScanError>,
}

/// True when a path segment is a `YYYY-MM-DD-` dated stem.
pub(crate) fn is_dated(segment: &str) -> bool {
    let b = segment.as_bytes();
    b.len() >= 11
        && b[0..4].iter().all(u8::is_ascii_digit)
        && b[4] == b'-'
        && b[5..7].iter().all(u8::is_ascii_digit)
        && b[7] == b'-'
        && b[8..10].iter().all(u8::is_ascii_digit)
        && b[10] == b'-'
}

/// True when a top-level directory name is a `20NN` year directory.
fn is_year_dir(name: &str) -> bool {
    let b = name.as_bytes();
    b.len() == 4 && &b[0..2] == b"20" && b[2].is_ascii_digit() && b[3].is_ascii_digit()
}

/// Collect repo-relative forward-slash paths of every `.md` file under the
/// corpus's year directories.
pub(crate) fn list_markdown(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(top) = std::fs::read_dir(root) else {
        return out;
    };
    for entry in top.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.path().is_dir() && is_year_dir(&name) {
            walk_md(&entry.path(), &name, &mut out);
        }
    }
    out.sort();
    out
}

/// Recursively collect `.md` files under `dir`, prefixing each with `rel`.
fn walk_md(dir: &Path, rel: &str, out: &mut Vec<String>) {
    let Ok(read) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let child_rel = format!("{rel}/{name}");
        let path = entry.path();
        if path.is_dir() {
            walk_md(&path, &child_rel, out);
        } else if name.ends_with(".md") {
            out.push(child_rel);
        }
    }
}

/// The frontmatter fields the index cares about, parsed from a document.
#[derive(Debug, Default, Clone)]
pub struct Frontmatter {
    /// `tags` as written (not yet normalised).
    pub tags: Vec<String>,
    /// `title` (trimmed), else empty.
    pub title: String,
    /// `authors` (trimmed, non-empty), else empty.
    pub authors: Vec<String>,
    /// `status` (trimmed), else empty.
    pub status: String,
}

/// Split a document into its frontmatter block lines and body. Returns
/// `(None, whole_document)` when there is no `---` fenced frontmatter.
fn split_frontmatter(content: &str) -> (Option<Vec<&str>>, String) {
    if !content.starts_with("---") {
        return (None, content.to_string());
    }
    let lines: Vec<&str> = content.split('\n').collect();
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            let block = lines[1..i].to_vec();
            let body = lines[i + 1..].join("\n");
            return (Some(block), body);
        }
    }
    (None, content.to_string())
}

/// Extract an inline or block sequence for `key` from frontmatter block lines.
fn fm_sequence(block: &[&str], key: &str) -> Vec<String> {
    let prefix = format!("{key}:");
    for (i, line) in block.iter().enumerate() {
        let Some(rest) = line.trim_start().strip_prefix(&prefix) else {
            continue;
        };
        let rest = rest.trim();
        if let Some(inner) = rest.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            return inner
                .split(',')
                .map(unquote)
                .filter(|t| !t.is_empty())
                .collect();
        }
        let mut items = Vec::new();
        for follow in &block[i + 1..] {
            let ft = follow.trim();
            if let Some(item) = ft.strip_prefix("- ") {
                items.push(unquote(item));
            } else if ft.is_empty() {
                continue;
            } else {
                break;
            }
        }
        return items.into_iter().filter(|t| !t.is_empty()).collect();
    }
    Vec::new()
}

/// Extract an inline scalar value for `key` from frontmatter block lines.
fn fm_scalar(block: &[&str], key: &str) -> String {
    let prefix = format!("{key}:");
    for line in block {
        if let Some(rest) = line.trim_start().strip_prefix(&prefix) {
            return unquote(rest);
        }
    }
    String::new()
}

/// Parse the frontmatter fields the index needs, plus the body text.
pub fn parse_frontmatter(content: &str) -> (Frontmatter, String) {
    let (block, body) = split_frontmatter(content);
    let Some(block) = block else {
        return (Frontmatter::default(), body);
    };
    let fm = Frontmatter {
        tags: fm_sequence(&block, "tags"),
        title: fm_scalar(&block, "title"),
        authors: fm_sequence(&block, "authors"),
        status: fm_scalar(&block, "status"),
    };
    (fm, body)
}

/// The raw frontmatter `tags` list (kept for callers that only need tags).
pub fn frontmatter_tags(content: &str) -> Vec<String> {
    parse_frontmatter(content).0.tags
}

/// The first body H1 (`# Title`), trimmed, else empty.
pub(crate) fn first_h1(body: &str) -> String {
    for line in body.split('\n') {
        if let Some(rest) = line.strip_prefix("# ") {
            return rest.trim().to_string();
        }
    }
    String::new()
}

/// Extract a one-line summary: the first paragraph under `## Session Overview`,
/// its lines joined by spaces. Empty when there is no such section or prose.
pub fn extract_summary(body: &str) -> String {
    let lines: Vec<&str> = body.split('\n').collect();
    let Some(start) = lines.iter().position(|l| l.trim() == "## Session Overview") else {
        return String::new();
    };
    let mut paragraph: Vec<&str> = Vec::new();
    for line in &lines[start + 1..] {
        let t = line.trim();
        if t.is_empty() {
            if !paragraph.is_empty() {
                break;
            }
            continue;
        }
        if t.starts_with('#') {
            break;
        }
        paragraph.push(t);
    }
    paragraph.join(" ").trim().to_string()
}

/// Derive an entry's type: the first `type`-facet tag (in declared order, other
/// than `general`) the entry carries, else `general`.
pub fn derive_type(tags: &[String], type_facet: &[String]) -> String {
    for candidate in type_facet {
        if candidate != "general" && tags.iter().any(|t| t == candidate) {
            return candidate.clone();
        }
    }
    "general".to_string()
}

/// Derive ticket keys: tags matching `ticket_re`, sorted and de-duplicated.
pub fn derive_tickets(tags: &[String], ticket_re: &Regex) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    for tag in tags {
        if ticket_re.is_match(tag) {
            set.insert(tag.clone());
        }
    }
    set.into_iter().collect()
}

/// Strip surrounding single/double quotes and whitespace from a scalar.
fn unquote(s: &str) -> String {
    let s = s.trim();
    let bytes = s.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Scan every dated entry under `root`, grouping a nested spike folder into one
/// logical entry. Tags are normalised through the taxonomy aliases; type and
/// tickets are derived. Entries are returned newest first.
pub fn scan_entries(root: &Path, taxonomy: &Taxonomy) -> ScanResult {
    let all = list_markdown(root);
    let ticket_re = taxonomy
        .ticket_regex()
        .unwrap_or_else(|_| Regex::new("$^").expect("empty regex compiles"));
    let type_facet = taxonomy.facets.get("type").cloned().unwrap_or_default();

    // Key every file by its slug-stem path so a top-level `slug.md` and a
    // sibling `slug/` spike folder collapse into one logical entry.
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for rel in &all {
        let parts: Vec<&str> = rel.split('/').collect();
        let base = *parts.last().unwrap_or(&rel.as_str());
        let parent = if parts.len() >= 2 {
            parts[parts.len() - 2]
        } else {
            ""
        };
        let in_spike = is_dated(parent);
        if !in_spike && !is_dated(base) {
            continue;
        }
        let key = if in_spike {
            parts[..parts.len() - 1].join("/")
        } else {
            rel.trim_end_matches(".md").to_string()
        };
        groups.entry(key).or_default().push(rel.clone());
    }

    let mut result = ScanResult::default();
    for (key, mut files) in groups {
        files.sort();
        let mut tag_set: BTreeSet<String> = BTreeSet::new();
        let mut parsed_files: Vec<String> = Vec::new();
        let mut fm_by_file: BTreeMap<String, (Frontmatter, String)> = BTreeMap::new();
        for rel in &files {
            let content = match std::fs::read_to_string(root.join(rel)) {
                Ok(c) => c,
                Err(e) => {
                    result.errors.push(ScanError {
                        file: rel.clone(),
                        error: format!("read failed: {e}"),
                    });
                    continue;
                }
            };
            parsed_files.push(rel.clone());
            let (fm, body) = parse_frontmatter(&content);
            for raw in &fm.tags {
                let n = normalise_tag(raw, &taxonomy.aliases);
                if !n.is_empty() {
                    tag_set.insert(n);
                }
            }
            fm_by_file.insert(rel.clone(), (fm, body));
        }
        if parsed_files.is_empty() {
            continue;
        }
        // Prefer the top-level `slug.md`, then an inventory file, then the first.
        let primary = parsed_files
            .iter()
            .find(|f| **f == format!("{key}.md"))
            .or_else(|| parsed_files.iter().find(|f| f.contains("inventory")))
            .unwrap_or(&parsed_files[0])
            .clone();

        let slug = key.split('/').next_back().unwrap_or(&key).to_string();
        let date = slug.chars().take(10).collect::<String>();
        let tags: Vec<String> = tag_set.into_iter().collect();
        let (fm, body) = fm_by_file.get(&primary).cloned().unwrap_or_default();
        let title = if fm.title.is_empty() {
            first_h1(&body)
        } else {
            fm.title
        };

        result.entries.push(Entry {
            file: primary,
            files: parsed_files,
            slug,
            date,
            title,
            entry_type: derive_type(&tags, &type_facet),
            tickets: derive_tickets(&tags, &ticket_re),
            tags,
            authors: fm.authors,
            status: fm.status,
            summary: extract_summary(&body),
        });
    }

    // Newest first: date descending, then slug ascending (mirrors the CLI).
    result
        .entries
        .sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.slug.cmp(&b.slug)));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tax() -> Taxonomy {
        Taxonomy::from_json(
            r#"{ "threshold": 3, "aliases": { "teams": "ms-teams" }, "facets": { "type": ["troubleshooting", "general"] }, "ticketPattern": "^x-[0-9]+$" }"#,
            "test",
        )
        .unwrap()
    }

    #[test]
    fn block_sequence_tags() {
        let content = "---\ntitle: T\ntags:\n  - alpha\n  - beta\n---\n\n# T\n";
        assert_eq!(frontmatter_tags(content), vec!["alpha", "beta"]);
    }

    #[test]
    fn flow_sequence_tags() {
        let content = "---\ntags: [alpha, \"beta\", 'gamma']\n---\n# T\n";
        assert_eq!(frontmatter_tags(content), vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn parses_title_authors_status() {
        let content = "---\ntitle: \"My Entry\"\nauthors:\n  - Alice\n  - Bob\nstatus: published\ntags:\n  - x\n---\n# Fallback\n";
        let (fm, _body) = parse_frontmatter(content);
        assert_eq!(fm.title, "My Entry");
        assert_eq!(fm.authors, vec!["Alice", "Bob"]);
        assert_eq!(fm.status, "published");
    }

    #[test]
    fn title_falls_back_to_h1() {
        let content = "---\ntags:\n  - x\n---\n\n# The Heading - July 1, 2026\n\ntext\n";
        let (fm, body) = parse_frontmatter(content);
        assert_eq!(fm.title, "");
        assert_eq!(first_h1(&body), "The Heading - July 1, 2026");
    }

    #[test]
    fn summary_from_session_overview() {
        let body =
            "# T\n\n## Session Overview\n\nFirst line.\nSecond line.\n\n## Next\n\nignored\n";
        assert_eq!(extract_summary(body), "First line. Second line.");
        assert_eq!(extract_summary("# T\n\nno overview\n"), "");
    }

    #[test]
    fn derive_type_and_tickets() {
        let type_facet = vec!["troubleshooting".to_string(), "general".to_string()];
        let re = Regex::new("^x-[0-9]+$").unwrap();
        assert_eq!(
            derive_type(&["a".into(), "troubleshooting".into()], &type_facet),
            "troubleshooting"
        );
        assert_eq!(derive_type(&["a".into()], &type_facet), "general");
        assert_eq!(
            derive_tickets(&["x-2".into(), "a".into(), "x-1".into(), "x-1".into()], &re),
            vec!["x-1", "x-2"]
        );
    }

    #[test]
    fn scan_builds_full_entry_and_collapses_spike_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let month = root.join("2026/07");
        std::fs::create_dir_all(&month).unwrap();
        std::fs::write(
            month.join("2026-07-01-example.md"),
            "---\ntitle: Example\nauthors:\n  - Alice\nstatus: published\ntags:\n  - Teams\n  - alpha\n  - troubleshooting\n  - x-42\n---\n\n# Example - July 1, 2026\n\n## Session Overview\n\nA short summary.\n",
        )
        .unwrap();
        let spike = month.join("2026-07-01-example");
        std::fs::create_dir_all(&spike).unwrap();
        std::fs::write(
            spike.join("notes.md"),
            "---\ntags:\n  - alpha\n  - beta\n---\n# Notes\n",
        )
        .unwrap();

        let result = scan_entries(root, &tax());
        assert!(result.errors.is_empty());
        assert_eq!(result.entries.len(), 1);
        let e = &result.entries[0];
        assert_eq!(e.file, "2026/07/2026-07-01-example.md");
        assert_eq!(e.files.len(), 2);
        assert_eq!(e.slug, "2026-07-01-example");
        assert_eq!(e.date, "2026-07-01");
        assert_eq!(e.title, "Example");
        assert_eq!(e.authors, vec!["Alice"]);
        assert_eq!(e.status, "published");
        assert_eq!(e.summary, "A short summary.");
        assert_eq!(e.entry_type, "troubleshooting");
        assert_eq!(e.tickets, vec!["x-42"]);
        // "Teams" -> "ms-teams" alias; alpha de-duplicated across files.
        assert!(e.tags.contains(&"ms-teams".to_string()));
        assert!(e.tags.contains(&"beta".to_string()));
    }

    #[test]
    fn scan_sorts_newest_first() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("2026/07")).unwrap();
        for (name, _) in [("2026-07-01-older", ()), ("2026-07-09-newer", ())] {
            std::fs::write(
                root.join(format!("2026/07/{name}.md")),
                "---\ntags:\n  - x\n---\n# T\n",
            )
            .unwrap();
        }
        let result = scan_entries(root, &tax());
        assert_eq!(result.entries[0].slug, "2026-07-09-newer");
        assert_eq!(result.entries[1].slug, "2026-07-01-older");
    }

    #[test]
    fn scan_ignores_non_year_dirs_and_undated_files() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
        std::fs::write(root.join("node_modules/pkg/readme.md"), "# x\n").unwrap();
        std::fs::create_dir_all(root.join("2026/07")).unwrap();
        std::fs::write(root.join("2026/07/undated-note.md"), "# note\n").unwrap();

        let result = scan_entries(root, &tax());
        assert!(result.entries.is_empty());
    }
}
