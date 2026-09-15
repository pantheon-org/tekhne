//! Reading branch context from git.
//!
//! `create` uses the current branch to derive a record's slug and type, and
//! `draft` uses the commits and diff against a base branch to tell an agent
//! what the decision actually changed. Every call shells out to `git` and
//! treats a non-zero exit as "no information", never as a fatal error: an ADR
//! must still be writable outside a repository, or in a fresh clone with no
//! base branch yet.

use std::path::Path;
use std::process::Command;

/// One commit on the branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// The commit subject line.
    pub subject: String,
    /// The author date as `YYYY-MM-DD`.
    pub date: String,
}

/// A summary of what the branch changed against its base.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diffstat {
    /// Number of files touched.
    pub files: usize,
    /// Lines added.
    pub insertions: usize,
    /// Lines removed.
    pub deletions: usize,
    /// Per-directory file counts and insertions, richest first.
    pub by_dir: Vec<(String, usize, usize)>,
}

/// Everything `draft` knows about the current branch.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BranchContext {
    /// The current branch name, when on one.
    pub branch: Option<String>,
    /// The base branch the diff was taken against.
    pub base: String,
    /// Commits on the branch but not on the base, newest first.
    pub commits: Vec<Commit>,
    /// The aggregate diff against the base.
    pub diffstat: Diffstat,
}

/// Run `git` with `args` in `dir`, returning stdout on a clean exit.
fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// The current branch name, or `None` on a detached HEAD or outside a repository.
pub fn current_branch(dir: &Path) -> Option<String> {
    git(dir, &["rev-parse", "--abbrev-ref", "HEAD"]).filter(|b| !b.is_empty() && b != "HEAD")
}

/// Whether `dir` is inside a git work tree.
pub fn in_repo(dir: &Path) -> bool {
    git(dir, &["rev-parse", "--is-inside-work-tree"]).as_deref() == Some("true")
}

/// The repository root containing `dir`.
pub fn repo_root(dir: &Path) -> Option<String> {
    git(dir, &["rev-parse", "--show-toplevel"])
}

/// The repository name, derived from its root directory.
pub fn repo_name(dir: &Path) -> Option<String> {
    repo_root(dir).and_then(|root| {
        Path::new(&root)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
    })
}

/// The configured `user.name`, for the record's author field.
pub fn user_name(dir: &Path) -> Option<String> {
    git(dir, &["config", "user.name"]).filter(|n| !n.is_empty())
}

/// The first of `candidates` that exists as a branch, falling back to the first.
///
/// A repository whose trunk is `master`, or whose base branch has not been
/// fetched, still produces a usable base rather than an error.
pub fn resolve_base(dir: &Path, explicit: Option<&str>, candidates: &[&str]) -> String {
    if let Some(base) = explicit.filter(|b| !b.is_empty()) {
        return base.to_string();
    }
    for candidate in candidates {
        if git(dir, &["rev-parse", "--verify", candidate]).is_some() {
            return (*candidate).to_string();
        }
    }
    candidates.first().unwrap_or(&"main").to_string()
}

/// Commits on the current branch that are not on `base`, newest first.
pub fn commits_since(dir: &Path, base: &str) -> Vec<Commit> {
    let range = format!("{base}..HEAD");
    let Some(out) = git(dir, &["log", "--format=%s%x1f%ad", "--date=short", &range]) else {
        return Vec::new();
    };
    out.lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let (subject, date) = line.split_once('\u{1f}')?;
            Some(Commit {
                subject: subject.to_string(),
                date: date.to_string(),
            })
        })
        .collect()
}

/// The diff of the current branch against `base`, aggregated per directory.
pub fn diffstat(dir: &Path, base: &str) -> Diffstat {
    let range = format!("{base}...HEAD");
    let Some(out) = git(dir, &["diff", "--numstat", &range]) else {
        return Diffstat::default();
    };

    let mut stat = Diffstat::default();
    let mut dirs: Vec<(String, usize, usize)> = Vec::new();
    for line in out.lines().filter(|l| !l.is_empty()) {
        let mut parts = line.split('\t');
        // Binary files report "-" for both counts; treat them as zero lines.
        let added: usize = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        let removed: usize = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        let Some(path) = parts.next() else { continue };

        stat.files += 1;
        stat.insertions += added;
        stat.deletions += removed;

        let dir_label = Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .filter(|p| !p.is_empty())
            .unwrap_or_else(|| ".".to_string());
        match dirs.iter_mut().find(|(d, _, _)| *d == dir_label) {
            Some(entry) => {
                entry.1 += 1;
                entry.2 += added;
            }
            None => dirs.push((dir_label, 1, added)),
        }
    }
    dirs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    stat.by_dir = dirs;
    stat
}

/// The paths the current branch changed against `base`.
pub fn changed_files(dir: &Path, base: &str) -> Vec<String> {
    let range = format!("{base}...HEAD");
    git(dir, &["diff", "--name-only", &range])
        .map(|out| {
            out.lines()
                .filter(|l| !l.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// Gather everything `draft` needs in one pass.
pub fn context(dir: &Path, explicit_base: Option<&str>, default_base: &str) -> BranchContext {
    let base = resolve_base(dir, explicit_base, &[default_base, "main", "master"]);
    BranchContext {
        branch: current_branch(dir),
        commits: commits_since(dir, &base),
        diffstat: diffstat(dir, &base),
        base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// A repository with one commit on `main`, or `None` when git is absent.
    fn repo() -> Option<(TempDir, PathBuf)> {
        let tmp = TempDir::new().ok()?;
        let path = tmp.path().to_path_buf();
        git(&path, &["init", "-q", "-b", "main", "."])?;
        git(&path, &["config", "user.name", "Test User"])?;
        git(&path, &["config", "user.email", "test@example.com"])?;
        fs::write(path.join("README.md"), "hello\n").ok()?;
        git(&path, &["add", "-A"])?;
        git(&path, &["commit", "-qm", "chore: init"])?;
        Some((tmp, path))
    }

    #[test]
    fn outside_a_repository_everything_degrades_quietly() {
        let tmp = TempDir::new().unwrap();
        assert!(!in_repo(tmp.path()));
        assert_eq!(current_branch(tmp.path()), None);
        assert!(commits_since(tmp.path(), "main").is_empty());
        assert_eq!(diffstat(tmp.path(), "main"), Diffstat::default());
    }

    #[test]
    fn current_branch_reads_the_checkout() {
        let Some((_tmp, path)) = repo() else { return };
        assert!(in_repo(&path));
        assert_eq!(current_branch(&path).as_deref(), Some("main"));

        git(&path, &["checkout", "-q", "-b", "feat/adopt-otel"]).unwrap();
        assert_eq!(current_branch(&path).as_deref(), Some("feat/adopt-otel"));
    }

    #[test]
    fn repo_name_and_user_come_from_git() {
        let Some((_tmp, path)) = repo() else { return };
        assert!(repo_name(&path).is_some());
        assert_eq!(user_name(&path).as_deref(), Some("Test User"));
    }

    #[test]
    fn resolve_base_prefers_the_explicit_value() {
        let Some((_tmp, path)) = repo() else { return };
        assert_eq!(resolve_base(&path, Some("develop"), &["main"]), "develop");
        assert_eq!(resolve_base(&path, Some(""), &["main"]), "main");
    }

    #[test]
    fn resolve_base_skips_a_branch_that_does_not_exist() {
        let Some((_tmp, path)) = repo() else { return };
        assert_eq!(resolve_base(&path, None, &["trunk", "main"]), "main");
    }

    #[test]
    fn commits_and_diffstat_describe_the_branch() {
        let Some((_tmp, path)) = repo() else { return };
        git(&path, &["checkout", "-q", "-b", "feat/x"]).unwrap();
        fs::create_dir_all(path.join("src")).unwrap();
        fs::write(path.join("src/a.rs"), "fn a() {}\nfn b() {}\n").unwrap();
        git(&path, &["add", "-A"]).unwrap();
        git(&path, &["commit", "-qm", "feat: add a"]).unwrap();

        let commits = commits_since(&path, "main");
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].subject, "feat: add a");
        assert_eq!(commits[0].date.len(), 10);

        let stat = diffstat(&path, "main");
        assert_eq!(stat.files, 1);
        assert_eq!(stat.insertions, 2);
        assert_eq!(stat.deletions, 0);
        assert_eq!(stat.by_dir, vec![("src".to_string(), 1, 2)]);
    }

    #[test]
    fn changed_files_lists_the_paths_the_branch_touched() {
        let Some((_tmp, path)) = repo() else { return };
        git(&path, &["checkout", "-q", "-b", "feat/z"]).unwrap();
        fs::create_dir_all(path.join("src")).unwrap();
        fs::write(path.join("src/a.rs"), "fn a() {}\n").unwrap();
        git(&path, &["add", "-A"]).unwrap();
        git(&path, &["commit", "-qm", "feat: add a"]).unwrap();

        assert_eq!(changed_files(&path, "main"), vec!["src/a.rs".to_string()]);
    }

    #[test]
    fn changed_files_is_empty_outside_a_repository() {
        let tmp = TempDir::new().unwrap();
        assert!(changed_files(tmp.path(), "main").is_empty());
    }

    #[test]
    fn context_gathers_branch_base_and_changes_together() {
        let Some((_tmp, path)) = repo() else { return };
        git(&path, &["checkout", "-q", "-b", "fix/y"]).unwrap();
        fs::write(path.join("README.md"), "hello\nworld\n").unwrap();
        git(&path, &["add", "-A"]).unwrap();
        git(&path, &["commit", "-qm", "fix: line"]).unwrap();

        let ctx = context(&path, None, "main");
        assert_eq!(ctx.branch.as_deref(), Some("fix/y"));
        assert_eq!(ctx.base, "main");
        assert_eq!(ctx.commits.len(), 1);
        assert_eq!(ctx.diffstat.files, 1);
    }
}
