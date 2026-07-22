//! D6 — Freedom Calibration (max 15). Ported from `d6_freedom_calibration.go`.
//!
//! `score = round(InstructionSpecificity * 15)` with no baseline. Zero markers
//! means no directive language, which is a genuine quality gap, so it scores 0.

use super::bridge::ValidatorBridge;

/// Score the Freedom Calibration dimension.
pub fn score(b: &ValidatorBridge) -> i32 {
    let Some(c) = &b.content else {
        return 0;
    };
    if c.strong_markers + c.weak_markers == 0 {
        return 0;
    }
    let score = (c.instruction_specificity * 15.0).round() as i32;
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
    fn high_specificity() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                strong_markers: 6,
                weak_markers: 0,
                instruction_specificity: 1.0,
                ..Default::default()
            }),
            structure: None,
        };
        assert_eq!(score(&b), 15);
    }

    #[test]
    fn mixed_specificity() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                strong_markers: 3,
                weak_markers: 2,
                instruction_specificity: 0.6,
                ..Default::default()
            }),
            structure: None,
        };
        assert_eq!(score(&b), (0.6f64 * 15.0).round() as i32);
    }

    #[test]
    fn zero_markers() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                strong_markers: 0,
                weak_markers: 0,
                instruction_specificity: 1.0,
                ..Default::default()
            }),
            structure: None,
        };
        assert_eq!(score(&b), 0);
    }

    #[test]
    fn nil_content() {
        assert_eq!(score(&nil_bridge()), 0);
    }

    #[test]
    fn low_specificity() {
        let b = ValidatorBridge {
            content: Some(ContentReport {
                strong_markers: 1,
                weak_markers: 4,
                instruction_specificity: 0.2,
                ..Default::default()
            }),
            structure: None,
        };
        assert_eq!(score(&b), (0.2f64 * 15.0).round() as i32);
    }
}
