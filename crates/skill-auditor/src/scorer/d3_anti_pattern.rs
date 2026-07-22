//! D3 — Anti-Pattern Quality (max 15). Ported from `d3_anti_pattern.go`.

use super::bridge::ValidatorBridge;
use super::helpers::{count_pattern, matches_regex};
use super::{err_diag, Diagnostic};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

#[derive(Deserialize)]
struct InstrData {
    #[serde(default)]
    instructions: Vec<Instr>,
}

#[derive(Deserialize)]
struct Instr {
    #[serde(default, rename = "type")]
    kind: String,
    #[serde(default)]
    original_snippets: Value,
    #[serde(default)]
    content: String,
}

/// Score the Anti-Pattern Quality dimension.
pub fn score(content: &str, skill_dir: &Path, b: &ValidatorBridge) -> (i32, Vec<Diagnostic>) {
    let mut score = 0;
    let mut diags = Vec::new();

    if let Some(c) = &b.content {
        let sm = c.strong_markers;
        if sm > 8 {
            score += 5;
        } else if sm > 4 {
            score += 3;
        } else if sm > 0 {
            score += 1;
        }
    } else {
        let never_count = count_pattern(content, "NEVER") as i32;
        if never_count > 3 {
            score += 3;
        } else if never_count > 0 {
            score += never_count;
        }
    }

    if matches_regex(content, r"(?is)BAD.*GOOD") {
        score += 2;
    }
    if count_pattern(content, "WHY:") > 0 {
        score += 2;
    }

    let instr_file = skill_dir.join("evals").join("instructions.json");
    if let Ok(data) = std::fs::read(&instr_file) {
        match serde_json::from_slice::<InstrData>(&data) {
            Err(_) => {
                diags.push(err_diag(
                    "D3",
                    "instructions.json exists but cannot be parsed",
                ));
            }
            Ok(instr_data) => {
                let anti_pat =
                    Regex::new(r"(?i)NEVER|ALWAYS|anti-pattern|avoid|do not").expect("valid regex");
                let mut anti_instr = 0;
                for instr in &instr_data.instructions {
                    let snippet_str = match &instr.original_snippets {
                        Value::String(s) => s.clone(),
                        Value::Array(items) => items
                            .iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join(" "),
                        _ => String::new(),
                    };
                    let snippet_str = if snippet_str.is_empty() {
                        instr.content.clone()
                    } else {
                        snippet_str
                    };
                    if instr.kind.eq_ignore_ascii_case("anti-pattern")
                        || anti_pat.is_match(&snippet_str)
                    {
                        anti_instr += 1;
                    }
                }
                if anti_instr >= 5 {
                    score += 2;
                } else if anti_instr >= 3 {
                    score += 1;
                }
            }
        }
    }

    score = score.clamp(0, 15);
    (score, diags)
}

#[cfg(test)]
mod tests {
    use super::super::bridge::ValidatorBridge;
    use super::*;
    use skill_validator_rs::ContentReport;
    use std::fs;
    use tempfile::tempdir;

    fn nil_bridge() -> ValidatorBridge {
        ValidatorBridge::default()
    }

    #[test]
    fn library_strong_markers() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                strong_markers: 10,
                ..Default::default()
            }),
            structure: None,
        };
        let content = "---\ndescription: x\n---\nNEVER do this. WHY: because reasons.";
        let (score, _) = score(content, tempdir().unwrap().path(), &b);
        assert!(score >= 5);
    }

    #[test]
    fn fallback_never_and_why() {
        let content = "---\ndescription: x\n---\nNEVER do this. WHY: because reasons.";
        let (score, _) = score(content, tempdir().unwrap().path(), &nil_bridge());
        assert!(score >= 3);
    }

    #[test]
    fn bad_good() {
        let content = "---\ndescription: x\n---\nBAD: do this GOOD: do that instead. WHY: reasons.";
        let (score, _) = score(content, tempdir().unwrap().path(), &nil_bridge());
        assert!(score >= 4);
    }

    #[test]
    fn anti_pattern_instructions_bonus() {
        let dir = tempdir().unwrap();
        let evals = dir.path().join("evals");
        fs::create_dir_all(&evals).unwrap();
        let instr = r#"{"instructions":[
            {"original_snippets":"NEVER do this","content":"x"},
            {"original_snippets":"ALWAYS validate","content":"x"},
            {"original_snippets":"avoid this pattern","content":"x"},
            {"original_snippets":"do not use","content":"x"},
            {"original_snippets":"anti-pattern here","content":"x"}]}"#;
        fs::write(evals.join("instructions.json"), instr).unwrap();
        let content = "---\ndescription: x\n---\n# Skill";
        let (score, diags) = score(content, dir.path(), &nil_bridge());
        assert!(diags.is_empty());
        assert!(score >= 2);
    }

    #[test]
    fn instructions_parse_error() {
        let dir = tempdir().unwrap();
        let evals = dir.path().join("evals");
        fs::create_dir_all(&evals).unwrap();
        fs::write(evals.join("instructions.json"), "{bad json").unwrap();
        let content = "---\ndescription: x\n---\n# Skill";
        let (_, diags) = score(content, dir.path(), &nil_bridge());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, "error");
        assert_eq!(diags[0].dimension, "D3");
    }
}
