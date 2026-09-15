//! Scoring an ADR's prose for completeness, 0 to 100.
//!
//! The rubric answers one question a hook can act on: is this record finished?
//! Five weighted rules score the body, three caps punish the failures that make
//! a record useless regardless of what else is filled in, and a boilerplate
//! filter rejects sections that contain words but say nothing.
//!
//! Scoring reads the body only. Frontmatter is structured data and is validated
//! by parsing, not by counting words.

use std::collections::BTreeMap;

use serde::Serialize;

/// The score at or above which a record may be sent for review.
pub const PASS_THRESHOLD: u32 = 80;

/// The score `--strict` demands instead.
pub const STRICT_THRESHOLD: u32 = 95;

/// Words that, in bulk, mean a section was never really written.
const BOILERPLATE: [&str; 7] = [
    "describe",
    "decision",
    "context",
    "placeholder",
    "example",
    "tbd",
    "todo",
];

/// The proportion of boilerplate words above which a section counts as unfilled.
const BOILERPLATE_RATIO: f64 = 0.6;

/// One rule's verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Item {
    /// The rule's name, as shown in output.
    pub name: &'static str,
    /// The points the rule is worth.
    pub weight: u32,
    /// Whether the rule passed.
    pub passing: bool,
    /// Why it failed, when it did.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// The outcome of scoring one record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Report {
    /// The final score after caps, 0 to 100.
    pub score: u32,
    /// Whether the score meets the threshold it was checked against.
    pub pass: bool,
    /// Rules that failed, in rubric order.
    pub missing: Vec<Item>,
    /// Rules that passed, in rubric order.
    pub filled: Vec<Item>,
    /// Human-readable descriptions of every cap that bit.
    pub caps_applied: Vec<String>,
}

/// Score `body` against the rubric, passing at `threshold`.
pub fn check(body: &str, threshold: u32) -> Report {
    let sections = parse_sections(body);
    let placeholders = count_placeholders(body);

    let (problem_ok, problem_words) =
        section_check(&sections, &["Problem Statement", "Context"], 30);
    let (solution_ok, solution_words) = section_check(&sections, &["Chosen Solution"], 30);
    let (rationale_ok, _) = section_check(&sections, &["Rationale"], 20);

    // A section with words in it still fails when those words are boilerplate.
    let problem_ok = problem_ok && !is_boilerplate(&sections, &["Problem Statement", "Context"]);
    let solution_ok = solution_ok && !is_boilerplate(&sections, &["Chosen Solution"]);
    let rationale_ok = rationale_ok && !is_boilerplate(&sections, &["Rationale"]);
    let impact_ok = impact_check(&sections);

    let mut score = 0;
    let mut missing = Vec::new();
    let mut filled = Vec::new();

    let mut rule = |name, weight, passing, note: Option<String>| {
        let item = Item {
            name,
            weight,
            passing,
            note,
        };
        if passing {
            score += weight;
            filled.push(item);
        } else {
            missing.push(item);
        }
    };

    rule(
        "No unfilled placeholders",
        20,
        placeholders == 0 && !sections.is_empty(),
        Some(if placeholders > 0 {
            format!("{placeholders} unfilled placeholder(s) remain")
        } else {
            "no section headings found (empty body)".to_string()
        }),
    );
    rule(
        "Problem Statement / Context",
        25,
        problem_ok,
        Some(thin_note(problem_words)),
    );
    rule(
        "Chosen Solution",
        25,
        solution_ok,
        Some(thin_note(solution_words)),
    );
    rule(
        "Rationale",
        20,
        rationale_ok,
        Some("section is empty, too short, or contains only boilerplate".to_string()),
    );
    rule(
        "Impact Assessment",
        10,
        impact_ok,
        Some("no non-\"none\" entry found".to_string()),
    );

    // Caps: the lowest triggered cap wins. Each names a failure that makes the
    // record unusable however well the remaining sections score.
    let mut cap = 100;
    let mut caps_applied = Vec::new();
    let apply = |limit: u32, reason: &str, cap: &mut u32, caps: &mut Vec<String>| {
        if *cap > limit {
            *cap = limit;
            caps.push(reason.to_string());
        }
    };
    if !problem_ok {
        apply(
            60,
            "Problem Statement thin/absent: score capped at 60",
            &mut cap,
            &mut caps_applied,
        );
    }
    if !solution_ok {
        apply(
            60,
            "Chosen Solution thin/absent: score capped at 60",
            &mut cap,
            &mut caps_applied,
        );
    }
    if placeholders >= 3 {
        apply(
            70,
            "3 or more unfilled placeholders: score capped at 70",
            &mut cap,
            &mut caps_applied,
        );
    }

    let score = score.min(cap);
    Report {
        score,
        pass: score >= threshold,
        missing,
        filled,
        caps_applied,
    }
}

/// The note explaining why a word-counted rule failed.
fn thin_note(words: usize) -> String {
    if words == 0 {
        "section is empty or absent".to_string()
    } else {
        "section is too short or contains only boilerplate".to_string()
    }
}

/// Split markdown into section name to body text.
///
/// Both `##` and `###` delimit sections, so `### Context` nested under
/// `## Problem Statement` is scored on its own content rather than being
/// shadowed by its empty parent heading.
fn parse_sections(content: &str) -> BTreeMap<String, String> {
    let mut sections = BTreeMap::new();
    let mut current: Option<String> = None;
    let mut body = String::new();

    for line in content.lines() {
        if let Some(name) = heading_name(line) {
            if let Some(key) = current.take() {
                sections.insert(key, std::mem::take(&mut body));
            }
            current = Some(name);
        } else if current.is_some() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(key) = current {
        sections.insert(key, body);
    }
    sections
}

/// The trimmed text of a `##` or `###` heading, if `line` is one.
fn heading_name(line: &str) -> Option<String> {
    for prefix in ["### ", "## "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

/// Whether any section matching one of `names` holds at least `threshold` words.
///
/// Names are tried in order and the richest matching section wins, so an empty
/// parent heading never masks a filled child. The returned count is the best
/// word count seen, which distinguishes "absent" from "too short" in the note.
fn section_check(
    sections: &BTreeMap<String, String>,
    names: &[&str],
    threshold: usize,
) -> (bool, usize) {
    let mut best = 0;
    for name in names {
        let words = sections
            .iter()
            .filter(|(sec, _)| sec.contains(name))
            .map(|(_, body)| count_words(&strip_comments(body)))
            .max()
            .unwrap_or(0);
        best = best.max(words);
        if words >= threshold {
            return (true, words);
        }
    }
    (false, best)
}

/// Whether Impact Assessment has at least one bullet whose value is real.
///
/// A record that declares every impact `none` has not assessed anything, so it
/// scores as unfilled.
fn impact_check(sections: &BTreeMap<String, String>) -> bool {
    sections
        .iter()
        .filter(|(sec, _)| sec.contains("Impact"))
        .flat_map(|(_, body)| body.lines())
        .filter(|line| line.trim_start().starts_with('-'))
        .filter_map(|line| {
            let cleaned = strip_comments(line);
            let (_, value) = cleaned.rsplit_once(':')?;
            Some(value.trim().to_ascii_lowercase())
        })
        .any(|value| !value.is_empty() && value != "none")
}

/// Whether the richest section matching `names` is mostly boilerplate words.
fn is_boilerplate(sections: &BTreeMap<String, String>, names: &[&str]) -> bool {
    let best = names
        .iter()
        .flat_map(|name| sections.iter().filter(move |(sec, _)| sec.contains(name)))
        .map(|(_, body)| strip_comments(body))
        .max_by_key(|body| count_words(body));

    let Some(body) = best else { return false };
    let tokens: Vec<&str> = body.split_whitespace().collect();
    if tokens.is_empty() {
        return false;
    }
    let matches = tokens
        .iter()
        .map(|t| {
            t.trim_matches(|c: char| ".,;:!?\"'()".contains(c))
                .to_ascii_lowercase()
        })
        .filter(|t| BOILERPLATE.contains(&t.as_str()))
        .count();
    matches as f64 / tokens.len() as f64 > BOILERPLATE_RATIO
}

/// Remove every `<!-- ... -->` comment from `text`.
///
/// Written by hand rather than with a regex so the crate keeps its small
/// dependency set; comments do not nest in markdown, so a scan suffices.
fn strip_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        match rest[start..].find("-->") {
            Some(end) => rest = &rest[start + end + 3..],
            // An unterminated comment swallows the remainder, matching how a
            // markdown renderer treats it.
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

/// The number of `<!-- ... -->` comments in `text`.
fn count_placeholders(text: &str) -> usize {
    let mut count = 0;
    let mut rest = text;
    while let Some(start) = rest.find("<!--") {
        match rest[start..].find("-->") {
            Some(end) => {
                count += 1;
                rest = &rest[start + end + 3..];
            }
            None => break,
        }
    }
    count
}

/// Whitespace-separated word count.
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{BranchType, Record};

    /// Thirty-plus words of real prose, enough to satisfy the heaviest rules.
    const PROSE: &str = "Requests cross four services and we cannot follow a single one end to \
        end because each service writes its own log format with no shared correlation identifier, \
        so reconstructing a slow request means grepping four log stores by timestamp and guessing.";

    fn filled_body() -> String {
        format!(
            "# Adopt OpenTelemetry\n\n\
             ## Problem Statement\n\n### Context\n{PROSE}\n\n\
             ## Decision Record\n\n### Options Considered\nA vendor agent, or OTLP.\n\n\
             ### Chosen Solution\n{PROSE}\n\n\
             ### Rationale\n{PROSE}\n\n\
             ## Impact Assessment\n\n- **Performance**: about 1% CPU at 10% sampling\n\
             - **Security**: attributes scrubbed at the collector\n"
        )
    }

    #[test]
    fn a_fully_written_record_scores_one_hundred() {
        let report = check(&filled_body(), PASS_THRESHOLD);
        assert_eq!(report.score, 100, "{:?}", report.missing);
        assert!(report.pass);
        assert!(report.caps_applied.is_empty());
        assert_eq!(report.filled.len(), 5);
    }

    #[test]
    fn the_untouched_template_scores_zero() {
        let record = Record::new("x", "X", BranchType::Feat, "A", "2026-09-15", None);
        let report = check(&record.body, PASS_THRESHOLD);

        assert_eq!(report.score, 0);
        assert!(!report.pass);
        assert_eq!(report.missing.len(), 5);
        assert!(
            report
                .missing
                .iter()
                .any(|m| m.note.as_deref().is_some_and(|n| n.contains("placeholder"))),
            "{:?}",
            report.missing
        );
    }

    #[test]
    fn weights_sum_to_one_hundred() {
        let report = check(&filled_body(), PASS_THRESHOLD);
        let total: u32 = report.filled.iter().map(|i| i.weight).sum();
        assert_eq!(total, 100);
    }

    #[test]
    fn a_thin_problem_statement_caps_the_score_at_sixty() {
        let body = filled_body().replace(&format!("### Context\n{PROSE}"), "### Context\nShort.");
        let report = check(&body, PASS_THRESHOLD);

        assert_eq!(report.score, 60);
        assert!(report
            .caps_applied
            .iter()
            .any(|c| c.contains("Problem Statement")));
    }

    #[test]
    fn a_thin_chosen_solution_caps_the_score_at_sixty() {
        let body = filled_body().replace(
            &format!("### Chosen Solution\n{PROSE}"),
            "### Chosen Solution\nWe did it.",
        );
        let report = check(&body, PASS_THRESHOLD);

        assert_eq!(report.score, 60);
        assert!(report
            .caps_applied
            .iter()
            .any(|c| c.contains("Chosen Solution")));
    }

    #[test]
    fn three_placeholders_cap_the_score_at_seventy() {
        let body = format!(
            "{}\n## Risks & Pitfalls\n<!-- a -->\n<!-- b -->\n<!-- c -->\n",
            filled_body()
        );
        let report = check(&body, PASS_THRESHOLD);

        assert_eq!(report.score, 70);
        assert!(report.caps_applied.iter().any(|c| c.contains("3 or more")));
    }

    #[test]
    fn one_placeholder_costs_its_weight_without_capping() {
        let body = format!("{}\n## Outcome & Lessons\n<!-- later -->\n", filled_body());
        let report = check(&body, PASS_THRESHOLD);

        assert_eq!(report.score, 80);
        assert!(report.caps_applied.is_empty());
    }

    #[test]
    fn boilerplate_prose_scores_as_unfilled() {
        let filler = "describe the decision context placeholder example tbd todo ".repeat(6);
        let body = filled_body().replace(
            &format!("### Chosen Solution\n{PROSE}"),
            &format!("### Chosen Solution\n{filler}"),
        );
        let report = check(&body, PASS_THRESHOLD);

        assert_eq!(report.score, 60);
        assert!(report
            .caps_applied
            .iter()
            .any(|c| c.contains("Chosen Solution")));
    }

    #[test]
    fn an_all_none_impact_assessment_fails_its_rule() {
        let body = filled_body().replace(
            "- **Performance**: about 1% CPU at 10% sampling\n- **Security**: attributes scrubbed at the collector\n",
            "- **Performance**: none\n- **Security**: none\n",
        );
        let report = check(&body, PASS_THRESHOLD);

        assert_eq!(report.score, 90);
        assert!(report.missing.iter().any(|m| m.name == "Impact Assessment"));
    }

    #[test]
    fn a_child_heading_is_not_shadowed_by_an_empty_parent() {
        let body = format!(
            "## Problem Statement\n\n### Context\n{PROSE}\n\n### Chosen Solution\n{PROSE}\n\n\
             ### Rationale\n{PROSE}\n\n## Impact Assessment\n- **Security**: real\n"
        );
        let report = check(&body, PASS_THRESHOLD);
        assert_eq!(report.score, 100, "{:?}", report.missing);
    }

    #[test]
    fn comments_do_not_count_towards_word_totals() {
        let padding = "<!-- word word word word word word word word word word -->".repeat(5);
        let body = format!(
            "## Problem Statement\n### Context\n{padding}\n\n### Chosen Solution\n{PROSE}\n\n\
             ### Rationale\n{PROSE}\n\n## Impact Assessment\n- **Security**: real\n"
        );
        let report = check(&body, PASS_THRESHOLD);
        assert!(report
            .missing
            .iter()
            .any(|m| m.name == "Problem Statement / Context"));
    }

    #[test]
    fn strict_threshold_rejects_a_record_that_passes_normally() {
        let body = format!("{}\n## Outcome & Lessons\n<!-- later -->\n", filled_body());
        assert!(check(&body, PASS_THRESHOLD).pass);
        assert!(!check(&body, STRICT_THRESHOLD).pass);
    }

    #[test]
    fn an_empty_body_scores_zero_without_panicking() {
        let report = check("", PASS_THRESHOLD);
        assert_eq!(report.score, 0);
        assert!(report
            .missing
            .iter()
            .any(|m| m.note.as_deref().is_some_and(|n| n.contains("empty body"))));
    }

    #[test]
    fn an_unterminated_comment_is_not_counted_as_a_placeholder() {
        assert_eq!(count_placeholders("<!-- open"), 0);
        assert_eq!(count_placeholders("<!-- a --><!-- b -->"), 2);
    }

    #[test]
    fn strip_comments_handles_an_unterminated_comment() {
        assert_eq!(strip_comments("keep <!-- drop"), "keep ");
        assert_eq!(strip_comments("a <!-- b --> c"), "a  c");
    }
}
