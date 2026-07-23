//! Heuristic per-skill quality signals.
//!
//! Ports `ml-pattern-detection.sh`. Despite the shell's "ML" branding (and
//! fabricated `accuracy: 92.3%` model metadata), the algorithm is a
//! deterministic weighted feature score - there is no model or learning. This
//! port keeps the scoring faithfully and names it honestly: `quality-signals`.
//! The fake model-file and accuracy claims are intentionally dropped.
//!
//! For each skill three component scores are computed from grep/awk-style
//! feature counts, normalised to 0-100, then combined
//! `0.3*structural + 0.4*content + 0.3*quality`. Only the features that feed the
//! score are extracted (the shell computed ~50 but used ~25). Integer
//! truncation matches the shell's `bc | cut -d. -f1`; intermediate `bc`
//! rounding means the score is faithful to intent, not bit-exact.

use regex::Regex;

use crate::duplication::SkillDoc;

/// Per-skill quality signal scores (all normalised 0-100 unless noted).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SkillQuality {
    pub name: String,
    pub structural: u32,
    pub content: u32,
    pub quality: u32,
    /// Final weighted score (0-100).
    pub score: u32,
    /// Classification band label.
    pub classification: &'static str,
    /// Improvement-area flags (empty when all patterns present).
    pub flags: Vec<&'static str>,
}

/// Aggregate quality signals across a set of skills.
#[derive(Debug, Clone, serde::Serialize)]
pub struct QualityReport {
    pub skills: Vec<SkillQuality>,
    pub average: u32,
    /// Skills scoring >= 85.
    pub high_quality: usize,
    /// Skills scoring < 60.
    pub needs_improvement: usize,
}

impl QualityReport {
    /// Repository grade from the average score (ports the shell's bands).
    pub fn grade(&self) -> &'static str {
        match self.average {
            a if a >= 85 => "A (Excellent)",
            a if a >= 75 => "B (Good)",
            a if a >= 65 => "C (Fair)",
            _ => "D (Needs Work)",
        }
    }
}

/// Extracted feature counts that feed the score.
struct Features {
    header_count: u32,
    code_blocks: u32,
    external_links: u32,
    table_count: u32,
    avg_line_length: f64,
    header_density: f64,
    tech_terms: u32,
    action_words: u32,
    examples: u32,
    warnings: u32,
    prerequisites: u32,
    vocabulary_richness: f64,
    actionability: f64,
    clarity_score: f64,
    has_frontmatter: bool,
    has_description: bool,
    has_examples: bool,
    has_usage: bool,
    has_code_snippets: bool,
    code_language_tags: u32,
    organized_sections: u32,
    error_handling: u32,
    troubleshooting: u32,
}

/// Compiled feature regexes.
struct Pats {
    tech: Regex,
    action: Regex,
    examples: Regex,
    warnings: Regex,
    prerequisites: Regex,
    questions: Regex,
    explanations: Regex,
    ext_link: Regex,
    code_lang: Regex,
    word: Regex,
    error_handling: Regex,
    troubleshooting: Regex,
}

impl Pats {
    fn new() -> Self {
        let re = |p: &str| Regex::new(p).expect("static regex is valid");
        Pats {
            // ACRONYMS / camelCase / common tooling names, case-insensitive.
            tech: re(
                r"(?i)\b[A-Z]{2,}\b|[a-z]*[A-Z][a-z]*[A-Z][a-z]*|docker|kubernetes|aws|git|npm|api|json|yaml",
            ),
            action: re(
                r"(?i)\b(create|build|implement|configure|setup|install|deploy|test|run|execute|generate|analyze|optimize|debug|fix|update|add|remove|delete|modify)\b",
            ),
            examples: re(r"(?i)example|for instance|such as|like|e\.g\."),
            warnings: re(r"(?i)warning|caution|important|note|careful|avoid|don.t|never"),
            prerequisites: re(r"(?i)prerequisite|require|need|must|before|first"),
            questions: re(r"\?"),
            explanations: re(r"(?i)because|since|therefore|thus|however|moreover|furthermore"),
            ext_link: re(r"\[.*\]\(http"),
            code_lang: re(r"^```[a-z]"),
            word: re(r"^[a-z]+$"),
            error_handling: re(r"(?i)error|fail|exception|catch|handle"),
            troubleshooting: re(r"(?i)troubleshoot|debug|problem|issue|solution"),
        }
    }
}

/// Count lines matching `re` (grep -c semantics: matching lines, not matches).
fn count_lines(content: &str, re: &Regex) -> u32 {
    content.lines().filter(|l| re.is_match(l)).count() as u32
}

/// Count total matches across the content (grep -o | wc -l semantics).
fn count_matches(content: &str, re: &Regex) -> u32 {
    content
        .lines()
        .map(|l| re.find_iter(l).count())
        .sum::<usize>() as u32
}

fn extract(content: &str, pats: &Pats) -> Features {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len().max(1) as f64;

    let header_count = lines.iter().filter(|l| l.starts_with('#')).count() as u32;
    let code_blocks = lines.iter().filter(|l| l.starts_with("```")).count() as u32;
    let table_count = lines.iter().filter(|l| l.starts_with('|')).count() as u32;
    let external_links = count_lines(content, &pats.ext_link);
    let organized_sections = lines.iter().filter(|l| l.starts_with("## ")).count() as u32;
    let code_language_tags = count_lines(content, &pats.code_lang);

    let avg_line_length = if lines.is_empty() {
        0.0
    } else {
        (lines.iter().map(|l| l.len()).sum::<usize>() / lines.len()) as f64
    };
    let header_density = header_count as f64 * 100.0 / total_lines;

    // Word metrics: lowercased pure-alpha tokens.
    let word_count = content.split_whitespace().count().max(1) as u32;
    let mut uniq = std::collections::HashSet::new();
    for tok in content.split_whitespace() {
        let low = tok.to_lowercase();
        if pats.word.is_match(&low) {
            uniq.insert(low);
        }
    }
    let unique_words = uniq.len() as u32;

    // tech_terms: unique matches of the tech regex.
    let mut tech_set = std::collections::HashSet::new();
    for m in pats.tech.find_iter(content) {
        tech_set.insert(m.as_str().to_lowercase());
    }
    let tech_terms = tech_set.len() as u32;

    let action_words = count_matches(content, &pats.action);
    let examples = count_lines(content, &pats.examples);
    let warnings = count_lines(content, &pats.warnings);
    let prerequisites = count_lines(content, &pats.prerequisites);
    let questions = count_lines(content, &pats.questions);
    let explanations = count_lines(content, &pats.explanations);
    let error_handling = count_lines(content, &pats.error_handling);
    let troubleshooting = count_lines(content, &pats.troubleshooting);

    let vocabulary_richness = unique_words as f64 * 100.0 / word_count as f64;
    let actionability = action_words as f64 * 100.0 / word_count as f64;
    let clarity_score = (questions + explanations + examples) as f64 * 100.0 / word_count as f64;

    let lc = content.to_lowercase();
    let has_frontmatter = lines.contains(&"---");
    let has_description = lines
        .iter()
        .any(|l| l.to_lowercase().starts_with("description:"));
    let has_examples = lc.contains("example");
    let has_usage = lc.contains("usage") || lc.contains("how to");
    let has_code_snippets = code_blocks > 0;

    Features {
        header_count,
        code_blocks,
        external_links,
        table_count,
        avg_line_length,
        header_density,
        tech_terms,
        action_words,
        examples,
        warnings,
        prerequisites,
        vocabulary_richness,
        actionability,
        clarity_score,
        has_frontmatter,
        has_description,
        has_examples,
        has_usage,
        has_code_snippets,
        code_language_tags,
        organized_sections,
        error_handling,
        troubleshooting,
    }
}

fn b(v: bool) -> f64 {
    if v {
        1.0
    } else {
        0.0
    }
}

/// Score one skill's features into normalised component + final scores.
fn score(f: &Features) -> (u32, u32, u32, u32) {
    let structural = f.header_count as f64 * 2.0
        + f.code_blocks as f64 * 3.0
        + f.external_links as f64 * 1.5
        + f.table_count as f64
        + b(f.avg_line_length > 40.0 && f.avg_line_length < 120.0) * 5.0
        + b(f.header_density > 2.0 && f.header_density < 8.0) * 3.0;

    let content = f.tech_terms as f64 * 1.5
        + f.action_words as f64 * 0.5
        + f.examples as f64 * 3.0
        + f.warnings as f64 * 2.0
        + f.prerequisites as f64
        + b(f.vocabulary_richness > 30.0) * 5.0
        + b(f.actionability > 2.0) * 4.0
        + b(f.clarity_score > 1.0) * 6.0;

    let quality = b(f.has_frontmatter) * 8.0
        + b(f.has_description) * 6.0
        + b(f.has_examples) * 5.0
        + b(f.has_usage) * 4.0
        + b(f.has_code_snippets) * 3.0
        + f.code_language_tags as f64 * 2.0
        + f.organized_sections as f64
        + f.error_handling as f64 * 2.0
        + f.troubleshooting as f64 * 3.0;

    // Normalise (÷30/40/50), truncate to int, cap at 100 (ports bc | cut).
    let structural_norm = ((structural * 100.0 / 30.0) as u32).min(100);
    let content_norm = ((content * 100.0 / 40.0) as u32).min(100);
    let quality_norm = ((quality * 100.0 / 50.0) as u32).min(100);

    let final_score = (structural_norm as f64 * 0.3
        + content_norm as f64 * 0.4
        + quality_norm as f64 * 0.3) as u32;

    (structural_norm, content_norm, quality_norm, final_score)
}

fn classify(score: u32) -> &'static str {
    match score {
        s if s >= 90 => "Excellent",
        s if s >= 75 => "Good",
        s if s >= 60 => "Fair",
        _ => "Needs Work",
    }
}

fn flags(structural: u32, content: u32, quality: u32, f: &Features) -> Vec<&'static str> {
    let mut out = Vec::new();
    if structural < 70 {
        out.push("structure");
    }
    if content < 70 {
        out.push("content_richness");
    }
    if quality < 70 {
        out.push("quality_markers");
    }
    if !f.has_examples {
        out.push("missing_examples");
    }
    if !f.has_code_snippets {
        out.push("missing_code");
    }
    if !f.has_frontmatter {
        out.push("missing_metadata");
    }
    out
}

/// Score a single skill's content.
pub fn score_skill(name: &str, content: &str) -> SkillQuality {
    let pats = Pats::new();
    let f = extract(content, &pats);
    let (structural, content_n, quality, final_score) = score(&f);
    SkillQuality {
        name: name.to_string(),
        structural,
        content: content_n,
        quality,
        score: final_score,
        classification: classify(final_score),
        flags: flags(structural, content_n, quality, &f),
    }
}

/// Score every skill and aggregate.
pub fn analyze(skills: &[SkillDoc]) -> QualityReport {
    let pats = Pats::new();
    let mut scored: Vec<SkillQuality> = skills
        .iter()
        .map(|s| {
            let f = extract(&s.content, &pats);
            let (structural, content, quality, final_score) = score(&f);
            SkillQuality {
                name: s.name.clone(),
                structural,
                content,
                quality,
                score: final_score,
                classification: classify(final_score),
                flags: flags(structural, content, quality, &f),
            }
        })
        .collect();

    let n = scored.len();
    let average = if n == 0 {
        0
    } else {
        scored.iter().map(|s| s.score).sum::<u32>() / n as u32
    };
    let high_quality = scored.iter().filter(|s| s.score >= 85).count();
    let needs_improvement = scored.iter().filter(|s| s.score < 60).count();

    // Sort by score descending, then name, for stable top/bottom reporting.
    scored.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));

    QualityReport {
        skills: scored,
        average,
        high_quality,
        needs_improvement,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(name: &str, content: &str) -> SkillDoc {
        SkillDoc {
            name: name.to_string(),
            path: format!("{name}/SKILL.md").into(),
            content: content.to_string(),
        }
    }

    #[test]
    fn rich_skill_scores_higher_than_sparse() {
        let rich = "---\nname: rich\ndescription: A thorough skill\n---\n\n# Rich\n\n## Usage\n\nCreate and configure the thing.\n\n## Examples\n\nFor example, run this:\n\n```bash\ndeploy --now\n```\n\n**Important**: never skip validation, because it catches errors.\n\n| A | B |\n|---|---|\n| 1 | 2 |\n";
        let sparse = "# Sparse\n\nsome text here\n";
        let r = score_skill("rich", rich);
        let s = score_skill("sparse", sparse);
        assert!(r.score > s.score, "rich {} vs sparse {}", r.score, s.score);
    }

    #[test]
    fn scores_are_capped_at_100() {
        let mut body = String::from("---\nname: x\ndescription: d\n---\n");
        for i in 0..200 {
            body.push_str(&format!("## Section {i}\n\n```bash\necho {i}\n```\n\nExample: because error handle troubleshoot.\n"));
        }
        let q = score_skill("x", &body);
        assert!(q.structural <= 100 && q.content <= 100 && q.quality <= 100);
        assert!(q.score <= 100);
    }

    #[test]
    fn flags_missing_patterns() {
        let q = score_skill("bare", "# Bare\n\nplain prose only\n");
        assert!(q.flags.contains(&"missing_examples"));
        assert!(q.flags.contains(&"missing_code"));
        assert!(q.flags.contains(&"missing_metadata"));
    }

    #[test]
    fn analyze_aggregates_average_and_bands() {
        let skills = vec![
            doc("a", "---\nname: a\ndescription: d\n---\n# A\n## Usage\n```bash\nx\n```\nExample here.\n"),
            doc("b", "# B\nnothing\n"),
        ];
        let r = analyze(&skills);
        assert_eq!(r.skills.len(), 2);
        assert!(r.average <= 100);
        // Sorted descending: first score >= second.
        assert!(r.skills[0].score >= r.skills[1].score);
    }

    #[test]
    fn classify_bands() {
        assert_eq!(classify(95), "Excellent");
        assert_eq!(classify(80), "Good");
        assert_eq!(classify(65), "Fair");
        assert_eq!(classify(10), "Needs Work");
    }

    #[test]
    fn empty_set_is_zero_average() {
        let r = analyze(&[]);
        assert_eq!(r.average, 0);
        assert_eq!(r.grade(), "D (Needs Work)");
    }
}
