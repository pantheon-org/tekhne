//! Audit a skill as it was at a git ref and report how its score moved.
//!
//! The whole skill folder (`SKILL.md`, `references/`, `evals/`, `assets/`) is
//! read with `git show` into a scratch directory, because D5 and D8 read more
//! than `SKILL.md`. The working tree is never touched. The approach follows
//! skilldiff's `src/baseline.ts` (MIT); see `NOTICE` beside this crate.

use crate::scorer::{self, Result as AuditResult};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Version of the comparison record printed by `--base ... --json`.
pub const SCHEMA_VERSION: u32 = 1;

/// A git ref resolved to the commit it names.
#[derive(Debug, Clone, Serialize)]
pub struct BaseRef {
    #[serde(rename = "ref")]
    pub name: String,
    pub sha: String,
}

impl BaseRef {
    fn short_sha(&self) -> &str {
        &self.sha[..self.sha.len().min(7)]
    }
}

/// Whether the current score moved against the ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The skill does not exist at the ref.
    New,
    /// Total or any dimension differs.
    Changed,
    /// Every dimension matches.
    Unchanged,
}

/// One skill scored now and at the ref.
#[derive(Debug, Clone, Serialize)]
pub struct Comparison {
    pub schema_version: u32,
    pub base: BaseRef,
    pub skill: String,
    pub status: Status,
    pub baseline: Option<AuditResult>,
    pub current: AuditResult,
    pub delta: Option<i32>,
    pub dimension_deltas: Option<BTreeMap<String, i32>>,
}

/// Reject a ref that git could read as an option, before it reaches git.
pub fn validate_ref(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("--base needs a git ref, but it was empty".to_string());
    }
    if name.starts_with('-') {
        return Err(format!(
            "git ref '{name}' is not allowed: a ref must not start with '-'"
        ));
    }
    Ok(())
}

/// Validate `name` and resolve it to a commit in the checkout at `repo_root`.
pub fn resolve(repo_root: &Path, name: &str) -> Result<BaseRef, String> {
    validate_ref(name)?;
    if !repo_root.join(".git").exists() {
        return Err(format!(
            "--repo-root {} is not a git checkout; --base needs a tekhne checkout, not an installed copy",
            repo_root.display()
        ));
    }
    let spec = format!("{name}^{{commit}}");
    let out = git(
        repo_root,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            "--end-of-options",
            &spec,
        ],
    )
    .map_err(|e| format!("cannot run git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git ref '{name}' not found in {}. A shallow clone must be deep enough to contain the ref (for example `git fetch --deepen=<n>`, or `fetch-depth: 0` in CI).",
            repo_root.display()
        ));
    }
    Ok(BaseRef {
        name: name.to_string(),
        sha: String::from_utf8_lossy(&out.stdout).trim().to_string(),
    })
}

/// Score the skill folder `skills/<skill_key>` as it was at `base`. `Ok(None)`
/// means the skill does not exist at the ref.
pub fn score_at_ref(
    repo_root: &Path,
    base: &BaseRef,
    skill_key: &str,
) -> Result<Option<AuditResult>, String> {
    let key_path = Path::new(skill_key);
    let escapes = key_path.is_absolute()
        || key_path
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)));
    if escapes {
        return Err(format!(
            "skill {skill_key} is not under {}/skills, so it has no version at a ref",
            repo_root.display()
        ));
    }

    let prefix = format!("skills/{skill_key}");
    let listing = git(
        repo_root,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            "--full-tree",
            &base.sha,
            "--",
            &format!("{prefix}/"),
        ],
    )
    .map_err(|e| format!("cannot run git: {e}"))?;
    if !listing.status.success() {
        return Err(format!(
            "git ls-tree failed at {}: {}",
            base.short_sha(),
            String::from_utf8_lossy(&listing.stderr).trim()
        ));
    }
    let skill_md = format!("{prefix}/SKILL.md");
    let files: Vec<String> = String::from_utf8_lossy(&listing.stdout)
        .split('\0')
        .filter(|p| !p.is_empty() && !p.contains("/.audits/"))
        .map(str::to_string)
        .collect();
    if !files.contains(&skill_md) {
        return Ok(None);
    }

    let scratch = Scratch::new().map_err(|e| format!("cannot create a scratch directory: {e}"))?;
    for file in &files {
        let blob = git(repo_root, &["show", &format!("{}:{file}", base.sha)])
            .map_err(|e| format!("cannot run git: {e}"))?;
        if !blob.status.success() {
            return Err(format!(
                "git show {}:{file} failed: {}",
                base.short_sha(),
                String::from_utf8_lossy(&blob.stderr).trim()
            ));
        }
        let dest = scratch.path().join(file);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("write {file}: {e}"))?;
        }
        std::fs::write(&dest, &blob.stdout).map_err(|e| format!("write {file}: {e}"))?;
    }

    let mut result = scorer::score(&scratch.path().join(&skill_md))
        .map_err(|e| format!("scoring {skill_key} at {} failed: {e}", base.name))?;
    result.skill = skill_key.to_string();
    Ok(Some(result))
}

/// Put the current and ref scores side by side.
pub fn compare(base: &BaseRef, current: AuditResult, baseline: Option<AuditResult>) -> Comparison {
    let (status, delta, dimension_deltas) = match &baseline {
        None => (Status::New, None, None),
        Some(old) => {
            let deltas: BTreeMap<String, i32> = current
                .dimensions
                .iter()
                .map(|(k, v)| (k.clone(), v - old.dimensions.get(k).copied().unwrap_or(0)))
                .collect();
            let moved = deltas.values().any(|d| *d != 0);
            let status = if moved {
                Status::Changed
            } else {
                Status::Unchanged
            };
            (status, Some(current.total - old.total), Some(deltas))
        }
    };
    Comparison {
        schema_version: SCHEMA_VERSION,
        base: base.clone(),
        skill: current.skill.clone(),
        status,
        baseline,
        current,
        delta,
        dimension_deltas,
    }
}

/// One-line header naming the ref, shared by the text outputs.
pub fn heading(base: &BaseRef) -> String {
    format!("Compared with {} ({})", base.name, base.short_sha())
}

/// The comparison as text: the headline, then each dimension that moved.
pub fn format_comparison(c: &Comparison) -> String {
    let head = heading(&c.base);
    let Some(old) = &c.baseline else {
        return format!("{head}: new skill, no baseline.\n");
    };
    let delta = c.delta.unwrap_or(0);
    if c.status == Status::Unchanged {
        return format!(
            "{head}: unchanged ({}/{}, {}).\n",
            c.current.total, c.current.max_total, c.current.grade
        );
    }
    let mut out = format!(
        "{head}: {} -> {} ({delta:+}), {} -> {}\n",
        old.total, c.current.total, old.grade, c.current.grade
    );
    for (dim, d) in c
        .dimension_deltas
        .iter()
        .flatten()
        .filter(|(_, d)| **d != 0)
    {
        let before = old.dimensions.get(dim).copied().unwrap_or(0);
        out.push_str(&format!("  {dim}: {before} -> {} ({d:+})\n", before + d));
    }
    out
}

fn git(repo_root: &Path, args: &[&str]) -> std::io::Result<std::process::Output> {
    Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
}

/// A scratch directory removed when dropped, so no run leaves a checkout copy
/// behind. Created with `create_dir`, which fails rather than reuse a name.
struct Scratch(PathBuf);

impl Scratch {
    fn new() -> std::io::Result<Self> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!(
            "pantheon-skill-auditor-base-{}-{nanos}",
            std::process::id()
        ));
        std::fs::create_dir(&dir)?;
        Ok(Self(dir))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dash_and_empty_refs_are_rejected() {
        assert!(validate_ref("-h").is_err());
        assert!(validate_ref("--output=x").is_err());
        assert!(validate_ref("").is_err());
        assert!(validate_ref("origin/main").is_ok());
        assert!(validate_ref("v1.2.3").is_ok());
    }

    #[test]
    fn a_skill_key_that_escapes_skills_is_rejected() {
        let base = BaseRef {
            name: "x".into(),
            sha: "0".repeat(40),
        };
        for key in ["../other", "/abs/path", "a/../b"] {
            assert!(score_at_ref(Path::new("."), &base, key).is_err(), "{key}");
        }
    }
}
