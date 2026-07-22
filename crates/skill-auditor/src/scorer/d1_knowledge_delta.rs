//! D1 — Knowledge Delta (max 20). Ported from `d1_knowledge_delta.go`.

use super::helpers::count_pattern;
use super::{err_diag, Diagnostic};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct InstrData {
    #[serde(default)]
    instructions: Vec<Instr>,
}

#[derive(Deserialize)]
struct Instr {
    #[serde(default)]
    why_given: String,
}

/// Score the Knowledge Delta dimension.
pub fn score(content: &str, skill_dir: &Path) -> (i32, Vec<Diagnostic>) {
    let mut score = 15;
    let mut diags = Vec::new();

    for pat in [
        "npm install",
        "yarn add",
        "pip install",
        "getting started",
        "introduction",
        "basic syntax",
        "hello world",
    ] {
        if count_pattern(content, pat) > 0 {
            score -= 2;
        }
    }

    for pat in [
        "anti-pattern",
        "NEVER",
        "ALWAYS",
        "production",
        "gotcha",
        "pitfall",
    ] {
        if count_pattern(content, pat) > 0 {
            score += 1;
        }
    }

    let instr_file = skill_dir.join("evals").join("instructions.json");
    if let Ok(data) = std::fs::read(&instr_file) {
        match serde_json::from_slice::<InstrData>(&data) {
            Err(_) => {
                diags.push(err_diag(
                    "D1",
                    "instructions.json exists but cannot be parsed",
                ));
            }
            Ok(instr_data) => {
                let total = instr_data.instructions.len() as i32;
                if total > 0 {
                    let mut new_know = 0;
                    let mut pref = 0;
                    for instr in &instr_data.instructions {
                        match instr.why_given.as_str() {
                            "new knowledge" => new_know += 1,
                            "preference" => pref += 1,
                            _ => {}
                        }
                    }
                    let expert_ratio = (new_know + pref) * 100 / total;
                    if expert_ratio >= 70 {
                        score += 2;
                    } else if expert_ratio < 30 {
                        score -= 2;
                    }
                }
            }
        }
    }

    score = score.clamp(0, 20);
    (score, diags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn penalties() {
        let content = "---\ndescription: something\n---\n# Skill\ngetting started with npm install";
        let (score, _) = score(content, tempdir().unwrap().path());
        assert_eq!(score, 11);
    }

    #[test]
    fn rewards_capped() {
        let content = "---\ndescription: something\n---\n# Skill\nNEVER do this. ALWAYS validate. anti-pattern here. production gotcha pitfall.";
        let (score, _) = score(content, tempdir().unwrap().path());
        assert_eq!(score, 20);
    }

    #[test]
    fn expert_ratio_bonus() {
        let dir = tempdir().unwrap();
        let evals = dir.path().join("evals");
        fs::create_dir_all(&evals).unwrap();
        let instr = r#"{"instructions":[
            {"why_given":"new knowledge"},{"why_given":"new knowledge"},
            {"why_given":"new knowledge"},{"why_given":"new knowledge"},
            {"why_given":"preference"},{"why_given":"preference"},
            {"why_given":"preference"},{"why_given":"preference"},
            {"why_given":"other"},{"why_given":"other"}]}"#;
        fs::write(evals.join("instructions.json"), instr).unwrap();
        let content = "---\ndescription: x\n---\n# Skill\nsome content";
        let (score, diags) = score(content, dir.path());
        assert!(diags.is_empty());
        assert_eq!(score, 17);
    }

    #[test]
    fn expert_ratio_penalty() {
        let dir = tempdir().unwrap();
        let evals = dir.path().join("evals");
        fs::create_dir_all(&evals).unwrap();
        let instr = r#"{"instructions":[
            {"why_given":"new knowledge"},{"why_given":"other"},
            {"why_given":"other"},{"why_given":"other"},{"why_given":"other"},
            {"why_given":"other"},{"why_given":"other"},{"why_given":"other"},
            {"why_given":"other"},{"why_given":"other"}]}"#;
        fs::write(evals.join("instructions.json"), instr).unwrap();
        let content = "---\ndescription: x\n---\n# Skill\nsome content";
        let (score, _) = score(content, dir.path());
        assert_eq!(score, 13);
    }

    #[test]
    fn instructions_parse_error() {
        let dir = tempdir().unwrap();
        let evals = dir.path().join("evals");
        fs::create_dir_all(&evals).unwrap();
        fs::write(evals.join("instructions.json"), "not json").unwrap();
        let content = "---\ndescription: x\n---\n# Skill";
        let (_, diags) = score(content, dir.path());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, "error");
        assert_eq!(diags[0].dimension, "D1");
    }
}
