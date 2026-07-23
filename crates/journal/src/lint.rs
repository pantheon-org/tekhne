//! Advisory tag lint over a scanned corpus.
//!
//! Port of the journal CLI's `edit-distance.ts`, `suggest-alias.ts`, and
//! `action.ts`. Two checks run against the taxonomy:
//!
//! 1. **Alias suggestions**: a tag that looks like a near-duplicate of another
//!    known tag (same letters ignoring separators, or Levenshtein distance 1
//!    for tags of length >= 4) is flagged as a candidate alias.
//! 2. **Unfaceted tags**: a tag whose corpus count reaches the taxonomy
//!    `threshold` but which belongs to no facet (and is not a ticket key) is
//!    flagged for facet assignment or suppression.
//!
//! The pass is advisory: it reports, it does not mutate the taxonomy.

use std::collections::{BTreeMap, BTreeSet};

use common::Result;
use serde::Serialize;

use crate::scan::Entry;
use crate::taxonomy::Taxonomy;

/// A tag that looks like a near-duplicate of a known tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AliasSuggestion {
    /// The tag as it appears in the corpus.
    pub tag: String,
    /// The known tag it resembles.
    pub suggestion: String,
}

/// A tag that crossed the threshold but belongs to no facet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnfacetedTag {
    /// The unfaceted tag.
    pub tag: String,
    /// Its occurrence count across the corpus.
    pub count: u32,
}

/// The outcome of a lint pass.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct LintReport {
    /// Near-duplicate tags that could be collapsed via an alias.
    pub alias_suggestions: Vec<AliasSuggestion>,
    /// Threshold-crossing tags with no facet.
    pub unfaceted: Vec<UnfacetedTag>,
}

impl LintReport {
    /// True when the pass found nothing to report.
    pub fn is_clean(&self) -> bool {
        self.alias_suggestions.is_empty() && self.unfaceted.is_empty()
    }
}

/// The Levenshtein edit distance between two strings.
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0usize; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

/// Strip separators (`-`, `_`, whitespace) and lower-case, so `ms-teams` and
/// `msteams` collapse to the same key.
fn collapse(tag: &str) -> String {
    tag.chars()
        .filter(|c| *c != '-' && *c != '_' && !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

/// Suggest a near-duplicate of `tag` from `known_tags`. Prefers a
/// separator-collapse match, then a Levenshtein-1 match for tags of length
/// >= 4. `known_tags` is expected to be sorted so the result is deterministic.
pub fn suggest_alias(
    tag: &str,
    known_tags: &[String],
    aliases: &BTreeMap<String, String>,
) -> Option<String> {
    if aliases.contains_key(tag) {
        return None;
    }
    let normalized = collapse(tag);
    for known in known_tags {
        if tag == known {
            continue;
        }
        if normalized == collapse(known) {
            return Some(known.clone());
        }
    }
    if tag.chars().count() >= 4 {
        for known in known_tags {
            if tag == known || known.chars().count() < 4 {
                continue;
            }
            if edit_distance(tag, known) <= 1 {
                return Some(known.clone());
            }
        }
    }
    None
}

/// Run the advisory lint. When `files` is non-empty, only tags carried by
/// entries whose path contains one of those substrings are linted, but tag
/// frequency and the known-tag set are always computed over the whole corpus.
pub fn lint(entries: &[Entry], taxonomy: &Taxonomy, files: &[String]) -> Result<LintReport> {
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    for entry in entries {
        for tag in &entry.tags {
            *counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    // Known tags: every tag seen plus every canonical facet member.
    let mut known: BTreeSet<String> = counts.keys().cloned().collect();
    for list in taxonomy.facets.values() {
        for tag in list {
            known.insert(tag.clone());
        }
    }
    let known: Vec<String> = known.into_iter().collect();

    // Which tags to lint: staged subset, or the whole corpus.
    let tags_to_lint: Vec<String> = if files.is_empty() {
        counts.keys().cloned().collect()
    } else {
        let mut staged: BTreeSet<String> = BTreeSet::new();
        for entry in entries {
            if files.iter().any(|f| entry.file.contains(f)) {
                for tag in &entry.tags {
                    staged.insert(tag.clone());
                }
            }
        }
        staged.into_iter().collect()
    };

    let mut report = LintReport::default();
    for tag in &tags_to_lint {
        if let Some(suggestion) = suggest_alias(tag, &known, &taxonomy.aliases) {
            report.alias_suggestions.push(AliasSuggestion {
                tag: tag.clone(),
                suggestion,
            });
        }
    }

    let ticket_re = taxonomy.ticket_regex()?;
    for tag in &tags_to_lint {
        let count = *counts.get(tag).unwrap_or(&0);
        if count >= taxonomy.threshold
            && !ticket_re.is_match(tag)
            && !taxonomy.is_faceted(tag)
            && !taxonomy.suppress.iter().any(|s| s == tag)
        {
            report.unfaceted.push(UnfacetedTag {
                tag: tag.clone(),
                count,
            });
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tax(json: &str) -> Taxonomy {
        Taxonomy::from_json(json, "test").unwrap()
    }

    fn entry(file: &str, tags: &[&str]) -> Entry {
        Entry {
            file: file.to_string(),
            files: vec![file.to_string()],
            slug: String::new(),
            date: String::new(),
            title: String::new(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            entry_type: "general".to_string(),
            tickets: vec![],
            authors: vec![],
            status: String::new(),
            summary: String::new(),
        }
    }

    #[test]
    fn edit_distance_basics() {
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("book", "back"), 2);
    }

    #[test]
    fn suggest_alias_collapses_separators() {
        let known = vec!["ms-teams".to_string()];
        assert_eq!(
            suggest_alias("msteams", &known, &BTreeMap::new()).as_deref(),
            Some("ms-teams")
        );
    }

    #[test]
    fn suggest_alias_edit_distance_one() {
        let known = vec!["webhooks".to_string()];
        assert_eq!(
            suggest_alias("webhook", &known, &BTreeMap::new()).as_deref(),
            Some("webhooks")
        );
    }

    #[test]
    fn suggest_alias_skips_short_and_already_aliased() {
        let known = vec!["ab".to_string()];
        assert!(suggest_alias("ac", &known, &BTreeMap::new()).is_none());
        let mut aliases = BTreeMap::new();
        aliases.insert("teams".to_string(), "ms-teams".to_string());
        let known = vec!["ms-teams".to_string()];
        assert!(suggest_alias("teams", &known, &aliases).is_none());
    }

    #[test]
    fn unfaceted_flags_over_threshold_only() {
        let t = tax(
            r#"{ "threshold": 2, "aliases": {}, "facets": { "tech": ["aws"] }, "ticketPattern": "^x-[0-9]+$" }"#,
        );
        let entries = vec![
            entry("2026/07/2026-07-01-a.md", &["aws", "orphan"]),
            entry("2026/07/2026-07-02-b.md", &["orphan"]),
            entry("2026/07/2026-07-03-c.md", &["singleton"]),
        ];
        let report = lint(&entries, &t, &[]).unwrap();
        // "orphan" appears twice, has no facet -> flagged. "aws" is faceted.
        // "singleton" appears once, below threshold. Ticket-like excluded.
        assert_eq!(
            report.unfaceted,
            vec![UnfacetedTag {
                tag: "orphan".into(),
                count: 2
            }]
        );
    }

    #[test]
    fn ticket_tags_are_exempt() {
        let t = tax(
            r#"{ "threshold": 1, "aliases": {}, "facets": {}, "ticketPattern": "^x-[0-9]+$" }"#,
        );
        let entries = vec![entry("2026/07/2026-07-01-a.md", &["x-123"])];
        let report = lint(&entries, &t, &[]).unwrap();
        assert!(report.unfaceted.is_empty());
    }

    #[test]
    fn file_filter_restricts_linted_tags() {
        let t = tax(
            r#"{ "threshold": 1, "aliases": {}, "facets": {}, "ticketPattern": "^x-[0-9]+$" }"#,
        );
        let entries = vec![
            entry("2026/07/2026-07-01-a.md", &["only-in-a"]),
            entry("2026/07/2026-07-02-b.md", &["only-in-b"]),
        ];
        let report = lint(&entries, &t, &["2026-07-01".to_string()]).unwrap();
        assert_eq!(
            report.unfaceted,
            vec![UnfacetedTag {
                tag: "only-in-a".into(),
                count: 1
            }]
        );
    }
}
