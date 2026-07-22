//! Plain-text report rendering (Go `reporter.Format`).

use super::DIMENSION_ORDER;
use crate::scorer::Result;
use std::fmt::Write;

/// Return a human-readable representation of a [`Result`] (Go `Format`).
pub fn format(r: &Result) -> String {
    let mut sb = String::new();

    let _ = writeln!(sb, "Skill: {}", r.skill);
    let _ = writeln!(sb, "Grade: {} ({}/{})", r.grade, r.total, r.max_total);
    let _ = writeln!(sb, "\nDimensions:");

    for (key, label, max) in DIMENSION_ORDER {
        let Some(score) = r.dimensions.get(*key) else {
            continue;
        };
        let _ = writeln!(sb, "  {label:<28} {score:>2}/{max}");
    }

    let _ = writeln!(sb, "\nErrors: {}  Warnings: {}", r.errors, r.warnings);

    if !r.error_details.is_empty() || !r.warning_details.is_empty() {
        let _ = writeln!(sb, "\nDiagnostics:");
        for d in &r.error_details {
            let _ = writeln!(sb, "  [E] {:<3} {}", d.dimension, d.message);
        }
        for d in &r.warning_details {
            let _ = writeln!(sb, "  [W] {:<3} {}", d.dimension, d.message);
        }
    }

    sb
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scorer::Result;
    use std::collections::BTreeMap;

    fn make_result() -> Result {
        Result {
            skill: "agentic-harness/skill-quality-auditor".to_string(),
            date: String::new(),
            total: 122,
            max_total: 140,
            grade: "B+".to_string(),
            lines: 0,
            has_references: false,
            reference_count: 0,
            reference_section_compliant: false,
            dimensions: BTreeMap::from([
                ("knowledgeDelta".to_string(), 14),
                ("mindsetProcedures".to_string(), 12),
                ("antiPatternQuality".to_string(), 9),
                ("specificationCompliance".to_string(), 13),
                ("progressiveDisclosure".to_string(), 15),
                ("freedomCalibration".to_string(), 12),
                ("patternRecognition".to_string(), 9),
                ("practicalUsability".to_string(), 13),
                ("evalValidation".to_string(), 15),
            ]),
            errors: 0,
            warnings: 0,
            error_details: Vec::new(),
            warning_details: Vec::new(),
        }
    }

    #[test]
    fn format_basic() {
        let out = format(&make_result());
        for want in [
            "Skill: agentic-harness/skill-quality-auditor",
            "Grade: B+ (122/140)",
            "Knowledge Delta",
            "Mindset + Procedures",
            "Anti-Pattern Quality",
            "Specification Compliance",
            "Progressive Disclosure",
            "Freedom Calibration",
            "Pattern Recognition",
            "Practical Usability",
            "Eval Validation",
            "Errors: 0  Warnings: 0",
        ] {
            assert!(out.contains(want), "missing {want:?}\n{out}");
        }
    }

    #[test]
    fn format_dimension_order() {
        let out = format(&make_result());
        let labels = [
            "Knowledge Delta",
            "Mindset + Procedures",
            "Anti-Pattern Quality",
            "Specification Compliance",
            "Progressive Disclosure",
            "Freedom Calibration",
            "Pattern Recognition",
            "Practical Usability",
            "Eval Validation",
        ];
        let mut pos: isize = -1;
        for label in labels {
            let idx = out.find(label).expect("label present") as isize;
            assert!(idx > pos, "{label:?} out of order");
            pos = idx;
        }
    }
}
