//! The command implementations, separated from argument parsing.
//!
//! Every command returns the text to print rather than writing to stdout, so
//! its output is asserted in tests instead of eyeballed. Commands that gate on
//! something (`check`, `review`, `pre-push`) return the text alongside a
//! verdict, because the caller needs both: the report is worth printing even
//! when the answer is no.

use std::path::Path;

use common::{Error, Result};

use crate::completeness::{self, Report, PASS_THRESHOLD, STRICT_THRESHOLD};
use crate::record::{BranchType, Record, Status};
use crate::{draft, git, hooks, index, store, sync};

/// Everything `create` needs, gathered so the signature stays readable.
#[derive(Debug, Default, Clone)]
pub struct CreateArgs<'a> {
    /// An explicit slug; taken from the branch when absent.
    pub slug: Option<&'a str>,
    /// The one-line description, used as the record's title.
    pub description: Option<&'a str>,
    /// An explicit branch type; taken from the branch prefix when absent.
    pub branch_type: Option<&'a str>,
    /// An explicit author; taken from `git config user.name` when absent.
    pub author: Option<&'a str>,
}

/// Everything `update` can change.
#[derive(Debug, Default, Clone)]
pub struct UpdateArgs<'a> {
    /// A replacement description, which is the record's title.
    pub description: Option<&'a str>,
    /// A new status.
    pub status: Option<&'a str>,
    /// A replacement tag list.
    pub tags: Option<&'a str>,
    /// The slug that supersedes this record.
    pub superseded_by: Option<&'a str>,
}

/// Create the ADR directory and, optionally, the pre-push hook.
pub fn init(
    dir: &Path,
    cwd: &Path,
    project: Option<&str>,
    install_hooks: bool,
    uninstall_hooks: bool,
) -> Result<String> {
    store::ensure_dir(dir)?;
    let project = project
        .map(str::to_string)
        .or_else(|| git::repo_name(cwd))
        .unwrap_or_else(|| "this project".to_string());

    let mut out = format!(
        "Initialised the ADR log for {project} at {}\n",
        dir.display()
    );

    if install_hooks || uninstall_hooks {
        let git_dir = git_dir(cwd)?;
        if install_hooks {
            let path = hooks::install(&git_dir)?;
            out.push_str(&format!(
                "Installed the pre-push hook at {}\n",
                path.display()
            ));
            out.push_str("To undo: run the same command with --uninstall-hooks\n");
        } else {
            match hooks::uninstall(&git_dir)? {
                Some(path) => out.push_str(&format!(
                    "Removed the pre-push hook at {}\n",
                    path.display()
                )),
                None => out.push_str("No pre-push hook was installed\n"),
            }
        }
    } else {
        out.push_str("Run `pantheon-adr sync` to wire agent hooks, or pass --install-hooks for the git hook\n");
    }
    Ok(out)
}

/// The `.git` directory for `cwd`.
fn git_dir(cwd: &Path) -> Result<std::path::PathBuf> {
    git::repo_root(cwd)
        .map(|root| Path::new(&root).join(".git"))
        .ok_or_else(|| Error::Config("not inside a git repository".to_string()))
}

/// Create one record from the template and register the branch it belongs to.
pub fn create(dir: &Path, cwd: &Path, today: &str, args: &CreateArgs<'_>) -> Result<String> {
    let branch = git::current_branch(cwd);
    let (slug, derived_type) = match args.slug {
        Some(explicit) => {
            let (slug, _) = crate::record::from_branch(explicit);
            (slug, BranchType::Chore)
        }
        None => {
            let branch = branch.as_deref().ok_or_else(|| {
                Error::Config(
                    "no branch to name the record after (detached HEAD?); pass a slug instead"
                        .to_string(),
                )
            })?;
            crate::record::from_branch(branch)
        }
    };

    let branch_type = match args.branch_type {
        Some(t) => BranchType::parse(t)?,
        None => derived_type,
    };

    if store::exists(dir, &slug) {
        return Err(Error::Config(format!(
            "{} already exists",
            store::path_for(dir, &slug).display()
        )));
    }

    let title = args
        .description
        .map(str::to_string)
        .unwrap_or_else(|| crate::record::title_from_slug(&slug));
    let author = args
        .author
        .map(str::to_string)
        .or_else(|| git::user_name(cwd))
        .unwrap_or_else(|| "unknown".to_string());

    let mut record = Record::new(
        &slug,
        &title,
        branch_type,
        &author,
        today,
        branch.as_deref(),
    );
    if let Some(b) = &branch {
        let base = git::resolve_base(cwd, None, &["main", "master"]);
        if &base != b {
            record.meta.changed_files = git::changed_files(cwd, &base);
        }
    }

    let path = store::save(dir, &record)?;
    Ok(format!("Created {}\n", path.display()))
}

/// Aggregate branch context for filling a record in.
pub fn draft_cmd(
    dir: &Path,
    cwd: &Path,
    slug: Option<&str>,
    bootstrap: bool,
    base: Option<&str>,
    with_pr: bool,
    json: bool,
) -> Result<String> {
    let all = store::load_all(dir)?;
    let slug = resolve_slug(cwd, slug, &all)?;
    let record = store::load(dir, &slug)?;

    let default_base = git::resolve_base(cwd, None, &["main", "master"]);
    let ctx = git::context(cwd, base, &default_base);
    let d = draft::build(&record, &ctx, &all);

    if json {
        return common::output::to_json_pretty(&d).map(|s| format!("{s}\n"));
    }
    if bootstrap {
        return Ok(draft::render_bootstrap(&d));
    }
    let feedback = if with_pr {
        draft::pr_comments(cwd)
    } else {
        None
    };
    Ok(draft::render(&d, feedback.as_deref()))
}

/// The slug to act on: the explicit one, or the current branch's.
fn resolve_slug(cwd: &Path, explicit: Option<&str>, all: &[Record]) -> Result<String> {
    if let Some(slug) = explicit.filter(|s| !s.is_empty()) {
        return Ok(slug.to_string());
    }
    let branch = git::current_branch(cwd).ok_or_else(|| {
        Error::Config("no branch to infer the record from; pass a slug".to_string())
    })?;
    let (slug, _) = crate::record::from_branch(&branch);
    if all.iter().any(|r| r.slug == slug) {
        return Ok(slug);
    }
    Err(Error::Config(format!(
        "branch {branch:?} has no record (expected slug {slug:?}); run `pantheon-adr create` first"
    )))
}

/// Score a record, returning the report text and whether it passed.
pub fn check(
    dir: &Path,
    slug: &str,
    strict: bool,
    json: bool,
) -> Result<(String, std::result::Result<(), Error>)> {
    let record = store::load(dir, slug)?;
    let threshold = if strict {
        STRICT_THRESHOLD
    } else {
        PASS_THRESHOLD
    };
    let report = completeness::check(&record.body, threshold);

    let text = if json {
        common::output::to_json_pretty(&report).map(|s| format!("{s}\n"))?
    } else {
        render_report(slug, &report)
    };
    let verdict = if report.pass {
        Ok(())
    } else {
        Err(Error::Config(format!(
            "{slug}: score {} is below {threshold}",
            report.score
        )))
    };
    Ok((text, verdict))
}

/// Render a completeness report for a human.
fn render_report(slug: &str, report: &Report) -> String {
    let mut out = format!("ADR: {slug}\n");
    out.push_str(&format!(
        "Score: {}/100  [{}]\n",
        report.score,
        if report.pass {
            "complete"
        } else {
            "incomplete"
        }
    ));

    if !report.caps_applied.is_empty() {
        out.push_str("\nCaps applied:\n");
        for cap in &report.caps_applied {
            out.push_str(&format!("  ! {cap}\n"));
        }
    }
    if !report.missing.is_empty() {
        out.push_str("\nMissing:\n");
        for item in &report.missing {
            let note = item.note.as_deref().unwrap_or("");
            out.push_str(&format!(
                "  x {:<34} (-{:>2})  {note}\n",
                item.name, item.weight
            ));
        }
    }
    if !report.filled.is_empty() {
        out.push_str("\nFilled:\n");
        for item in &report.filled {
            out.push_str(&format!("  ok {}\n", item.name));
        }
    }
    out
}

/// Move a record to `review-requested`, if it scores well enough.
pub fn review(dir: &Path, slug: &str, now: &str) -> Result<String> {
    let mut record = store::load(dir, slug)?;
    let report = completeness::check(&record.body, PASS_THRESHOLD);

    if !report.pass {
        let mut out = format!(
            "{slug} is incomplete (score {}/100); finish it before requesting review.\nMissing:\n",
            report.score
        );
        for item in &report.missing {
            out.push_str(&format!("  x {}\n", item.name));
        }
        return Err(Error::Config(out));
    }

    record.transition(Status::ReviewRequested, now);
    store::save(dir, &record)?;
    Ok(format!(
        "{slug} marked for review (score {}/100)\n",
        report.score
    ))
}

/// Change a record's metadata.
pub fn update(dir: &Path, slug: &str, now: &str, args: &UpdateArgs<'_>) -> Result<String> {
    let mut record = store::load(dir, slug)?;
    let mut changes = Vec::new();

    if let Some(description) = args.description {
        record.meta.title = description.to_string();
        changes.push("description".to_string());
    }
    if let Some(tags) = args.tags {
        record.meta.tags = tags
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(str::to_string)
            .collect();
        changes.push("tags".to_string());
    }
    if let Some(status) = args.status {
        record.transition(Status::parse(status)?, now);
        changes.push(format!("status to {status}"));
    }
    if let Some(next) = args.superseded_by {
        if !store::exists(dir, next) {
            return Err(Error::Config(format!(
                "no record named {next:?}; create the replacement before superseding {slug:?}"
            )));
        }
        record.meta.superseded_by = Some(next.to_string());
        record.transition(Status::Superseded, now);
        changes.push(format!("superseded by {next}"));

        // Record the reverse link on the replacement, so the chain is
        // navigable from either end without a second command.
        let mut replacement = store::load(dir, next)?;
        if replacement.meta.supersedes.as_deref() != Some(slug) {
            replacement.meta.supersedes = Some(slug.to_string());
            store::save(dir, &replacement)?;
        }
    }

    if changes.is_empty() {
        return Err(Error::Config(format!(
            "nothing to change on {slug:?}; pass at least one of --description, --status, --tags or --superseded-by"
        )));
    }
    store::save(dir, &record)?;
    Ok(format!("Updated {slug}: {}\n", changes.join(", ")))
}

/// List records, optionally filtered.
pub fn list(
    dir: &Path,
    status: Option<&str>,
    branch_type: Option<&str>,
    json: bool,
) -> Result<String> {
    let mut records = store::load_all(dir)?;
    if let Some(s) = status {
        let want = Status::parse(s)?;
        records.retain(|r| r.meta.status == want);
    }
    if let Some(t) = branch_type {
        let want = BranchType::parse(t)?;
        records.retain(|r| r.meta.branch_type == want);
    }

    if json {
        return common::output::to_json_pretty(&index::rows(&records)).map(|s| format!("{s}\n"));
    }
    if records.is_empty() {
        return Ok("No ADRs found.\n".to_string());
    }

    let slug_width = records.iter().map(|r| r.slug.len()).max().unwrap_or(0);
    let status_width = records
        .iter()
        .map(|r| r.meta.status.as_str().len())
        .max()
        .unwrap_or(0);

    let mut out = format!("{} ADR(s)\n", records.len());
    for r in &records {
        out.push_str(&format!(
            "  {:<5} {:<slug_width$} {} {:<status_width$} {}\n",
            r.meta.branch_type.as_str(),
            r.slug,
            r.meta.date,
            r.meta.status.as_str(),
            r.meta.title,
        ));
    }
    Ok(out)
}

/// Summarise the collection.
pub fn status(dir: &Path, cwd: &Path, json: bool) -> Result<String> {
    let records = store::load_all(dir)?;
    let summary = store::summarise(&records);

    if json {
        let payload = serde_json::json!({
            "total": summary.total,
            "by_status": counts_to_object(&summary.by_status),
            "by_type": counts_to_object(&summary.by_type),
        });
        return common::output::to_json_pretty(&payload).map(|s| format!("{s}\n"));
    }

    let project = git::repo_name(cwd).unwrap_or_else(|| "this project".to_string());
    let mut out = format!(
        "ADR Status\n  Total:   {}\n  Project: {project}\n",
        summary.total
    );
    if !summary.by_status.is_empty() {
        out.push_str("\n  By status:\n");
        for (name, n) in &summary.by_status {
            out.push_str(&format!("    {name:<17}{n}\n"));
        }
    }
    if !summary.by_type.is_empty() {
        out.push_str("\n  By type:\n");
        for (name, n) in &summary.by_type {
            out.push_str(&format!("    {name:<17}{n}\n"));
        }
    }
    Ok(out)
}

/// Turn a list of name/count pairs into a JSON object, preserving order.
fn counts_to_object(
    counts: &[(&'static str, usize)],
) -> serde_json::Map<String, serde_json::Value> {
    counts
        .iter()
        .map(|(name, n)| ((*name).to_string(), serde_json::json!(n)))
        .collect()
}

/// Regenerate the catalogue.
pub fn index_cmd(dir: &Path, today: &str) -> Result<String> {
    let records = store::load_all(dir)?;
    let path = index::write(dir, &records, today)?;
    Ok(format!(
        "Wrote {} ({} record{})\n",
        path.display(),
        records.len(),
        if records.len() == 1 { "" } else { "s" }
    ))
}

/// Wire the ADR hooks into whichever agent harnesses the project uses.
pub fn sync_cmd(cwd: &Path) -> Result<String> {
    let root = git::repo_root(cwd).unwrap_or_else(|| cwd.display().to_string());
    let results = sync::sync_all(Path::new(&root))?;

    let detected: Vec<&sync::SyncResult> = results
        .iter()
        .filter(|r| r.outcome != sync::Outcome::NotDetected)
        .collect();
    if detected.is_empty() {
        return Ok(
            "No agent configurations detected. Expected one of: .claude/, opencode.json, .opencode/\n"
                .to_string(),
        );
    }

    let mut out = String::new();
    for r in detected {
        let verb = match r.outcome {
            sync::Outcome::Synced => "synced  ",
            sync::Outcome::AlreadySynced => "ok      ",
            sync::Outcome::NotDetected => continue,
        };
        out.push_str(&format!("{verb} {} -> {}\n", r.name, r.path.display()));
    }
    Ok(out)
}

/// Print ADR context at the start of an agent session.
pub fn session_start(dir: &Path, cwd: &Path) -> String {
    let Ok(records) = store::load_all(dir) else {
        return String::new();
    };
    let project = git::repo_name(cwd).unwrap_or_else(|| "this project".to_string());
    hooks::session_start(&records, &project).unwrap_or_default()
}

/// Nudge once when the current branch has no record.
pub fn post_tool_use(dir: &Path, cwd: &Path) -> String {
    let Ok(records) = store::load_all(dir) else {
        return String::new();
    };
    let Some(root) = git::repo_root(cwd) else {
        return String::new();
    };
    let branch = git::current_branch(cwd);
    let slug = branch
        .as_deref()
        .map(|b| crate::record::from_branch(b).0)
        .unwrap_or_default();
    hooks::post_tool_use(
        &Path::new(&root).join(".git"),
        &records,
        branch.as_deref(),
        &slug,
    )
    .unwrap_or_default()
}

/// Remind about unfinished records at the end of a session.
pub fn session_end(dir: &Path) -> String {
    let Ok(records) = store::load_all(dir) else {
        return String::new();
    };
    hooks::session_end(&records).unwrap_or_default()
}

/// The pre-push gate.
pub fn pre_push(dir: &Path) -> (String, std::result::Result<(), Error>) {
    match store::load_all(dir) {
        Ok(records) => hooks::pre_push(&records),
        // A malformed or absent log must not block a push; that is what
        // `check` is for.
        Err(_) => (String::new(), Ok(())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const PROSE: &str = "Requests cross four services and we cannot follow a single one end to \
        end because each service writes its own log format with no shared correlation identifier, \
        so reconstructing a slow request means grepping four log stores and guessing at ordering.";

    fn dirs() -> (TempDir, std::path::PathBuf) {
        let tmp = TempDir::new().expect("temp dir");
        let dir = tmp.path().join("docs").join("adr");
        (tmp, dir)
    }

    fn make(dir: &Path, slug: &str) -> Record {
        let r = Record::new(
            slug,
            slug,
            BranchType::Feat,
            "Test User",
            "2026-09-15",
            None,
        );
        store::save(dir, &r).expect("save");
        r
    }

    fn fill(dir: &Path, slug: &str) {
        let mut r = store::load(dir, slug).expect("load");
        r.body = format!(
            "## Problem Statement\n### Context\n{PROSE}\n\n### Chosen Solution\n{PROSE}\n\n\
             ### Rationale\n{PROSE}\n\n## Impact Assessment\n- **Security**: reviewed\n"
        );
        store::save(dir, &r).expect("save");
    }

    #[test]
    fn init_creates_the_directory_and_says_where() {
        let (tmp, dir) = dirs();
        let out = init(&dir, tmp.path(), Some("demo"), false, false).expect("init");

        assert!(dir.is_dir());
        assert!(out.contains("demo"), "{out}");
        assert!(out.contains("sync"), "{out}");
    }

    #[test]
    fn init_hooks_need_a_repository() {
        let (tmp, dir) = dirs();
        let err = init(&dir, tmp.path(), None, true, false)
            .unwrap_err()
            .to_string();
        assert!(err.contains("git repository"), "{err}");
    }

    #[test]
    fn create_refuses_to_overwrite_an_existing_record() {
        let (tmp, dir) = dirs();
        make(&dir, "adopt-otel");

        let args = CreateArgs {
            slug: Some("adopt-otel"),
            ..Default::default()
        };
        let err = create(&dir, tmp.path(), "2026-09-15", &args)
            .unwrap_err()
            .to_string();
        assert!(err.contains("already exists"), "{err}");
    }

    #[test]
    fn create_with_an_explicit_slug_works_outside_a_repository() {
        let (tmp, dir) = dirs();
        let args = CreateArgs {
            slug: Some("Adopt OpenTelemetry!"),
            description: Some("Adopt OpenTelemetry for tracing"),
            ..Default::default()
        };
        let out = create(&dir, tmp.path(), "2026-09-15", &args).expect("create");

        assert!(out.contains("adopt-opentelemetry.md"), "{out}");
        let r = store::load(&dir, "adopt-opentelemetry").expect("load");
        assert_eq!(r.meta.title, "Adopt OpenTelemetry for tracing");
        assert_eq!(r.meta.status, Status::Proposed);
    }

    #[test]
    fn create_without_a_description_titles_from_the_slug() {
        let (tmp, dir) = dirs();
        let args = CreateArgs {
            slug: Some("adopt-otel"),
            ..Default::default()
        };
        create(&dir, tmp.path(), "2026-09-15", &args).expect("create");
        assert_eq!(
            store::load(&dir, "adopt-otel").expect("load").meta.title,
            "Adopt otel"
        );
    }

    #[test]
    fn create_rejects_an_unknown_type() {
        let (tmp, dir) = dirs();
        let args = CreateArgs {
            slug: Some("x"),
            branch_type: Some("refactor"),
            ..Default::default()
        };
        assert!(create(&dir, tmp.path(), "2026-09-15", &args).is_err());
    }

    #[test]
    fn create_outside_a_repository_without_a_slug_explains_itself() {
        let (tmp, dir) = dirs();
        let err = create(&dir, tmp.path(), "2026-09-15", &CreateArgs::default())
            .unwrap_err()
            .to_string();
        assert!(err.contains("pass a slug"), "{err}");
    }

    #[test]
    fn check_scores_the_template_at_zero_and_fails() {
        let (_tmp, dir) = dirs();
        make(&dir, "blank");

        let (text, verdict) = check(&dir, "blank", false, false).expect("check");
        assert!(text.contains("Score: 0/100  [incomplete]"), "{text}");
        assert!(text.contains("Caps applied:"), "{text}");
        assert!(verdict.is_err());
    }

    #[test]
    fn check_passes_a_finished_record() {
        let (_tmp, dir) = dirs();
        make(&dir, "ready");
        fill(&dir, "ready");

        let (text, verdict) = check(&dir, "ready", false, false).expect("check");
        assert!(text.contains("Score: 100/100  [complete]"), "{text}");
        assert!(verdict.is_ok());
    }

    #[test]
    fn check_json_is_machine_readable() {
        let (_tmp, dir) = dirs();
        make(&dir, "blank");

        let (text, _) = check(&dir, "blank", false, true).expect("check");
        let value: serde_json::Value = serde_json::from_str(&text).expect("valid json");
        assert_eq!(value["score"], 0);
        assert_eq!(value["pass"], false);
        assert!(value["missing"].is_array());
    }

    #[test]
    fn check_reports_a_missing_record_as_not_found() {
        let (_tmp, dir) = dirs();
        store::ensure_dir(&dir).expect("dir");
        assert!(matches!(
            check(&dir, "nope", false, false).unwrap_err(),
            Error::NotFound(_)
        ));
    }

    #[test]
    fn strict_check_demands_more() {
        let (_tmp, dir) = dirs();
        make(&dir, "ready");
        fill(&dir, "ready");
        let mut r = store::load(&dir, "ready").expect("load");
        r.body.push_str("\n## Outcome & Lessons\n<!-- later -->\n");
        store::save(&dir, &r).expect("save");

        assert!(check(&dir, "ready", false, false).expect("check").1.is_ok());
        assert!(check(&dir, "ready", true, false).expect("check").1.is_err());
    }

    #[test]
    fn review_refuses_an_unfinished_record() {
        let (_tmp, dir) = dirs();
        make(&dir, "blank");

        let err = review(&dir, "blank", "2026-09-15T10:00:00Z")
            .unwrap_err()
            .to_string();
        assert!(err.contains("incomplete"), "{err}");
        assert_eq!(
            store::load(&dir, "blank").expect("load").meta.status,
            Status::Proposed,
            "a refused review must not change the record"
        );
    }

    #[test]
    fn review_advances_a_finished_record_and_records_history() {
        let (_tmp, dir) = dirs();
        make(&dir, "ready");
        fill(&dir, "ready");

        let out = review(&dir, "ready", "2026-09-15T10:00:00Z").expect("review");
        assert!(out.contains("marked for review"), "{out}");

        let r = store::load(&dir, "ready").expect("load");
        assert_eq!(r.meta.status, Status::ReviewRequested);
        assert_eq!(r.meta.history.len(), 1);
        assert_eq!(r.meta.history[0].at, "2026-09-15T10:00:00Z");
    }

    #[test]
    fn update_changes_description_tags_and_status_together() {
        let (_tmp, dir) = dirs();
        make(&dir, "adopt-otel");

        let args = UpdateArgs {
            description: Some("Adopt OpenTelemetry"),
            status: Some("accepted"),
            tags: Some("tracing, observability ,"),
            superseded_by: None,
        };
        let out = update(&dir, "adopt-otel", "2026-09-15T10:00:00Z", &args).expect("update");
        assert!(out.contains("description"), "{out}");

        let r = store::load(&dir, "adopt-otel").expect("load");
        assert_eq!(r.meta.title, "Adopt OpenTelemetry");
        assert_eq!(r.meta.status, Status::Accepted);
        assert_eq!(
            r.meta.tags,
            vec!["tracing".to_string(), "observability".to_string()]
        );
    }

    #[test]
    fn update_with_no_flags_says_so() {
        let (_tmp, dir) = dirs();
        make(&dir, "x");
        let err = update(&dir, "x", "2026-09-15T10:00:00Z", &UpdateArgs::default())
            .unwrap_err()
            .to_string();
        assert!(err.contains("nothing to change"), "{err}");
    }

    #[test]
    fn update_rejects_an_invalid_status() {
        let (_tmp, dir) = dirs();
        make(&dir, "x");
        let args = UpdateArgs {
            status: Some("rejected"),
            ..Default::default()
        };
        assert!(update(&dir, "x", "2026-09-15T10:00:00Z", &args).is_err());
    }

    #[test]
    fn superseding_links_both_records() {
        let (_tmp, dir) = dirs();
        make(&dir, "adopt-otel");
        make(&dir, "adopt-tempo");

        let args = UpdateArgs {
            superseded_by: Some("adopt-tempo"),
            ..Default::default()
        };
        update(&dir, "adopt-otel", "2026-09-15T10:00:00Z", &args).expect("update");

        let old = store::load(&dir, "adopt-otel").expect("load");
        assert_eq!(old.meta.status, Status::Superseded);
        assert_eq!(old.meta.superseded_by.as_deref(), Some("adopt-tempo"));

        let new = store::load(&dir, "adopt-tempo").expect("load");
        assert_eq!(
            new.meta.supersedes.as_deref(),
            Some("adopt-otel"),
            "the replacement must point back"
        );
    }

    #[test]
    fn superseding_by_a_record_that_does_not_exist_is_refused() {
        let (_tmp, dir) = dirs();
        make(&dir, "adopt-otel");

        let args = UpdateArgs {
            superseded_by: Some("ghost"),
            ..Default::default()
        };
        let err = update(&dir, "adopt-otel", "2026-09-15T10:00:00Z", &args)
            .unwrap_err()
            .to_string();
        assert!(err.contains("create the replacement"), "{err}");
        assert_eq!(
            store::load(&dir, "adopt-otel").expect("load").meta.status,
            Status::Proposed
        );
    }

    #[test]
    fn list_is_empty_on_a_fresh_log() {
        let (_tmp, dir) = dirs();
        assert_eq!(
            list(&dir, None, None, false).expect("list"),
            "No ADRs found.\n"
        );
    }

    #[test]
    fn list_filters_by_status_and_type() {
        let (_tmp, dir) = dirs();
        make(&dir, "a");
        let mut b = Record::new("b", "B", BranchType::Docs, "T", "2026-09-15", None);
        b.meta.status = Status::Accepted;
        store::save(&dir, &b).expect("save");

        let all = list(&dir, None, None, false).expect("list");
        assert!(all.contains("2 ADR(s)"), "{all}");

        let accepted = list(&dir, Some("accepted"), None, false).expect("list");
        assert!(
            accepted.contains("1 ADR(s)") && accepted.contains(" b "),
            "{accepted}"
        );

        let docs = list(&dir, None, Some("docs"), false).expect("list");
        assert!(docs.contains("1 ADR(s)"), "{docs}");

        let none = list(&dir, Some("deprecated"), None, false).expect("list");
        assert_eq!(none, "No ADRs found.\n");
    }

    #[test]
    fn list_json_carries_every_field() {
        let (_tmp, dir) = dirs();
        make(&dir, "adopt-otel");

        let text = list(&dir, None, None, true).expect("list");
        let rows: serde_json::Value = serde_json::from_str(&text).expect("valid json");
        assert_eq!(rows[0]["slug"], "adopt-otel");
        assert_eq!(rows[0]["status"], "proposed");
        assert_eq!(rows[0]["branch_type"], "feat");
    }

    #[test]
    fn list_rejects_an_unknown_filter_value() {
        let (_tmp, dir) = dirs();
        assert!(list(&dir, Some("nonsense"), None, false).is_err());
        assert!(list(&dir, None, Some("nonsense"), false).is_err());
    }

    #[test]
    fn status_json_matches_the_shape_ci_gates_read() {
        let (tmp, dir) = dirs();
        make(&dir, "a");
        make(&dir, "b");

        let text = status(&dir, tmp.path(), true).expect("status");
        let value: serde_json::Value = serde_json::from_str(&text).expect("valid json");
        assert_eq!(value["total"], 2);
        assert_eq!(value["by_status"]["proposed"], 2);
        assert_eq!(value["by_type"]["feat"], 2);
    }

    #[test]
    fn status_human_output_lists_the_buckets() {
        let (tmp, dir) = dirs();
        make(&dir, "a");

        let out = status(&dir, tmp.path(), false).expect("status");
        assert!(out.contains("Total:   1"), "{out}");
        assert!(out.contains("By status:"), "{out}");
        assert!(out.contains("proposed"), "{out}");
    }

    #[test]
    fn index_writes_the_catalogue_and_counts_records() {
        let (_tmp, dir) = dirs();
        make(&dir, "a");

        let out = index_cmd(&dir, "2026-09-15").expect("index");
        assert!(out.contains("index.md"), "{out}");
        assert!(out.contains("1 record)"), "{out}");
        assert!(dir.join("index.md").is_file());
    }

    #[test]
    fn the_catalogue_is_not_mistaken_for_a_record() {
        let (_tmp, dir) = dirs();
        make(&dir, "a");
        index_cmd(&dir, "2026-09-15").expect("index");

        let out = list(&dir, None, None, false).expect("list");
        assert!(out.contains("1 ADR(s)"), "{out}");
    }

    #[test]
    fn sync_reports_when_no_harness_is_present() {
        let (tmp, _dir) = dirs();
        let out = sync_cmd(tmp.path()).expect("sync");
        assert!(out.contains("No agent configurations detected"), "{out}");
    }

    #[test]
    fn hook_commands_are_silent_on_an_empty_log() {
        let (tmp, dir) = dirs();
        store::ensure_dir(&dir).expect("dir");

        assert!(session_start(&dir, tmp.path()).is_empty());
        assert!(session_end(&dir).is_empty());
        assert!(post_tool_use(&dir, tmp.path()).is_empty());

        let (text, verdict) = pre_push(&dir);
        assert!(text.is_empty());
        assert!(verdict.is_ok());
    }

    #[test]
    fn pre_push_blocks_on_an_unfinished_proposed_record() {
        let (_tmp, dir) = dirs();
        make(&dir, "blank");

        let (text, verdict) = pre_push(&dir);
        assert!(text.contains("blank"), "{text}");
        assert!(verdict.is_err());
    }

    #[test]
    fn pre_push_never_blocks_on_a_broken_log() {
        let (_tmp, dir) = dirs();
        store::ensure_dir(&dir).expect("dir");
        std::fs::write(dir.join("broken.md"), "not a record\n").expect("write");

        let (_, verdict) = pre_push(&dir);
        assert!(verdict.is_ok(), "a malformed log must not stop a push");
    }

    #[test]
    fn draft_needs_a_record_to_draft() {
        let (tmp, dir) = dirs();
        store::ensure_dir(&dir).expect("dir");
        assert!(draft_cmd(&dir, tmp.path(), Some("ghost"), false, None, false, false).is_err());
    }

    #[test]
    fn draft_renders_the_human_and_bootstrap_views() {
        let (tmp, dir) = dirs();
        make(&dir, "adopt-otel");

        let human = draft_cmd(
            &dir,
            tmp.path(),
            Some("adopt-otel"),
            false,
            None,
            false,
            false,
        )
        .expect("draft");
        assert!(human.contains("## Branch Context"), "{human}");
        assert!(
            human.contains("## Suggested sections to address"),
            "{human}"
        );

        let boot = draft_cmd(
            &dir,
            tmp.path(),
            Some("adopt-otel"),
            true,
            None,
            false,
            false,
        )
        .expect("draft");
        assert!(
            boot.contains("## Bootstrap Questions for adopt-otel"),
            "{boot}"
        );

        let json = draft_cmd(
            &dir,
            tmp.path(),
            Some("adopt-otel"),
            false,
            None,
            false,
            true,
        )
        .expect("draft");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(value["slug"], "adopt-otel");
    }
}
