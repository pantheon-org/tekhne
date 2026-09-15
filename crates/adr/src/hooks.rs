//! The agent-harness and git hook commands.
//!
//! These four commands are wired by [`crate::sync`] and by `init
//! --install-hooks`, and are called by machinery rather than by people. Three of
//! them only ever print: a hook that fails a session because an ADR is untidy
//! is a hook that gets uninstalled. `pre-push` is the single exception, because
//! blocking is the whole point of it.
//!
//! Marker files for the once-per-branch nudge live under `.git/`, never in the
//! ADR directory, so a committed `docs/adr` does not fill up with bookkeeping.

use std::fs;
use std::path::{Path, PathBuf};

use common::{Error, Result};

use crate::completeness::{self, PASS_THRESHOLD};
use crate::record::{Record, Status};

/// The pre-push hook script, which delegates every decision to the binary.
const PRE_PUSH_SCRIPT: &str = "#!/bin/sh\n\
    # Installed by `pantheon-adr init --install-hooks`. Remove with\n\
    # `pantheon-adr init --uninstall-hooks`.\n\
    exec pantheon-adr pre-push\n";

/// The marker that identifies our hook, so we never delete someone else's.
const HOOK_MARKER: &str = "pantheon-adr pre-push";

/// Where the once-per-branch nudge records that it has already fired.
const NOTIFY_DIR: &str = "adr-notify";

/// Records with a given status, sorted by slug.
fn with_status(records: &[Record], status: Status) -> Vec<&Record> {
    let mut out: Vec<&Record> = records.iter().filter(|r| r.meta.status == status).collect();
    out.sort_by(|a, b| a.slug.cmp(&b.slug));
    out
}

/// One line describing a record, with its description when it has one.
fn line_for(r: &Record) -> String {
    if r.meta.title.is_empty() {
        format!("  - {}\n", r.slug)
    } else {
        format!("  - {}: {}\n", r.slug, r.meta.title)
    }
}

/// Context injected at the start of an agent session.
///
/// Returns `None` when there is nothing worth saying, so the hook adds no noise
/// to a repository that has no decisions yet.
pub fn session_start(records: &[Record], project: &str) -> Option<String> {
    if records.is_empty() {
        return None;
    }
    let counts: Vec<String> = Status::ALL
        .iter()
        .filter_map(|s| {
            let n = records.iter().filter(|r| r.meta.status == *s).count();
            (n > 0).then(|| format!("{n} {s}"))
        })
        .collect();

    let mut out = format!("## ADRs ({project}): {}\n", counts.join(", "));

    let awaiting = with_status(records, Status::ReviewRequested);
    if !awaiting.is_empty() {
        out.push_str("\nAwaiting human review (no agent action needed):\n");
        for r in awaiting {
            out.push_str(&line_for(r));
        }
    }
    Some(out)
}

/// A nudge when the current branch has no ADR, fired at most once per branch.
///
/// Returns `None` on the trunk, when the branch already has a record, or when
/// the nudge has already fired, so an agent is not told the same thing twice.
pub fn post_tool_use(
    git_dir: &Path,
    records: &[Record],
    branch: Option<&str>,
    slug: &str,
) -> Option<String> {
    let branch = branch?;
    if branch == "main" || branch == "master" {
        return None;
    }
    if records.iter().any(|r| r.slug == slug) {
        return None;
    }

    let marker = notify_path(git_dir, branch);
    if marker.exists() {
        return None;
    }
    // Best effort: if the marker cannot be written the nudge repeats, which is
    // noisier but never wrong, so there is nothing to report here.
    if let Some(parent) = marker.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&marker, "");

    Some(format!(
        "Branch {branch:?} has no ADR. Run `pantheon-adr create -d \"<why>\"` to document this decision.\n"
    ))
}

/// The marker file path for `branch`, with separators flattened.
fn notify_path(git_dir: &Path, branch: &str) -> PathBuf {
    let safe: String = branch
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    git_dir.join(NOTIFY_DIR).join(format!("{safe}.seen"))
}

/// A reminder at session end about records that are not finished.
///
/// Returns `None` when every record is settled.
pub fn session_end(records: &[Record]) -> Option<String> {
    let mut out = String::new();

    let awaiting = with_status(records, Status::ReviewRequested);
    if !awaiting.is_empty() {
        out.push_str("ADRs awaiting human review:\n");
        for r in awaiting {
            out.push_str(&line_for(r));
        }
    }

    let proposed = with_status(records, Status::Proposed);
    if !proposed.is_empty() {
        out.push_str("Proposed ADRs not yet accepted:\n");
        for r in proposed {
            let report = completeness::check(&r.body, PASS_THRESHOLD);
            out.push_str(&format!(
                "  - {} [score {}/100]{}\n",
                r.slug,
                report.score,
                if report.pass {
                    ", ready for `pantheon-adr review`"
                } else {
                    ""
                }
            ));
        }
    }
    (!out.is_empty()).then_some(out)
}

/// The pre-push gate: every proposed record must score at or above the threshold.
///
/// Returns the report to print, and an error when any record falls short, which
/// is what makes the push fail.
pub fn pre_push(records: &[Record]) -> (String, Result<()>) {
    let proposed = with_status(records, Status::Proposed);
    if proposed.is_empty() {
        return (String::new(), Ok(()));
    }

    let mut out = String::new();
    let mut failed = 0;
    for r in proposed {
        let report = completeness::check(&r.body, PASS_THRESHOLD);
        if report.pass {
            continue;
        }
        failed += 1;
        out.push_str(&format!("  x {}  [score {}/100]\n", r.slug, report.score));
        for item in &report.missing {
            out.push_str(&format!("      x {}\n", item.name));
        }
    }

    let verdict = if failed > 0 {
        Err(Error::Config(format!(
            "{failed} proposed ADR(s) are incomplete; run `pantheon-adr check <slug>` for details"
        )))
    } else {
        Ok(())
    };
    (out, verdict)
}

/// Install the pre-push hook into `git_dir`, refusing to clobber a foreign one.
pub fn install(git_dir: &Path) -> Result<PathBuf> {
    let hooks = git_dir.join("hooks");
    fs::create_dir_all(&hooks).map_err(|e| Error::io(&hooks, e))?;
    let path = hooks.join("pre-push");

    if let Ok(existing) = fs::read_to_string(&path) {
        if !existing.contains(HOOK_MARKER) {
            return Err(Error::Config(format!(
                "{} already exists and was not installed by pantheon-adr; \
                 add `pantheon-adr pre-push` to it by hand instead",
                path.display()
            )));
        }
    }
    fs::write(&path, PRE_PUSH_SCRIPT).map_err(|e| Error::io(&path, e))?;
    make_executable(&path)?;
    Ok(path)
}

/// Remove the pre-push hook, leaving a hook we did not write alone.
pub fn uninstall(git_dir: &Path) -> Result<Option<PathBuf>> {
    let path = git_dir.join("hooks").join("pre-push");
    let Ok(existing) = fs::read_to_string(&path) else {
        return Ok(None);
    };
    if !existing.contains(HOOK_MARKER) {
        return Err(Error::Config(format!(
            "{} was not installed by pantheon-adr; leaving it alone",
            path.display()
        )));
    }
    fs::remove_file(&path).map_err(|e| Error::io(&path, e))?;
    Ok(Some(path))
}

/// Mark `path` executable on Unix; a no-op elsewhere.
#[cfg(unix)]
fn make_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)
        .map_err(|e| Error::io(path, e))?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).map_err(|e| Error::io(path, e))
}

/// Mark `path` executable on Unix; a no-op elsewhere.
#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{BranchType, Record};
    use tempfile::TempDir;

    const PROSE: &str = "Requests cross four services and we cannot follow a single one end to \
        end because each service writes its own log format with no shared correlation identifier, \
        so reconstructing a slow request means grepping four log stores and guessing at ordering.";

    fn record(slug: &str, status: Status) -> Record {
        let mut r = Record::new(
            slug,
            slug,
            BranchType::Feat,
            "Test User",
            "2026-09-15",
            None,
        );
        r.meta.status = status;
        r
    }

    fn complete(slug: &str, status: Status) -> Record {
        let mut r = record(slug, status);
        r.body = format!(
            "## Problem Statement\n### Context\n{PROSE}\n\n### Chosen Solution\n{PROSE}\n\n\
             ### Rationale\n{PROSE}\n\n## Impact Assessment\n- **Security**: reviewed\n"
        );
        r
    }

    #[test]
    fn session_start_is_silent_with_no_records() {
        assert_eq!(session_start(&[], "tekhne"), None);
    }

    #[test]
    fn session_start_counts_by_status_and_names_the_project() {
        let out = session_start(
            &[record("a", Status::Proposed), record("b", Status::Accepted)],
            "tekhne",
        )
        .unwrap();
        assert!(out.contains("## ADRs (tekhne)"), "{out}");
        assert!(out.contains("1 proposed"), "{out}");
        assert!(out.contains("1 accepted"), "{out}");
    }

    #[test]
    fn session_start_surfaces_records_awaiting_review() {
        let out = session_start(&[record("needs-eyes", Status::ReviewRequested)], "p").unwrap();
        assert!(out.contains("Awaiting human review"), "{out}");
        assert!(out.contains("needs-eyes"), "{out}");
    }

    #[test]
    fn post_tool_use_is_silent_on_the_trunk() {
        let tmp = TempDir::new().unwrap();
        assert_eq!(post_tool_use(tmp.path(), &[], Some("main"), "main"), None);
        assert_eq!(
            post_tool_use(tmp.path(), &[], Some("master"), "master"),
            None
        );
    }

    #[test]
    fn post_tool_use_is_silent_with_no_branch() {
        let tmp = TempDir::new().unwrap();
        assert_eq!(post_tool_use(tmp.path(), &[], None, ""), None);
    }

    #[test]
    fn post_tool_use_is_silent_when_the_branch_has_a_record() {
        let tmp = TempDir::new().unwrap();
        let records = [record("adopt-otel", Status::Proposed)];
        assert_eq!(
            post_tool_use(tmp.path(), &records, Some("feat/adopt-otel"), "adopt-otel"),
            None
        );
    }

    #[test]
    fn post_tool_use_nudges_once_per_branch() {
        let tmp = TempDir::new().unwrap();
        let first = post_tool_use(tmp.path(), &[], Some("feat/adopt-otel"), "adopt-otel");
        assert!(first.is_some_and(|m| m.contains("has no ADR")));

        let second = post_tool_use(tmp.path(), &[], Some("feat/adopt-otel"), "adopt-otel");
        assert_eq!(second, None, "the nudge must not repeat for one branch");
    }

    #[test]
    fn the_nudge_marker_lives_under_the_git_dir() {
        let tmp = TempDir::new().unwrap();
        post_tool_use(tmp.path(), &[], Some("feat/a/b"), "a-b");
        assert!(tmp.path().join(NOTIFY_DIR).join("feat-a-b.seen").is_file());
    }

    #[test]
    fn session_end_is_silent_when_everything_is_settled() {
        assert_eq!(session_end(&[record("a", Status::Accepted)]), None);
    }

    #[test]
    fn session_end_reports_scores_for_proposed_records() {
        let out = session_end(&[complete("ready", Status::Proposed)]).unwrap();
        assert!(out.contains("Proposed ADRs not yet accepted"), "{out}");
        assert!(out.contains("[score 100/100]"), "{out}");
        assert!(out.contains("ready for `pantheon-adr review`"), "{out}");
    }

    #[test]
    fn session_end_omits_the_review_hint_for_an_unfinished_record() {
        let out = session_end(&[record("blank", Status::Proposed)]).unwrap();
        assert!(out.contains("[score 0/100]"), "{out}");
        assert!(!out.contains("ready for"), "{out}");
    }

    #[test]
    fn pre_push_passes_with_nothing_proposed() {
        let (out, verdict) = pre_push(&[record("a", Status::Accepted)]);
        assert!(out.is_empty());
        assert!(verdict.is_ok());
    }

    #[test]
    fn pre_push_passes_a_complete_proposed_record() {
        let (out, verdict) = pre_push(&[complete("ready", Status::Proposed)]);
        assert!(out.is_empty(), "{out}");
        assert!(verdict.is_ok());
    }

    #[test]
    fn pre_push_blocks_an_incomplete_record_and_names_it() {
        let (out, verdict) = pre_push(&[record("blank", Status::Proposed)]);
        assert!(out.contains("blank"), "{out}");
        assert!(out.contains("score 0/100"), "{out}");

        let err = verdict.unwrap_err().to_string();
        assert!(err.contains("1 proposed ADR(s) are incomplete"), "{err}");
    }

    #[test]
    fn install_writes_an_executable_hook() {
        let tmp = TempDir::new().unwrap();
        let path = install(tmp.path()).unwrap();

        assert_eq!(path, tmp.path().join("hooks").join("pre-push"));
        let script = fs::read_to_string(&path).unwrap();
        assert!(script.contains("pantheon-adr pre-push"), "{script}");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o111, 0o111, "hook must be executable");
        }
    }

    #[test]
    fn install_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        install(tmp.path()).unwrap();
        let path = install(tmp.path()).unwrap();
        assert_eq!(fs::read_to_string(path).unwrap(), PRE_PUSH_SCRIPT);
    }

    #[test]
    fn install_refuses_to_clobber_a_foreign_hook() {
        let tmp = TempDir::new().unwrap();
        let hooks = tmp.path().join("hooks");
        fs::create_dir_all(&hooks).unwrap();
        fs::write(
            hooks.join("pre-push"),
            "#!/bin/sh\necho someone elses hook\n",
        )
        .unwrap();

        let err = install(tmp.path()).unwrap_err().to_string();
        assert!(err.contains("not installed by pantheon-adr"), "{err}");
        assert!(
            fs::read_to_string(hooks.join("pre-push"))
                .unwrap()
                .contains("someone elses hook"),
            "the existing hook must survive"
        );
    }

    #[test]
    fn uninstall_removes_our_hook_and_reports_nothing_when_absent() {
        let tmp = TempDir::new().unwrap();
        assert_eq!(uninstall(tmp.path()).unwrap(), None);

        install(tmp.path()).unwrap();
        assert!(uninstall(tmp.path()).unwrap().is_some());
        assert!(!tmp.path().join("hooks").join("pre-push").exists());
    }

    #[test]
    fn uninstall_leaves_a_foreign_hook_alone() {
        let tmp = TempDir::new().unwrap();
        let hooks = tmp.path().join("hooks");
        fs::create_dir_all(&hooks).unwrap();
        fs::write(hooks.join("pre-push"), "#!/bin/sh\nnot ours\n").unwrap();

        assert!(uninstall(tmp.path()).is_err());
        assert!(hooks.join("pre-push").is_file());
    }
}
