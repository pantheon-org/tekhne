//! Unclosed-code-fence detection (Go `structure/markdown.go`). Fence scanning
//! is byte-oriented, matching the Go implementation exactly.

use std::path::Path;

use super::fsutil::{read_dir_sorted, read_file};
use crate::types::{ResultContext, ValidationResult};

/// Check SKILL.md and reference `.md` files for unclosed fences
/// (Go `structure.CheckMarkdown`).
pub fn check_markdown(dir: &Path, body: &str) -> Vec<ValidationResult> {
    let ctx = ResultContext::new("Markdown");
    let mut results = Vec::new();

    if let Some(line) = find_unclosed_fence(body) {
        results.push(ctx.error_at_line(
            "SKILL.md",
            line,
            format!(
                "SKILL.md has an unclosed code fence starting at line {line} — this may cause \
                 agents to misinterpret everything after it as code"
            ),
        ));
    }

    let refs_dir = dir.join("references");
    let entries = read_dir_sorted(&refs_dir);
    if entries.is_empty() && !refs_dir.is_dir() {
        if results.is_empty() {
            results.push(ctx.pass("no unclosed code fences found"));
        }
        return results;
    }

    for e in entries {
        if e.is_dir || e.name.starts_with('.') {
            continue;
        }
        if !e.name.to_lowercase().ends_with(".md") {
            continue;
        }
        let Some(data) = read_file(&e.path) else { continue };
        let rel = format!("references/{}", e.name);
        if let Some(line) = find_unclosed_fence(&data) {
            results.push(ctx.error_at_line(
                &rel,
                line,
                format!(
                    "{rel} has an unclosed code fence starting at line {line} — this may cause \
                     agents to misinterpret everything after it as code"
                ),
            ));
        }
    }

    if results.is_empty() {
        results.push(ctx.pass("no unclosed code fences found"));
    }
    results
}

/// Find the 1-based line of an unclosed `` ``` `` or `~~~` fence, if any
/// (Go `structure.FindUnclosedFence`).
pub fn find_unclosed_fence(content: &str) -> Option<usize> {
    let mut in_fence = false;
    let mut fence_char = 0u8;
    let mut fence_len = 0usize;
    let mut fence_line = 0usize;

    for (i, line) in content.split('\n').enumerate() {
        // Strip up to 3 leading spaces.
        let mut stripped = line.as_bytes();
        for _ in 0..3 {
            if !stripped.is_empty() && stripped[0] == b' ' {
                stripped = &stripped[1..];
            } else {
                break;
            }
        }

        if !in_fence {
            if let Some((ch, n)) = fence_prefix(stripped) {
                if n >= 3 {
                    in_fence = true;
                    fence_char = ch;
                    fence_len = n;
                    fence_line = i + 1;
                }
            }
        } else if let Some((ch, n)) = fence_prefix(stripped) {
            if n >= fence_len && ch == fence_char {
                // Closing fence: the remainder of the line must be blank.
                let rest = &stripped[n..];
                if rest.iter().all(u8::is_ascii_whitespace) {
                    in_fence = false;
                }
            }
        }
    }

    if in_fence {
        Some(fence_line)
    } else {
        None
    }
}

/// Return the fence character and its run length when a line starts with 3+
/// backticks or tildes (Go `structure.fencePrefix`).
fn fence_prefix(line: &[u8]) -> Option<(u8, usize)> {
    let ch = *line.first()?;
    if ch != b'`' && ch != b'~' {
        return None;
    }
    let n = line.iter().take_while(|&&b| b == ch).count();
    if n < 3 {
        return None;
    }
    Some((ch, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_unclosed_backtick_fence() {
        assert_eq!(find_unclosed_fence("intro\n```rust\ncode\nmore"), Some(2));
    }

    #[test]
    fn closed_fence_is_ok() {
        assert_eq!(find_unclosed_fence("```\ncode\n```\n"), None);
    }

    #[test]
    fn closing_shorter_than_opening_stays_open() {
        // opening ````(4), closing ```(3) < 4 -> still open.
        assert_eq!(find_unclosed_fence("````\ncode\n```\n"), Some(1));
    }

    #[test]
    fn tilde_and_backtick_do_not_cross_close() {
        assert_eq!(find_unclosed_fence("~~~\ncode\n```\n"), Some(1));
    }
}
