//! Collapsed per-skill detail blocks for the Skill Audit pull request comment.

use super::remediation::{dimension_advice, gaps};
use crate::scorer::{grade_rank, Result};

/// How many dimensions each block lists, largest loss first.
const MAX_DIMENSIONS: usize = 3;

const REFERENCE_LINE: &str =
    "- References section is missing, or is not the last section with a link.\n";

/// Render one collapsed block per skill graded below A, worst score first
/// (ties by path), within `budget` bytes.
///
/// Whole blocks are dropped from the end when the budget is exceeded, replaced
/// by one line saying how many were left out. The budget is measured in bytes,
/// which is stricter than GitHub's character limit on a comment. Returns an
/// empty string when no skill is below A.
pub fn pr_detail(results: &[Result], budget: usize) -> String {
    let a = grade_rank("A").unwrap_or(0);
    let mut below: Vec<&Result> = results
        .iter()
        .filter(|r| grade_rank(&r.grade).unwrap_or(0) < a)
        .collect();
    below.sort_by(|x, y| {
        x.total
            .cmp(&y.total)
            .then_with(|| relative_skill_path(&x.skill).cmp(&relative_skill_path(&y.skill)))
    });
    let blocks: Vec<String> = below.into_iter().map(block).collect();
    fit_to_budget(&blocks, budget)
}

fn fit_to_budget(blocks: &[String], budget: usize) -> String {
    let total = blocks.len();
    if total == 0 {
        return String::new();
    }
    for keep in (1..=total).rev() {
        let text = join_with_omitted(&blocks[..keep], total - keep);
        if text.len() <= budget {
            return text;
        }
    }
    join_with_omitted(&[], total)
}

fn join_with_omitted(kept: &[String], omitted: usize) -> String {
    let mut text = kept.join("\n");
    if omitted > 0 {
        if !text.is_empty() {
            text.push('\n');
        }
        let plural = if omitted == 1 { "" } else { "s" };
        text.push_str(&format!(
            "_{omitted} more skill{plural} below A not shown, to keep this comment short._\n"
        ));
    }
    text
}

fn block(r: &Result) -> String {
    let mut lines = String::new();
    for gap in gaps(r).into_iter().take(MAX_DIMENSIONS) {
        match dimension_advice(gap.key) {
            Some(advice) => lines.push_str(&format!(
                "- {} ({}/{}): {advice}\n",
                gap.label, gap.score, gap.max
            )),
            None => lines.push_str(&format!("- {} ({}/{})\n", gap.label, gap.score, gap.max)),
        }
    }
    if !r.reference_section_compliant {
        lines.push_str(REFERENCE_LINE);
    }
    if lines.is_empty() {
        lines.push_str("- No per-dimension detail available.\n");
    }
    format!(
        "<details>\n<summary><code>{}</code> {}, {}/{}</summary>\n\n{lines}\n</details>\n",
        escape(&relative_skill_path(&r.skill)),
        escape(&r.grade),
        r.total,
        r.max_total
    )
}

/// Make text taken from a pull request safe to place in the comment: no raw
/// markup, no mentions, no code-span breaks, and no line breaks.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '`' => out.push_str("&#96;"),
            '@' => out.push_str("&#64;"),
            '\n' | '\r' => out.push(' '),
            other => out.push(other),
        }
    }
    out
}

/// The skill path as the workflow's table shows it: everything up to the last
/// `/skills/` removed, and a trailing `/SKILL.md` removed.
fn relative_skill_path(skill: &str) -> String {
    let rest = skill
        .rsplit_once("/skills/")
        .map_or(skill, |(_, tail)| tail);
    rest.strip_suffix("/SKILL.md").unwrap_or(rest).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::remediation::dimension_advice;
    use super::*;
    use std::collections::BTreeMap;

    const FULL: &[(&str, i32)] = &[
        ("knowledgeDelta", 20),
        ("mindsetProcedures", 15),
        ("antiPatternQuality", 15),
        ("specificationCompliance", 15),
        ("progressiveDisclosure", 15),
        ("freedomCalibration", 15),
        ("patternRecognition", 10),
        ("practicalUsability", 15),
        ("evalValidation", 20),
    ];

    /// A result whose dimensions are the maximums, lowered by `lost`.
    fn result(skill: &str, grade: &str, total: i32, lost: &[(&str, i32)]) -> Result {
        let dimensions: BTreeMap<String, i32> = FULL
            .iter()
            .map(|(key, max)| {
                let drop = lost.iter().find(|(k, _)| k == key).map_or(0, |(_, d)| *d);
                (key.to_string(), max - drop)
            })
            .collect();
        Result {
            skill: skill.to_string(),
            date: String::new(),
            dimensions,
            total,
            max_total: 140,
            grade: grade.to_string(),
            lines: 0,
            has_references: true,
            reference_count: 1,
            reference_section_compliant: true,
            errors: 0,
            warnings: 0,
            error_details: Vec::new(),
            warning_details: Vec::new(),
        }
    }

    fn b_skill(path: &str) -> Result {
        result(
            path,
            "B",
            115,
            &[
                ("antiPatternQuality", 6),
                ("progressiveDisclosure", 5),
                ("mindsetProcedures", 4),
                ("practicalUsability", 4),
            ],
        )
    }

    fn count(haystack: &str, needle: &str) -> usize {
        haystack.matches(needle).count()
    }

    #[test]
    fn all_a_or_better_gives_an_empty_string() {
        let results = vec![
            result("skills/a/one/SKILL.md", "A", 126, &[]),
            result("skills/a/two/SKILL.md", "A+", 140, &[]),
        ];
        assert_eq!(pr_detail(&results, 40_000), "");
    }

    #[test]
    fn an_empty_batch_gives_an_empty_string() {
        assert_eq!(pr_detail(&[], 40_000), "");
    }

    #[test]
    fn a_b_skill_shows_its_three_biggest_losses_with_advice() {
        let out = pr_detail(
            &[b_skill(
                "/home/runner/work/tekhne/tekhne/skills/ci-cd/fluentbit/generator/SKILL.md",
            )],
            40_000,
        );
        let line = |label: &str, key: &str, score: i32, max: i32| {
            format!(
                "- {label} ({score}/{max}): {}\n",
                dimension_advice(key).expect("advice for known key")
            )
        };
        let expected = format!(
            "<details>\n<summary><code>ci-cd/fluentbit/generator</code> B, 115/140</summary>\n\n{}{}{}\n</details>\n",
            line("Anti-Pattern Quality", "antiPatternQuality", 9, 15),
            line("Progressive Disclosure", "progressiveDisclosure", 10, 15),
            line("Mindset + Procedures", "mindsetProcedures", 11, 15),
        );
        assert_eq!(out, expected);
        assert!(!out.contains("Practical Usability"), "fourth loss is cut");
    }

    #[test]
    fn a_skill_with_fewer_than_three_losses_lists_only_those() {
        let r = result(
            "skills/x/y/SKILL.md",
            "B+",
            121,
            &[("antiPatternQuality", 3), ("evalValidation", 2)],
        );
        let out = pr_detail(&[r], 40_000);
        assert_eq!(count(&out, "\n- "), 2);
        assert!(out.contains("Anti-Pattern Quality (12/15)"));
        assert!(out.contains("Eval Validation (18/20)"));
    }

    #[test]
    fn the_reference_line_appears_only_when_the_section_is_not_compliant() {
        let mut bad = b_skill("skills/x/bad/SKILL.md");
        bad.reference_section_compliant = false;
        let good = b_skill("skills/x/good/SKILL.md");
        let line = "- References section is missing, or is not the last section with a link.";
        assert!(pr_detail(&[bad], 40_000).contains(line));
        assert!(!pr_detail(&[good], 40_000).contains(line));
    }

    #[test]
    fn text_taken_from_the_skill_path_is_escaped() {
        let r = b_skill("x/<b>&`@team\nnext/SKILL.md");
        let out = pr_detail(&[r], 40_000);
        assert!(
            out.contains("<code>x/&lt;b&gt;&amp;&#96;&#64;team next</code>"),
            "{out}"
        );
        assert!(!out.contains("<b>"));
        assert!(!out.contains("@team"));
    }

    #[test]
    fn skills_are_ordered_by_score_then_path() {
        let mut low = b_skill("skills/z/low/SKILL.md");
        low.total = 112;
        let mut tie_b = b_skill("skills/m/tie-b/SKILL.md");
        tie_b.total = 118;
        let mut tie_a = b_skill("skills/a/tie-a/SKILL.md");
        tie_a.total = 118;
        let out = pr_detail(&[tie_b, low, tie_a], 40_000);
        let pos = |s: &str| out.find(s).unwrap_or_else(|| panic!("{s} missing"));
        assert!(pos("z/low") < pos("a/tie-a"));
        assert!(pos("a/tie-a") < pos("m/tie-b"));
    }

    #[test]
    fn output_is_the_same_whatever_the_input_order() {
        let a = b_skill("skills/a/one/SKILL.md");
        let b = b_skill("skills/b/two/SKILL.md");
        assert_eq!(
            pr_detail(&[a.clone(), b.clone()], 40_000),
            pr_detail(&[b, a], 40_000)
        );
    }

    #[test]
    fn whole_blocks_are_dropped_from_the_end_when_over_budget() {
        let results: Vec<Result> = (0..5)
            .map(|i| b_skill(&format!("skills/d/skill-{i}/SKILL.md")))
            .collect();
        let one = pr_detail(&results[..1], usize::MAX).len();
        // Room for two blocks plus the omitted-count line, but not three.
        let budget = one * 2 + 120;
        let out = pr_detail(&results, budget);
        assert!(out.len() <= budget, "{} > {budget}", out.len());
        assert_eq!(count(&out, "<details>"), 2);
        assert_eq!(count(&out, "</details>"), 2);
        assert!(out.contains("3 more skills below A not shown"), "{out}");
    }

    #[test]
    fn a_first_block_larger_than_the_budget_leaves_only_the_omitted_line() {
        let results = vec![
            b_skill("skills/d/one/SKILL.md"),
            b_skill("skills/d/two/SKILL.md"),
        ];
        let out = pr_detail(&results, 50);
        assert_eq!(count(&out, "<details>"), 0);
        assert!(out.contains("2 more skills below A not shown"), "{out}");
    }

    #[test]
    fn one_omitted_skill_is_worded_in_the_singular() {
        let results = vec![
            b_skill("skills/d/one/SKILL.md"),
            b_skill("skills/d/two/SKILL.md"),
        ];
        let one = pr_detail(&results[..1], usize::MAX).len();
        let out = pr_detail(&results, one + 120);
        assert!(out.contains("1 more skill below A not shown"), "{out}");
    }

    #[test]
    fn a_generous_budget_keeps_every_block_and_adds_no_omitted_line() {
        let results: Vec<Result> = (0..4)
            .map(|i| b_skill(&format!("skills/d/skill-{i}/SKILL.md")))
            .collect();
        let out = pr_detail(&results, usize::MAX);
        assert_eq!(count(&out, "<details>"), 4);
        assert!(!out.contains("not shown"));
    }

    #[test]
    fn the_path_is_made_relative_to_the_skills_directory() {
        assert_eq!(
            relative_skill_path("/home/runner/work/skills/skills/ci-cd/helm/validator/SKILL.md"),
            "ci-cd/helm/validator"
        );
        assert_eq!(
            relative_skill_path("ci-cd/helm/validator"),
            "ci-cd/helm/validator"
        );
        assert_eq!(relative_skill_path("a/b/SKILL.md"), "a/b");
    }
}
