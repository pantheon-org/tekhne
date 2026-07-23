//! Heuristic semantic similarity between skills.
//!
//! Ports `semantic-analysis.sh`. The shell called this "NLP / TF-IDF / vector"
//! analysis, but it is a set of frequency-count heuristics over four feature
//! layers; there are no vectors or embeddings. This port reproduces the
//! *intended* algorithm (the shell's awk had several bugs - double-counted
//! totals, a `topics.txt` path that never resolved) and names it honestly.
//!
//! Composite similarity per pair is a weighted blend:
//! `concept*30 + vocabulary*25 + topic*25 + intent*20`, all `/100`.

use std::collections::HashMap;

use regex::Regex;

use crate::duplication::SkillDoc;

/// Report threshold (percent) and high-similarity threshold, from the shell.
pub const MIN_SEMANTIC_SCORE: u32 = 25;
pub const HIGH_SEMANTIC_SCORE: u32 = 45;

/// A semantic-similarity result for a skill pair.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SemanticPair {
    pub name1: String,
    pub name2: String,
    pub composite: u32,
    pub concept: u32,
    pub vocabulary: u32,
    pub topic: u32,
    pub intent: u32,
    pub category: &'static str,
}

/// Aggregate semantic analysis across all pairs.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SemanticReport {
    pub pairs: Vec<SemanticPair>,
    pub total_comparisons: usize,
    /// Pairs at or above [`MIN_SEMANTIC_SCORE`].
    pub similar_pairs: usize,
    /// Pairs at or above [`HIGH_SEMANTIC_SCORE`].
    pub high_similarity_pairs: usize,
}

/// The topic categories and their keyword sets (ports the shell's `extract_topics`).
const TOPICS: &[(&str, &[&str])] = &[
    (
        "infrastructure",
        &[
            "deploy",
            "infrastructure",
            "cloud",
            "server",
            "docker",
            "kubernetes",
        ],
    ),
    (
        "development",
        &["code", "function", "class", "api", "library", "framework"],
    ),
    ("testing", &["test", "spec", "mock", "assert", "coverage"]),
    (
        "documentation",
        &["document", "readme", "guide", "tutorial", "example"],
    ),
    (
        "quality",
        &["quality", "lint", "format", "audit", "analyze", "review"],
    ),
    (
        "security",
        &[
            "security",
            "auth",
            "token",
            "encrypt",
            "secure",
            "vulnerability",
        ],
    ),
];

/// Common stop-words excluded from the vocabulary layer (ports the shell list).
const STOPWORDS: &[&str] = &[
    "this", "that", "will", "with", "have", "been", "from", "they", "were", "when", "what",
    "where", "your", "then", "more", "some", "like", "into", "time", "work",
];

/// Per-skill extracted semantic features.
struct Vectors {
    concepts: HashMap<String, u32>,
    vocabulary: HashMap<String, u32>,
    topics: Vec<&'static str>,
    intents: Vec<String>,
}

struct Pats {
    concept: Regex,
    code_span: Regex,
    framework: Regex,
    md_strip_link: Regex,
    word4: Regex,
    intent: Regex,
}

impl Pats {
    fn new() -> Self {
        let re = |p: &str| Regex::new(p).expect("static regex is valid");
        Pats {
            concept: re(r"(?i)\b[A-Z]{2,}[a-z]*\b|\b[A-Z][a-z]*[A-Z][a-zA-Z]*\b"),
            code_span: re(r"`[^`]+`"),
            framework: re(
                r"(?i)\b(docker|kubernetes|aws|terraform|ansible|jenkins|git|npm|yarn|pnpm|webpack|vite|rollup)\b",
            ),
            md_strip_link: re(r"\[([^\]]*)\]\([^)]*\)"),
            word4: re(r"^[a-z]{4,}$"),
            intent: re(
                r"(?i)\b(create|build|implement|configure|setup|install|deploy|test|validate|analyze|optimize|debug)\b",
            ),
        }
    }
}

fn incr(m: &mut HashMap<String, u32>, k: String) {
    *m.entry(k).or_insert(0) += 1;
}

fn extract(content: &str, pats: &Pats) -> Vectors {
    // 1. Concepts: technical terms, code spans (backticks stripped), framework names.
    let mut concepts: HashMap<String, u32> = HashMap::new();
    for m in pats.concept.find_iter(content) {
        incr(&mut concepts, m.as_str().to_lowercase());
    }
    for m in pats.code_span.find_iter(content) {
        let inner = m.as_str().trim_matches('`').to_lowercase();
        if !inner.is_empty() {
            incr(&mut concepts, inner);
        }
    }
    for m in pats.framework.find_iter(content) {
        incr(&mut concepts, m.as_str().to_lowercase());
    }

    // 2. Vocabulary: markdown stripped, lowercased 4+ letter words minus stopwords.
    let stripped = strip_markdown(content, pats);
    let mut vocabulary: HashMap<String, u32> = HashMap::new();
    for tok in stripped.split_whitespace() {
        let low = tok.to_lowercase();
        if pats.word4.is_match(&low) && !STOPWORDS.contains(&low.as_str()) {
            incr(&mut vocabulary, low);
        }
    }

    // 3. Topics present (keyword substring match, case-insensitive).
    let lc = content.to_lowercase();
    let topics: Vec<&'static str> = TOPICS
        .iter()
        .filter(|(_, kws)| kws.iter().any(|kw| lc.contains(kw)))
        .map(|(name, _)| *name)
        .collect();

    // 4. Intents present (distinct action words).
    let mut intent_set: Vec<String> = pats
        .intent
        .find_iter(content)
        .map(|m| m.as_str().to_lowercase())
        .collect();
    intent_set.sort();
    intent_set.dedup();

    Vectors {
        concepts,
        vocabulary,
        topics,
        intents: intent_set,
    }
}

/// Strip bold/italic/code markers and reduce links to their text.
fn strip_markdown(content: &str, pats: &Pats) -> String {
    let no_links = pats.md_strip_link.replace_all(content, "$1");
    // Remove bold/italic (`*`) and inline-code (`` ` ``) markers in one pass.
    no_links.replace(['*', '`'], "")
}

/// Concept overlap: frequency-weighted Jaccard (percent).
fn concept_similarity(a: &HashMap<String, u32>, b: &HashMap<String, u32>) -> u32 {
    let mut common = 0u32;
    for (k, &av) in a {
        if let Some(&bv) = b.get(k) {
            common += av.min(bv);
        }
    }
    let total1: u32 = a.values().sum();
    let total2: u32 = b.values().sum();
    let union = total1 + total2 - common;
    (common * 100).checked_div(union).unwrap_or(0)
}

/// Vocabulary overlap: mean harmonic frequency over shared terms (ports the
/// shell's `(f1*f2)/((f1+f2)/2)` averaged across shared terms).
fn vocabulary_similarity(a: &HashMap<String, u32>, b: &HashMap<String, u32>) -> u32 {
    let mut total = 0.0f64;
    let mut terms = 0u32;
    for (k, &av) in a {
        if let Some(&bv) = b.get(k) {
            let f1 = av as f64;
            let f2 = bv as f64;
            total += (f1 * f2) / ((f1 + f2) / 2.0);
            terms += 1;
        }
    }
    if terms == 0 {
        0
    } else {
        (total / terms as f64) as u32
    }
}

/// Jaccard over two label sets (percent).
fn set_jaccard(a: &[impl AsRef<str>], b: &[impl AsRef<str>]) -> u32 {
    use std::collections::HashSet;
    let sa: HashSet<&str> = a.iter().map(|s| s.as_ref()).collect();
    let sb: HashSet<&str> = b.iter().map(|s| s.as_ref()).collect();
    let common = sa.intersection(&sb).count() as u32;
    let union = (sa.len() + sb.len()) as u32 - common;
    (common * 100).checked_div(union).unwrap_or(0)
}

fn category(composite: u32) -> &'static str {
    match composite {
        c if c >= 60 => "High Semantic Overlap",
        c if c >= 35 => "Moderate Semantic Similarity",
        c if c >= 20 => "Low Semantic Overlap",
        _ => "Minimal Semantic Connection",
    }
}

/// Compute the composite semantic similarity between two skills' vectors.
fn similarity(a: &Vectors, b: &Vectors) -> SemanticPair {
    let concept = concept_similarity(&a.concepts, &b.concepts);
    let vocabulary = vocabulary_similarity(&a.vocabulary, &b.vocabulary);
    let topic = set_jaccard(&a.topics, &b.topics);
    let intent = set_jaccard(&a.intents, &b.intents);
    let composite = (concept * 30 + vocabulary * 25 + topic * 25 + intent * 20) / 100;
    SemanticPair {
        name1: String::new(),
        name2: String::new(),
        composite,
        concept,
        vocabulary,
        topic,
        intent,
        category: category(composite),
    }
}

/// Analyse all skill pairs, returning pairs at or above [`MIN_SEMANTIC_SCORE`]
/// sorted by composite descending then name. Features are extracted once per
/// skill (the shell re-extracted the inner skill per pair).
pub fn analyze(skills: &[SkillDoc]) -> SemanticReport {
    let pats = Pats::new();
    let vectors: Vec<Vectors> = skills.iter().map(|s| extract(&s.content, &pats)).collect();

    let mut pairs = Vec::new();
    let mut total_comparisons = 0usize;
    let mut similar = 0usize;
    let mut high = 0usize;

    for i in 0..skills.len() {
        for j in (i + 1)..skills.len() {
            total_comparisons += 1;
            let mut pair = similarity(&vectors[i], &vectors[j]);
            if pair.composite >= MIN_SEMANTIC_SCORE {
                similar += 1;
                if pair.composite >= HIGH_SEMANTIC_SCORE {
                    high += 1;
                }
                pair.name1 = skills[i].name.clone();
                pair.name2 = skills[j].name.clone();
                pairs.push(pair);
            }
        }
    }

    pairs.sort_by(|a, b| {
        b.composite
            .cmp(&a.composite)
            .then(a.name1.cmp(&b.name1))
            .then(a.name2.cmp(&b.name2))
    });

    SemanticReport {
        pairs,
        total_comparisons,
        similar_pairs: similar,
        high_similarity_pairs: high,
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
    fn identical_skills_are_highly_similar() {
        let body = "# Deploy\n\nConfigure and deploy with Docker and Kubernetes.\nRun the test suite. Build the API.\n`docker build`\n";
        let skills = vec![doc("a", body), doc("b", body)];
        let r = analyze(&skills);
        assert_eq!(r.total_comparisons, 1);
        assert_eq!(r.pairs.len(), 1);
        assert!(
            r.pairs[0].composite >= HIGH_SEMANTIC_SCORE,
            "{:?}",
            r.pairs[0]
        );
        assert_eq!(r.pairs[0].category, "High Semantic Overlap");
    }

    #[test]
    fn disjoint_skills_below_threshold() {
        let a = doc("a", "# Cooking\n\nChop onions and simmer sauce slowly.\n");
        let b = doc(
            "b",
            "# Astronomy\n\nObserve distant galaxies through telescopes.\n",
        );
        let r = analyze(&[a, b]);
        assert_eq!(r.total_comparisons, 1);
        // Below MIN_SEMANTIC_SCORE, so not reported.
        assert_eq!(r.pairs.len(), 0);
    }

    #[test]
    fn topic_jaccard_counts_shared_categories() {
        let a = vec!["infrastructure", "testing"];
        let b = vec!["infrastructure", "security"];
        // common 1, union 3 -> 33
        assert_eq!(set_jaccard(&a, &b), 33);
    }

    #[test]
    fn concept_similarity_frequency_weighted() {
        let mut a = HashMap::new();
        a.insert("docker".to_string(), 3u32);
        a.insert("api".to_string(), 1u32);
        let mut b = HashMap::new();
        b.insert("docker".to_string(), 1u32);
        b.insert("git".to_string(), 2u32);
        // common = min(3,1)=1; union = 4 + 3 - 1 = 6; 1*100/6 = 16
        assert_eq!(concept_similarity(&a, &b), 16);
    }

    #[test]
    fn category_bands() {
        assert_eq!(category(60), "High Semantic Overlap");
        assert_eq!(category(35), "Moderate Semantic Similarity");
        assert_eq!(category(20), "Low Semantic Overlap");
        assert_eq!(category(10), "Minimal Semantic Connection");
    }
}
