//! D8 — Practical Usability (max 15). Ported from `d8_practical_usability.go`.
//!
//! Code-block count and language tags come from the library; the run-command
//! check stays custom.

use super::bridge::ValidatorBridge;
use super::helpers::{code_block_count, count_pattern};

/// Score the Practical Usability dimension.
pub fn score(content: &str, b: &ValidatorBridge) -> i32 {
    let mut score = 5;

    if let Some(c) = &b.content {
        if c.code_block_count > 5 {
            score += 4;
        } else if c.code_block_count > 2 {
            score += 2;
        } else if c.code_block_count > 0 {
            score += 1;
        }
        if !c.code_languages.is_empty() {
            score += 2;
        }
    } else {
        let blocks = code_block_count(content);
        if blocks > 5 {
            score += 4;
        } else if blocks > 2 {
            score += 2;
        }
    }

    let has_run_cmd = count_pattern(content, "./") > 0
        || count_pattern(content, "npm run") > 0
        || count_pattern(content, "yarn ") > 0
        || count_pattern(content, "pnpm run") > 0
        || count_pattern(content, "bun run") > 0
        || count_pattern(content, "make ") > 0
        || count_pattern(content, "python ") > 0
        || count_pattern(content, "go run") > 0;
    if has_run_cmd {
        score += 4;
    }

    score.clamp(0, 15)
}

#[cfg(test)]
mod tests {
    use super::super::bridge::ValidatorBridge;
    use super::*;
    use skill_validator_rs::ContentReport;

    fn nil_bridge() -> ValidatorBridge {
        ValidatorBridge::default()
    }

    #[test]
    fn many_code_blocks() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                code_block_count: 6,
                code_languages: vec!["bash".to_string(), "typescript".to_string()],
                ..Default::default()
            }),
            structure: None,
        };
        let content = "---\ndescription: x\n---\nRun ./script.sh to start.";
        assert_eq!(score(content, &b), 15);
    }

    #[test]
    fn few_code_blocks_fallback() {
        let content = "---\ndescription: x\n---\n# Skill\nNo code blocks here.";
        assert_eq!(score(content, &nil_bridge()), 5);
    }

    #[test]
    fn medium_code_blocks_fallback() {
        let content = "---\ndescription: x\n---\n```\nfoo\n```\n```\nbar\n```\n```\nbaz\n```\n";
        assert_eq!(score(content, &nil_bridge()), 7);
    }

    #[test]
    fn library_code_block_count() {
        for (count, want) in [(0, 5), (1, 6), (3, 7), (6, 9)] {
            let b = ValidatorBridge {
                content: Some(ContentReport {
                    code_block_count: count,
                    ..Default::default()
                }),
                structure: None,
            };
            let content = "---\ndescription: x\n---\n# Skill";
            assert_eq!(score(content, &b), want, "count={count}");
        }
    }

    #[test]
    fn library_language_tags() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                code_block_count: 1,
                code_languages: vec!["bash".to_string()],
                ..Default::default()
            }),
            structure: None,
        };
        let content = "---\ndescription: x\n---\n```bash\necho hi\n```\n";
        assert_eq!(score(content, &b), 8);
    }

    #[test]
    fn run_commands() {
        let cases = [
            "Run ./build.sh to start.",
            "npm run build",
            "yarn install",
            "pnpm run test",
            "bun run dev",
            "make install",
            "python main.py",
            "go run ./cmd/main.go",
        ];
        for snippet in cases {
            let content = format!("---\ndescription: x\n---\n# Skill\n{snippet}");
            assert_eq!(score(&content, &nil_bridge()), 9, "snippet={snippet:?}");
        }
    }
}
