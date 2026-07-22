//! Cross-language contamination analysis (Go `contamination` package).
//!
//! Determinism note: in the Go source, `getLanguageCategories`,
//! `findPrimaryCategory`, and `findPrimaryApplicationCategory` iterate the
//! `languageCategories` map in Go's randomised order and `break` on the first
//! member match. Only `kotlin` is ambiguous (it belongs to both `java` and
//! `mobile`), so the Go tool is non-deterministic for that one identifier.
//! This port fixes a stable category order (the Go literal order) so `kotlin`
//! deterministically resolves to `java`. Fixtures containing `kotlin` are the
//! sole known parity caveat and are excluded from the golden corpus.

use std::collections::{BTreeMap, BTreeSet};

use crate::types::ContaminationReport;
use crate::util::round_to;

/// Tools whose skills legitimately use many languages (Go `multiInterfaceTools`).
const MULTI_INTERFACE_TOOLS: &[&str] = &[
    "mongodb",
    "aws",
    "docker",
    "kubernetes",
    "redis",
    "postgresql",
    "mysql",
    "elasticsearch",
    "firebase",
    "terraform",
    "graphql",
    "grpc",
    "kafka",
    "rabbitmq",
    "stripe",
];

/// Ordered language categories and their member identifiers (Go
/// `languageCategories`, in source-literal order for deterministic first-match).
const LANGUAGE_CATEGORIES: &[(&str, &[&str])] = &[
    (
        "shell",
        &[
            "bash",
            "shell",
            "sh",
            "zsh",
            "fish",
            "powershell",
            "cmd",
            "bat",
        ],
    ),
    (
        "javascript",
        &["javascript", "js", "typescript", "ts", "jsx", "tsx", "node"],
    ),
    ("python", &["python", "py", "python3"]),
    ("java", &["java", "kotlin", "scala", "groovy"]),
    ("systems", &["c", "cpp", "c++", "rust", "go", "zig"]),
    ("ruby", &["ruby", "rb"]),
    ("dotnet", &["csharp", "cs", "fsharp", "vb"]),
    (
        "config",
        &["yaml", "yml", "json", "toml", "ini", "xml", "hcl"],
    ),
    ("query", &["sql", "graphql", "cypher", "sparql"]),
    (
        "markup",
        &["html", "css", "scss", "sass", "less", "markdown", "md"],
    ),
    (
        "mobile",
        &["swift", "kotlin", "dart", "objective-c", "objc"],
    ),
];

/// Framework/runtime name -> language category (Go `techPatterns`).
const TECH_PATTERNS: &[(&str, &str)] = &[
    ("node.js", "javascript"),
    ("react", "javascript"),
    ("express", "javascript"),
    ("django", "python"),
    ("flask", "python"),
    ("fastapi", "python"),
    ("spring", "java"),
    ("rails", "ruby"),
    ("asp.net", "dotnet"),
    (".net", "dotnet"),
    ("swift", "mobile"),
    ("flutter", "mobile"),
];

const APPLICATION_CATEGORIES: &[&str] = &[
    "javascript",
    "python",
    "java",
    "systems",
    "ruby",
    "dotnet",
    "mobile",
];

const AUXILIARY_CATEGORIES: &[&str] = &["shell", "config", "query", "markup"];

fn is_application(cat: &str) -> bool {
    APPLICATION_CATEGORIES.contains(&cat)
}

fn is_auxiliary(cat: &str) -> bool {
    AUXILIARY_CATEGORIES.contains(&cat)
}

/// First category (in fixed order) whose members contain `lang`, optionally
/// restricted to application categories. Mirrors the Go `break`-on-first loop.
fn category_of(lang: &str, app_only: bool) -> Option<&'static str> {
    for (cat, members) in LANGUAGE_CATEGORIES {
        if app_only && !is_application(cat) {
            continue;
        }
        if members.contains(&lang) {
            return Some(cat);
        }
    }
    None
}

/// Similarity weight for a pair of categories (Go `mismatchWeight`).
fn mismatch_weight(cat1: &str, cat2: &str) -> f64 {
    let app1 = is_application(cat1);
    let app2 = is_application(cat2);
    if app1 && app2 {
        return 1.0;
    }
    let aux1 = is_auxiliary(cat1);
    let aux2 = is_auxiliary(cat2);
    if aux1 && aux2 {
        return 0.1;
    }
    if (app1 && aux2) || (aux1 && app2) {
        return 0.25;
    }
    1.0
}

/// Compute contamination metrics (Go `contamination.Analyze`).
pub fn analyze(name: &str, content: &str, code_languages: &[String]) -> ContaminationReport {
    let multi_tools = detect_multi_interface_tools(name, content);
    let lang_categories = get_language_categories(code_languages);
    let tech_refs = detect_technology_references(content);

    // Combined scope indicators (categories from code + tech references).
    let mut all_scopes: BTreeSet<String> = BTreeSet::new();
    all_scopes.extend(lang_categories.iter().cloned());
    all_scopes.extend(tech_refs.iter().cloned());
    let scope_breadth = all_scopes.len();

    let primary_category = find_primary_category(code_languages, false);
    let primary_app_category = find_primary_category(code_languages, true);

    let mut mismatched: BTreeSet<String> = BTreeSet::new();
    if let Some(primary_app) = &primary_app_category {
        for cat in &lang_categories {
            if cat != primary_app && is_application(cat) {
                mismatched.insert(cat.clone());
            }
        }
    }
    let language_mismatch = !mismatched.is_empty();

    let mut factors = 0.0;
    if !multi_tools.is_empty() {
        factors += 0.3;
    }

    let mut mismatch_weights: BTreeMap<String, f64> = BTreeMap::new();
    if language_mismatch {
        let mut weighted = 0.0;
        for cat in &mismatched {
            let w = mismatch_weight(primary_category.as_deref().unwrap_or(""), cat);
            mismatch_weights.insert(cat.clone(), w);
            weighted += w;
        }
        let severity = (weighted / 3.0).min(1.0);
        factors += 0.4 * severity;
    }

    if scope_breadth > 2 {
        let breadth_score = (((scope_breadth - 2) as f64) / 4.0).min(1.0);
        factors += 0.3 * breadth_score;
    }

    let score = round_to(factors.min(1.0), 4);
    let level = if score >= 0.5 {
        "high"
    } else if score >= 0.2 {
        "medium"
    } else {
        "low"
    };

    ContaminationReport {
        multi_interface_tools: multi_tools,
        code_languages: code_languages.to_vec(),
        language_categories: sorted_set(&lang_categories),
        primary_category: primary_category.unwrap_or_default(),
        mismatched_categories: mismatched.into_iter().collect(),
        mismatch_weights,
        language_mismatch,
        tech_references: tech_refs.into_iter().collect(),
        scope_breadth,
        contamination_score: score,
        contamination_level: level.to_string(),
    }
}

fn sorted_set(set: &BTreeSet<String>) -> Vec<String> {
    set.iter().cloned().collect()
}

fn detect_multi_interface_tools(name: &str, content: &str) -> Vec<String> {
    let name_lower = name.to_lowercase();
    let content_lower = content.to_lowercase();
    let mut matches: Vec<String> = Vec::new();
    for tool in MULTI_INTERFACE_TOOLS {
        if name_lower.contains(tool) || content_lower.contains(tool) {
            matches.push((*tool).to_string());
        }
    }
    matches.sort();
    matches
}

fn get_language_categories(languages: &[String]) -> BTreeSet<String> {
    let mut categories = BTreeSet::new();
    for lang in languages {
        if let Some(cat) = category_of(&lang.to_lowercase(), false) {
            categories.insert(cat.to_string());
        }
    }
    categories
}

fn detect_technology_references(content: &str) -> BTreeSet<String> {
    let content_lower = content.to_lowercase();
    let mut refs = BTreeSet::new();
    for (tech, category) in TECH_PATTERNS {
        if content_lower.contains(tech) {
            refs.insert((*category).to_string());
        }
    }
    refs
}

/// Most common category by document order, ties broken by first appearance
/// (Go `findPrimaryCategory` / `findPrimaryApplicationCategory`).
fn find_primary_category(code_languages: &[String], app_only: bool) -> Option<String> {
    if code_languages.is_empty() {
        return None;
    }
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for lang in code_languages {
        if let Some(cat) = category_of(&lang.to_lowercase(), app_only) {
            *counts.entry(cat.to_string()).or_insert(0) += 1;
            if seen.insert(cat.to_string()) {
                order.push(cat.to_string());
            }
        }
    }
    if counts.is_empty() {
        return None;
    }
    let mut max_count = 0;
    let mut primary = String::new();
    for cat in &order {
        let c = counts[cat];
        if c > max_count {
            max_count = c;
            primary = cat.clone();
        }
    }
    Some(primary)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn langs(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn auxiliary_only_is_low() {
        let r = analyze(
            "aux",
            "```bash\nx\n```",
            &langs(&["bash", "yaml", "json", "sh"]),
        );
        assert_eq!(r.contamination_level, "low");
        assert!(!r.language_mismatch);
        assert_eq!(r.primary_category, "shell");
    }

    #[test]
    fn app_to_app_mismatch() {
        let r = analyze("mix", "code", &langs(&["python", "python", "java"]));
        assert!(r.language_mismatch);
        assert_eq!(r.primary_category, "python");
        assert_eq!(r.mismatched_categories, vec!["java".to_string()]);
        assert_eq!(r.mismatch_weights.get("java"), Some(&1.0));
    }

    #[test]
    fn multi_interface_tool_detected() {
        let r = analyze("aws-helper", "use aws cli", &langs(&["python"]));
        assert_eq!(r.multi_interface_tools, vec!["aws".to_string()]);
        // 0.3 from the tool alone -> medium.
        assert_eq!(r.contamination_level, "medium");
    }
}
