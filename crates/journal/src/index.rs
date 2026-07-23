//! Generate the journal browse index.
//!
//! Port of the journal CLI's `index/` tree. The NDJSON data file is the
//! machine-readable source of truth (one record per logical entry); the
//! markdown view is rendered from the same scanned entries so the two never
//! disagree. Both are deterministic: entries arrive newest-first from the scan,
//! and records serialise in a fixed key order.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use common::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::scan::Entry;
use crate::taxonomy::Taxonomy;

/// Default machine-readable index path (the source of truth).
pub const INDEX_DATA_PATH: &str = "docs/journal-index.ndjson";
/// Default human-readable index path (rendered from the data).
pub const INDEX_VIEW_PATH: &str = "docs/journal-index.md";

/// One record in the index NDJSON: one line equals one logical entry. Field
/// order here is the serialisation order, kept fixed for deterministic output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexRecord {
    pub file: String,
    pub files: Vec<String>,
    pub slug: String,
    pub date: String,
    pub title: String,
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub tickets: Vec<String>,
    pub authors: Vec<String>,
    pub status: String,
    pub summary: String,
}

/// Project a scanned entry onto the record written to NDJSON.
pub fn entry_to_record(entry: &Entry) -> IndexRecord {
    IndexRecord {
        file: entry.file.clone(),
        files: entry.files.clone(),
        slug: entry.slug.clone(),
        date: entry.date.clone(),
        title: entry.title.clone(),
        tags: entry.tags.clone(),
        entry_type: entry.entry_type.clone(),
        tickets: entry.tickets.clone(),
        authors: entry.authors.clone(),
        status: entry.status.clone(),
        summary: entry.summary.clone(),
    }
}

/// Serialise records to NDJSON: one compact JSON object per line, with a
/// trailing newline when non-empty. Order is preserved from the caller.
pub fn records_to_ndjson(records: &[IndexRecord]) -> Result<String> {
    if records.is_empty() {
        return Ok(String::new());
    }
    let mut out = String::new();
    for r in records {
        let line = serde_json::to_string(r).map_err(|e| Error::json("index record", e))?;
        out.push_str(&line);
        out.push('\n');
    }
    Ok(out)
}

/// The effective canonical tag set: tags at or over threshold, plus `pin`,
/// minus `suppress`.
pub fn compute_canonical(
    counts: &BTreeMap<String, usize>,
    taxonomy: &Taxonomy,
) -> BTreeSet<String> {
    let mut canonical: BTreeSet<String> = BTreeSet::new();
    for (tag, count) in counts {
        if *count as u32 >= taxonomy.threshold {
            canonical.insert(tag.clone());
        }
    }
    for pinned in &taxonomy.pin {
        canonical.insert(pinned.clone());
    }
    for suppressed in &taxonomy.suppress {
        canonical.remove(suppressed);
    }
    canonical
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// "June 2026" from a `YYYY-MM-DD` date, or the raw date on a parse failure.
fn month_label(date: &str) -> String {
    let month: usize = date.get(5..7).and_then(|m| m.parse().ok()).unwrap_or(0);
    let year = date.get(0..4).unwrap_or(date);
    match month {
        1..=12 => format!("{} {}", MONTHS[month - 1], year),
        _ => date.to_string(),
    }
}

/// A `- [YYYY-MM-DD title](../file)` bullet.
fn bullet(e: &Entry) -> String {
    format!("- [{} {}](../{})", e.date, e.title, e.file)
}

/// `## Recent Entries`: a table of the newest `limit` entries.
fn render_recent(entries: &[Entry], limit: usize) -> String {
    let recent = &entries[..entries.len().min(limit)];
    if recent.is_empty() {
        return String::new();
    }
    let mut rows = String::from("| Date | Title | Tags |\n|------|-------|------|");
    for e in recent {
        rows.push_str(&format!(
            "\n| {} | [{}](../{}) | {} |",
            e.date,
            e.title,
            e.file,
            e.tags.join(", ")
        ));
    }
    format!("## Recent Entries\n\n{rows}")
}

/// `## By Month`: entries grouped by month, months newest-first.
fn render_chronological(entries: &[Entry]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    // Entries arrive newest-first, so first-seen month order is newest-first.
    let mut order: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, Vec<&Entry>> = BTreeMap::new();
    for e in entries {
        let label = month_label(&e.date);
        if !groups.contains_key(&label) {
            order.push(label.clone());
        }
        groups.entry(label).or_default().push(e);
    }
    let sections: Vec<String> = order
        .iter()
        .map(|label| {
            let ents = &groups[label];
            let noun = if ents.len() == 1 { "entry" } else { "entries" };
            let bullets = ents
                .iter()
                .map(|e| bullet(e))
                .collect::<Vec<_>>()
                .join("\n");
            format!("### {label} ({} {noun})\n\n{bullets}", ents.len())
        })
        .collect();
    format!("## By Month\n\n{}", sections.join("\n\n"))
}

/// `## By Type`: entries grouped by type, ordered by the `type` facet.
fn render_by_type(entries: &[Entry], taxonomy: &Taxonomy) -> String {
    if entries.is_empty() {
        return String::new();
    }
    let mut groups: BTreeMap<String, Vec<&Entry>> = BTreeMap::new();
    for e in entries {
        groups.entry(e.entry_type.clone()).or_default().push(e);
    }
    let type_order = taxonomy.facets.get("type").cloned().unwrap_or_default();
    let mut types: Vec<String> = groups.keys().cloned().collect();
    types.sort_by(|a, b| {
        let ai = type_order.iter().position(|t| t == a);
        let bi = type_order.iter().position(|t| t == b);
        match (ai, bi) {
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.cmp(b),
        }
    });
    let sections: Vec<String> = types
        .iter()
        .map(|ty| {
            let ents = &groups[ty];
            let bullets = ents
                .iter()
                .map(|e| bullet(e))
                .collect::<Vec<_>>()
                .join("\n");
            format!("### {ty} ({})\n\n{bullets}", ents.len())
        })
        .collect();
    format!("## By Type\n\n{}", sections.join("\n\n"))
}

/// `## By Tag (canonical only)`: canonical tags grouped by facet, with an
/// emerging-tags details block for below-threshold tags.
fn render_by_tag(entries: &[Entry], taxonomy: &Taxonomy, ticket_re: &Regex) -> String {
    if entries.is_empty() {
        return String::new();
    }
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for e in entries {
        for tag in &e.tags {
            if !ticket_re.is_match(tag) {
                *counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
    }
    let canonical = compute_canonical(&counts, taxonomy);

    // tag -> entries carrying it (scan order preserved: newest-first).
    let mut tag_entries: BTreeMap<String, Vec<&Entry>> = BTreeMap::new();
    for e in entries {
        for tag in &e.tags {
            if !ticket_re.is_match(tag) && canonical.contains(tag) {
                tag_entries.entry(tag.clone()).or_default().push(e);
            }
        }
    }

    let facet_order: Vec<String> = taxonomy.facets.keys().cloned().collect();
    let mut sections: Vec<String> = Vec::new();

    let render_group = |tags: &mut Vec<(String, usize)>| -> String {
        tags.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        tags.iter()
            .map(|(tag, count)| {
                let bullets = tag_entries
                    .get(tag)
                    .map(|ents| {
                        ents.iter()
                            .map(|e| bullet(e))
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .unwrap_or_default();
                format!("#### {tag} ({count})\n\n{bullets}")
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    };

    for facet in &facet_order {
        let members = &taxonomy.facets[facet];
        let mut in_facet: Vec<(String, usize)> = canonical
            .iter()
            .filter(|t| members.contains(t))
            .map(|t| (t.clone(), counts.get(t).copied().unwrap_or(0)))
            .collect();
        if in_facet.is_empty() {
            continue;
        }
        sections.push(format!("### {facet}\n\n{}", render_group(&mut in_facet)));
    }

    // Canonical tags in no facet.
    let mut uncat: Vec<(String, usize)> = canonical
        .iter()
        .filter(|t| !facet_order.iter().any(|f| taxonomy.facets[f].contains(t)))
        .map(|t| (t.clone(), counts.get(t).copied().unwrap_or(0)))
        .collect();
    if !uncat.is_empty() {
        sections.push(format!("### Uncategorised\n\n{}", render_group(&mut uncat)));
    }

    // Emerging: counted but not canonical.
    let mut emerging: Vec<(String, usize)> = counts
        .iter()
        .filter(|(t, _)| !canonical.contains(*t))
        .map(|(t, c)| (t.clone(), *c))
        .collect();
    emerging.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    if !emerging.is_empty() {
        let list = emerging
            .iter()
            .map(|(t, c)| format!("{t} ({c})"))
            .collect::<Vec<_>>()
            .join(", ");
        sections.push(format!(
            "<details><summary>Other tags (emerging)</summary>\n\n{list}\n\n</details>"
        ));
    }

    if sections.is_empty() {
        return String::new();
    }
    format!("## By Tag (canonical only)\n\n{}", sections.join("\n\n"))
}

/// `## By Ticket`: entries grouped by ticket key, tickets sorted ascending.
fn render_by_ticket(entries: &[Entry]) -> String {
    let mut ticket_entries: BTreeMap<String, Vec<&Entry>> = BTreeMap::new();
    for e in entries {
        for ticket in &e.tickets {
            ticket_entries.entry(ticket.clone()).or_default().push(e);
        }
    }
    if ticket_entries.is_empty() {
        return String::new();
    }
    let sections: Vec<String> = ticket_entries
        .iter()
        .map(|(ticket, ents)| {
            let bullets = ents
                .iter()
                .map(|e| bullet(e))
                .collect::<Vec<_>>()
                .join("\n");
            format!("### {ticket} ({})\n\n{bullets}", ents.len())
        })
        .collect();
    format!("## By Ticket\n\n{}", sections.join("\n\n"))
}

/// Assemble the complete index document. `today` is a `YYYY-MM-DD` stamp for the
/// header (injected so the output is deterministic in tests).
pub fn render_index(entries: &[Entry], taxonomy: &Taxonomy, today: &str) -> Result<String> {
    let ticket_re = taxonomy.ticket_regex()?;
    let header = format!(
        "# Journal Index\n\n> Auto-generated. Do not edit.\n> {} entries · last updated {}\n",
        entries.len(),
        today
    );
    let sections: Vec<String> = [
        render_recent(entries, 15),
        render_chronological(entries),
        render_by_type(entries, taxonomy),
        render_by_tag(entries, taxonomy, &ticket_re),
        render_by_ticket(entries),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect();
    Ok(format!("{header}\n\n{}\n", sections.join("\n\n")))
}

/// A record's normalised-tag shape: lowercase alphanumerics separated by `-`
/// or `/` (the hierarchical-tag convention).
fn tag_re() -> &'static Regex {
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z0-9]+(?:[-/][a-z0-9]+)*$").expect("valid tag regex"))
}

/// Validate NDJSON index text: parse each line, schema-check, then run
/// cross-record invariants. `file_exists` probes whether an entry file is on
/// disk (skipped when `None`). Returns human-readable errors; empty means valid.
pub fn validate_ndjson(text: &str, file_exists: Option<&dyn Fn(&str) -> bool>) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    let mut records: Vec<IndexRecord> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<IndexRecord>(line) {
            Ok(r) => records.push(r),
            Err(e) => errors.push(format!("record {i}: invalid JSON/schema: {e}")),
        }
    }

    let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("valid date regex");
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for (i, r) in records.iter().enumerate() {
        if r.file.is_empty() {
            errors.push(format!("record {i}: empty file"));
        }
        if r.files.is_empty() {
            errors.push(format!("record {i}: files must be non-empty"));
        }
        if !date_re.is_match(&r.date) {
            errors.push(format!(
                "record {i}: date \"{}\" must be YYYY-MM-DD",
                r.date
            ));
        }
        if !seen.insert(r.file.clone()) {
            errors.push(format!("record {i}: duplicate file \"{}\"", r.file));
        }
        if r.slug.chars().take(10).collect::<String>() != r.date {
            errors.push(format!(
                "record {i}: date \"{}\" does not match slug prefix",
                r.date
            ));
        }
        for t in &r.tags {
            if !tag_re().is_match(t) {
                errors.push(format!("record {i}: tag \"{t}\" is not lowercase-hyphen"));
            }
        }
        if let Some(probe) = file_exists {
            if !probe(&r.file) {
                errors.push(format!("record {i}: file \"{}\" does not exist", r.file));
            }
        }
    }
    errors
}

/// Build the index artifacts (ndjson + markdown) from scanned entries.
/// Validates the ndjson before returning; errors surface uncommittable output.
pub fn build_index(
    entries: &[Entry],
    taxonomy: &Taxonomy,
    today: &str,
) -> Result<(String, String)> {
    let records: Vec<IndexRecord> = entries.iter().map(entry_to_record).collect();
    let ndjson = records_to_ndjson(&records)?;
    let errors = validate_ndjson(&ndjson, None);
    if !errors.is_empty() {
        return Err(Error::Config(format!(
            "generated index failed validation:\n  {}",
            errors.join("\n  ")
        )));
    }
    let view = render_index(entries, taxonomy, today)?;
    Ok((ndjson, view))
}

/// Load an entry path resolver bound to `root` for the `file_exists` probe.
pub fn file_exists_under(root: &Path) -> impl Fn(&str) -> bool + '_ {
    move |rel: &str| root.join(rel).exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Entry;

    fn tax() -> Taxonomy {
        Taxonomy::from_json(
            r#"{ "threshold": 2, "aliases": {}, "facets": { "type": ["troubleshooting", "general"], "tech": ["aws"] }, "pin": [], "suppress": [], "ticketPattern": "^x-[0-9]+$" }"#,
            "test",
        )
        .unwrap()
    }

    fn entry(slug: &str, tags: &[&str], tickets: &[&str], ty: &str) -> Entry {
        Entry {
            file: format!("2026/07/{slug}.md"),
            files: vec![format!("2026/07/{slug}.md")],
            slug: slug.to_string(),
            date: slug.chars().take(10).collect(),
            title: "Title".to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            entry_type: ty.to_string(),
            tickets: tickets.iter().map(|s| s.to_string()).collect(),
            authors: vec![],
            status: String::new(),
            summary: String::new(),
        }
    }

    #[test]
    fn ndjson_is_compact_and_newline_terminated() {
        let recs = vec![entry_to_record(&entry(
            "2026-07-01-a",
            &["aws"],
            &[],
            "general",
        ))];
        let out = records_to_ndjson(&recs).unwrap();
        assert!(out.ends_with('\n'));
        assert_eq!(out.lines().count(), 1);
        assert!(out.contains("\"type\":\"general\""));
        assert!(out.contains("\"file\":\"2026/07/2026-07-01-a.md\""));
    }

    #[test]
    fn empty_records_yield_empty_ndjson() {
        assert_eq!(records_to_ndjson(&[]).unwrap(), "");
    }

    #[test]
    fn canonical_respects_threshold_pin_suppress() {
        let mut counts = BTreeMap::new();
        counts.insert("aws".to_string(), 3usize);
        counts.insert("rare".to_string(), 1usize);
        let mut t = tax();
        t.pin = vec!["rare".to_string()];
        t.suppress = vec!["aws".to_string()];
        let canon = compute_canonical(&counts, &t);
        assert!(canon.contains("rare")); // pinned despite low count
        assert!(!canon.contains("aws")); // suppressed despite high count
    }

    #[test]
    fn render_index_has_all_sections() {
        let entries = vec![
            entry(
                "2026-07-09-b",
                &["aws", "orphan"],
                &["x-1"],
                "troubleshooting",
            ),
            entry("2026-07-01-a", &["aws", "orphan"], &[], "general"),
        ];
        let view = render_index(&entries, &tax(), "2026-07-24").unwrap();
        assert!(view.contains("# Journal Index"));
        assert!(view.contains("2 entries · last updated 2026-07-24"));
        assert!(view.contains("## Recent Entries"));
        assert!(view.contains("## By Month"));
        assert!(view.contains("## By Type"));
        assert!(view.contains("## By Tag (canonical only)"));
        assert!(view.contains("## By Ticket"));
        assert!(view.contains("### x-1 (1)"));
        // aws is faceted (tech); orphan is canonical-but-unfaceted.
        assert!(view.contains("### tech"));
        assert!(view.contains("### Uncategorised"));
    }

    #[test]
    fn validate_flags_bad_records() {
        // date/slug mismatch + uppercase tag + duplicate file.
        let bad = r#"{"file":"2026/07/a.md","files":["2026/07/a.md"],"slug":"2026-07-01-a","date":"2026-07-02","title":"T","tags":["BadTag"],"type":"general","tickets":[],"authors":[],"status":"","summary":""}
{"file":"2026/07/a.md","files":["2026/07/a.md"],"slug":"2026-07-01-a","date":"2026-07-01","title":"T","tags":[],"type":"general","tickets":[],"authors":[],"status":"","summary":""}"#;
        let errors = validate_ndjson(bad, None);
        assert!(errors
            .iter()
            .any(|e| e.contains("does not match slug prefix")));
        assert!(errors.iter().any(|e| e.contains("not lowercase-hyphen")));
        assert!(errors.iter().any(|e| e.contains("duplicate file")));
    }

    #[test]
    fn validate_accepts_clean_records() {
        let recs = vec![entry_to_record(&entry(
            "2026-07-01-a",
            &["aws"],
            &[],
            "general",
        ))];
        let ndjson = records_to_ndjson(&recs).unwrap();
        assert!(validate_ndjson(&ndjson, None).is_empty());
    }
}
