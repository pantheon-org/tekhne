//! Content quality analysis (Go `content` package). Computes word/code/sentence
//! metrics and directive-marker ratios. All floats are rounded to 4dp, matching
//! the Go tool. Regexes use `(?-u:...)` where Go relies on ASCII `\w`/`\b`, and
//! `\s` is spelled out as `[ \t\n\x0C\r]` to match Go's RE2 `\s` exactly
//! (Rust's ASCII `\s` would also include the vertical tab `\x0B`).

use std::sync::LazyLock;

use regex::Regex;

use crate::types::ContentReport;
use crate::util::{self, round_to};

/// Strong directive-language markers (Go `content.strongMarkerRes`).
const STRONG_MARKERS: &[&str] = &[
    r"must",
    r"always",
    r"never",
    r"shall",
    r"required",
    r"do not",
    r"don't",
    r"ensure",
    r"critical",
    r"mandatory",
];

/// Weak/advisory-language markers (Go `content.weakMarkerRes`).
const WEAK_MARKERS: &[&str] = &[
    r"may",
    r"consider",
    r"could",
    r"might",
    r"optional",
    r"possibly",
    r"suggested",
    r"prefer",
    r"try to",
    r"if possible",
];

fn compile_markers(words: &[&str]) -> Vec<Regex> {
    words
        .iter()
        .map(|w| Regex::new(&format!(r"(?-u:\b{w}\b)")).expect("marker regex"))
        .collect()
}

static STRONG_RES: LazyLock<Vec<Regex>> = LazyLock::new(|| compile_markers(STRONG_MARKERS));
static WEAK_RES: LazyLock<Vec<Regex>> = LazyLock::new(|| compile_markers(WEAK_MARKERS));

/// Common imperative verbs (Go `content.imperativeVerbs`).
const IMPERATIVE_VERBS: &[&str] = &[
    "use",
    "run",
    "create",
    "add",
    "set",
    "install",
    "configure",
    "write",
    "read",
    "check",
    "verify",
    "make",
    "build",
    "test",
    "ensure",
    "include",
    "remove",
    "delete",
    "update",
    "call",
    "import",
    "export",
    "define",
    "implement",
    "return",
    "pass",
    "handle",
    "parse",
    "generate",
    "format",
    "validate",
    "convert",
    "follow",
    "apply",
    "start",
    "stop",
    "avoid",
    "keep",
    "do",
    "execute",
    "open",
    "close",
    "save",
    "load",
    "send",
    "receive",
];

static CODE_LANG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?-u:(?:```|~~~)(\w+))").expect("CODE_LANG"));

static SENTENCE_SPLIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[.!?][ \t\n\x0C\r]+|[.!?]$|\n\n+").expect("SENTENCE_SPLIT"));

static LEADING_FORMAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[#*>\-\t\n\x0C\r ]+").expect("LEADING_FORMAT"));

static SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^#{2,}[ \t\n\x0C\r]+").expect("SECTION"));

static LIST_ITEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^[ \t\n\x0C\r]*[-*+][ \t\n\x0C\r]+|^[ \t\n\x0C\r]*[0-9]+\.[ \t\n\x0C\r]+")
        .expect("LIST_ITEM")
});

/// Compute content metrics for SKILL.md content (Go `content.Analyze`).
pub fn analyze(content: &str) -> ContentReport {
    if content.trim().is_empty() {
        return ContentReport::default();
    }

    let word_count = content.split_whitespace().count();

    // Code block analysis.
    let mut code_block_count = 0usize;
    let mut code_block_words = 0usize;
    for cap in util::CODE_BLOCK_PATTERN.captures_iter(content) {
        code_block_count += 1;
        if let Some(body) = cap.get(1) {
            code_block_words += body.as_str().split_whitespace().count();
        }
    }
    let code_block_ratio = if word_count > 0 {
        code_block_words as f64 / word_count as f64
    } else {
        0.0
    };

    // Code languages (document order).
    let code_languages: Vec<String> = CODE_LANG
        .captures_iter(content)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect();

    // Sentence analysis.
    let sentences = count_sentences(content);
    let sentence_count = sentences.len();
    let imperative_count = count_imperative_sentences(&sentences);
    let imperative_ratio = if sentence_count > 0 {
        imperative_count as f64 / sentence_count as f64
    } else {
        0.0
    };

    // Information density.
    let information_density = if code_block_count > 0 {
        code_block_ratio * 0.5 + imperative_ratio * 0.5
    } else {
        imperative_ratio
    };

    // Marker analysis.
    let strong_count = count_marker_matches(content, &STRONG_RES);
    let weak_count = count_marker_matches(content, &WEAK_RES);
    let total_markers = strong_count + weak_count;
    let instruction_specificity = if total_markers > 0 {
        strong_count as f64 / total_markers as f64
    } else {
        0.0
    };

    let section_count = SECTION.find_iter(content).count();
    let list_item_count = LIST_ITEM.find_iter(content).count();

    ContentReport {
        word_count,
        code_block_count,
        code_block_ratio: round_to(code_block_ratio, 4),
        code_languages,
        sentence_count,
        imperative_count,
        imperative_ratio: round_to(imperative_ratio, 4),
        information_density: round_to(information_density, 4),
        strong_markers: strong_count,
        weak_markers: weak_count,
        instruction_specificity: round_to(instruction_specificity, 4),
        section_count,
        list_item_count,
    }
}

fn count_sentences(text: &str) -> Vec<String> {
    let stripped = util::CODE_BLOCK_STRIP.replace_all(text, "");
    let stripped = util::INLINE_CODE_STRIP.replace_all(&stripped, "");
    let mut sentences = Vec::new();
    for part in SENTENCE_SPLIT.split(&stripped) {
        let s = part.trim();
        // Go: `len(s) > 5` is a BYTE-length check.
        if !s.is_empty() && s.len() > 5 {
            sentences.push(s.to_string());
        }
    }
    sentences
}

fn count_imperative_sentences(sentences: &[String]) -> usize {
    let mut count = 0;
    for sentence in sentences {
        let cleaned = LEADING_FORMAT.replace(sentence, "");
        if let Some(first) = cleaned.split_whitespace().next() {
            if IMPERATIVE_VERBS.contains(&first.to_lowercase().as_str()) {
                count += 1;
            }
        }
    }
    count
}

fn count_marker_matches(text: &str, patterns: &[Regex]) -> usize {
    let lower = text.to_lowercase();
    patterns.iter().map(|re| re.find_iter(&lower).count()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_content_is_default() {
        assert_eq!(analyze("   \n  "), ContentReport::default());
    }

    #[test]
    fn code_languages_preserve_document_order() {
        let md = "```python\nx\n```\ntext\n```bash\ny\n```";
        let r = analyze(md);
        assert_eq!(r.code_languages, vec!["python", "bash"]);
        assert_eq!(r.code_block_count, 2);
    }

    #[test]
    fn markers_and_specificity() {
        let md = "You must always do this. You may consider that.";
        let r = analyze(md);
        // strong: must, always -> 2 (plain "do" is not a strong marker; only
        // "do not" is) ; weak: may, consider -> 2
        assert_eq!(r.strong_markers, 2);
        assert_eq!(r.weak_markers, 2);
        assert_eq!(r.instruction_specificity, round_to(2.0 / 4.0, 4));
    }

    #[test]
    fn ascii_word_boundary_ignores_substrings() {
        // "must" inside "mustache" must NOT match (ASCII \b).
        let r = analyze("The mustache is optional here now.");
        assert_eq!(r.strong_markers, 0);
        assert_eq!(r.weak_markers, 1); // "optional"
    }
}
