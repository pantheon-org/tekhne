//! Combined pattern-analysis pipeline.
//!
//! Ports `pattern-recognition-pipeline.sh`, which ran the three analyses and
//! scraped their text reports with grep to recover metrics. This calls the
//! [`duplication`], [`semantic`] and [`quality_signals`] modules directly and
//! reads their structured results, then computes the same composite repository
//! health score:
//!
//! ```text
//! penalty   = critical_pairs*2 + (duplicate_pairs - critical_pairs)
//! bonus     = 5 if semantic_pairs < skill_count/4 else 0
//! composite = clamp(avg_quality - penalty + bonus, 0, 100)
//! ```

use crate::duplication::{self, Band};
use crate::quality_signals;
use crate::semantic;

/// Combined repository health metrics.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PipelineReport {
    pub skill_count: usize,
    pub duplicate_pairs: usize,
    pub critical_pairs: usize,
    pub semantic_pairs: usize,
    pub average_quality: u32,
    pub high_quality: usize,
    pub duplication_penalty: u32,
    pub semantic_bonus: u32,
    pub composite_score: u32,
    pub grade: &'static str,
}

fn grade(score: u32) -> &'static str {
    match score {
        s if s >= 90 => "A+ (Exceptional)",
        s if s >= 80 => "A (Excellent)",
        s if s >= 70 => "B (Good)",
        s if s >= 60 => "C (Fair)",
        _ => "D (Needs Work)",
    }
}

/// Run the three analyses over `skills` and compute the composite health score.
pub fn run(skills: &[duplication::SkillDoc]) -> PipelineReport {
    let dup = duplication::detect_enhanced(skills);
    let sem = semantic::analyze(skills);
    let qual = quality_signals::analyze(skills);

    let skill_count = skills.len();
    let duplicate_pairs = dup.len();
    let critical_pairs = dup.iter().filter(|p| p.band == Band::Critical).count();
    let semantic_pairs = sem.similar_pairs;
    let average_quality = qual.average;

    let duplication_penalty =
        (critical_pairs as u32 * 2) + (duplicate_pairs as u32 - critical_pairs as u32);
    let semantic_bonus = if semantic_pairs < skill_count / 4 {
        5
    } else {
        0
    };

    let composite_score = (average_quality as i64 - duplication_penalty as i64
        + semantic_bonus as i64)
        .clamp(0, 100) as u32;

    PipelineReport {
        skill_count,
        duplicate_pairs,
        critical_pairs,
        semantic_pairs,
        average_quality,
        high_quality: qual.high_quality,
        duplication_penalty,
        semantic_bonus,
        composite_score,
        grade: grade(composite_score),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duplication::SkillDoc;

    fn doc(name: &str, content: &str) -> SkillDoc {
        SkillDoc {
            name: name.to_string(),
            path: format!("{name}/SKILL.md").into(),
            content: content.to_string(),
        }
    }

    #[test]
    fn grade_bands() {
        assert_eq!(grade(90), "A+ (Exceptional)");
        assert_eq!(grade(80), "A (Excellent)");
        assert_eq!(grade(70), "B (Good)");
        assert_eq!(grade(60), "C (Fair)");
        assert_eq!(grade(0), "D (Needs Work)");
    }

    #[test]
    fn empty_repo_is_zero() {
        let r = run(&[]);
        assert_eq!(r.skill_count, 0);
        assert_eq!(r.composite_score, 0);
        assert_eq!(r.grade, "D (Needs Work)");
    }

    #[test]
    fn duplication_penalty_and_composite_clamped() {
        // Two near-identical rich skills: high duplication + decent quality.
        let body = "---\nname: s\ndescription: deployment skill\n---\n# Deploy\n## Usage\nConfigure and deploy with Docker.\n## Examples\nFor example run:\n```bash\ndeploy --now\n```\n**Important**: never skip validation because errors happen.\n";
        let skills = vec![doc("deploy-one", body), doc("deploy-two", body)];
        let r = run(&skills);
        assert_eq!(r.skill_count, 2);
        // Identical content -> a critical duplicate pair -> penalty applied.
        assert!(r.duplicate_pairs >= 1);
        assert!(r.composite_score <= 100);
    }

    #[test]
    fn distinct_skills_have_no_critical_pairs() {
        let a = doc("cooking", "# Cooking\nChop onions, simmer sauce.\n");
        let b = doc(
            "astronomy",
            "# Astronomy\nObserve galaxies with telescopes.\n",
        );
        let r = run(&[a, b]);
        // Topically distinct skills raise no critical duplicates (a moderate
        // line-overlap pair may still exist for very short skills, so the
        // penalty is not asserted to be exactly zero).
        assert_eq!(r.critical_pairs, 0);
    }
}
