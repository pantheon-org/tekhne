//! Audit status fields across this repo's own follow-ups, plans, merge-queue
//! entries, and superseded journal-entry pairs, and render a markdown digest.
//!
//! Port of the journal CLI's `status/` tree. Unlike `index`/`kb-index`/`lint`/
//! `taxonomy`/`archive-media`, this reads a *consuming repo's own* operational
//! conventions -- `.context/index.yaml` and `.context/merge-queue.yaml` are not
//! part of the generic `journal-entry-creator` skill, and a repo that doesn't
//! use them will simply gather nothing from those sources rather than fail.
//! This module never edits frontmatter or the files it inspects; it only
//! reports, so a human decides every proposed status change.

use std::path::Path;

use common::{Error, Result};
use regex::Regex;

use crate::scan::{fm_scalar, is_dated, split_frontmatter};

/// Default digest output path.
pub const DEFAULT_OUTPUT_PATH: &str = "docs/journal-status-digest.md";

/// One `.context/index.yaml` entry with `status: active` or `draft`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveItem {
    pub path: String,
    pub title: String,
    pub status: String,
}

/// Every `.context/index.yaml` entry (across all top-level groups: plans,
/// follow-ups, findings, etc.) with `status: active` or `draft`. `done` and
/// `superseded` entries are excluded.
pub fn gather_active_items(index_path: &Path) -> Result<Vec<ActiveItem>> {
    let raw = std::fs::read_to_string(index_path).map_err(|e| Error::io(index_path, e))?;
    let parsed: serde_yaml_ng::Value = serde_yaml_ng::from_str(&raw)
        .map_err(|e| Error::Config(format!("cannot parse {}: {e}", index_path.display())))?;

    let mut items = Vec::new();
    if let Some(map) = parsed.as_mapping() {
        for (_group, entries) in map {
            let Some(seq) = entries.as_sequence() else {
                continue;
            };
            for entry in seq {
                let status = entry["status"].as_str().unwrap_or("");
                if status == "active" || status == "draft" {
                    items.push(ActiveItem {
                        path: entry["path"].as_str().unwrap_or("").to_string(),
                        title: entry["title"].as_str().unwrap_or("").to_string(),
                        status: status.to_string(),
                    });
                }
            }
        }
    }
    Ok(items)
}

/// One `.context/merge-queue.yaml` entry not already `status: merged`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeQueueItem {
    pub branch: String,
    pub status: String,
    pub note: Option<String>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct MergeQueueFile {
    #[serde(default)]
    queue: Vec<MergeQueueEntryRaw>,
}

#[derive(Debug, serde::Deserialize)]
struct MergeQueueEntryRaw {
    branch: String,
    status: String,
    #[serde(default)]
    note: Option<String>,
}

/// Every `.context/merge-queue.yaml` entry not already `status: merged`.
pub fn gather_merge_queue_items(queue_path: &Path) -> Result<Vec<MergeQueueItem>> {
    let raw = std::fs::read_to_string(queue_path).map_err(|e| Error::io(queue_path, e))?;
    let parsed: MergeQueueFile = serde_yaml_ng::from_str(&raw)
        .map_err(|e| Error::Config(format!("cannot parse {}: {e}", queue_path.display())))?;

    Ok(parsed
        .queue
        .into_iter()
        .filter(|e| e.status != "merged")
        .map(|e| MergeQueueItem {
            branch: e.branch,
            status: e.status,
            note: e.note,
        })
        .collect())
}

/// One `docs/journal-index.ndjson` record with `status: superseded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersededCandidate {
    pub path: String,
    pub title: String,
}

/// Every `docs/journal-index.ndjson` record with `status: superseded`. Reuses
/// [`crate::index::IndexRecord`] rather than re-deriving the ndjson schema.
pub fn gather_superseded_candidates(ndjson_path: &Path) -> Result<Vec<SupersededCandidate>> {
    let raw = std::fs::read_to_string(ndjson_path).map_err(|e| Error::io(ndjson_path, e))?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let record: crate::index::IndexRecord = serde_json::from_str(trimmed)
            .map_err(|e| Error::json(ndjson_path.display().to_string(), e))?;
        if record.status == "superseded" {
            out.push(SupersededCandidate {
                path: record.file,
                title: record.title,
            });
        }
    }
    Ok(out)
}

/// The outcome of checking a superseded entry's `continued_by` link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersededLinkResult {
    pub ok: bool,
    pub reason: Option<String>,
    pub target_path: Option<String>,
}

/// Join `relative` against `from_file`'s directory, then collapse `.`/`..`
/// segments -- a small POSIX-style logical path normalizer (no filesystem
/// access, no symlink resolution), matching how these repo-relative
/// frontmatter links are written and compared.
fn resolve_relative(from_file: &str, relative: &str) -> String {
    let dir = match from_file.rfind('/') {
        Some(i) => &from_file[..i],
        None => "",
    };
    let joined = if dir.is_empty() {
        relative.to_string()
    } else {
        format!("{dir}/{relative}")
    };
    normalize_posix(&joined)
}

fn normalize_posix(path: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => continue,
            ".." => {
                out.pop();
            }
            s => out.push(s),
        }
    }
    out.join("/")
}

/// Verify a superseded entry's `continued_by` link resolves and the target
/// entry links back via `continues_from`. Either half missing is a broken
/// link, not a partial pass, since a reader following either direction must
/// land somewhere consistent. `root` is the journal root both `path` and any
/// resolved target are read relative to.
pub fn check_superseded_link(root: &Path, path: &str) -> SupersededLinkResult {
    let broken = |reason: String, target_path: Option<String>| SupersededLinkResult {
        ok: false,
        reason: Some(reason),
        target_path,
    };

    let content = match std::fs::read_to_string(root.join(path)) {
        Ok(c) => c,
        Err(e) => return broken(format!("{path} could not be read: {e}"), None),
    };
    let (block, _) = split_frontmatter(&content);
    let continued_by = block
        .as_deref()
        .map(|b| fm_scalar(b, "continued_by"))
        .unwrap_or_default();
    if continued_by.is_empty() {
        return broken(
            format!("{path} has no continued_by despite status: superseded"),
            None,
        );
    }

    let target_path = resolve_relative(path, &continued_by);
    let target_content = match std::fs::read_to_string(root.join(&target_path)) {
        Ok(c) => c,
        Err(_) => {
            return broken(
                format!("continued_by target {target_path} does not exist"),
                Some(target_path),
            )
        }
    };
    let (target_block, _) = split_frontmatter(&target_content);
    let continues_from = target_block
        .as_deref()
        .map(|b| fm_scalar(b, "continues_from"))
        .unwrap_or_default();
    let back_link_resolves_to_source = !continues_from.is_empty()
        && resolve_relative(&target_path, &continues_from) == normalize_posix(path);

    if !back_link_resolves_to_source {
        return broken(
            format!("{target_path} does not have a reciprocal continues_from back to {path}"),
            Some(target_path),
        );
    }

    SupersededLinkResult {
        ok: true,
        reason: None,
        target_path: Some(target_path),
    }
}

/// One commit on `main`, as `sha subject`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitSubject {
    pub sha: String,
    pub subject: String,
}

fn parse_commit_line(line: &str) -> CommitSubject {
    match line.split_once(' ') {
        Some((sha, subject)) => CommitSubject {
            sha: sha.to_string(),
            subject: subject.to_string(),
        },
        None => CommitSubject {
            sha: line.to_string(),
            subject: String::new(),
        },
    }
}

/// Parse `sha subject` lines (as `git log --format=%H %s` prints them) into
/// commit subjects, skipping blank lines.
pub fn parse_commit_subjects(raw: &str) -> Vec<CommitSubject> {
    raw.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(parse_commit_line)
        .collect()
}

/// Every commit subject on `main`, fetched once so many candidates can be
/// matched against it in-memory rather than re-invoking `git log` per
/// candidate.
pub fn list_main_commit_subjects(root: &Path) -> Result<Vec<CommitSubject>> {
    let output = std::process::Command::new("git")
        .args(["log", "main", "--format=%H %s"])
        .current_dir(root)
        .output()
        .map_err(|e| Error::Config(format!("cannot run git log: {e}")))?;
    if !output.status.success() {
        return Err(Error::Config(format!(
            "git log main failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(parse_commit_subjects(&String::from_utf8_lossy(
        &output.stdout,
    )))
}

/// A candidate string to search for among commit subjects: either an exact
/// ticket ID (a "high" confidence signal) or a distinguishing slug (only ever
/// "review-needed", since a commit can mention a slug in passing without
/// actually resolving the item it came from).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Candidate {
    Ticket(String),
    Slug(String),
}

impl Candidate {
    fn value(&self) -> &str {
        match self {
            Candidate::Ticket(v) | Candidate::Slug(v) => v,
        }
    }
}

/// A ticket ID extracted from free text, or a fallback slug when none was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TicketOrSlug {
    pub ticket: Option<String>,
    pub slug: Option<String>,
}

/// `ticketPattern` is anchored (`^...$`) for validating a whole tag; scanning
/// free text needs word boundaries instead, and the prefix/digits captured
/// separately so a hyphen-less mention (e.g. "cc1324") still canonicalises to
/// this repo's "cc-1324" tag form.
fn find_ticket(text: &str, ticket_pattern: &str) -> Option<String> {
    let inner = ticket_pattern.strip_prefix('^').unwrap_or(ticket_pattern);
    let inner = inner.strip_suffix('$').unwrap_or(inner);
    let inner = inner.replace("-?[0-9]+", "-?(\\d+)");
    let re = Regex::new(&format!(r"(?i)\b{inner}\b")).ok()?;
    let caps = re.captures(text)?;
    let prefix = caps.get(1)?.as_str().to_lowercase();
    let digits = caps.get(2)?.as_str();
    Some(format!("{prefix}-{digits}"))
}

fn strip_date_prefix(s: &str) -> &str {
    if is_dated(s) {
        &s[11..]
    } else {
        s
    }
}

/// Extract a ticket ID (per this repo's `ticketPattern`, e.g. `cc-1324`) from
/// a title first, then a filename, since a title-stated ticket is the more
/// deliberate signal. Falls back to a distinguishing slug (the filename's
/// date prefix stripped) when no ticket is found in either.
pub fn extract_ticket_or_slug(title: &str, filename: &str, ticket_pattern: &str) -> TicketOrSlug {
    if let Some(ticket) = find_ticket(title, ticket_pattern) {
        return TicketOrSlug {
            ticket: Some(ticket),
            slug: None,
        };
    }
    if let Some(ticket) = find_ticket(filename, ticket_pattern) {
        return TicketOrSlug {
            ticket: Some(ticket),
            slug: None,
        };
    }
    let stem = filename.strip_suffix(".md").unwrap_or(filename);
    let slug = strip_date_prefix(stem);
    TicketOrSlug {
        ticket: None,
        slug: if slug.is_empty() {
            None
        } else {
            Some(slug.to_string())
        },
    }
}

fn to_candidate(title: &str, filename: &str, ticket_pattern: &str) -> Candidate {
    let TicketOrSlug { ticket, slug } = extract_ticket_or_slug(title, filename, ticket_pattern);
    match ticket {
        Some(t) => Candidate::Ticket(t),
        None => Candidate::Slug(slug.unwrap_or_else(|| filename.to_string())),
    }
}

/// "high" only ever comes from an exact ticket-ID match in a landed `main`
/// commit, or a failed superseded/`continued_by` bidirectional check.
/// Everything else is "review-needed": the routine never edits frontmatter
/// itself either way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    High,
    ReviewNeeded,
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Confidence::High => "high",
            Confidence::ReviewNeeded => "review-needed",
        })
    }
}

/// The result of searching for a candidate among an already-fetched commit list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchResult {
    pub confidence: Confidence,
    pub commits: Vec<CommitSubject>,
}

/// Find every commit whose subject references the candidate.
pub fn match_commit_on_main(candidate: &Candidate, commits: &[CommitSubject]) -> MatchResult {
    let needle = candidate.value().to_lowercase();
    let matches: Vec<CommitSubject> = commits
        .iter()
        .filter(|c| c.subject.to_lowercase().contains(&needle))
        .cloned()
        .collect();
    let confidence = if !matches.is_empty() && matches!(candidate, Candidate::Ticket(_)) {
        Confidence::High
    } else {
        Confidence::ReviewNeeded
    };
    MatchResult {
        confidence,
        commits: matches,
    }
}

fn describe_match(match_result: &MatchResult, candidate_value: &str) -> String {
    match match_result.commits.first() {
        None => format!("no matching commit found on main for \"{candidate_value}\""),
        Some(first) => format!("{} {}", first.sha, first.subject),
    }
}

/// One line in the rendered digest: what it is, what it's flagged as, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestItem {
    pub path: String,
    pub title: String,
    pub current_status: String,
    pub proposed_status: Option<String>,
    pub evidence: String,
    pub confidence: Confidence,
}

/// Everything gathered from this repo's own status-bearing sources, before
/// assessment.
#[derive(Debug, Clone, Default)]
pub struct GatheredItems {
    pub active_items: Vec<ActiveItem>,
    pub merge_queue_items: Vec<MergeQueueItem>,
    pub superseded_candidates: Vec<SupersededCandidate>,
}

/// Assess every gathered item and assemble one flat digest. Nothing here
/// writes frontmatter; a proposed status is a report field only.
pub fn build_digest(
    root: &Path,
    gathered: &GatheredItems,
    ticket_pattern: &str,
    commits: &[CommitSubject],
) -> Vec<DigestItem> {
    let mut items = Vec::new();

    for item in &gathered.active_items {
        let filename = item.path.rsplit('/').next().unwrap_or(&item.path);
        let candidate = to_candidate(&item.title, filename, ticket_pattern);
        let m = match_commit_on_main(&candidate, commits);
        let evidence = describe_match(&m, candidate.value());
        let proposed_status =
            (!m.commits.is_empty() && m.confidence == Confidence::High).then(|| "done".to_string());

        items.push(DigestItem {
            path: item.path.clone(),
            title: item.title.clone(),
            current_status: item.status.clone(),
            proposed_status,
            evidence,
            confidence: m.confidence,
        });
    }

    for item in &gathered.merge_queue_items {
        let candidate = to_candidate(&item.branch, &item.branch, ticket_pattern);
        let m = match_commit_on_main(&candidate, commits);
        let evidence = describe_match(&m, candidate.value());
        let proposed_status = (!m.commits.is_empty() && m.confidence == Confidence::High)
            .then(|| "merged".to_string());

        items.push(DigestItem {
            path: item.branch.clone(),
            title: item.branch.clone(),
            current_status: item.status.clone(),
            proposed_status,
            evidence,
            confidence: m.confidence,
        });
    }

    for candidate in &gathered.superseded_candidates {
        let link = check_superseded_link(root, &candidate.path);
        let (proposed_status, evidence) = if link.ok {
            (
                None,
                "continued_by / continues_from link verified".to_string(),
            )
        } else {
            (
                Some(
                    link.reason
                        .clone()
                        .unwrap_or_else(|| "superseded link is broken".to_string()),
                ),
                link.reason
                    .unwrap_or_else(|| "link check failed".to_string()),
            )
        };
        items.push(DigestItem {
            path: candidate.path.clone(),
            title: candidate.title.clone(),
            current_status: "superseded".to_string(),
            proposed_status,
            evidence,
            confidence: Confidence::High,
        });
    }

    items
}

fn render_item(item: &DigestItem) -> String {
    let transition = match &item.proposed_status {
        Some(p) => format!("{} \u{2192} {p}", item.current_status),
        None => format!("{} (no change proposed)", item.current_status),
    };
    format!(
        "## {}\n\n- **Path:** `{}`\n- **Status:** {}\n- **Confidence:** {}\n- **Evidence:** {}",
        item.title, item.path, transition, item.confidence, item.evidence
    )
}

/// Render a digest as markdown, ending with a machine-checkable "last ran"
/// line (`run-daily-status-routine.sh` parses its leading `YYYY-MM-DD` back
/// out to decide whether today's run already happened).
pub fn render_digest(items: &[DigestItem], generated_at: &str) -> String {
    let sections = if items.is_empty() {
        "No active follow-ups, plans, merge-queue entries, or superseded pairs needed review today."
            .to_string()
    } else {
        items
            .iter()
            .map(render_item)
            .collect::<Vec<_>>()
            .join("\n\n")
    };

    format!(
        "# Daily Journal Status Digest\n\n> Auto-generated. Do not edit. Regenerated by `pantheon-journal status`.\n\n{sections}\n\nStatus routine last ran: {generated_at}, {} item{} flagged",
        items.len(),
        if items.len() == 1 { "" } else { "s" }
    )
}

/// Gather every status-bearing source under `root`, assess it, and render the
/// digest. Returns the digest text and the number of items flagged.
pub fn generate_digest(root: &Path) -> Result<(String, usize)> {
    let active_items = gather_active_items(&root.join(".context/index.yaml"))?;
    let merge_queue_items = gather_merge_queue_items(&root.join(".context/merge-queue.yaml"))?;
    let superseded_candidates =
        gather_superseded_candidates(&root.join(crate::index::INDEX_DATA_PATH))?;
    let commits = list_main_commit_subjects(root)?;
    let (taxonomy, _source) = crate::taxonomy::Taxonomy::resolve(root, None)?;

    let gathered = GatheredItems {
        active_items,
        merge_queue_items,
        superseded_candidates,
    };
    let items = build_digest(root, &gathered, &taxonomy.ticket_pattern, &commits);

    let now = crate::date::Timestamp::now();
    let generated_at = format!("{}T{:02}:{:02}:00Z", now.date.iso(), now.hour, now.minute);
    let count = items.len();
    Ok((render_digest(&items, &generated_at), count))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, rel: &str, content: &str) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    const TICKET_PATTERN: &str = "^(cc|ai|plgsd|sds|srr)-?[0-9]+$";

    // -- gather_active_items --------------------------------------------

    const SAMPLE_INDEX: &str = r#"
plans:
  - path: ".context/plans/2026-01-01-active-plan.md"
    title: "Active Plan"
    status: "active"
    date: 2026-01-01
  - path: ".context/plans/2026-01-02-done-plan.md"
    title: "Done Plan"
    status: "done"
    date: 2026-01-02
follow-ups:
  - path: ".context/follow-ups/2026-01-03-draft-follow-up.md"
    title: "Draft Follow-up"
    status: "draft"
    date: 2026-01-03
  - path: ".context/follow-ups/2026-01-04-done-follow-up.md"
    title: "Done Follow-up"
    status: "done"
    date: 2026-01-04
other:
  - path: ".context/other/2026-01-05-active-other.md"
    title: "Active Other"
    status: "active"
    date: 2026-01-05
"#;

    #[test]
    fn gather_active_items_returns_only_active_and_draft_across_groups() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "index.yaml", SAMPLE_INDEX);
        let mut items = gather_active_items(&tmp.path().join("index.yaml")).unwrap();
        items.sort_by(|a, b| a.path.cmp(&b.path));
        let paths: Vec<&str> = items.iter().map(|i| i.path.as_str()).collect();
        assert_eq!(
            paths,
            vec![
                ".context/follow-ups/2026-01-03-draft-follow-up.md",
                ".context/other/2026-01-05-active-other.md",
                ".context/plans/2026-01-01-active-plan.md",
            ]
        );
        assert!(items.iter().all(|i| i.status != "done"));
    }

    #[test]
    fn gather_active_items_empty_groups_is_empty() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "index.yaml", "plans: []\n");
        let items = gather_active_items(&tmp.path().join("index.yaml")).unwrap();
        assert!(items.is_empty());
    }

    // -- gather_merge_queue_items -----------------------------------------

    const SAMPLE_QUEUE: &str = r#"
queue:
  - branch: "feature/CC-1226-fixture-driven-apply"
    priority: 0
    queued_at: "2026-08-11"
    status: "merged"
    note: "done"
  - branch: "feature/CC-1400-pending-work"
    priority: 0
    queued_at: "2026-08-12"
    status: "pending"
  - branch: "feature/CC-1401-blocked-work"
    priority: 1
    queued_at: "2026-08-13"
    status: "blocked"
    note: "rebase conflict"
"#;

    #[test]
    fn gather_merge_queue_items_excludes_merged() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "queue.yaml", SAMPLE_QUEUE);
        let mut items = gather_merge_queue_items(&tmp.path().join("queue.yaml")).unwrap();
        items.sort_by(|a, b| a.branch.cmp(&b.branch));
        let branches: Vec<&str> = items.iter().map(|i| i.branch.as_str()).collect();
        assert_eq!(
            branches,
            vec![
                "feature/CC-1400-pending-work",
                "feature/CC-1401-blocked-work"
            ]
        );
        assert!(items.iter().all(|i| i.status != "merged"));
    }

    #[test]
    fn gather_merge_queue_items_empty_queue_is_empty() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "queue.yaml", "queue: []\n");
        let items = gather_merge_queue_items(&tmp.path().join("queue.yaml")).unwrap();
        assert!(items.is_empty());
    }

    // -- gather_superseded_candidates -------------------------------------

    #[test]
    fn gather_superseded_candidates_filters_by_status() {
        let tmp = tempfile::tempdir().unwrap();
        let ndjson = format!(
            "{}\n{}\n",
            r#"{"file":"2026/01/2026-01-01-a.md","files":["2026/01/2026-01-01-a.md"],"slug":"2026-01-01-a","date":"2026-01-01","title":"A","tags":[],"type":"general","tickets":[],"authors":[],"status":"published","summary":""}"#,
            r#"{"file":"2026/01/2026-01-02-b.md","files":["2026/01/2026-01-02-b.md"],"slug":"2026-01-02-b","date":"2026-01-02","title":"B","tags":[],"type":"general","tickets":[],"authors":[],"status":"superseded","summary":""}"#,
        );
        write(tmp.path(), "index.ndjson", &ndjson);
        let items = gather_superseded_candidates(&tmp.path().join("index.ndjson")).unwrap();
        assert_eq!(
            items,
            vec![SupersededCandidate {
                path: "2026/01/2026-01-02-b.md".to_string(),
                title: "B".to_string(),
            }]
        );
    }

    #[test]
    fn gather_superseded_candidates_skips_blank_lines() {
        let tmp = tempfile::tempdir().unwrap();
        let ndjson = format!(
            "{}\n\n",
            r#"{"file":"a.md","files":["a.md"],"slug":"a","date":"2026-01-01","title":"A","tags":[],"type":"general","tickets":[],"authors":[],"status":"superseded","summary":""}"#
        );
        write(tmp.path(), "index.ndjson", &ndjson);
        let items = gather_superseded_candidates(&tmp.path().join("index.ndjson")).unwrap();
        assert_eq!(items.len(), 1);
    }

    // -- check_superseded_link ---------------------------------------------

    const OLDER: &str = "---\ntitle: \"Older\"\nstatus: superseded\ncontinued_by: \"../02/2026-02-01-newer.md\"\n---\n";
    const NEWER_RECIPROCAL: &str =
        "---\ntitle: \"Newer\"\ncontinues_from: \"../01/2026-01-01-older.md\"\n---\n";
    const NEWER_NOT_RECIPROCAL: &str = "---\ntitle: \"Newer\"\n---\n";

    #[test]
    fn check_superseded_link_passes_when_reciprocal() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "2026/01/2026-01-01-older.md", OLDER);
        write(tmp.path(), "2026/02/2026-02-01-newer.md", NEWER_RECIPROCAL);
        let result = check_superseded_link(tmp.path(), "2026/01/2026-01-01-older.md");
        assert!(result.ok);
    }

    #[test]
    fn check_superseded_link_fails_when_continued_by_missing() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "2026/01/2026-01-01-older.md",
            "---\ntitle: \"Older\"\nstatus: superseded\n---\n",
        );
        let result = check_superseded_link(tmp.path(), "2026/01/2026-01-01-older.md");
        assert!(!result.ok);
        assert!(result.reason.unwrap().contains("no continued_by"));
    }

    #[test]
    fn check_superseded_link_fails_when_target_does_not_exist() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "2026/01/2026-01-01-older.md", OLDER);
        let result = check_superseded_link(tmp.path(), "2026/01/2026-01-01-older.md");
        assert!(!result.ok);
        assert!(result.reason.unwrap().contains("target"));
    }

    #[test]
    fn check_superseded_link_fails_when_target_does_not_link_back() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "2026/01/2026-01-01-older.md", OLDER);
        write(
            tmp.path(),
            "2026/02/2026-02-01-newer.md",
            NEWER_NOT_RECIPROCAL,
        );
        let result = check_superseded_link(tmp.path(), "2026/01/2026-01-01-older.md");
        assert!(!result.ok);
        assert!(result.reason.unwrap().contains("reciprocal"));
    }

    // -- extract_ticket_or_slug ---------------------------------------------

    #[test]
    fn extract_ticket_or_slug_finds_ticket_in_title() {
        let result = extract_ticket_or_slug(
            "CC-1324 (empty-CSV validation bug) is open and unassigned",
            "2026-08-11-cc-1324-stale-unfixed-recurring-incident.md",
            TICKET_PATTERN,
        );
        assert_eq!(result.ticket.as_deref(), Some("cc-1324"));
        assert_eq!(result.slug, None);
    }

    #[test]
    fn extract_ticket_or_slug_finds_ticket_in_filename() {
        let result = extract_ticket_or_slug(
            "Register real Jenkins jobs",
            "2026-08-05-cc-1393-jenkins-job-registration.md",
            TICKET_PATTERN,
        );
        assert_eq!(result.ticket.as_deref(), Some("cc-1393"));
    }

    #[test]
    fn extract_ticket_or_slug_falls_back_to_slug() {
        let result = extract_ticket_or_slug(
            "Nightly context digest for active priorities and open loops",
            "2026-08-11-nightly-context-digest-for-active-priorities-and-open-loops.md",
            TICKET_PATTERN,
        );
        assert_eq!(result.ticket, None);
        assert_eq!(
            result.slug.as_deref(),
            Some("nightly-context-digest-for-active-priorities-and-open-loops")
        );
    }

    #[test]
    fn extract_ticket_or_slug_prefers_title_over_filename() {
        let result = extract_ticket_or_slug(
            "Follow-up referencing PLGSD-100",
            "2026-08-05-cc-1393-jenkins-job-registration.md",
            TICKET_PATTERN,
        );
        assert_eq!(result.ticket.as_deref(), Some("plgsd-100"));
    }

    // -- match_commit_on_main ------------------------------------------------

    #[test]
    fn match_commit_on_main_high_confidence_for_ticket_match() {
        let result = match_commit_on_main(
            &Candidate::Ticket("cc-1324".to_string()),
            &[CommitSubject {
                sha: "abc1234".to_string(),
                subject: "fix(cc-1324): resolve empty-csv validation".to_string(),
            }],
        );
        assert_eq!(result.confidence, Confidence::High);
        assert_eq!(result.commits.len(), 1);
    }

    #[test]
    fn match_commit_on_main_review_needed_for_slug_match() {
        let result = match_commit_on_main(
            &Candidate::Slug("nightly-context-digest".to_string()),
            &[CommitSubject {
                sha: "def5678".to_string(),
                subject: "docs: mention nightly-context-digest in passing".to_string(),
            }],
        );
        assert_eq!(result.confidence, Confidence::ReviewNeeded);
        assert_eq!(result.commits.len(), 1);
    }

    #[test]
    fn match_commit_on_main_no_match() {
        let result = match_commit_on_main(
            &Candidate::Ticket("cc-9999".to_string()),
            &[CommitSubject {
                sha: "abc".to_string(),
                subject: "unrelated change".to_string(),
            }],
        );
        assert_eq!(result.confidence, Confidence::ReviewNeeded);
        assert!(result.commits.is_empty());
    }

    #[test]
    fn match_commit_on_main_case_insensitive() {
        let result = match_commit_on_main(
            &Candidate::Ticket("cc-1324".to_string()),
            &[CommitSubject {
                sha: "abc".to_string(),
                subject: "FIX(CC-1324): done".to_string(),
            }],
        );
        assert_eq!(result.commits.len(), 1);
    }

    // -- parse_commit_subjects -----------------------------------------------

    #[test]
    fn parse_commit_subjects_parses_sha_subject_pairs() {
        let commits = parse_commit_subjects(
            "abc1234 fix(cc-1324): resolve empty-csv validation\ndef5678 docs: unrelated change\n",
        );
        assert_eq!(
            commits,
            vec![
                CommitSubject {
                    sha: "abc1234".to_string(),
                    subject: "fix(cc-1324): resolve empty-csv validation".to_string(),
                },
                CommitSubject {
                    sha: "def5678".to_string(),
                    subject: "docs: unrelated change".to_string(),
                },
            ]
        );
    }

    #[test]
    fn parse_commit_subjects_skips_blank_lines() {
        let commits = parse_commit_subjects("abc1234 fix: something\n\n\n");
        assert_eq!(commits.len(), 1);
    }

    #[test]
    fn parse_commit_subjects_empty_log_is_empty() {
        assert!(parse_commit_subjects("").is_empty());
    }

    // -- build_digest ----------------------------------------------------

    #[test]
    fn build_digest_proposes_done_on_exact_ticket_match() {
        let tmp = tempfile::tempdir().unwrap();
        let gathered = GatheredItems {
            active_items: vec![ActiveItem {
                path: ".context/follow-ups/2026-08-11-cc-1324-bug.md".to_string(),
                title: "CC-1324 bug is open".to_string(),
                status: "active".to_string(),
            }],
            ..Default::default()
        };
        let commits = vec![CommitSubject {
            sha: "abc123".to_string(),
            subject: "fix(cc-1324): done".to_string(),
        }];
        let items = build_digest(tmp.path(), &gathered, TICKET_PATTERN, &commits);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].proposed_status.as_deref(), Some("done"));
        assert_eq!(items[0].confidence, Confidence::High);
        assert_eq!(items[0].evidence, "abc123 fix(cc-1324): done");
    }

    #[test]
    fn build_digest_review_needed_when_nothing_matches() {
        let tmp = tempfile::tempdir().unwrap();
        let gathered = GatheredItems {
            active_items: vec![ActiveItem {
                path: "a.md".to_string(),
                title: "Untracked follow-up".to_string(),
                status: "draft".to_string(),
            }],
            ..Default::default()
        };
        let items = build_digest(tmp.path(), &gathered, TICKET_PATTERN, &[]);
        assert_eq!(items[0].proposed_status, None);
        assert_eq!(items[0].confidence, Confidence::ReviewNeeded);
    }

    #[test]
    fn build_digest_proposes_merged_for_merge_queue_ticket_match() {
        let tmp = tempfile::tempdir().unwrap();
        let gathered = GatheredItems {
            merge_queue_items: vec![MergeQueueItem {
                branch: "feature/CC-1400-fix".to_string(),
                status: "pending".to_string(),
                note: None,
            }],
            ..Default::default()
        };
        let commits = vec![CommitSubject {
            sha: "def456".to_string(),
            subject: "fix(cc-1400): landed".to_string(),
        }];
        let items = build_digest(tmp.path(), &gathered, TICKET_PATTERN, &commits);
        assert_eq!(items[0].path, "feature/CC-1400-fix");
        assert_eq!(items[0].current_status, "pending");
        assert_eq!(items[0].proposed_status.as_deref(), Some("merged"));
        assert_eq!(items[0].confidence, Confidence::High);
    }

    #[test]
    fn build_digest_flags_broken_superseded_link_and_passes_a_good_one() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "old2.md",
            "---\ntitle: \"Old2\"\nstatus: superseded\ncontinued_by: \"new2.md\"\n---\n",
        );
        write(
            tmp.path(),
            "new2.md",
            "---\ntitle: \"New2\"\ncontinues_from: \"old2.md\"\n---\n",
        );
        // old.md is deliberately never written, so its continued_by read fails.
        let gathered = GatheredItems {
            superseded_candidates: vec![
                SupersededCandidate {
                    path: "old.md".to_string(),
                    title: "Old".to_string(),
                },
                SupersededCandidate {
                    path: "old2.md".to_string(),
                    title: "Old2".to_string(),
                },
            ],
            ..Default::default()
        };
        let items = build_digest(tmp.path(), &gathered, TICKET_PATTERN, &[]);
        assert_eq!(items[0].path, "old.md");
        assert_eq!(items[0].confidence, Confidence::High);
        assert!(items[0].proposed_status.is_some());
        assert_eq!(items[1].path, "old2.md");
        assert_eq!(items[1].proposed_status, None);
    }

    // -- render_digest -----------------------------------------------------

    #[test]
    fn render_digest_renders_items_with_status_and_evidence() {
        let items = vec![
            DigestItem {
                path: "a.md".to_string(),
                title: "A".to_string(),
                current_status: "active".to_string(),
                proposed_status: Some("done".to_string()),
                evidence: "abc123 fix(cc-1): done".to_string(),
                confidence: Confidence::High,
            },
            DigestItem {
                path: "b.md".to_string(),
                title: "B".to_string(),
                current_status: "draft".to_string(),
                proposed_status: None,
                evidence: "no matching commit found on main for \"b\"".to_string(),
                confidence: Confidence::ReviewNeeded,
            },
        ];
        let text = render_digest(&items, "2026-08-18T10:00:00.000Z");
        assert!(text.contains("# Daily Journal Status Digest"));
        assert!(text.contains("active \u{2192} done"));
        assert!(text.contains("high"));
        assert!(text.contains("review-needed"));
        assert!(text.contains("Status routine last ran: 2026-08-18T10:00:00.000Z, 2 items flagged"));
    }

    #[test]
    fn render_digest_emits_last_ran_line_with_zero_items() {
        let text = render_digest(&[], "2026-08-18T10:00:00.000Z");
        assert!(text.contains("Status routine last ran: 2026-08-18T10:00:00.000Z, 0 items flagged"));
    }
}
