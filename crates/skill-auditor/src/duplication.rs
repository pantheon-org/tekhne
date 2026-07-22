//! Cross-skill duplication detection.
//!
//! Ports two shell scripts:
//! - `detect-duplication.sh`: basic whole-line overlap similarity.
//! - `detect-duplication-enhanced.sh`: a composite of semantic, structural and
//!   lexical similarity.
//!
//! The enhanced shell script relied on several non-portable or buggy constructs
//! (a `sed` header strip that was effectively a no-op, `<()` process
//! substitution under `#!/usr/bin/env sh`, `local` in POSIX sh, and a
//! case-insensitive `grep -i` on `[A-Z]` classes). This port reproduces the
//! *intended* algorithm - Jaccard similarity over extracted feature sets with a
//! 40/35/25 composite weighting - rather than the byte-exact shell behaviour.
//! Deviations from the shell are called out inline with `NOTE:`.
//!
//! Features are extracted once per skill (the shell re-extracted the inner
//! skill's features on every pair); this is a pure performance fix and does not
//! change results.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Serialize;

/// Reporting threshold used by the basic algorithm and as the floor for the
/// enhanced "moderate" band (percent).
pub const MODERATE_THRESHOLD: u32 = 20;
/// Enhanced semantic/high band threshold (percent).
pub const HIGH_THRESHOLD: u32 = 30;
/// Enhanced critical band threshold (percent).
pub const CRITICAL_THRESHOLD: u32 = 50;

/// Composite weightings (percent), matching the shell's `40/35/25`.
const WEIGHT_SEMANTIC: u32 = 40;
const WEIGHT_STRUCTURAL: u32 = 35;
const WEIGHT_LEXICAL: u32 = 25;

/// A single SKILL.md discovered under the skills root.
#[derive(Debug, Clone)]
pub struct SkillDoc {
    /// The skill's directory name (`basename(dirname(SKILL.md))`).
    pub name: String,
    /// Absolute or relative path to the SKILL.md file.
    pub path: PathBuf,
    /// Raw file contents.
    pub content: String,
}

/// Similarity band for an enhanced-analysis pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Band {
    Critical,
    High,
    Moderate,
}

impl Band {
    /// The band label as used in report headings.
    pub fn label(self) -> &'static str {
        match self {
            Band::Critical => "CRITICAL",
            Band::High => "HIGH",
            Band::Moderate => "MODERATE",
        }
    }

    /// The recommended action string (ported from the shell).
    pub fn action(self) -> &'static str {
        match self {
            Band::Critical => "IMMEDIATE MERGE REQUIRED",
            Band::High => "Review for aggregation",
            Band::Moderate => "Consider consolidation",
        }
    }
}

/// A basic (line-overlap) duplication result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BasicPair {
    pub name1: String,
    pub name2: String,
    pub similarity: u32,
    pub common_lines: usize,
}

/// An enhanced (composite) duplication result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EnhancedPair {
    pub name1: String,
    pub name2: String,
    pub composite: u32,
    pub semantic: u32,
    pub structural: u32,
    pub lexical: u32,
    pub band: Band,
}

/// Recursively find every `SKILL.md` under `root`, excluding any path under a
/// `.deprecated/` directory, sorted by path (ports the shell
/// `find ... -name SKILL.md -not -path "*/.deprecated/*" | sort`).
pub fn find_skills(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect_skill_files(root, &mut out);
    out.retain(|p| !p.to_string_lossy().contains("/.deprecated/"));
    out.sort();
    out
}

fn collect_skill_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(ft) = entry.file_type() else { continue };
        if ft.is_dir() {
            collect_skill_files(&path, out);
        } else if ft.is_file() && path.file_name().is_some_and(|n| n == "SKILL.md") {
            out.push(path);
        }
    }
}

/// Load the discovered skills into [`SkillDoc`]s, skipping any file that cannot
/// be read as UTF-8.
pub fn load_skills(root: &Path) -> Vec<SkillDoc> {
    find_skills(root)
        .into_iter()
        .filter_map(|path| {
            let content = std::fs::read_to_string(&path).ok()?;
            let name = skill_name(&path);
            Some(SkillDoc {
                name,
                path,
                content,
            })
        })
        .collect()
}

/// `basename(dirname(path))` - the skill's own directory name.
fn skill_name(skill_md: &Path) -> String {
    skill_md
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string()
}

/// Line count equivalent to `wc -l` for newline-terminated files (every real
/// SKILL.md ends in a newline, so `lines().count()` matches).
fn line_count(s: &str) -> usize {
    s.lines().count()
}

/// Build a multiset (line -> count) of a file's lines.
fn line_multiset(s: &str) -> HashMap<&str, usize> {
    let mut m: HashMap<&str, usize> = HashMap::new();
    for l in s.lines() {
        *m.entry(l).or_insert(0) += 1;
    }
    m
}

/// Count of lines common to both multisets, summing `min(count)` per line -
/// equivalent to `comm -12` over two `sort`ed (non-unique) files.
fn multiset_common(a: &HashMap<&str, usize>, b: &HashMap<&str, usize>) -> usize {
    a.iter()
        .map(|(k, &av)| b.get(k).map_or(0, |&bv| av.min(bv)))
        .sum()
}

/// Basic line-overlap similarity (percent): `common * 100 / avg(total_lines)`,
/// integer arithmetic throughout to match the shell.
pub fn basic_similarity(a: &str, b: &str) -> (u32, usize) {
    let ma = line_multiset(a);
    let mb = line_multiset(b);
    let common = multiset_common(&ma, &mb);
    let avg = (line_count(a) + line_count(b)) / 2;
    if avg == 0 {
        return (0, common);
    }
    ((common as u32 * 100) / avg as u32, common)
}

/// Run the basic algorithm across all skill pairs, returning pairs at or above
/// `threshold`, sorted by similarity descending then by name.
pub fn detect_basic(skills: &[SkillDoc], threshold: u32) -> Vec<BasicPair> {
    let mut pairs = Vec::new();
    for i in 0..skills.len() {
        for j in (i + 1)..skills.len() {
            let (similarity, common_lines) =
                basic_similarity(&skills[i].content, &skills[j].content);
            if similarity > threshold {
                pairs.push(BasicPair {
                    name1: skills[i].name.clone(),
                    name2: skills[j].name.clone(),
                    similarity,
                    common_lines,
                });
            }
        }
    }
    sort_pairs(&mut pairs, |p| (p.similarity, &p.name1, &p.name2));
    pairs
}

/// Jaccard similarity (percent) over two sets of feature strings:
/// `common * 100 / union`, integer arithmetic.
fn jaccard(a: &[String], b: &[String]) -> u32 {
    use std::collections::HashSet;
    let sa: HashSet<&String> = a.iter().collect();
    let sb: HashSet<&String> = b.iter().collect();
    let common = sa.intersection(&sb).count();
    let union = sa.len() + sb.len() - common;
    if union == 0 {
        0
    } else {
        (common as u32 * 100) / union as u32
    }
}

/// Extracted, comparable views of a skill used by the enhanced algorithm.
struct Features {
    semantic: Vec<String>,
    structural: Vec<String>,
    lexical: Vec<String>,
}

/// Extract the semantic feature set: header text, technical terms, quoted/code
/// spans, and the first 20 bullet items, de-duplicated.
///
/// NOTE: the shell's header `sed 's/^#+\s*//'` was a no-op on most lines (BRE
/// `#+` is a literal `#+`, not "one or more `#`"); we strip the intended
/// leading `#`+whitespace. The term regex drops the shell's `grep -i`, which on
/// `[A-Z]` classes matched any case and appears unintentional.
fn semantic_features(content: &str, term_re: &Regex, quote_re: &Regex) -> Vec<String> {
    let mut set: Vec<String> = Vec::new();

    for line in content.lines() {
        // Headers: lines starting with one or more '#'.
        if line.starts_with('#') {
            let text = line.trim_start_matches('#').trim();
            if !text.is_empty() {
                set.push(text.to_string());
            }
        }
    }

    // Technical terms (camelCase / snake-or-kebab / ALLCAPS).
    for m in term_re.find_iter(content) {
        set.push(m.as_str().to_string());
    }

    // Quoted strings and inline code spans, with the quote/backtick removed.
    for m in quote_re.find_iter(content) {
        let stripped: String = m
            .as_str()
            .chars()
            .filter(|c| *c != '`' && *c != '"')
            .collect();
        if !stripped.is_empty() {
            set.push(stripped);
        }
    }

    // First 20 bullet items, marker stripped.
    let mut bullets = 0;
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('-') || trimmed.starts_with('*') {
            let text = trimmed[1..].trim_start().to_string();
            set.push(text);
            bullets += 1;
            if bullets >= 20 {
                break;
            }
        }
    }

    dedup_sorted(set)
}

/// Extract the structural feature lines: header-depth pattern, per-section line
/// counts, and code-block / list / link counts.
fn structural_features(content: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    // Header hierarchy pattern, e.g. "# ## ###", as a single space-joined line.
    let hierarchy: Vec<String> = content
        .lines()
        .filter(|l| l.starts_with('#'))
        .map(|l| l.chars().take_while(|c| *c == '#').collect::<String>())
        .collect();
    out.push(hierarchy.join(" "));
    out.push(String::new()); // the shell emits a blank line here

    // Section lengths: count non-header lines between headers, printed when > 0
    // (ports the awk counter; the trailing section is never emitted, matching
    // the shell, which only prints on encountering the next header).
    let mut counts: Vec<usize> = Vec::new();
    let mut len = 0usize;
    for line in content.lines() {
        if line.starts_with('#') {
            if len > 0 {
                counts.push(len);
            }
            len = 0;
        } else {
            len += 1;
        }
    }
    counts.sort_unstable();
    for c in counts {
        out.push(c.to_string());
    }

    // Code-block / list / link counts (single numbers, as their own lines).
    let code_blocks = content.lines().filter(|l| l.starts_with("```")).count();
    let lists = content
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            t.starts_with('-') || t.starts_with('*') || t.starts_with('+')
        })
        .count();
    let link_re = Regex::new(r"\[.*\]\(.*\)").expect("static link regex is valid");
    let links = content.lines().filter(|l| link_re.is_match(l)).count();
    out.push(code_blocks.to_string());
    out.push(lists.to_string());
    out.push(links.to_string());

    out
}

/// Structural similarity (percent): `common_lines * 100 / avg(line_count)`,
/// comparing the structural feature lines as a multiset (ports
/// `comm -12` over `sort`ed structural files).
fn structural_similarity(a: &[String], b: &[String]) -> u32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let mut ma: HashMap<&str, usize> = HashMap::new();
    for l in a {
        *ma.entry(l.as_str()).or_insert(0) += 1;
    }
    let mut mb: HashMap<&str, usize> = HashMap::new();
    for l in b {
        *mb.entry(l.as_str()).or_insert(0) += 1;
    }
    let common: usize = ma
        .iter()
        .map(|(k, &av)| mb.get(k).map_or(0, |&bv| av.min(bv)))
        .sum();
    let total = (a.len() + b.len()) / 2;
    if total == 0 {
        0
    } else {
        (common as u32 * 100) / total as u32
    }
}

/// Lexical feature set: lowercased pure-alpha words of length >= 3, unique.
fn lexical_features(content: &str) -> Vec<String> {
    let mut set: Vec<String> = content
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() >= 3 && w.chars().all(|c| c.is_ascii_lowercase()))
        .collect();
    dedup_sorted(std::mem::take(&mut set))
}

fn dedup_sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

fn extract_features(content: &str, term_re: &Regex, quote_re: &Regex) -> Features {
    Features {
        semantic: semantic_features(content, term_re, quote_re),
        structural: structural_features(content),
        lexical: lexical_features(content),
    }
}

/// Classify a composite score into a band, or `None` when below the moderate
/// floor.
fn classify(composite: u32) -> Option<Band> {
    if composite >= CRITICAL_THRESHOLD {
        Some(Band::Critical)
    } else if composite >= HIGH_THRESHOLD {
        Some(Band::High)
    } else if composite >= MODERATE_THRESHOLD {
        Some(Band::Moderate)
    } else {
        None
    }
}

/// Run the enhanced composite algorithm across all skill pairs, returning pairs
/// at or above the moderate floor, sorted by composite descending then by name.
pub fn detect_enhanced(skills: &[SkillDoc]) -> Vec<EnhancedPair> {
    // Term regex: camelCase / PascalCase, snake-or-kebab, and ALLCAPS runs.
    let term_re = Regex::new(r"\b[A-Z][a-z]*[A-Z][a-zA-Z]*\b|\b[a-z]+[_-][a-z]+\b|\b[A-Z]{2,}\b")
        .expect("static term regex is valid");
    let quote_re = Regex::new(r#"`[^`]+`|"[^"]*""#).expect("static quote regex is valid");

    let features: Vec<Features> = skills
        .iter()
        .map(|s| extract_features(&s.content, &term_re, &quote_re))
        .collect();

    let mut pairs = Vec::new();
    for i in 0..skills.len() {
        for j in (i + 1)..skills.len() {
            let semantic = jaccard(&features[i].semantic, &features[j].semantic);
            let structural =
                structural_similarity(&features[i].structural, &features[j].structural);
            let lexical = jaccard(&features[i].lexical, &features[j].lexical);
            let composite = (semantic * WEIGHT_SEMANTIC
                + structural * WEIGHT_STRUCTURAL
                + lexical * WEIGHT_LEXICAL)
                / 100;
            if let Some(band) = classify(composite) {
                pairs.push(EnhancedPair {
                    name1: skills[i].name.clone(),
                    name2: skills[j].name.clone(),
                    composite,
                    semantic,
                    structural,
                    lexical,
                    band,
                });
            }
        }
    }
    sort_pairs(&mut pairs, |p| (p.composite, &p.name1, &p.name2));
    pairs
}

/// Sort pairs by a `(score, name1, name2)` key: score descending, names
/// ascending as a stable tie-break.
fn sort_pairs<T, F>(pairs: &mut [T], key: F)
where
    F: Fn(&T) -> (u32, &String, &String),
{
    pairs.sort_by(|a, b| {
        let (sa, a1, a2) = key(a);
        let (sb, b1, b2) = key(b);
        sb.cmp(&sa).then(a1.cmp(b1)).then(a2.cmp(b2))
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_content_is_fully_similar() {
        let a = "# Title\nline one\nline two\nline three\n";
        let (sim, common) = basic_similarity(a, a);
        assert_eq!(sim, 100);
        assert_eq!(common, 4);
    }

    #[test]
    fn disjoint_content_is_zero() {
        let a = "aaa\nbbb\nccc\n";
        let b = "xxx\nyyy\nzzz\n";
        let (sim, common) = basic_similarity(a, b);
        assert_eq!(sim, 0);
        assert_eq!(common, 0);
    }

    #[test]
    fn partial_overlap_uses_average_denominator() {
        // a has 4 lines, b has 4 lines, 2 in common -> avg 4, 2*100/4 = 50.
        let a = "shared one\nshared two\na only\na only 2\n";
        let b = "shared one\nshared two\nb only\nb only 2\n";
        let (sim, common) = basic_similarity(a, b);
        assert_eq!(common, 2);
        assert_eq!(sim, 50);
    }

    #[test]
    fn detect_basic_filters_and_sorts() {
        let skills = vec![
            SkillDoc {
                name: "alpha".into(),
                path: "a/SKILL.md".into(),
                content: "one\ntwo\nthree\nfour\n".into(),
            },
            SkillDoc {
                name: "beta".into(),
                path: "b/SKILL.md".into(),
                content: "one\ntwo\nthree\nfour\n".into(),
            },
            SkillDoc {
                name: "gamma".into(),
                path: "c/SKILL.md".into(),
                content: "wholly\ndifferent\ncontent\nhere\n".into(),
            },
        ];
        let pairs = detect_basic(&skills, MODERATE_THRESHOLD);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].name1, "alpha");
        assert_eq!(pairs[0].name2, "beta");
        assert_eq!(pairs[0].similarity, 100);
    }

    #[test]
    fn classify_bands() {
        assert_eq!(classify(60), Some(Band::Critical));
        assert_eq!(classify(50), Some(Band::Critical));
        assert_eq!(classify(35), Some(Band::High));
        assert_eq!(classify(20), Some(Band::Moderate));
        assert_eq!(classify(19), None);
    }

    #[test]
    fn enhanced_flags_near_identical_skills() {
        let body = "# Overview\n\nThis skill handles deployment automation.\n\n## Usage\n\n- run the deploy command\n- check the status output\n\n`deploy --now`\n";
        let skills = vec![
            SkillDoc {
                name: "deploy-one".into(),
                path: "a/SKILL.md".into(),
                content: body.to_string(),
            },
            SkillDoc {
                name: "deploy-two".into(),
                path: "b/SKILL.md".into(),
                content: body.to_string(),
            },
        ];
        let pairs = detect_enhanced(&skills);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].band, Band::Critical);
        assert_eq!(pairs[0].composite, 100);
    }

    #[test]
    fn jaccard_basic() {
        let a = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let b = vec!["y".to_string(), "z".to_string(), "w".to_string()];
        // common = 2 (y,z), union = 4 -> 50
        assert_eq!(jaccard(&a, &b), 50);
    }

    #[test]
    fn skill_name_is_parent_dir() {
        assert_eq!(
            skill_name(Path::new("skills/domain/my-skill/SKILL.md")),
            "my-skill"
        );
    }
}
