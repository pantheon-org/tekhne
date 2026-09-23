//! Classify an unfaceted tag into a taxonomy facet, and check an append-only
//! taxonomy policy for shrinkage.
//!
//! Port of the journal CLI's `classify-tag.ts` and `check-append-only.ts`.
//! Together with `lint::lint`'s existing threshold/ticket-pattern-based
//! unfaceted-tag detection, this is the logic behind `taxonomy sync`: every
//! tag `lint` already flags as unfaceted gets classified here, a confident
//! classification is auto-appended to its facet, an ambiguous one is left for
//! a human to resolve via `taxonomy add`.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::lint::edit_distance;
use crate::taxonomy::Taxonomy;

/// How confidently a tag was classified into a facet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ClassifyConfidence {
    Confident,
    Ambiguous,
}

/// The outcome of classifying one tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassifyResult {
    pub facet: Option<String>,
    pub confidence: ClassifyConfidence,
    pub why: String,
}

const PROVENANCE_MARKERS: &[&str] = &["weekly-journal", "ticket-detail", "archive", "novamedia"];

const COMPLIANCE_KEYWORDS: &[&str] = &[
    "gdpr",
    "sar",
    "privacy",
    "anonymis",
    "deletion",
    "retention",
    "consent",
    "compliance",
    "blocking",
];
const PROCESS_KEYWORDS: &[&str] = &[
    "ci-cd",
    "pipeline",
    "deploy",
    "logging",
    "alert",
    "investigation",
    "review",
    "validation",
    "migration",
    "automat",
    "consolidat",
    "observ",
    "architect",
    "security",
    "credential",
    "permission",
    "regex",
    "production",
    "documentation",
    "proof-of-concept",
];
const TECH_KEYWORDS: &[&str] = &[
    "aws", "sql", "docker", "queue", "database", "cloud", "server", "kibana", "jvm", "iam", "ec2",
    "rds", "cdk",
];
const TOPIC_SUFFIXES: &[&str] = &["-manager", "-service", "-engine", "-gateway", "-portal"];

const EDIT_DISTANCE_THRESHOLD: usize = 1;
const MIN_LENGTH_FOR_FUZZY_MATCH: usize = 4;

fn is_year(tag: &str) -> bool {
    tag.len() == 4
        && tag.starts_with(['1', '2'])
        && tag.chars().all(|c| c.is_ascii_digit())
        && matches!(&tag[0..2], "19" | "20")
}

fn facet_of(tag: &str, taxonomy: &Taxonomy) -> Option<String> {
    taxonomy
        .facets
        .iter()
        .find(|(_, tags)| tags.iter().any(|t| t == tag))
        .map(|(facet, _)| facet.clone())
}

fn near_facet_matches(tag: &str, taxonomy: &Taxonomy) -> Vec<String> {
    if tag.chars().count() < MIN_LENGTH_FOR_FUZZY_MATCH {
        return Vec::new();
    }
    let mut facets = BTreeSet::new();
    for (facet, tags) in &taxonomy.facets {
        for known in tags {
            if known.chars().count() < MIN_LENGTH_FOR_FUZZY_MATCH {
                continue;
            }
            if edit_distance(tag, known) <= EDIT_DISTANCE_THRESHOLD {
                facets.insert(facet.clone());
                break;
            }
        }
    }
    facets.into_iter().collect()
}

fn lexicon_facets(tag: &str) -> Vec<String> {
    let mut facets = BTreeSet::new();
    if COMPLIANCE_KEYWORDS.iter().any(|k| tag.contains(k)) {
        facets.insert("compliance".to_string());
    }
    if PROCESS_KEYWORDS.iter().any(|k| tag.contains(k)) {
        facets.insert("process".to_string());
    }
    if TECH_KEYWORDS.iter().any(|k| tag.contains(k)) {
        facets.insert("tech".to_string());
    }
    if TOPIC_SUFFIXES.iter().any(|s| tag.ends_with(s)) {
        facets.insert("topic".to_string());
    }
    facets.into_iter().collect()
}

/// Deterministically classify a tag into a facet. Rules run in strict
/// priority order and the first strong signal wins:
///
/// 1. alias -> the canonical tag's facet (reuse, not a new tag)
/// 2. 4-digit year or a provenance marker -> `meta`
/// 3. exact, then near (edit-distance <= 1, both sides >= 4 chars), match to
///    an already-faceted tag -> that facet. This runs *before* the lexicon
///    rules below on purpose: it is what stops "api-gateway" (already
///    `tech`) from being reclassified into `topic` by the "-gateway" suffix
///    rule.
/// 4. lexicon keyword/suffix rules, evaluated independently per facet
///
/// A step that finds signal for more than one facet, or the lexicon step
/// finding none, resolves to `ambiguous` rather than guessing.
pub fn classify_tag(tag: &str, taxonomy: &Taxonomy) -> ClassifyResult {
    if let Some(alias) = taxonomy.aliases.get(tag) {
        if let Some(facet) = facet_of(alias, taxonomy) {
            return ClassifyResult {
                facet: Some(facet.clone()),
                confidence: ClassifyConfidence::Confident,
                why: format!("alias of \"{alias}\", already in \"{facet}\""),
            };
        }
    }

    if is_year(tag) || PROVENANCE_MARKERS.contains(&tag) {
        return ClassifyResult {
            facet: Some("meta".to_string()),
            confidence: ClassifyConfidence::Confident,
            why: "year or provenance marker".to_string(),
        };
    }

    if let Some(facet) = facet_of(tag, taxonomy) {
        return ClassifyResult {
            facet: Some(facet.clone()),
            confidence: ClassifyConfidence::Confident,
            why: format!("already assigned to \"{facet}\""),
        };
    }

    let near = near_facet_matches(tag, taxonomy);
    if near.len() == 1 {
        let facet = near[0].clone();
        return ClassifyResult {
            why: format!("near match to an existing \"{facet}\" tag"),
            facet: Some(facet),
            confidence: ClassifyConfidence::Confident,
        };
    }
    if near.len() > 1 {
        return ClassifyResult {
            facet: None,
            confidence: ClassifyConfidence::Ambiguous,
            why: format!("near matches span multiple facets: {}", near.join(", ")),
        };
    }

    let lexicon = lexicon_facets(tag);
    if lexicon.len() == 1 {
        let facet = lexicon[0].clone();
        return ClassifyResult {
            why: format!("lexicon match for \"{facet}\""),
            facet: Some(facet),
            confidence: ClassifyConfidence::Confident,
        };
    }
    if lexicon.len() > 1 {
        return ClassifyResult {
            facet: None,
            confidence: ClassifyConfidence::Ambiguous,
            why: format!(
                "lexicon signals span multiple facets: {}",
                lexicon.join(", ")
            ),
        };
    }

    ClassifyResult {
        facet: None,
        confidence: ClassifyConfidence::Ambiguous,
        why: "no rule matched".to_string(),
    }
}

/// A violation of the append-only taxonomy invariant: the taxonomy at
/// `appendOnlyBaseline` may only grow (aliases and facet assignments added,
/// never removed or reassigned).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AppendOnlyViolation {
    pub kind: AppendOnlyViolationKind,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppendOnlyViolationKind {
    AliasRemoved,
    AliasChanged,
    FacetShrink,
    TagInTwoFacets,
}

/// Compare `baseline` against `current`, reporting every violation found
/// rather than stopping at the first.
pub fn check_append_only(baseline: &Taxonomy, current: &Taxonomy) -> Vec<AppendOnlyViolation> {
    let mut violations = Vec::new();

    for (tag, canonical) in &baseline.aliases {
        match current.aliases.get(tag) {
            None => violations.push(AppendOnlyViolation {
                kind: AppendOnlyViolationKind::AliasRemoved,
                detail: format!("alias \"{tag}\" -> \"{canonical}\" was removed"),
            }),
            Some(current_canonical) if current_canonical != canonical => {
                violations.push(AppendOnlyViolation {
                    kind: AppendOnlyViolationKind::AliasChanged,
                    detail: format!(
                        "alias \"{tag}\" changed from \"{canonical}\" to \"{current_canonical}\""
                    ),
                })
            }
            _ => {}
        }
    }

    for (facet, tags) in &baseline.facets {
        let current_tags: BTreeSet<&String> = current
            .facets
            .get(facet)
            .map(|t| t.iter().collect())
            .unwrap_or_default();
        for tag in tags {
            if !current_tags.contains(tag) {
                violations.push(AppendOnlyViolation {
                    kind: AppendOnlyViolationKind::FacetShrink,
                    detail: format!("tag \"{tag}\" removed from facet \"{facet}\""),
                });
            }
        }
    }

    let mut seen_in: std::collections::BTreeMap<&String, &String> =
        std::collections::BTreeMap::new();
    for (facet, tags) in &current.facets {
        for tag in tags {
            match seen_in.get(tag) {
                Some(already) if *already != facet => {
                    violations.push(AppendOnlyViolation {
                        kind: AppendOnlyViolationKind::TagInTwoFacets,
                        detail: format!(
                            "tag \"{tag}\" appears in both \"{already}\" and \"{facet}\""
                        ),
                    });
                }
                _ => {
                    seen_in.insert(tag, facet);
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn taxonomy(facets: &[(&str, &[&str])], aliases: &[(&str, &str)]) -> Taxonomy {
        Taxonomy {
            schema: None,
            threshold: 3,
            aliases: aliases
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            facets: facets
                .iter()
                .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
                .collect(),
            pin: Vec::new(),
            suppress: Vec::new(),
            ticket_pattern: "^x-[0-9]+$".to_string(),
            append_only_baseline: None,
        }
    }

    #[test]
    fn classify_year_is_confident_meta() {
        let tax = taxonomy(&[("meta", &[])], &[]);
        let r = classify_tag("2026", &tax);
        assert_eq!(r.facet.as_deref(), Some("meta"));
        assert_eq!(r.confidence, ClassifyConfidence::Confident);
    }

    #[test]
    fn classify_alias_resolves_to_canonical_facet() {
        let tax = taxonomy(&[("tech", &["aws"])], &[("amazon-web-services", "aws")]);
        let r = classify_tag("amazon-web-services", &tax);
        assert_eq!(r.facet.as_deref(), Some("tech"));
        assert_eq!(r.confidence, ClassifyConfidence::Confident);
    }

    #[test]
    fn classify_near_match_single_facet_is_confident() {
        let tax = taxonomy(&[("tech", &["docker"])], &[]);
        let r = classify_tag("dockerr", &tax);
        assert_eq!(r.facet.as_deref(), Some("tech"));
        assert_eq!(r.confidence, ClassifyConfidence::Confident);
    }

    #[test]
    fn classify_lexicon_single_match_is_confident() {
        let tax = taxonomy(&[("tech", &[])], &[]);
        let r = classify_tag("aws-lambda", &tax);
        assert_eq!(r.facet.as_deref(), Some("tech"));
        assert_eq!(r.confidence, ClassifyConfidence::Confident);
    }

    #[test]
    fn classify_no_signal_is_ambiguous() {
        let tax = taxonomy(&[("tech", &[])], &[]);
        let r = classify_tag("zzz-nonsense-tag", &tax);
        assert_eq!(r.facet, None);
        assert_eq!(r.confidence, ClassifyConfidence::Ambiguous);
    }

    #[test]
    fn append_only_detects_removed_alias() {
        let baseline = taxonomy(&[], &[("foo", "bar")]);
        let current = taxonomy(&[], &[]);
        let violations = check_append_only(&baseline, &current);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, AppendOnlyViolationKind::AliasRemoved);
    }

    #[test]
    fn append_only_detects_changed_alias() {
        let baseline = taxonomy(&[], &[("foo", "bar")]);
        let current = taxonomy(&[], &[("foo", "baz")]);
        let violations = check_append_only(&baseline, &current);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, AppendOnlyViolationKind::AliasChanged);
    }

    #[test]
    fn append_only_detects_facet_shrink() {
        let baseline = taxonomy(&[("tech", &["aws", "docker"])], &[]);
        let current = taxonomy(&[("tech", &["aws"])], &[]);
        let violations = check_append_only(&baseline, &current);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, AppendOnlyViolationKind::FacetShrink);
    }

    #[test]
    fn append_only_detects_tag_in_two_facets() {
        let baseline = taxonomy(&[], &[]);
        let current = taxonomy(&[("tech", &["aws"]), ("topic", &["aws"])], &[]);
        let violations = check_append_only(&baseline, &current);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, AppendOnlyViolationKind::TagInTwoFacets);
    }

    #[test]
    fn append_only_growth_is_clean() {
        let baseline = taxonomy(&[("tech", &["aws"])], &[]);
        let current = taxonomy(&[("tech", &["aws", "docker"])], &[]);
        assert!(check_append_only(&baseline, &current).is_empty());
    }
}
