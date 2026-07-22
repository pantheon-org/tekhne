//! Link extraction from markdown (Go `links/extract.go`). Only the offline
//! extraction surface is ported; external HTTP validation (`links/check.go`,
//! `links/safenet.go`) is out of scope (network, non-deterministic) per the S1
//! spec.

use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use crate::util;

/// `[text](url)` markdown links; url is capture group 2.
static MD_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").expect("MD_LINK"));

/// Bare `http(s)://` URLs; url is capture group 1. `\s` is spelled out to match
/// Go's RE2 `\s` exactly; `^` (no multiline) matches only the start of text.
static BARE_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("(?:^|[ \t\n\x0C\r])(https?://[^ \t\n\x0C\r<>)`]+)").expect("BARE_URL")
});

/// Trailing HTML entity references (Go `entitySuffix`).
static ENTITY_SUFFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"&[a-zA-Z0-9]+;$").expect("ENTITY_SUFFIX"));

/// Extract all unique links from a markdown body (Go `ExtractLinks`).
pub fn extract_links(body: &str) -> Vec<String> {
    let cleaned = util::CODE_BLOCK_STRIP.replace_all(body, "");
    let cleaned = util::INLINE_CODE_STRIP.replace_all(&cleaned, "");

    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut links: Vec<String> = Vec::new();

    for cap in MD_LINK.captures_iter(&cleaned) {
        if let Some(m) = cap.get(2) {
            let url = m.as_str().trim().to_string();
            if seen.insert(url.clone()) {
                links.push(url);
            }
        }
    }

    for cap in BARE_URL.captures_iter(&cleaned) {
        if let Some(m) = cap.get(1) {
            let url = trim_trailing_delimiters(m.as_str().trim());
            if seen.insert(url.clone()) {
                links.push(url);
            }
        }
    }

    links
}

const TRAILING_PUNCT: &[char] = &['?', '!', '.', ',', ':', '*', '_', '~', '\'', '"', ';', '<'];

/// Strip trailing punctuation and entity references from a bare URL,
/// following cmark-gfm autolink rules (Go `trimTrailingDelimiters`).
fn trim_trailing_delimiters(url: &str) -> String {
    let mut url = url.to_string();
    // Trim repeatedly until an iteration makes no change (reaches the `break`).
    loop {
        if url.ends_with(';') {
            if let Some(m) = ENTITY_SUFFIX.find(&url) {
                url.truncate(m.start());
                continue;
            }
        }

        if url.ends_with(')') {
            let open = url.matches('(').count();
            let close = url.matches(')').count();
            if close > open {
                url.pop();
                continue;
            }
        }

        if let Some(last) = url.chars().last() {
            if TRAILING_PUNCT.contains(&last) {
                url.pop();
                continue;
            }
        }

        break;
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_and_bare_urls_deduped_in_order() {
        let body = "See [guide](references/guide.md) and https://example.com/page.\nAlso [guide](references/guide.md).";
        let links = extract_links(body);
        assert_eq!(
            links,
            vec![
                "references/guide.md".to_string(),
                "https://example.com/page".to_string(),
            ]
        );
    }

    #[test]
    fn urls_in_code_are_ignored() {
        let body = "`https://in-code.example` and ```\nhttps://fenced.example\n```";
        assert!(extract_links(body).is_empty());
    }

    #[test]
    fn trailing_entity_and_punct_trimmed() {
        assert_eq!(
            trim_trailing_delimiters("https://x.com/a&amp;"),
            "https://x.com/a"
        );
        assert_eq!(
            trim_trailing_delimiters("https://x.com/a."),
            "https://x.com/a"
        );
        assert_eq!(
            trim_trailing_delimiters("https://x.com/a(b)"),
            "https://x.com/a(b)"
        );
        assert_eq!(
            trim_trailing_delimiters("https://x.com/a)"),
            "https://x.com/a"
        );
    }
}
