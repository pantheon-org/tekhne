//! Prune old audit snapshots under `.context/audits`.
//!
//! Ports `prune-audits.sh`, reconciled to the Rust reporter's on-disk layout.
//! The shell assumed `.context/audits/skill-audit/<date>/` with a `latest`
//! symlink; the Rust reporter (see `reporter::store`) writes
//! `.context/audits/<skill>/<date>/`, where `<skill>` is a `domain/skill-name`
//! path. This walks any depth and, for every directory that directly contains
//! date-named snapshot subdirectories, keeps the `keep` most recent and removes
//! the rest. ISO `YYYY-MM-DD` names sort lexicographically in date order, so
//! "most recent" is a reverse name sort.

use std::path::{Path, PathBuf};

/// Default number of snapshots to retain per skill (matches the shell).
pub const DEFAULT_KEEP: usize = 5;

/// A directory of dated snapshots and the prune decision for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrunePlan {
    /// The directory holding the dated snapshots (e.g. `.context/audits/d/s`).
    pub dir: PathBuf,
    /// Snapshot dates retained, most recent first.
    pub kept: Vec<String>,
    /// Snapshot dates marked for removal, most recent first.
    pub removed: Vec<String>,
}

/// True when `s` is an ISO `YYYY-MM-DD` date name.
fn is_date_name(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b[..4].iter().all(u8::is_ascii_digit)
        && b[5..7].iter().all(u8::is_ascii_digit)
        && b[8..].iter().all(u8::is_ascii_digit)
}

/// Immediate subdirectory names of `dir` (empty when unreadable).
fn subdirs(dir: &Path) -> Vec<(String, PathBuf)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<(String, PathBuf)> = entries
        .flatten()
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .collect();
    out.sort();
    out
}

/// Compute the prune plans under `audits_root` without deleting anything.
///
/// A directory that directly contains one or more date-named subdirectories is
/// a snapshot holder: its dated children are sorted most-recent-first, the
/// first `keep` retained and the remainder marked for removal. Non-date
/// subdirectories are recursed into (so nested `domain/skill/<date>` layouts are
/// found); date subdirectories are treated as leaf snapshots.
pub fn plan(audits_root: &Path, keep: usize) -> Vec<PrunePlan> {
    let mut plans = Vec::new();
    walk(audits_root, keep, &mut plans);
    plans
}

fn walk(dir: &Path, keep: usize, plans: &mut Vec<PrunePlan>) {
    let children = subdirs(dir);
    let mut dates: Vec<String> = Vec::new();
    for (name, path) in &children {
        if is_date_name(name) {
            dates.push(name.clone());
        } else {
            walk(path, keep, plans);
        }
    }
    if dates.is_empty() {
        return;
    }
    dates.sort_by(|a, b| b.cmp(a)); // most recent first
    let (kept, removed) = if dates.len() > keep {
        let removed = dates.split_off(keep);
        (dates, removed)
    } else {
        (dates, Vec::new())
    };
    plans.push(PrunePlan {
        dir: dir.to_path_buf(),
        kept,
        removed,
    });
}

/// Execute the prune under `audits_root`. When `dry_run` is true the plans are
/// computed but nothing is deleted. Returns the plans (including those with an
/// empty `removed` list, so callers can report what was kept).
pub fn prune(audits_root: &Path, keep: usize, dry_run: bool) -> std::io::Result<Vec<PrunePlan>> {
    let plans = plan(audits_root, keep);
    if !dry_run {
        for p in &plans {
            for date in &p.removed {
                std::fs::remove_dir_all(p.dir.join(date))?;
            }
        }
    }
    Ok(plans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn mk_snapshot(root: &Path, skill: &str, date: &str) {
        let dir = root.join(skill).join(date);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("audit.json"), "{}").unwrap();
    }

    #[test]
    fn is_date_name_matches_iso_only() {
        assert!(is_date_name("2026-07-22"));
        assert!(!is_date_name("2026-7-22"));
        assert!(!is_date_name("latest"));
        assert!(!is_date_name("2026-07-22x"));
    }

    #[test]
    fn keeps_most_recent_n() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        for d in ["2026-01-01", "2026-02-01", "2026-03-01", "2026-04-01"] {
            mk_snapshot(root, "domain/skill", d);
        }
        let plans = plan(root, 2);
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].kept, vec!["2026-04-01", "2026-03-01"]);
        assert_eq!(plans[0].removed, vec!["2026-02-01", "2026-01-01"]);
    }

    #[test]
    fn nothing_removed_when_under_limit() {
        let tmp = tempdir().unwrap();
        mk_snapshot(tmp.path(), "domain/skill", "2026-01-01");
        let plans = plan(tmp.path(), 5);
        assert_eq!(plans.len(), 1);
        assert!(plans[0].removed.is_empty());
        assert_eq!(plans[0].kept, vec!["2026-01-01"]);
    }

    #[test]
    fn prunes_multiple_skills_independently() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        for d in ["2026-01-01", "2026-02-01", "2026-03-01"] {
            mk_snapshot(root, "a/one", d);
        }
        mk_snapshot(root, "b/two", "2026-01-01");
        let plans = plan(root, 1);
        let by_dir: std::collections::HashMap<_, _> =
            plans.iter().map(|p| (p.dir.clone(), p)).collect();
        assert_eq!(by_dir[&root.join("a/one")].removed.len(), 2);
        assert_eq!(by_dir[&root.join("b/two")].removed.len(), 0);
    }

    #[test]
    fn dry_run_removes_nothing() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        for d in ["2026-01-01", "2026-02-01", "2026-03-01"] {
            mk_snapshot(root, "domain/skill", d);
        }
        prune(root, 1, true).unwrap();
        assert!(root.join("domain/skill/2026-01-01").exists());
    }

    #[test]
    fn prune_removes_old_snapshots() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        for d in ["2026-01-01", "2026-02-01", "2026-03-01"] {
            mk_snapshot(root, "domain/skill", d);
        }
        prune(root, 1, false).unwrap();
        assert!(!root.join("domain/skill/2026-01-01").exists());
        assert!(!root.join("domain/skill/2026-02-01").exists());
        assert!(root.join("domain/skill/2026-03-01").exists());
    }
}
