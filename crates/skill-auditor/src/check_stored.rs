//! Compare a skill's latest stored audit with a fresh evaluation.
//!
//! A stored audit is "stale" when its total or any dimension differs from what
//! the scorer produces now. The audit date and the stored `skill` field are
//! ignored, so re-storing an unchanged skill, or storing it from a different
//! checkout, never counts as a change.
//!
//! (Proof run: this comment-only change must trigger the full-tree check and pass.)

use crate::prune::{is_date_name, subdirs};
use crate::scorer::Result as Audit;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// Why a stored audit no longer matches a fresh evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    NoStoredAudit,
    GradeChanged {
        stored: String,
        current: String,
    },
    ScoreChanged {
        stored: i32,
        current: i32,
    },
    DimensionChanged {
        name: String,
        stored: i32,
        current: i32,
    },
}

/// The parts of a stored `audit.json` that the comparison uses.
///
/// Deliberately not the full scorer result: audits written by earlier versions
/// of the tool lack fields such as `date`, and the comparison ignores them
/// anyway. Total, grade and dimensions are required, so a file missing any of
/// those is still reported as malformed.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StoredAudit {
    pub total: i32,
    pub grade: String,
    pub dimensions: BTreeMap<String, i32>,
}

/// One stale or missing audit, in the shape printed with `--json`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub skill: String,
    pub reason: String,
    pub stored_grade: Option<String>,
    pub current_grade: String,
    pub stored_total: Option<i32>,
    pub current_total: i32,
}

/// The newest dated audit under `audits_dir` (`None` when there is none).
///
/// "Newest" is the largest `YYYY-MM-DD` folder name. Anything that is not a
/// plain dated folder (a `latest` link, stray files) is ignored. A newest
/// folder whose `audit.json` is missing or unreadable is an error, never a
/// silent fall back to an older audit.
pub fn latest_stored(audits_dir: &Path) -> Result<Option<StoredAudit>, String> {
    let newest = subdirs(audits_dir)
        .into_iter()
        .filter(|(name, _)| is_date_name(name))
        .max_by(|(a, _), (b, _)| a.cmp(b));
    let Some((name, dir)) = newest else {
        return Ok(None);
    };
    let file = dir.join("audit.json");
    let bytes = std::fs::read(&file)
        .map_err(|e| format!("cannot read stored audit {name}/audit.json: {e}"))?;
    serde_json::from_slice::<StoredAudit>(&bytes)
        .map(Some)
        .map_err(|e| format!("malformed stored audit {name}/audit.json: {e}"))
}

/// The first reason the stored audit is out of date, in the order: missing,
/// grade, total, then the first differing dimension. `None` when current.
pub fn compare(stored: Option<&StoredAudit>, fresh: &Audit) -> Option<Reason> {
    let Some(stored) = stored else {
        return Some(Reason::NoStoredAudit);
    };
    if stored.grade != fresh.grade {
        return Some(Reason::GradeChanged {
            stored: stored.grade.clone(),
            current: fresh.grade.clone(),
        });
    }
    if stored.total != fresh.total {
        return Some(Reason::ScoreChanged {
            stored: stored.total,
            current: fresh.total,
        });
    }
    // The dimension maps are sorted, so the first difference is stable. A
    // dimension present on only one side counts as 0 on the other.
    let names: std::collections::BTreeSet<&String> = stored
        .dimensions
        .keys()
        .chain(fresh.dimensions.keys())
        .collect();
    names.into_iter().find_map(|name| {
        let (s, c) = (
            stored.dimensions.get(name).copied().unwrap_or(0),
            fresh.dimensions.get(name).copied().unwrap_or(0),
        );
        (s != c).then(|| Reason::DimensionChanged {
            name: name.clone(),
            stored: s,
            current: c,
        })
    })
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::NoStoredAudit => write!(f, "no stored audit"),
            Reason::GradeChanged { stored, current } => {
                write!(f, "grade changed (stored {stored}, now {current})")
            }
            Reason::ScoreChanged { stored, current } => {
                write!(f, "score changed (stored {stored}, now {current})")
            }
            Reason::DimensionChanged {
                name,
                stored,
                current,
            } => {
                write!(
                    f,
                    "dimension changed ({name}: stored {stored}, now {current})"
                )
            }
        }
    }
}

/// Build the finding for one skill, or `None` when its stored audit is current.
pub fn finding(skill: &str, stored: Option<&StoredAudit>, fresh: &Audit) -> Option<Finding> {
    compare(stored, fresh).map(|reason| Finding {
        skill: skill.to_string(),
        reason: reason.to_string(),
        stored_grade: stored.map(|s| s.grade.clone()),
        current_grade: fresh.grade.clone(),
        stored_total: stored.map(|s| s.total),
        current_total: fresh.total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn audit(total: i32, grade: &str, dims: &[(&str, i32)]) -> Audit {
        Audit {
            skill: "/some/checkout/skills/x/SKILL.md".to_string(),
            date: "2026-10-02".to_string(),
            total,
            max_total: 140,
            grade: grade.to_string(),
            lines: 10,
            has_references: false,
            reference_count: 0,
            reference_section_compliant: false,
            dimensions: dims
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect::<BTreeMap<_, _>>(),
            errors: 0,
            warnings: 0,
            error_details: Vec::new(),
            warning_details: Vec::new(),
        }
    }

    fn stored(a: &Audit) -> StoredAudit {
        StoredAudit {
            total: a.total,
            grade: a.grade.clone(),
            dimensions: a.dimensions.clone(),
        }
    }

    fn write_audit(audits: &Path, date: &str, a: &Audit) {
        let dir = audits.join(date);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("audit.json"),
            serde_json::to_string_pretty(a).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn identical_audit_is_current() {
        let a = audit(114, "B", &[("d1", 10), ("d2", 12)]);
        assert_eq!(compare(Some(&stored(&a)), &a), None);
    }

    #[test]
    fn date_and_skill_path_are_ignored() {
        let old = audit(114, "B", &[("d1", 10)]);
        let mut fresh = old.clone();
        fresh.date = "2027-01-01".to_string();
        fresh.skill = "agentic-harness/pin".to_string();
        assert_eq!(compare(Some(&stored(&old)), &fresh), None);
    }

    #[test]
    fn missing_audit_is_reported() {
        let fresh = audit(114, "B", &[("d1", 10)]);
        assert_eq!(compare(None, &fresh), Some(Reason::NoStoredAudit));
    }

    #[test]
    fn grade_change_is_reported_before_score_change() {
        let old = audit(111, "C+", &[("d1", 10)]);
        let fresh = audit(114, "B", &[("d1", 13)]);
        assert_eq!(
            compare(Some(&stored(&old)), &fresh),
            Some(Reason::GradeChanged {
                stored: "C+".into(),
                current: "B".into()
            })
        );
    }

    #[test]
    fn score_change_with_same_grade_is_reported() {
        let old = audit(114, "B", &[("d1", 10)]);
        let fresh = audit(118, "B", &[("d1", 14)]);
        assert_eq!(
            compare(Some(&stored(&old)), &fresh),
            Some(Reason::ScoreChanged {
                stored: 114,
                current: 118
            })
        );
    }

    #[test]
    fn dimension_change_with_same_total_is_reported() {
        let old = audit(114, "B", &[("d1", 10), ("d2", 12)]);
        let fresh = audit(114, "B", &[("d1", 12), ("d2", 10)]);
        assert_eq!(
            compare(Some(&stored(&old)), &fresh),
            Some(Reason::DimensionChanged {
                name: "d1".into(),
                stored: 10,
                current: 12
            })
        );
    }

    #[test]
    fn reasons_read_as_the_contract_says() {
        assert_eq!(Reason::NoStoredAudit.to_string(), "no stored audit");
        assert_eq!(
            Reason::GradeChanged {
                stored: "B".into(),
                current: "B+".into()
            }
            .to_string(),
            "grade changed (stored B, now B+)"
        );
        assert_eq!(
            Reason::ScoreChanged {
                stored: 114,
                current: 118
            }
            .to_string(),
            "score changed (stored 114, now 118)"
        );
        assert_eq!(
            Reason::DimensionChanged {
                name: "d1".into(),
                stored: 11,
                current: 13
            }
            .to_string(),
            "dimension changed (d1: stored 11, now 13)"
        );
    }

    #[test]
    fn finding_carries_both_sides() {
        let old = audit(114, "B", &[("d1", 10)]);
        let fresh = audit(118, "B", &[("d1", 14)]);
        let f = finding("ci-cd/helm", Some(&stored(&old)), &fresh).unwrap();
        assert_eq!(f.skill, "ci-cd/helm");
        assert_eq!(f.reason, "score changed (stored 114, now 118)");
        assert_eq!((f.stored_total, f.current_total), (Some(114), 118));
        assert_eq!(
            (f.stored_grade.as_deref(), f.current_grade.as_str()),
            (Some("B"), "B")
        );
        assert!(finding("ci-cd/helm", Some(&stored(&fresh)), &fresh).is_none());
    }

    #[test]
    fn latest_picks_the_newest_dated_folder() {
        let root = tempdir().unwrap();
        let audits = root.path().join(".audits");
        write_audit(&audits, "2026-03-02", &audit(100, "C", &[("d1", 1)]));
        write_audit(&audits, "2026-10-02", &audit(118, "B", &[("d1", 9)]));
        write_audit(&audits, "2026-04-11", &audit(110, "C+", &[("d1", 5)]));
        let got = latest_stored(&audits).unwrap().unwrap();
        assert_eq!(got.total, 118);
    }

    #[test]
    fn latest_ignores_non_dated_entries() {
        let root = tempdir().unwrap();
        let audits = root.path().join(".audits");
        write_audit(&audits, "2026-03-02", &audit(100, "C", &[("d1", 1)]));
        std::fs::create_dir_all(audits.join("zzz-not-a-date")).unwrap();
        std::fs::write(audits.join("notes.txt"), "x").unwrap();
        let got = latest_stored(&audits).unwrap().unwrap();
        assert_eq!(got.total, 100);
    }

    #[test]
    fn latest_is_none_without_an_audits_folder() {
        let root = tempdir().unwrap();
        assert!(latest_stored(&root.path().join(".audits"))
            .unwrap()
            .is_none());
    }

    #[test]
    fn latest_is_none_for_an_empty_audits_folder() {
        let root = tempdir().unwrap();
        let audits = root.path().join(".audits");
        std::fs::create_dir_all(&audits).unwrap();
        assert!(latest_stored(&audits).unwrap().is_none());
    }

    #[test]
    fn an_old_format_audit_without_a_date_still_loads_and_compares() {
        let root = tempdir().unwrap();
        let audits = root.path().join(".audits");
        let dir = audits.join("2026-03-02");
        std::fs::create_dir_all(&dir).unwrap();
        // Written by an earlier tool version: no `date`, no `maxTotal`, no details.
        std::fs::write(
            dir.join("audit.json"),
            r#"{"skill":"x","total":108,"grade":"C+","dimensions":{"d1":10,"d2":12}}"#,
        )
        .unwrap();
        let got = latest_stored(&audits).unwrap().expect("loads");
        assert_eq!((got.total, got.grade.as_str()), (108, "C+"));
        let fresh = audit(118, "B+", &[("d1", 12), ("d2", 14)]);
        assert_eq!(
            compare(Some(&got), &fresh),
            Some(Reason::GradeChanged {
                stored: "C+".into(),
                current: "B+".into()
            })
        );
    }

    #[test]
    fn an_audit_missing_the_total_is_still_malformed() {
        let root = tempdir().unwrap();
        let audits = root.path().join(".audits");
        let dir = audits.join("2026-03-02");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("audit.json"),
            r#"{"skill":"x","grade":"C+","dimensions":{}}"#,
        )
        .unwrap();
        assert!(latest_stored(&audits).unwrap_err().contains("audit.json"));
    }

    #[test]
    fn newest_folder_without_audit_json_is_an_error_not_a_fallback() {
        let root = tempdir().unwrap();
        let audits = root.path().join(".audits");
        write_audit(&audits, "2026-03-02", &audit(100, "C", &[("d1", 1)]));
        std::fs::create_dir_all(audits.join("2026-10-02")).unwrap();
        let err = latest_stored(&audits).unwrap_err();
        assert!(
            err.contains("2026-10-02"),
            "error should name the folder: {err}"
        );
    }

    #[test]
    fn malformed_audit_json_is_an_error() {
        let root = tempdir().unwrap();
        let audits = root.path().join(".audits");
        let dir = audits.join("2026-10-02");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("audit.json"), "{ not json").unwrap();
        let err = latest_stored(&audits).unwrap_err();
        assert!(
            err.contains("audit.json"),
            "error should name the file: {err}"
        );
    }
}
