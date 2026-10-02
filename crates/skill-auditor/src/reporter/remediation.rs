//! Markdown remediation plan (Go `reporter.Remediation`). Output strings,
//! including the emoji prefixes and heading punctuation, are reproduced
//! verbatim from the Go source for downstream parity.

use super::DIMENSION_ORDER;
use crate::scorer::{Diagnostic, Result};
use std::collections::HashMap;
use std::fmt::Write;

/// Generic per-dimension improvement advice, keyed by camelCase dimension name
/// (Go `dimensionAdvice`).
fn dimension_advice(key: &str) -> Option<&'static str> {
    match key {
        "knowledgeDelta" => Some("Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world)."),
        "mindsetProcedures" => Some("Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections."),
        "antiPatternQuality" => Some("Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples."),
        "specificationCompliance" => Some("Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks."),
        "progressiveDisclosure" => Some("Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score."),
        "freedomCalibration" => Some("Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may)."),
        "patternRecognition" => Some("Expand the `description` frontmatter field to more than 15 qualifying words (words longer than 3 characters)."),
        "practicalUsability" => Some("Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash, ```typescript)."),
        "evalValidation" => Some("Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`."),
        _ => None,
    }
}

/// Map a dimension display label to its D-code for diagnostic lookup (Go
/// `dimLabelToCode`).
fn dim_label_to_code(label: &str) -> &'static str {
    match label {
        "Knowledge Delta" => "D1",
        "Mindset + Procedures" => "D2",
        "Anti-Pattern Quality" => "D3",
        "Specification Compliance" => "D4",
        "Progressive Disclosure" => "D5",
        "Freedom Calibration" => "D6",
        "Pattern Recognition" => "D7",
        "Practical Usability" => "D8",
        "Eval Validation" => "D9",
        _ => "",
    }
}

pub(super) struct Gap {
    pub(super) key: &'static str,
    pub(super) label: &'static str,
    pub(super) score: i32,
    pub(super) max: i32,
}

/// The dimensions of `r` that scored below their maximum, largest shortfall
/// first. Built in dimension order and sorted stably, so ties keep that order.
pub(super) fn gaps(r: &Result) -> Vec<Gap> {
    let mut gaps: Vec<Gap> = Vec::new();
    for (key, label, max) in DIMENSION_ORDER {
        let Some(&score) = r.dimensions.get(*key) else {
            continue;
        };
        if score < *max {
            gaps.push(Gap {
                key,
                label,
                score,
                max: *max,
            });
        }
    }
    gaps.sort_by_key(|g| std::cmp::Reverse(g.max - g.score));
    gaps
}

/// Return a markdown prioritised action plan derived from a [`Result`] (Go
/// `Remediation`).
pub fn remediation(r: &Result) -> String {
    let mut sb = String::new();

    let _ = write!(sb, "# Remediation Plan \u{2014} {}\n\n", r.skill);
    let _ = write!(
        sb,
        "**Current Grade:** {} ({}/{})\n\n",
        r.grade, r.total, r.max_total
    );

    let gaps = gaps(r);

    if gaps.is_empty() {
        let _ = writeln!(
            sb,
            "All dimensions are at maximum score. Nothing to remediate."
        );
        return sb;
    }

    let _ = write!(sb, "## Priority Actions\n\n");

    // Collect diagnostics keyed by D-code.
    let mut diags_by_dim: HashMap<&str, Vec<&Diagnostic>> = HashMap::new();
    for d in r.error_details.iter().chain(r.warning_details.iter()) {
        diags_by_dim
            .entry(d.dimension.as_str())
            .or_default()
            .push(d);
    }

    for g in &gaps {
        let available = g.max - g.score;
        let _ = write!(
            sb,
            "### {} ({}/{}) \u{2014} {} pt{} available\n\n",
            g.label,
            g.score,
            g.max,
            available,
            plural(available)
        );

        let dim_key = dim_label_to_code(g.label);
        if let Some(diags) = diags_by_dim.get(dim_key) {
            for d in diags {
                let prefix = if d.severity() == "error" {
                    "\u{1f534}"
                } else {
                    "\u{26a0}\u{fe0f}"
                };
                let _ = write!(sb, "{} {}\n\n", prefix, d.message);
            }
        }

        if let Some(advice) = dimension_advice(g.key) {
            let _ = write!(sb, "{advice}\n\n");
        }
    }

    sb
}

fn plural(n: i32) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn fixture() -> Result {
        Result {
            skill: "ci-cd/example/generator".to_string(),
            date: String::new(),
            total: 115,
            max_total: 140,
            grade: "B".to_string(),
            lines: 249,
            has_references: true,
            reference_count: 3,
            reference_section_compliant: false,
            dimensions: BTreeMap::from([
                ("knowledgeDelta".to_string(), 20),
                ("mindsetProcedures".to_string(), 11),
                ("antiPatternQuality".to_string(), 9),
                ("specificationCompliance".to_string(), 14),
                ("progressiveDisclosure".to_string(), 10),
                ("freedomCalibration".to_string(), 13),
                ("patternRecognition".to_string(), 10),
                ("practicalUsability".to_string(), 11),
                ("evalValidation".to_string(), 17),
            ]),
            errors: 1,
            warnings: 1,
            error_details: vec![Diagnostic {
                dimension: "D3".to_string(),
                message: "missing WHY".to_string(),
                severity: "error".to_string(),
            }],
            warning_details: vec![Diagnostic {
                dimension: "D5".to_string(),
                message: "no references".to_string(),
                severity: "warning".to_string(),
            }],
        }
    }

    /// Pins the exact output so the gap/advice code can be shared with other
    /// renderers without changing what `remediation` prints. `{D}`, `{R}` and
    /// `{W}` stand for the em dash, red circle and warning sign, which the
    /// source writes as escapes.
    #[test]
    fn remediation_output_is_pinned() {
        let expected = r####"# Remediation Plan {D} ci-cd/example/generator

**Current Grade:** B (115/140)

## Priority Actions

### Anti-Pattern Quality (9/15) {D} 6 pts available

{R} missing WHY

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Progressive Disclosure (10/15) {D} 5 pts available

{W} no references

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Mindset + Procedures (11/15) {D} 4 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Practical Usability (11/15) {D} 4 pts available

Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash, ```typescript).

### Eval Validation (17/20) {D} 3 pts available

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Freedom Calibration (13/15) {D} 2 pts available

Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).

### Specification Compliance (14/15) {D} 1 pt available

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

"####
        .replace("{D}", "\u{2014}")
        .replace("{R}", "\u{1f534}")
        .replace("{W}", "\u{26a0}\u{fe0f}");
        assert_eq!(remediation(&fixture()), expected);
    }
}
