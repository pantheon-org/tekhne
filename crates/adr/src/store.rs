//! Reading and writing ADR records in a directory.
//!
//! The directory *is* the collection: every `*.md` file with frontmatter is a
//! record, and its file stem is its slug. There is no authoritative index to
//! keep in step, so a record cannot go missing by being absent from a manifest,
//! and two branches each adding a decision never collide on a shared file.
//!
//! The generated catalogue written by [`crate::index`] is excluded from loads,
//! since it is derived from these records rather than one of them.

use std::fs;
use std::path::{Path, PathBuf};

use common::{Error, Result};

use crate::record::{Record, Status};

/// File stems that live in an ADR directory without being records.
const NON_RECORD_STEMS: [&str; 2] = ["index", "readme"];

/// Create `dir` if it is absent.
pub fn ensure_dir(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).map_err(|e| Error::io(dir, e))
}

/// The path a record with `slug` occupies under `dir`.
pub fn path_for(dir: &Path, slug: &str) -> PathBuf {
    dir.join(format!("{slug}.md"))
}

/// Whether a record with `slug` already exists under `dir`.
pub fn exists(dir: &Path, slug: &str) -> bool {
    path_for(dir, slug).is_file()
}

/// Write `record` into `dir`, creating the directory if needed.
pub fn save(dir: &Path, record: &Record) -> Result<PathBuf> {
    ensure_dir(dir)?;
    let path = record.path_in(dir);
    fs::write(&path, record.render()?).map_err(|e| Error::io(&path, e))?;
    Ok(path)
}

/// Load the record with `slug` from `dir`.
///
/// Reports [`Error::NotFound`] when the file is absent, so callers can tell a
/// missing record from a malformed one.
pub fn load(dir: &Path, slug: &str) -> Result<Record> {
    let path = path_for(dir, slug);
    if !path.is_file() {
        return Err(Error::NotFound(path));
    }
    let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
    Record::parse(slug, &text)
}

/// Load every record in `dir`, sorted by slug.
///
/// A file that fails to parse is reported rather than skipped: a record with
/// broken frontmatter is a problem to fix, not one to hide. An absent directory
/// yields an empty collection, so `list` on a fresh repository is not an error.
pub fn load_all(dir: &Path) -> Result<Vec<Record>> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut slugs = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| Error::io(dir, e))? {
        let entry = entry.map_err(|e| Error::io(dir, e))?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if NON_RECORD_STEMS.contains(&stem.to_ascii_lowercase().as_str()) {
            continue;
        }
        slugs.push(stem.to_string());
    }
    slugs.sort();

    slugs.into_iter().map(|slug| load(dir, &slug)).collect()
}

/// A count of records by status and by branch type, for `adr status`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Summary {
    /// Total number of records.
    pub total: usize,
    /// Counts keyed by status wire form, in lifecycle order.
    pub by_status: Vec<(&'static str, usize)>,
    /// Counts keyed by branch-type wire form.
    pub by_type: Vec<(&'static str, usize)>,
}

/// Summarise `records` by status and branch type, omitting empty buckets.
pub fn summarise(records: &[Record]) -> Summary {
    let by_status = Status::ALL
        .iter()
        .filter_map(|status| {
            let n = records.iter().filter(|r| r.meta.status == *status).count();
            (n > 0).then_some((status.as_str(), n))
        })
        .collect();
    let by_type = crate::record::BranchType::ALL
        .iter()
        .filter_map(|t| {
            let n = records.iter().filter(|r| r.meta.branch_type == *t).count();
            (n > 0).then_some((t.as_str(), n))
        })
        .collect();
    Summary {
        total: records.len(),
        by_status,
        by_type,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{BranchType, Record};
    use tempfile::TempDir;

    fn record(slug: &str, t: BranchType) -> Record {
        Record::new(slug, slug, t, "Test User", "2026-09-15", None)
    }

    #[test]
    fn save_then_load_round_trips() {
        let tmp = TempDir::new().unwrap();
        let original = record("adopt-otel", BranchType::Feat);
        let path = save(tmp.path(), &original).unwrap();

        assert_eq!(path, tmp.path().join("adopt-otel.md"));
        assert_eq!(load(tmp.path(), "adopt-otel").unwrap(), original);
    }

    #[test]
    fn save_creates_a_missing_directory() {
        let tmp = TempDir::new().unwrap();
        let nested = tmp.path().join("docs").join("adr");
        save(&nested, &record("x", BranchType::Chore)).unwrap();
        assert!(nested.join("x.md").is_file());
    }

    #[test]
    fn load_reports_a_missing_record_as_not_found() {
        let tmp = TempDir::new().unwrap();
        let err = load(tmp.path(), "nope").unwrap_err();
        assert!(matches!(err, Error::NotFound(_)), "{err}");
    }

    #[test]
    fn load_all_is_empty_for_an_absent_directory() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("nothing-here");
        assert!(load_all(&missing).unwrap().is_empty());
    }

    #[test]
    fn load_all_sorts_by_slug_and_skips_non_records() {
        let tmp = TempDir::new().unwrap();
        save(tmp.path(), &record("zebra", BranchType::Fix)).unwrap();
        save(tmp.path(), &record("alpha", BranchType::Feat)).unwrap();
        fs::write(tmp.path().join("README.md"), "# Not a record\n").unwrap();
        fs::write(tmp.path().join("index.md"), "# Generated\n").unwrap();
        fs::write(tmp.path().join("notes.txt"), "ignored\n").unwrap();

        let slugs: Vec<String> = load_all(tmp.path())
            .unwrap()
            .into_iter()
            .map(|r| r.slug)
            .collect();
        assert_eq!(slugs, vec!["alpha", "zebra"]);
    }

    #[test]
    fn load_all_surfaces_a_malformed_record() {
        let tmp = TempDir::new().unwrap();
        save(tmp.path(), &record("good", BranchType::Feat)).unwrap();
        fs::write(tmp.path().join("broken.md"), "no frontmatter here\n").unwrap();

        let err = load_all(tmp.path()).unwrap_err().to_string();
        assert!(err.contains("broken"), "{err}");
    }

    #[test]
    fn exists_reflects_the_filesystem() {
        let tmp = TempDir::new().unwrap();
        assert!(!exists(tmp.path(), "adopt-otel"));
        save(tmp.path(), &record("adopt-otel", BranchType::Feat)).unwrap();
        assert!(exists(tmp.path(), "adopt-otel"));
    }

    #[test]
    fn summarise_counts_by_status_and_type_omitting_empties() {
        let mut a = record("a", BranchType::Feat);
        let mut b = record("b", BranchType::Docs);
        let c = record("c", BranchType::Feat);
        a.transition(Status::Accepted, "2026-09-15T10:00:00Z");
        b.transition(Status::Accepted, "2026-09-15T10:00:00Z");

        let s = summarise(&[a, b, c]);
        assert_eq!(s.total, 3);
        assert_eq!(s.by_status, vec![("proposed", 1), ("accepted", 2)]);
        assert_eq!(s.by_type, vec![("feat", 2), ("docs", 1)]);
    }

    #[test]
    fn summarise_of_nothing_is_empty() {
        assert_eq!(summarise(&[]), Summary::default());
    }
}
