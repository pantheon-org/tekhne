//! D2 — Mindset + Procedures (max 15). Ported from `d2_mindset_procedures.go`.

use super::bridge::ValidatorBridge;
use super::helpers::{count_pattern, matches_regex};

/// Score the Mindset + Procedures dimension.
pub fn score(content: &str, b: &ValidatorBridge) -> i32 {
    let mut score = 0;

    if matches_regex(content, r"(?im)##\s*(mindset|philosophy|principles)") {
        score += 2;
    }

    if let Some(c) = &b.content {
        if c.imperative_ratio >= 0.4 {
            score += 4;
        } else if c.imperative_ratio >= 0.25 {
            score += 3;
        } else if c.imperative_ratio >= 0.1 {
            score += 2;
        }
        if c.list_item_count > 3 {
            score += 2;
        } else if c.list_item_count > 0 {
            score += 1;
        }
    } else if matches_regex(content, r"(?m)^\s*[0-9]+\.") {
        score += 2;
    }

    if count_pattern(content, "when to use") > 0 || count_pattern(content, "when to apply") > 0 {
        score += 4;
    }
    if count_pattern(content, "when not to") > 0 {
        score += 3;
    }

    if score > 15 {
        score = 15;
    }
    score
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
    fn when_to_use() {
        let content = "---\ndescription: x\n---\n## When to Use\nuse it when needed\n\n## When NOT to Use\nnot this time";
        assert_eq!(score(content, &nil_bridge()), 7);
    }

    #[test]
    fn mindset_heading() {
        let content = "---\ndescription: x\n---\n## Mindset\nthink carefully";
        assert_eq!(score(content, &nil_bridge()), 2);
    }

    #[test]
    fn philosophy_heading() {
        let content = "---\ndescription: x\n---\n## Philosophy\nthink this way";
        assert_eq!(score(content, &nil_bridge()), 2);
    }

    #[test]
    fn library_imperative_ratio() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                imperative_ratio: 0.45,
                list_item_count: 5,
                ..Default::default()
            }),
            structure: None,
        };
        let content = "---\ndescription: x\n---\n## When to Use\ndo this";
        assert_eq!(score(content, &b), 10);
    }
}
