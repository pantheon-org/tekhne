//! Scan a journal corpus into logical entries with normalised tags.
//!
//! Port of the journal CLI's `scan-entries.ts` plus the tag-extraction slice of
//! `parse-frontmatter.ts`. Only the year directories (`20NN/`) are walked, which
//! naturally excludes `node_modules`, `.git`, `docs`, and the like. A dated
//! entry file and a sibling spike folder of the same slug collapse into one
//! logical entry so their tags are not double-counted.
//!
//! Frontmatter tags are extracted with a focused, dependency-light parser
//! (matching how `validate.rs` hand-parses frontmatter) rather than a full YAML
//! dependency. It understands the two shapes the corpus uses: a block sequence
//! (`tags:` then `  - item` lines) and an inline flow sequence
//! (`tags: [a, b]`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::taxonomy::{normalise_tag, Taxonomy};

/// A logical journal entry: its primary file and the union of normalised tags
/// across every file grouped under it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Repo-relative forward-slash path of the entry's primary file.
    pub file: String,
    /// Sorted, de-duplicated, alias-normalised tags.
    pub tags: Vec<String>,
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
    /// Logical entries, sorted by file path for determinism.
    pub entries: Vec<Entry>,
    /// Files that could not be read.
    pub errors: Vec<ScanError>,
}

/// True when a path segment is a `YYYY-MM-DD-` dated stem.
fn is_dated(segment: &str) -> bool {
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
fn list_markdown(root: &Path) -> Vec<String> {
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

/// Extract the raw frontmatter `tags` list from a markdown document. Returns an
/// empty list when there is no frontmatter or no `tags` key.
pub fn frontmatter_tags(content: &str) -> Vec<String> {
    if !content.starts_with("---") {
        return Vec::new();
    }
    let lines: Vec<&str> = content.split('\n').collect();
    let mut end = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            end = Some(i);
            break;
        }
    }
    let Some(end) = end else {
        return Vec::new();
    };

    for (i, line) in lines[1..end].iter().enumerate() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("tags:") else {
            continue;
        };
        let rest = rest.trim();
        // Inline flow sequence: tags: [a, b]
        if let Some(inner) = rest.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            return inner
                .split(',')
                .map(unquote)
                .filter(|t| !t.is_empty())
                .collect();
        }
        // Block sequence: subsequent `  - item` lines until dedent / next key.
        let mut tags = Vec::new();
        for follow in &lines[1 + i + 1..end] {
            let ft = follow.trim();
            if let Some(item) = ft.strip_prefix("- ") {
                tags.push(unquote(item));
            } else if ft.is_empty() {
                continue;
            } else {
                break;
            }
        }
        return tags.into_iter().filter(|t| !t.is_empty()).collect();
    }
    Vec::new()
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
/// logical entry. Tags are normalised through the taxonomy aliases.
pub fn scan_entries(root: &Path, taxonomy: &Taxonomy) -> ScanResult {
    let all = list_markdown(root);

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
            for raw in frontmatter_tags(&content) {
                let n = normalise_tag(&raw, &taxonomy.aliases);
                if !n.is_empty() {
                    tag_set.insert(n);
                }
            }
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
        result.entries.push(Entry {
            file: primary,
            tags: tag_set.into_iter().collect(),
        });
    }

    result.entries.sort_by(|a, b| a.file.cmp(&b.file));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tax() -> Taxonomy {
        Taxonomy::from_json(
            r#"{ "threshold": 3, "aliases": { "teams": "ms-teams" }, "facets": {}, "ticketPattern": "^x-[0-9]+$" }"#,
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
    fn stops_at_next_key() {
        let content = "---\ntags:\n  - alpha\nauthors:\n  - me\n---\n# T\n";
        assert_eq!(frontmatter_tags(content), vec!["alpha"]);
    }

    #[test]
    fn no_frontmatter_no_tags() {
        assert!(frontmatter_tags("# Just a heading\n").is_empty());
        assert!(frontmatter_tags("---\ntitle: T\n---\n# T\n").is_empty());
    }

    #[test]
    fn scan_normalises_and_dedupes_across_spike_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let month = root.join("2026/07");
        std::fs::create_dir_all(&month).unwrap();
        // Top-level entry file.
        std::fs::write(
            month.join("2026-07-01-example.md"),
            "---\ntags:\n  - Teams\n  - alpha\n---\n# Example - July 1, 2026\n",
        )
        .unwrap();
        // Sibling spike folder file for the same slug.
        let spike = month.join("2026-07-01-example");
        std::fs::create_dir_all(&spike).unwrap();
        std::fs::write(
            spike.join("notes.md"),
            "---\ntags:\n  - alpha\n  - beta\n---\n# Notes\n",
        )
        .unwrap();

        let result = scan_entries(root, &tax());
        assert!(result.errors.is_empty());
        assert_eq!(
            result.entries.len(),
            1,
            "spike folder collapses into one entry"
        );
        let entry = &result.entries[0];
        assert_eq!(entry.file, "2026/07/2026-07-01-example.md");
        // "Teams" -> "ms-teams" (alias + lowercase); alpha de-duplicated.
        assert_eq!(entry.tags, vec!["alpha", "beta", "ms-teams"]);
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
