//! Markdown report rendering for the analysis subcommands.
//!
//! Kept binary-local (not part of the library API): the library modules own the
//! data types and algorithms, while these functions own the human-facing CLI
//! presentation. Keeping them here keeps `main.rs` to CLI definition and
//! dispatch.

use skill_auditor::duplication::{Band, BasicPair, EnhancedPair};
use skill_auditor::pattern_analysis::PipelineReport;
use skill_auditor::quality_signals::QualityReport;
use skill_auditor::semantic::{self, SemanticReport};

/// Render the basic (line-overlap) duplication report.
pub fn duplication_basic(dir: &str, count: usize, threshold: u32, pairs: &[BasicPair]) -> String {
    let mut s = String::new();
    s.push_str("# Skill Duplication Report\n\n## Summary\n");
    s.push_str(&format!("- Skills analyzed: {count}\n"));
    s.push_str(&format!("- Directory: {dir}\n"));
    s.push_str(&format!("- Threshold: >{threshold}% similarity\n\n"));
    s.push_str("## High Duplication Pairs\n\n");
    if pairs.is_empty() {
        s.push_str("None above threshold.\n");
    } else {
        for p in pairs {
            s.push_str(&format!("### {} \u{2194} {}\n", p.name1, p.name2));
            s.push_str(&format!("- Similarity: {}%\n", p.similarity));
            s.push_str(&format!("- Common lines: {}\n", p.common_lines));
            s.push_str("- Recommendation: Consider aggregation\n\n");
        }
    }
    s
}

/// Render the enhanced (composite) duplication report, grouped by band.
pub fn duplication_enhanced(dir: &str, count: usize, pairs: &[EnhancedPair]) -> String {
    use skill_auditor::duplication;

    let mut s = String::new();
    s.push_str("# Enhanced Skill Duplication Report\n\n## Executive Summary\n");
    s.push_str(&format!("- Skills analyzed: {count}\n"));
    s.push_str(&format!("- Directory: {dir}\n"));
    s.push_str(&format!(
        "- Thresholds: Moderate >={}%, High >={}%, Critical >={}%\n\n",
        duplication::MODERATE_THRESHOLD,
        duplication::HIGH_THRESHOLD,
        duplication::CRITICAL_THRESHOLD
    ));

    for band in [Band::Critical, Band::High, Band::Moderate] {
        s.push_str(&format!("## {} Duplications\n\n", band.label()));
        let group: Vec<&EnhancedPair> = pairs.iter().filter(|p| p.band == band).collect();
        if group.is_empty() {
            s.push_str("None.\n\n");
            continue;
        }
        for p in group {
            s.push_str(&format!("### {} \u{2194} {}\n", p.name1, p.name2));
            s.push_str(&format!(
                "**Overall Similarity**: {}% {}\n\n",
                p.composite,
                band.label()
            ));
            s.push_str("| Metric | Score | Weight |\n|--------|-------|--------|\n");
            s.push_str(&format!("| Semantic | {}% | 40% |\n", p.semantic));
            s.push_str(&format!("| Structural | {}% | 35% |\n", p.structural));
            s.push_str(&format!("| Lexical | {}% | 25% |\n\n", p.lexical));
            s.push_str(&format!("**Action Required**: {}\n\n", band.action()));
        }
    }
    s
}

/// Render per-skill quality signals as a Markdown report.
pub fn quality(report: &QualityReport) -> String {
    let mut s = String::from("# Skill Quality Signals\n\n");
    s.push_str(&format!("- Skills analyzed: {}\n", report.skills.len()));
    s.push_str(&format!("- Average score: {}%\n", report.average));
    s.push_str(&format!(
        "- High-quality (>=85%): {}\n",
        report.high_quality
    ));
    s.push_str(&format!(
        "- Needs improvement (<60%): {}\n",
        report.needs_improvement
    ));
    s.push_str(&format!("- Repository grade: {}\n\n", report.grade()));
    s.push_str("| Skill | Score | Structural | Content | Quality | Class |\n");
    s.push_str("|-------|-------|-----------|---------|---------|-------|\n");
    for q in &report.skills {
        s.push_str(&format!(
            "| {} | {}% | {}% | {}% | {}% | {} |\n",
            q.name, q.score, q.structural, q.content, q.quality, q.classification
        ));
    }
    s
}

/// Render pairwise semantic similarity as a Markdown report.
pub fn semantic(report: &SemanticReport, skills_len: usize) -> String {
    let mut s = String::from("# Semantic Similarity Analysis\n\n");
    s.push_str(&format!("- Skills analyzed: {skills_len}\n"));
    s.push_str(&format!(
        "- Total comparisons: {}\n",
        report.total_comparisons
    ));
    s.push_str(&format!(
        "- Similar pairs (>={}%): {}\n",
        semantic::MIN_SEMANTIC_SCORE,
        report.similar_pairs
    ));
    s.push_str(&format!(
        "- High similarity (>={}%): {}\n\n",
        semantic::HIGH_SEMANTIC_SCORE,
        report.high_similarity_pairs
    ));
    for p in &report.pairs {
        s.push_str(&format!(
            "### {} \u{2194} {} \u{2014} {}% ({})\n",
            p.name1, p.name2, p.composite, p.category
        ));
        s.push_str(&format!(
            "concept {}% | vocabulary {}% | topic {}% | intent {}%\n\n",
            p.concept, p.vocabulary, p.topic, p.intent
        ));
    }
    s
}

/// Render the combined pattern-analysis pipeline report.
pub fn pattern(report: &PipelineReport) -> String {
    let mut s = String::from("# Comprehensive Pattern Analysis\n\n");
    s.push_str(&format!(
        "Repository health: {}% - {}\n\n",
        report.composite_score, report.grade
    ));
    s.push_str("| Metric | Value |\n|--------|-------|\n");
    s.push_str(&format!("| Total skills | {} |\n", report.skill_count));
    s.push_str(&format!(
        "| Duplicate pairs | {} ({} critical) |\n",
        report.duplicate_pairs, report.critical_pairs
    ));
    s.push_str(&format!("| Semantic pairs | {} |\n", report.semantic_pairs));
    s.push_str(&format!(
        "| Average quality | {}% |\n",
        report.average_quality
    ));
    s.push_str(&format!(
        "| High-quality skills | {} |\n",
        report.high_quality
    ));
    s.push_str(&format!(
        "| Duplication penalty | -{} |\n",
        report.duplication_penalty
    ));
    s.push_str(&format!(
        "| Semantic bonus | +{} |\n",
        report.semantic_bonus
    ));
    s
}
