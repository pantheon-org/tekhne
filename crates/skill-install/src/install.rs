//! Installing a bundled skill into a detected agent directory.
//!
//! Ports the install action from `cli/lib/install`: resolve the target
//! directory for an agent (`resolveTargetDir`), then either symlink the source
//! skill into it (`createSymlink`, using relative link text) or copy it
//! recursively.

use std::path::{Component, Path, PathBuf};

use common::{Error, Result};

use crate::agents::AgentConfig;
use crate::env::Environment;

/// How a bundled skill is placed into a target directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallMode {
    /// Recursively copy the skill directory.
    Copy,
    /// Create a symlink pointing at the source skill directory.
    Symlink,
}

/// Resolve the directory an agent's skills install into.
///
/// Ports `resolveTargetDir`: when `global` is true, use the agent's global
/// skills directory (falling back to the project-relative directory under
/// `cwd`); otherwise use `cwd/<skillsDir>`.
pub fn resolve_target_dir(
    agent: &AgentConfig,
    global: bool,
    env: &Environment,
    exists: &dyn Fn(&Path) -> bool,
) -> PathBuf {
    if global {
        agent.global_skills_dir(env, exists)
    } else {
        env.cwd.join(agent.skills_dir)
    }
}

/// Install the skill at `source` into `target_dir` using `mode`.
///
/// The skill lands at `target_dir/<source-dir-name>`. Parent directories are
/// created as needed. Returns the installed path.
pub fn install_skill(source: &Path, target_dir: &Path, mode: InstallMode) -> Result<PathBuf> {
    let name = source.file_name().ok_or_else(|| {
        Error::Config(format!(
            "source has no directory name: {}",
            source.display()
        ))
    })?;
    let target = target_dir.join(name);
    std::fs::create_dir_all(target_dir).map_err(|source| Error::io(target_dir, source))?;
    match mode {
        InstallMode::Copy => copy_dir_all(source, &target)?,
        InstallMode::Symlink => symlink_skill(source, target_dir, &target)?,
    }
    Ok(target)
}

/// Remove a previously-installed skill named `skill_name` from `target_dir`.
///
/// Handles both placement modes used by [`install_skill`]: a copied directory
/// tree (removed recursively) and a symlink (only the link is removed, its
/// target is left untouched). Idempotent: returns `Ok(false)` when nothing was
/// present to remove, `Ok(true)` when a skill was removed.
pub fn uninstall_skill(target_dir: &Path, skill_name: &str) -> Result<bool> {
    let target = target_dir.join(skill_name);
    let meta = match std::fs::symlink_metadata(&target) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(e) => return Err(Error::io(&target, e)),
    };
    if meta.file_type().is_dir() {
        // A copied skill directory: remove the whole tree.
        std::fs::remove_dir_all(&target).map_err(|e| Error::io(&target, e))?;
    } else {
        // A symlink (even one pointing at a directory) or a stray file: unlink
        // it, mirroring the stale-symlink cleanup in the install command.
        std::fs::remove_file(&target).map_err(|e| Error::io(&target, e))?;
    }
    Ok(true)
}

/// Recursively copy `src` into `dst`, creating `dst` and any parents.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst).map_err(|e| Error::io(dst, e))?;
    let entries = std::fs::read_dir(src).map_err(|e| Error::io(src, e))?;
    for entry in entries {
        let entry = entry.map_err(|e| Error::io(src, e))?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let file_type = entry.file_type().map_err(|e| Error::io(&from, e))?;
        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            std::fs::copy(&from, &to).map_err(|e| Error::io(&from, e))?;
        }
    }
    Ok(())
}

/// Symlink `source` into `target`, using relative link text computed from the
/// real (canonicalized) target directory, matching `createSymlink`.
fn symlink_skill(source: &Path, target_dir: &Path, target: &Path) -> Result<()> {
    let resolved_source = source.canonicalize().map_err(|e| Error::io(source, e))?;
    let resolved_target_dir = target_dir
        .canonicalize()
        .unwrap_or_else(|_| target_dir.to_path_buf());
    let link_text = relative_path(&resolved_target_dir, &resolved_source);
    create_dir_symlink(&link_text, target)
}

/// Compute a relative path from directory `from` to `to`, both assumed
/// absolute and normalized (equivalent to Node's `path.relative`).
fn relative_path(from: &Path, to: &Path) -> PathBuf {
    let from: Vec<Component> = from.components().collect();
    let to: Vec<Component> = to.components().collect();
    let common = from
        .iter()
        .zip(to.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut result = PathBuf::new();
    for _ in common..from.len() {
        result.push("..");
    }
    for component in &to[common..] {
        result.push(component.as_os_str());
    }
    result
}

#[cfg(unix)]
fn create_dir_symlink(link_text: &Path, target: &Path) -> Result<()> {
    std::os::unix::fs::symlink(link_text, target).map_err(|e| Error::io(target, e))
}

#[cfg(windows)]
fn create_dir_symlink(link_text: &Path, target: &Path) -> Result<()> {
    std::os::windows::fs::symlink_dir(link_text, target).map_err(|e| Error::io(target, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::by_name;

    fn env_at(cwd: &Path) -> Environment {
        Environment::resolve(
            PathBuf::from("/home/user"),
            cwd.to_path_buf(),
            None,
            None,
            None,
        )
    }

    fn make_skill(root: &Path) -> PathBuf {
        let skill = root.join("my-skill");
        std::fs::create_dir_all(skill.join("references")).unwrap();
        std::fs::write(skill.join("SKILL.md"), "# skill").unwrap();
        std::fs::write(skill.join("references/ref.md"), "ref").unwrap();
        skill
    }

    #[test]
    fn resolve_target_dir_local_uses_cwd() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_at(tmp.path());
        let claude = by_name("claude-code").unwrap();
        let dir = resolve_target_dir(claude, false, &env, &|_| false);
        assert_eq!(dir, tmp.path().join(".claude/skills"));
    }

    #[test]
    fn resolve_target_dir_global_uses_global_dir() {
        let env = env_at(Path::new("/work"));
        let claude = by_name("claude-code").unwrap();
        let dir = resolve_target_dir(claude, true, &env, &|_| false);
        assert_eq!(dir, PathBuf::from("/home/user/.claude/skills"));
    }

    #[test]
    fn copy_mode_duplicates_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let skill = make_skill(&tmp.path().join("bundle"));
        let target_dir = tmp.path().join("dest/skills");

        let installed = install_skill(&skill, &target_dir, InstallMode::Copy).unwrap();

        assert_eq!(installed, target_dir.join("my-skill"));
        assert!(!installed
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(
            std::fs::read_to_string(installed.join("SKILL.md")).unwrap(),
            "# skill"
        );
        assert_eq!(
            std::fs::read_to_string(installed.join("references/ref.md")).unwrap(),
            "ref"
        );
    }

    #[test]
    fn symlink_mode_links_to_source() {
        let tmp = tempfile::tempdir().unwrap();
        let skill = make_skill(&tmp.path().join("bundle"));
        let target_dir = tmp.path().join("dest/skills");

        let installed = install_skill(&skill, &target_dir, InstallMode::Symlink).unwrap();

        assert!(installed
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink());
        // The link resolves back to the source skill's content.
        assert_eq!(
            std::fs::read_to_string(installed.join("SKILL.md")).unwrap(),
            "# skill"
        );
        assert_eq!(
            installed.canonicalize().unwrap(),
            skill.canonicalize().unwrap()
        );
    }

    #[test]
    fn uninstall_removes_copied_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let skill = make_skill(&tmp.path().join("bundle"));
        let target_dir = tmp.path().join("dest/skills");
        let installed = install_skill(&skill, &target_dir, InstallMode::Copy).unwrap();
        assert!(installed.exists());

        let removed = uninstall_skill(&target_dir, "my-skill").unwrap();
        assert!(removed, "a copied skill should report as removed");
        assert!(!installed.exists(), "the copied tree should be gone");
    }

    #[test]
    fn uninstall_removes_symlink_but_keeps_source() {
        let tmp = tempfile::tempdir().unwrap();
        let skill = make_skill(&tmp.path().join("bundle"));
        let target_dir = tmp.path().join("dest/skills");
        let installed = install_skill(&skill, &target_dir, InstallMode::Symlink).unwrap();
        assert!(installed
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink());

        let removed = uninstall_skill(&target_dir, "my-skill").unwrap();
        assert!(removed, "a symlinked skill should report as removed");
        assert!(
            installed.symlink_metadata().is_err(),
            "the symlink should be gone"
        );
        // The source skill the link pointed at is untouched.
        assert!(skill.join("SKILL.md").is_file());
    }

    #[test]
    fn uninstall_absent_is_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let target_dir = tmp.path().join("dest/skills");
        std::fs::create_dir_all(&target_dir).unwrap();
        let removed = uninstall_skill(&target_dir, "my-skill").unwrap();
        assert!(!removed, "removing an absent skill is a no-op");
    }

    #[test]
    fn relative_path_walks_up_and_down() {
        let rel = relative_path(Path::new("/a/b/dest"), Path::new("/a/b/src/skill"));
        assert_eq!(rel, PathBuf::from("../src/skill"));
    }
}
