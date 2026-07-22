//! Small string and filesystem helpers shared across dimension scorers.
//! Ported from the Go scorer's `util.go` and the helper functions in
//! `dimensions.go`.

use regex::Regex;
use std::path::Path;

/// Count case-insensitive, non-overlapping substring occurrences (Go
/// `countPattern`).
pub fn count_pattern(content: &str, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    let lower = content.to_lowercase();
    let pat = pattern.to_lowercase();
    let mut count = 0;
    let mut start = 0;
    while let Some(idx) = lower[start..].find(&pat) {
        count += 1;
        start += idx + pat.len();
    }
    count
}

/// Count non-empty (after trimming) lines (Go `countLines`). Retained from the
/// ported helper surface and exercised by unit tests; not on the scoring path.
#[allow(dead_code)]
pub fn count_lines(content: &str) -> usize {
    content
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .count()
}

/// Parse a YAML frontmatter field value between `---` delimiters (Go
/// `extractFrontmatterField`).
pub fn extract_frontmatter_field(content: &str, field: &str) -> String {
    let mut in_frontmatter = false;
    let mut fm_started = false;
    for line in content.split('\n') {
        let trimmed = line.trim_end_matches('\r');
        if trimmed == "---" {
            if !fm_started {
                fm_started = true;
                in_frontmatter = true;
                continue;
            }
            break;
        }
        if in_frontmatter {
            let prefix = format!("{field}:");
            if let Some(rest) = trimmed.strip_prefix(&prefix) {
                let val = rest.trim();
                return val.trim_matches(|c| c == '"' || c == '\'').to_string();
            }
        }
    }
    String::new()
}

/// Count triple-backtick fence pairs (Go `codeBlockCount`).
pub fn code_block_count(content: &str) -> usize {
    let mut count = 0;
    for line in content.split('\n') {
        if line.trim_start().starts_with("```") {
            count += 1;
        }
    }
    count / 2
}

/// Strip fenced code blocks, mirroring the awk snippet in the Go source (Go
/// `removeCodeBlocks`).
pub fn remove_code_blocks(content: &str) -> String {
    let mut result = String::new();
    let mut skip = false;
    for line in content.split('\n') {
        if line.trim().starts_with("```") {
            skip = !skip;
            continue;
        }
        if !skip {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

/// Match `content` against a regex pattern (inline flags carry any case
/// sensitivity). Returns false if the pattern fails to compile (Go
/// `matchesRegexCI`).
pub fn matches_regex(content: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .map(|re| re.is_match(content))
        .unwrap_or(false)
}

/// True if the path exists and is a regular file (Go `scorer.fileExists`).
pub fn file_exists(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.is_file())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_lines_ignores_blank() {
        let content = "line one\n\nline three\nline four\n";
        assert_eq!(count_lines(content), 3);
    }

    #[test]
    fn count_pattern_is_case_insensitive() {
        assert_eq!(count_pattern("NEVER do this. Never again.", "never"), 2);
    }

    #[test]
    fn remove_code_blocks_strips_body_keeps_prose() {
        let content = "before\n```\ncode line\n```\nafter\n";
        let result = remove_code_blocks(content);
        assert!(!result.contains("code line"));
        assert!(result.contains("before"));
        assert!(result.contains("after"));
    }
}
