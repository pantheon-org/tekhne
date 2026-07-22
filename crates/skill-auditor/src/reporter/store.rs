//! Persist audit artefacts to `.context/audits/` (Go `reporter.Store`).

use super::{analysis, remediation};
use crate::scorer::Result;
use std::io;
use std::path::Path;

/// Write `audit.json`, `Analysis.md` and `Remediation.md` to
/// `<repo_root>/.context/audits/<skill_path>/<date>/`, each via a temp file
/// and atomic rename (Go `Store`). `audit.json` uses 2-space indentation to
/// match Go's `json.MarshalIndent`.
pub fn store(repo_root: &Path, skill_path: &str, r: &Result) -> io::Result<()> {
    let dir = repo_root
        .join(".context")
        .join("audits")
        .join(skill_path)
        .join(&r.date);

    std::fs::create_dir_all(&dir)?;

    let audit_json = serde_json::to_string_pretty(r)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let files = [
        ("audit.json", audit_json),
        ("Analysis.md", analysis(r)),
        ("Remediation.md", remediation(r)),
    ];

    for (name, data) in files {
        let dest = dir.join(name);
        let tmp = dir.join(format!("{name}.tmp"));
        std::fs::write(&tmp, data.as_bytes())?;
        if let Err(e) = std::fs::rename(&tmp, &dest) {
            let _ = std::fs::remove_file(&tmp);
            return Err(e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scorer::Result;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    const TEST_DATE: &str = "2026-01-01";

    fn make_store_result() -> Result {
        Result {
            skill: "agentic-harness/skill-quality-auditor".to_string(),
            date: TEST_DATE.to_string(),
            total: 122,
            max_total: 140,
            grade: "B+".to_string(),
            lines: 0,
            has_references: false,
            reference_count: 0,
            reference_section_compliant: false,
            dimensions: BTreeMap::from([("knowledgeDelta".to_string(), 14)]),
            errors: 0,
            warnings: 0,
            error_details: Vec::new(),
            warning_details: Vec::new(),
        }
    }

    #[test]
    fn store_creates_file() {
        let root = tempdir().unwrap();
        let skill = "agentic-harness/skill-quality-auditor";
        store(root.path(), skill, &make_store_result()).unwrap();
        let expected = root
            .path()
            .join(".context")
            .join("audits")
            .join(skill)
            .join(TEST_DATE)
            .join("audit.json");
        assert!(expected.exists());
    }

    #[test]
    fn store_content() {
        let root = tempdir().unwrap();
        let skill = "agentic-harness/skill-quality-auditor";
        store(root.path(), skill, &make_store_result()).unwrap();
        let dest = root
            .path()
            .join(".context")
            .join("audits")
            .join(skill)
            .join(TEST_DATE)
            .join("audit.json");
        let data = std::fs::read_to_string(&dest).unwrap();
        let got: Result = serde_json::from_str(&data).expect("valid JSON");
        assert_eq!(got.skill, skill);
        assert_eq!(got.total, 122);
        assert_eq!(got.grade, "B+");
        assert!(data.contains("  "), "expected indented JSON");
    }
}
