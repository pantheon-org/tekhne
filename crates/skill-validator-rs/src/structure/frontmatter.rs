//! Frontmatter validation (Go `structure/frontmatter.go`): required fields,
//! byte-length limits, the name pattern, keyword-stuffing heuristics, and
//! unrecognised/metadata field checks. Length limits are byte lengths, matching
//! Go's `len(string)`. The `description: (N chars)` message is parsed verbatim
//! by the auditor, so it is emitted exactly.

use std::path::Path;
use std::sync::LazyLock;

use regex::{Captures, Regex};
use serde_yaml_ng::Value;

use super::Options;
use crate::skill::Skill;
use crate::types::{ResultContext, ValidationResult};
use crate::util::go_quote;

static NAME_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").expect("NAME_PATTERN"));

static QUOTED_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""[^"]*""#).expect("QUOTED_STRING"));

const MIN_QUOTED_STRINGS: usize = 5;
const MIN_COMMA_SEGMENTS: usize = 8;
const MAX_SHORT_SEGMENT_PCT: usize = 60;
const MIN_AVG_WORDS_PER_SEGMENT: usize = 3;

const PERIOD_PLACEHOLDER: &str = "\u{222f}";

static NON_BREAKING_ABBREVS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?-u:\b)(e\.g|i\.e|vs|al|approx|dept|govt|incl|assoc|avg|est|max|min|misc|ref|spec|tech)\.[ \t\n\x0C\r]",
    )
    .expect("NON_BREAKING_ABBREVS")
});

static DIGIT_PERIOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([0-9])\.([0-9])").expect("DIGIT_PERIOD"));

static SENTENCE_BOUNDARY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[.!?][ \t\n\x0C\r]+([A-Z])").expect("SENTENCE_BOUNDARY"));

/// Validate frontmatter of a parsed skill (Go `structure.CheckFrontmatter`).
pub fn check_frontmatter(s: &Skill, opts: &Options) -> Vec<ValidationResult> {
    let ctx = ResultContext::with_file("Frontmatter", "SKILL.md");
    let mut results: Vec<ValidationResult> = Vec::new();

    // name
    let name = &s.frontmatter.name;
    if name.is_empty() {
        results.push(ctx.error("name is required"));
    } else {
        if name.len() > 64 {
            results.push(ctx.error(format!("name exceeds 64 characters ({})", name.len())));
        }
        if !NAME_PATTERN.is_match(name) {
            results.push(ctx.error(format!(
                "name {} must be lowercase alphanumeric with hyphens, no leading/trailing/consecutive hyphens",
                go_quote(name)
            )));
        }
        let dir_name = base_name(&s.dir);
        if name != &dir_name {
            results.push(ctx.error(format!(
                "name does not match directory name (expected {}, got {})",
                go_quote(&dir_name),
                go_quote(name)
            )));
        }
        if results.is_empty() || NAME_PATTERN.is_match(name) {
            results.push(ctx.pass(format!("name: {} (valid)", go_quote(name))));
        }
    }

    // description
    let desc = &s.frontmatter.description;
    if desc.is_empty() {
        results.push(ctx.error("description is required"));
    } else if desc.len() > 1024 {
        results.push(ctx.error(format!(
            "description exceeds 1024 characters ({})",
            desc.len()
        )));
    } else if desc.trim().is_empty() {
        results.push(ctx.error("description must not be empty/whitespace-only"));
    } else {
        results.push(ctx.pass(format!("description: ({} chars)", desc.len())));
        results.extend(check_description_keyword_stuffing(&ctx, desc));
    }

    // license
    if !s.frontmatter.license.is_empty() {
        results.push(ctx.pass(format!("license: {}", go_quote(&s.frontmatter.license))));
    }

    // compatibility
    let compat = &s.frontmatter.compatibility;
    if !compat.is_empty() {
        if compat.len() > 500 {
            results.push(ctx.error(format!(
                "compatibility exceeds 500 characters ({})",
                compat.len()
            )));
        } else {
            results.push(ctx.pass(format!("compatibility: ({} chars)", compat.len())));
        }
    }

    // metadata (checked against the raw parse)
    if let Some(meta) = metadata_value(&s.raw_frontmatter) {
        match meta {
            Value::Mapping(map) => {
                let mut all_strings = true;
                let mut keys: Vec<(String, &Value)> = map
                    .iter()
                    .filter_map(|(k, v)| k.as_str().map(|k| (k.to_string(), v)))
                    .collect();
                keys.sort_by(|a, b| a.0.cmp(&b.0));
                for (k, v) in &keys {
                    if !matches!(v, Value::String(_)) {
                        results.push(
                            ctx.error(format!("metadata[{}] value must be a string", go_quote(k))),
                        );
                        all_strings = false;
                    }
                }
                if all_strings {
                    results.push(ctx.pass(format!("metadata: ({} entries)", map.len())));
                }
            }
            _ => {
                results.push(ctx.error("metadata must be a map of string keys to string values"));
            }
        }
    }

    // allowed-tools
    if !s.frontmatter.allowed_tools.is_empty() {
        results.push(ctx.pass(format!(
            "allowed-tools: {}",
            go_quote(&s.frontmatter.allowed_tools.value)
        )));
        if s.frontmatter.allowed_tools.was_list {
            results.push(ctx.info(
                "allowed-tools is a YAML list; the spec defines this as a space-delimited string — \
                 both are accepted, but a string is more portable across agent implementations",
            ));
        }
    }

    // unrecognised fields
    if !opts.allow_extra_frontmatter {
        for field in s.unrecognized_fields() {
            results.push(ctx.warn(format!("unrecognized field: {}", go_quote(&field))));
        }
    }

    results
}

/// Return the metadata value from the raw frontmatter, treating a missing key
/// or explicit null as absent (mirrors Go's `RawFrontmatter["metadata"] != nil`).
fn metadata_value(raw: &Value) -> Option<&Value> {
    match raw {
        Value::Mapping(map) => match map.get(Value::String("metadata".to_string())) {
            Some(Value::Null) | None => None,
            Some(v) => Some(v),
        },
        _ => None,
    }
}

/// Go `filepath.Base` for the skill dir string.
fn base_name(dir: &str) -> String {
    Path::new(dir)
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| ".".to_string())
}

fn check_description_keyword_stuffing(ctx: &ResultContext, desc: &str) -> Vec<ValidationResult> {
    // Heuristic 1: many quoted strings with little surrounding prose.
    let quote_count = QUOTED_STRING.find_iter(desc).count();
    if quote_count >= MIN_QUOTED_STRINGS {
        let prose = QUOTED_STRING.replace_all(desc, "");
        let mut prose_word_count = 0usize;
        for w in prose.split_whitespace() {
            let cleaned = w.trim_matches(|c: char| !c.is_ascii_alphanumeric());
            if !cleaned.is_empty() {
                prose_word_count += 1;
            }
        }
        if prose_word_count < quote_count {
            return vec![ctx.warn(format!(
                "description contains {quote_count} quoted strings with little surrounding prose — \
                 this looks like keyword stuffing; per the spec, the description should \
                 concisely describe what the skill does and when to use it, not just list trigger phrases"
            ))];
        }
    }

    // Heuristic 2: many short comma-separated segments in a single sentence.
    let without_quotes = QUOTED_STRING.replace_all(desc, "");
    for sentence in split_sentences(&without_quotes) {
        let segments: Vec<&str> = sentence
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .collect();
        if segments.len() < MIN_COMMA_SEGMENTS {
            continue;
        }
        let mut short_count = 0usize;
        let mut total_words = 0usize;
        for seg in &segments {
            let words = seg.split_whitespace().count();
            total_words += words;
            if words <= 3 {
                short_count += 1;
            }
        }
        if total_words >= MIN_AVG_WORDS_PER_SEGMENT * segments.len() {
            continue;
        }
        if short_count * 100 / segments.len() >= MAX_SHORT_SEGMENT_PCT {
            return vec![ctx.warn(format!(
                "description has {} comma-separated segments, most very short — \
                 this looks like a keyword list; per the spec, the description should \
                 concisely describe what the skill does and when to use it",
                segments.len()
            ))];
        }
    }

    Vec::new()
}

/// Lightweight sentence splitter (Go `structure.splitSentences`). Protects
/// abbreviation and numeric periods, then splits before capitalised words.
fn split_sentences(text: &str) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    let protected = NON_BREAKING_ABBREVS.replace_all(text, |caps: &Captures| {
        caps[0].replacen('.', PERIOD_PLACEHOLDER, 1)
    });
    let protected = DIGIT_PERIOD
        .replace_all(
            &protected,
            format!("${{1}}{PERIOD_PLACEHOLDER}${{2}}").as_str(),
        )
        .into_owned();

    let mut sentences = Vec::new();
    let mut start = 0usize;
    for caps in SENTENCE_BOUNDARY.captures_iter(&protected) {
        let cap_start = caps.get(1).map(|m| m.start()).unwrap_or(start);
        let s = protected[start..cap_start].trim();
        if !s.is_empty() {
            sentences.push(s.replace(PERIOD_PLACEHOLDER, "."));
        }
        start = cap_start;
    }
    if start < protected.len() {
        let s = protected[start..].trim();
        if !s.is_empty() {
            sentences.push(s.replace(PERIOD_PLACEHOLDER, "."));
        }
    }
    sentences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_sentences_protects_abbrev_and_numbers() {
        // Matches the Go TestSplitSentences cases: the abbreviation and the
        // version-number periods do not create sentence boundaries. Go protects
        // only the first period of "e.g."; the boundary regex additionally
        // requires a capital letter after the period, so "e.g. vector" never
        // splits regardless.
        let s = split_sentences("Use for e.g. vector search and embeddings. Next sentence here.");
        assert_eq!(
            s,
            vec![
                "Use for e.g. vector search and embeddings.".to_string(),
                "Next sentence here.".to_string(),
            ]
        );

        let v = split_sentences("Requires Node.js v18.0 or higher. Also supports Deno.");
        assert_eq!(
            v,
            vec![
                "Requires Node.js v18.0 or higher.".to_string(),
                "Also supports Deno.".to_string(),
            ]
        );

        // "etc." is intentionally NOT protected, so it is a boundary.
        let e = split_sentences("Handles auth, storage, etc. Supports caching.");
        assert_eq!(
            e,
            vec![
                "Handles auth, storage, etc.".to_string(),
                "Supports caching.".to_string(),
            ]
        );
    }

    #[test]
    fn split_sentences_empty() {
        assert!(split_sentences("").is_empty());
    }
}
