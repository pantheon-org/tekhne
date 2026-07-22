//! D9 — Eval Validation (max 20). Ported from `d9_eval_validation.go`.

use super::helpers::file_exists;
use super::{err_diag, warn_diag, Diagnostic};
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

#[derive(Deserialize)]
struct InstrData {
    #[serde(default)]
    instructions: Vec<Value>,
}

#[derive(Deserialize, Default)]
struct SummaryData {
    #[serde(default)]
    instructions_coverage: Coverage,
}

#[derive(Deserialize, Default)]
struct Coverage {
    #[serde(default)]
    coverage_percentage: Value,
}

#[derive(Deserialize)]
struct CriteriaData {
    #[serde(default)]
    checklist: Vec<ChecklistItem>,
}

#[derive(Deserialize)]
struct ChecklistItem {
    #[serde(default)]
    max_score: i32,
}

/// Score the Eval Validation dimension.
pub fn score(evals_dir: &Path) -> (i32, Vec<Diagnostic>) {
    let mut score = 0;
    let mut diags = Vec::new();

    if !evals_dir.is_dir() {
        diags.push(warn_diag("D9", "evals/ directory missing entirely"));
        return (score, diags);
    }
    score += 4;

    let instr_file = evals_dir.join("instructions.json");
    if let Ok(data) = std::fs::read(&instr_file) {
        match serde_json::from_slice::<InstrData>(&data) {
            Err(_) => {
                diags.push(err_diag(
                    "D9",
                    "instructions.json exists but is not valid JSON",
                ));
            }
            Ok(instr_data) => {
                if !instr_data.instructions.is_empty() || !data.is_empty() {
                    score += 3;
                }
            }
        }
    }

    let summary_file = evals_dir.join("summary.json");
    if let Ok(data) = std::fs::read(&summary_file) {
        match serde_json::from_slice::<SummaryData>(&data) {
            Err(_) => {
                diags.push(err_diag("D9", "summary.json exists but is not valid JSON"));
            }
            Ok(summary_data) => {
                let coverage = parse_coverage_percentage(
                    &summary_data.instructions_coverage.coverage_percentage,
                );
                if coverage >= 0 {
                    score += 3;
                    if coverage >= 80 {
                        score += 3;
                    } else {
                        diags.push(warn_diag(
                            "D9",
                            &format!("summary.json coverage is {coverage}% (below 80% threshold)"),
                        ));
                    }
                } else if !data.is_empty() {
                    score += 3;
                }
            }
        }
    }

    let (valid_scenarios, scenario_diags) = count_valid_scenarios_with_diags(evals_dir);
    diags.extend(scenario_diags);
    if valid_scenarios >= 3 {
        score += 4;
    } else if valid_scenarios >= 1 {
        score += 2;
    }

    if score > 20 {
        score = 20;
    }
    (score, diags)
}

/// Parse a coverage percentage to an int, or -1 if unparseable (Go
/// `parseCoveragePercentage`).
pub fn parse_coverage_percentage(v: &Value) -> i32 {
    match v {
        Value::Number(n) => n.as_f64().map(|f| f as i32).unwrap_or(-1),
        Value::String(val) => {
            let trimmed = val.trim();
            let s = trimmed.trim_end_matches('%');
            if s.is_empty() {
                return -1;
            }
            let s = match s.find('.') {
                Some(idx) => &s[..idx],
                None => s,
            };
            let mut n = 0;
            for ch in s.chars() {
                if !ch.is_ascii_digit() {
                    return -1;
                }
                n = n * 10 + (ch as i32 - '0' as i32);
            }
            n
        }
        _ => -1,
    }
}

/// Thin wrapper used by tests (Go `countValidScenarios`).
#[allow(dead_code)]
pub fn count_valid_scenarios(evals_dir: &Path) -> i32 {
    count_valid_scenarios_with_diags(evals_dir).0
}

/// Count valid scenario dirs and emit diagnostics for problems (Go
/// `countValidScenariosWithDiags`). Directory entries are sorted by name to
/// match Go's `os.ReadDir` ordering, keeping diagnostic order deterministic.
fn count_valid_scenarios_with_diags(evals_dir: &Path) -> (i32, Vec<Diagnostic>) {
    let mut diags = Vec::new();
    let Ok(read) = std::fs::read_dir(evals_dir) else {
        return (0, diags);
    };
    let mut entries: Vec<_> = read.flatten().collect();
    entries.sort_by_key(|e| e.file_name());

    // Detect flat scenario-NN.md files (legacy format).
    let mut flat_count = 0;
    for entry in &entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !entry.path().is_dir() && name.starts_with("scenario-") && name.ends_with(".md") {
            flat_count += 1;
        }
    }

    let mut valid = 0;
    for entry in &entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !entry.path().is_dir() || !name.starts_with("scenario-") {
            continue;
        }
        let scenario_dir = entry.path();
        let has_task = file_exists(&scenario_dir.join("task.md"));
        let has_criteria = file_exists(&scenario_dir.join("criteria.json"));
        let has_capability = file_exists(&scenario_dir.join("capability.txt"));

        if !has_task || !has_criteria || !has_capability {
            let mut missing = Vec::new();
            if !has_task {
                missing.push("task.md");
            }
            if !has_criteria {
                missing.push("criteria.json");
            }
            if !has_capability {
                missing.push("capability.txt");
            }
            diags.push(warn_diag(
                "D9",
                &format!("{name} missing: {}", missing.join(", ")),
            ));
            continue;
        }

        let Ok(data) = std::fs::read(scenario_dir.join("criteria.json")) else {
            valid += 1;
            continue;
        };
        match serde_json::from_slice::<CriteriaData>(&data) {
            Err(_) => {
                diags.push(err_diag(
                    "D9",
                    &format!("{name}/criteria.json is not valid JSON"),
                ));
                valid += 1;
            }
            Ok(criteria_data) => {
                let sum: i32 = criteria_data.checklist.iter().map(|c| c.max_score).sum();
                if sum == 100 {
                    valid += 1;
                } else {
                    diags.push(warn_diag(
                        "D9",
                        &format!("{name}/criteria.json checklist does not sum to 100 (got {sum})"),
                    ));
                }
            }
        }
    }

    if valid == 0 && flat_count > 0 {
        diags.push(warn_diag(
            "D9",
            &format!(
                "{flat_count} flat scenario-NN.md file(s) found; migrate to scenario-N/ subdirectory format to score on D9"
            ),
        ));
    }

    (valid, diags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_file(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    #[test]
    fn no_evals_dir() {
        let dir = tempdir().unwrap();
        let (score, diags) = score(&dir.path().join("nonexistent"));
        assert_eq!(score, 0);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].dimension, "D9");
    }

    #[test]
    fn full_score() {
        let dir = tempdir().unwrap();
        let evals = dir.path();
        write_file(
            &evals.join("instructions.json"),
            r#"{"instructions":[{"type":"a"},{"type":"b"}]}"#,
        );
        write_file(
            &evals.join("summary.json"),
            r#"{"instructions_coverage":{"coverage_percentage":85}}"#,
        );
        for i in 1..=3 {
            let sdir = evals.join(format!("scenario-{i}"));
            write_file(&sdir.join("task.md"), "# Task");
            write_file(&sdir.join("capability.txt"), "cap");
            write_file(
                &sdir.join("criteria.json"),
                r#"{"checklist":[{"description":"x","max_score":60},{"description":"y","max_score":40}]}"#,
            );
        }
        let (score, _) = score(evals);
        assert_eq!(score, 17);
    }

    #[test]
    fn low_coverage_warning() {
        let dir = tempdir().unwrap();
        write_file(
            &dir.path().join("summary.json"),
            r#"{"instructions_coverage":{"coverage_percentage":72}}"#,
        );
        let (_, diags) = score(dir.path());
        assert!(diags
            .iter()
            .any(|d| d.dimension == "D9" && d.severity == "warning"));
    }

    #[test]
    fn invalid_instructions_json() {
        let dir = tempdir().unwrap();
        write_file(&dir.path().join("instructions.json"), "not json");
        let (_, diags) = score(dir.path());
        assert!(diags
            .iter()
            .any(|d| d.dimension == "D9" && d.severity == "error"));
    }

    #[test]
    fn invalid_summary_json() {
        let dir = tempdir().unwrap();
        write_file(&dir.path().join("summary.json"), "{bad}");
        let (_, diags) = score(dir.path());
        assert!(diags
            .iter()
            .any(|d| d.dimension == "D9" && d.severity == "error"));
    }

    #[test]
    fn scenario_missing_files() {
        let dir = tempdir().unwrap();
        write_file(&dir.path().join("scenario-1").join("task.md"), "# Task");
        let (_, diags) = score(dir.path());
        assert!(diags
            .iter()
            .any(|d| d.dimension == "D9" && d.severity == "warning"));
    }

    #[test]
    fn criteria_not_summing_to_100() {
        let dir = tempdir().unwrap();
        let sdir = dir.path().join("scenario-1");
        write_file(&sdir.join("task.md"), "# Task");
        write_file(&sdir.join("capability.txt"), "cap");
        write_file(
            &sdir.join("criteria.json"),
            r#"{"checklist":[{"description":"x","max_score":60},{"description":"y","max_score":30}]}"#,
        );
        let (score, diags) = score(dir.path());
        assert!(diags
            .iter()
            .any(|d| d.dimension == "D9" && d.severity == "warning"));
        assert!(score <= 10);
    }

    #[test]
    fn invalid_criteria_json() {
        let dir = tempdir().unwrap();
        let sdir = dir.path().join("scenario-1");
        write_file(&sdir.join("task.md"), "# Task");
        write_file(&sdir.join("capability.txt"), "cap");
        write_file(&sdir.join("criteria.json"), "{bad json}");
        let (_, diags) = score(dir.path());
        assert!(diags
            .iter()
            .any(|d| d.dimension == "D9" && d.severity == "error"));
    }

    #[test]
    fn flat_scenario_format_warning() {
        let dir = tempdir().unwrap();
        write_file(&dir.path().join("scenario-01.md"), "# Scenario 1");
        write_file(&dir.path().join("scenario-02.md"), "# Scenario 2");
        let (score, diags) = score(dir.path());
        assert!(diags.iter().any(|d| d.dimension == "D9"
            && d.severity == "warning"
            && d.message.contains("flat scenario")));
        assert_eq!(score, 4);
    }

    #[test]
    fn count_valid_scenarios_wrapper() {
        let dir = tempdir().unwrap();
        for i in 1..=2 {
            let sdir = dir.path().join(format!("scenario-{i}"));
            write_file(&sdir.join("task.md"), "# Task");
            write_file(&sdir.join("capability.txt"), "cap");
            write_file(
                &sdir.join("criteria.json"),
                r#"{"checklist":[{"max_score":100}]}"#,
            );
        }
        assert_eq!(count_valid_scenarios(dir.path()), 2);
    }

    #[test]
    fn parse_coverage() {
        assert_eq!(parse_coverage_percentage(&Value::from(85.0)), 85);
        assert_eq!(parse_coverage_percentage(&Value::from(72)), 72);
        assert_eq!(parse_coverage_percentage(&Value::from("90%")), 90);
        assert_eq!(parse_coverage_percentage(&Value::from("75.5%")), 75);
        assert_eq!(parse_coverage_percentage(&Value::from("")), -1);
        assert_eq!(parse_coverage_percentage(&Value::Null), -1);
        assert_eq!(parse_coverage_percentage(&Value::from("abc")), -1);
        assert_eq!(parse_coverage_percentage(&Value::from(true)), -1);
    }
}
