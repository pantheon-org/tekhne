//! D7 — Pattern Recognition (max 10). Ported from `d7_pattern_recognition.go`.
//!
//! Uses the library description length (chars) from the frontmatter check,
//! mapped to score bands, and falls back to a fixed score when the library
//! result is unavailable.

use super::bridge::ValidatorBridge;
use super::{warn_diag, Diagnostic};

/// Score the Pattern Recognition dimension.
pub fn score(b: &ValidatorBridge) -> (i32, Vec<Diagnostic>) {
    let mut diags = Vec::new();

    let desc_len = b.description_len();
    if desc_len >= 0 {
        if desc_len > 200 {
            return (10, diags);
        } else if desc_len > 120 {
            return (9, diags);
        } else if desc_len > 60 {
            return (8, diags);
        } else {
            if desc_len <= 30 {
                // Message reproduced verbatim from the Go source for output
                // parity (the character it contains is load-bearing data).
                diags.push(warn_diag(
                    "D7",
                    &format!(
                        "description is only {desc_len} chars \u{2014} aim for >60 for a useful pattern signal"
                    ),
                ));
            }
            return (6, diags);
        }
    }

    if b.structure.is_none() {
        diags.push(warn_diag(
            "D7",
            "description length unavailable (validator bridge failed)",
        ));
        return (6, diags);
    }
    (6, diags)
}

#[cfg(test)]
mod tests {
    use super::super::bridge::ValidatorBridge;
    use super::*;
    use skill_validator_rs::{Level, Report, ValidationResult};

    fn nil_bridge() -> ValidatorBridge {
        ValidatorBridge::default()
    }

    fn bridge_with_desc(chars: usize) -> ValidatorBridge {
        ValidatorBridge {
            content: None,
            structure: Some(Report {
                results: vec![ValidationResult {
                    level: Level::Pass,
                    category: "Frontmatter".to_string(),
                    message: format!("description: ({chars} chars)"),
                    file: String::new(),
                    line: 0,
                }],
                ..Default::default()
            }),
        }
    }

    #[test]
    fn very_long_description() {
        let (score, diags) = score(&bridge_with_desc(210));
        assert_eq!(score, 10);
        assert!(diags.is_empty());
    }

    #[test]
    fn medium_description() {
        let (score, diags) = score(&bridge_with_desc(150));
        assert_eq!(score, 9);
        assert!(diags.is_empty());
    }

    #[test]
    fn short_description() {
        let (score, diags) = score(&bridge_with_desc(80));
        assert_eq!(score, 8);
        assert!(diags.is_empty());
    }

    #[test]
    fn very_short_description() {
        let (score, diags) = score(&bridge_with_desc(20));
        assert_eq!(score, 6);
        assert!(!diags.is_empty());
    }

    #[test]
    fn fallback_no_bridge() {
        let (score, diags) = score(&nil_bridge());
        assert_eq!(score, 6);
        assert!(!diags.is_empty());
    }

    #[test]
    fn desc_len_31_to_60() {
        let (score, diags) = score(&bridge_with_desc(45));
        assert_eq!(score, 6);
        assert!(diags.is_empty());
    }
}
