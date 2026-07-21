//! Filesystem helpers with path-aware errors and skill discovery.

use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// Read a file to a string, attaching the path to any i/o error.
pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path).map_err(|source| Error::io(path, source))
}

/// The conventional entry-point filename for a skill.
pub const SKILL_FILE: &str = "SKILL.md";

/// Recursively find every skill directory under `root`.
///
/// A skill directory is any directory that directly contains a `SKILL.md`.
/// Hidden directories (those whose name begins with `.`) are skipped, and the
/// result is sorted for deterministic output. The search recurses into skill
/// directories too, so nested (toolkit) skills are discovered.
pub fn find_skill_dirs(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    collect_skill_dirs(root.as_ref(), &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_skill_dirs(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    let read = std::fs::read_dir(dir).map_err(|source| Error::io(dir, source))?;
    let mut has_skill = false;
    let mut subdirs = Vec::new();
    for entry in read {
        let entry = entry.map_err(|source| Error::io(dir, source))?;
        let path = entry.path();
        if path.is_dir() {
            let hidden = path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('.'));
            if !hidden {
                subdirs.push(path);
            }
        } else if path.file_name().and_then(|n| n.to_str()) == Some(SKILL_FILE) {
            has_skill = true;
        }
    }
    if has_skill {
        out.push(dir.to_path_buf());
    }
    for sub in subdirs {
        collect_skill_dirs(&sub, out)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_to_string_missing_file_reports_path() {
        let err = read_to_string("/definitely/missing/file.txt").unwrap_err();
        assert!(matches!(err, Error::Io { .. }));
        assert!(err.to_string().contains("/definitely/missing/file.txt"));
    }

    #[test]
    fn find_skill_dirs_discovers_flat_and_nested() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        // flat skill: <root>/a/SKILL.md
        std::fs::create_dir_all(root.join("a")).unwrap();
        std::fs::write(root.join("a/SKILL.md"), "x").unwrap();
        // nested (toolkit) skill: <root>/kit/sub/SKILL.md
        std::fs::create_dir_all(root.join("kit/sub")).unwrap();
        std::fs::write(root.join("kit/sub/SKILL.md"), "x").unwrap();
        // non-skill dir and a hidden dir are ignored
        std::fs::create_dir_all(root.join("a/references")).unwrap();
        std::fs::create_dir_all(root.join(".hidden")).unwrap();
        std::fs::write(root.join(".hidden/SKILL.md"), "x").unwrap();

        let found = find_skill_dirs(root).unwrap();
        assert_eq!(found, vec![root.join("a"), root.join("kit/sub")]);
    }
}
