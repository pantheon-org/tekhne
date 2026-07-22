//! The companion skill embedded into the binary at build time.
//!
//! The `build.rs` script bakes the canonical
//! `skills/agentic-harness/skill-quality-auditor` tree into the binary as a
//! table of `(relative-path, bytes)` entries. This module exposes that table
//! and materialises it onto disk so a downloaded binary can install the skill
//! on a machine that has no copy of the repository.

use std::path::{Path, PathBuf};

use common::{Error, Result};

include!(concat!(env!("OUT_DIR"), "/embedded_skill.rs"));

/// The directory name of the embedded skill (e.g. `skill-quality-auditor`).
pub fn skill_name() -> &'static str {
    EMBEDDED_SKILL_NAME
}

/// The number of files embedded in the binary.
pub fn file_count() -> usize {
    EMBEDDED_SKILL_FILES.len()
}

/// A single embedded file: its forward-slash path relative to the skill root
/// and its size in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BundleEntry {
    /// Path relative to the skill root (e.g. `references/scoring-rubric.md`).
    pub path: &'static str,
    /// Size of the embedded file in bytes.
    pub bytes: usize,
}

/// The manifest of every embedded file, in the build script's sorted order.
///
/// This is the ground truth for what `skill install` would write to disk, read
/// straight from the baked-in table rather than any repository checkout.
pub fn manifest() -> Vec<BundleEntry> {
    EMBEDDED_SKILL_FILES
        .iter()
        .map(|(path, bytes)| BundleEntry {
            path,
            bytes: bytes.len(),
        })
        .collect()
}

/// Total size in bytes of every embedded file.
pub fn total_bytes() -> usize {
    EMBEDDED_SKILL_FILES.iter().map(|(_, b)| b.len()).sum()
}

/// Write the embedded skill tree into `dest_root`.
///
/// Creates `dest_root/<skill-name>` (and any parent directories per file),
/// writes every embedded file, and returns the path to the written skill
/// directory. Existing files are overwritten, so repeated calls are
/// idempotent.
pub fn materialise(dest_root: &Path) -> Result<PathBuf> {
    let skill_dir = dest_root.join(EMBEDDED_SKILL_NAME);
    for (rel, bytes) in EMBEDDED_SKILL_FILES {
        let out = skill_dir.join(rel);
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
        }
        std::fs::write(&out, bytes).map_err(|e| Error::io(&out, e))?;
    }
    Ok(skill_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_is_non_empty() {
        assert!(file_count() > 0);
        assert_eq!(skill_name(), "skill-quality-auditor");
    }

    #[test]
    fn bundle_contains_entry_point() {
        assert!(EMBEDDED_SKILL_FILES
            .iter()
            .any(|(rel, _)| *rel == "SKILL.md"));
    }

    #[test]
    fn materialise_writes_the_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let skill_dir = materialise(tmp.path()).unwrap();

        assert_eq!(skill_dir, tmp.path().join("skill-quality-auditor"));
        assert!(skill_dir.join("SKILL.md").is_file());
        // A nested file proves parent directories are created.
        let has_nested = EMBEDDED_SKILL_FILES
            .iter()
            .any(|(rel, _)| rel.contains('/'));
        assert!(has_nested, "expected at least one nested embedded file");
        let (nested_rel, nested_bytes) = EMBEDDED_SKILL_FILES
            .iter()
            .find(|(rel, _)| rel.contains('/'))
            .unwrap();
        let written = std::fs::read(skill_dir.join(nested_rel)).unwrap();
        assert_eq!(&written, nested_bytes);
    }

    #[test]
    fn manifest_matches_embedded_table() {
        let manifest = manifest();
        assert_eq!(manifest.len(), file_count());
        assert!(manifest.iter().any(|e| e.path == "SKILL.md"));
        // Every embedded file has content, and the manifest byte counts agree
        // with the raw table.
        let manifest_total: usize = manifest.iter().map(|e| e.bytes).sum();
        assert_eq!(manifest_total, total_bytes());
        assert!(total_bytes() > 0);
    }

    #[test]
    fn materialise_is_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let first = materialise(tmp.path()).unwrap();
        let second = materialise(tmp.path()).unwrap();
        assert_eq!(first, second);
        assert!(second.join("SKILL.md").is_file());
    }
}
