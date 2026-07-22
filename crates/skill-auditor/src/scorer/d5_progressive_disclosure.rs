//! D5 — Progressive Disclosure (max 15). Ported from
//! `d5_progressive_disclosure.go`.

use super::bridge::ValidatorBridge;
use super::helpers::matches_regex;
use std::path::Path;

/// Score the Progressive Disclosure dimension (score only). Mirrors the Go
/// `scoreD5` wrapper and is exercised by unit tests.
#[allow(dead_code)]
pub fn score(content: &str, skill_dir: &Path, b: &ValidatorBridge) -> i32 {
    score_with_meta(content, skill_dir, b).0
}

/// Score plus metadata used in the [`super::Result`]: `(score, lines,
/// ref_count, has_refs)`. Token thresholds are calibrated at ~8 tokens/line;
/// falls back to line count when the library cannot produce a token count.
pub fn score_with_meta(
    content: &str,
    skill_dir: &Path,
    b: &ValidatorBridge,
) -> (i32, i32, i32, bool) {
    let mut has_refs = false;
    let mut ref_count = 0;

    let refs_dir = skill_dir.join("references");
    if refs_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&refs_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    continue;
                }
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.ends_with(".md") && !name.starts_with('.') {
                    has_refs = true;
                    ref_count += 1;
                }
            }
        }
    }

    // Go uses len(strings.Split(content, "\n")): total line count including
    // blank lines, i.e. one more than the number of newlines.
    let lines = content.split('\n').count() as i32;

    let tokens = b.skill_md_tokens();
    if tokens > 0 {
        score_by_tokens(tokens, lines, ref_count, has_refs)
    } else {
        score_by_lines(lines, ref_count, has_refs)
    }
}

/// Token-threshold scoring path (Go `scoreD5ByTokens`).
pub fn score_by_tokens(
    tokens: i32,
    lines: i32,
    ref_count: i32,
    has_refs: bool,
) -> (i32, i32, i32, bool) {
    let score = if has_refs {
        if tokens < 800 {
            15
        } else if tokens < 1200 {
            13
        } else if tokens < 1600 {
            11
        } else {
            10
        }
    } else if tokens < 1200 {
        12
    } else if tokens < 2400 {
        10
    } else if tokens < 4000 {
        7
    } else {
        5
    };
    (score, lines, ref_count, has_refs)
}

/// Line-count scoring path (Go `scoreD5ByLines`).
fn score_by_lines(lines: i32, ref_count: i32, has_refs: bool) -> (i32, i32, i32, bool) {
    let score = if has_refs {
        if lines < 100 {
            15
        } else if lines < 150 {
            13
        } else if lines < 200 {
            11
        } else {
            10
        }
    } else if lines < 200 {
        12
    } else if lines < 300 {
        10
    } else if lines < 500 {
        7
    } else {
        5
    };
    (score, lines, ref_count, has_refs)
}

/// True when `## References` is the last H2 and at least one bullet link is
/// present (Go `isReferenceSectionCompliant`).
pub fn is_reference_section_compliant(content: &str) -> bool {
    let mut last_h2 = "";
    for line in content.split('\n') {
        if let Some(rest) = line.strip_prefix("## ") {
            last_h2 = rest;
        }
    }
    last_h2.trim() == "References" && matches_regex(content, r"(?m)^- \[.+\]\(.+\)")
}

#[cfg(test)]
mod tests {
    use super::super::bridge::ValidatorBridge;
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn nil_bridge() -> ValidatorBridge {
        ValidatorBridge::default()
    }

    fn make_lines(n: usize) -> String {
        let mut s = String::from("---\ndescription: x\n---\n");
        for _ in 0..n {
            s.push_str("line content here\n");
        }
        s
    }

    fn write_ref_file(skill_dir: &Path, name: &str, content: &str) {
        let dir = skill_dir.join("references");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), content).unwrap();
    }

    #[test]
    fn with_refs_short() {
        let dir = tempdir().unwrap();
        write_ref_file(dir.path(), "deep.md", "# Deep Reference");
        assert_eq!(score(&make_lines(50), dir.path(), &nil_bridge()), 15);
    }

    #[test]
    fn with_refs_medium() {
        let dir = tempdir().unwrap();
        write_ref_file(dir.path(), "deep.md", "# Deep Reference");
        assert_eq!(score(&make_lines(120), dir.path(), &nil_bridge()), 13);
    }

    #[test]
    fn with_refs_large() {
        let dir = tempdir().unwrap();
        write_ref_file(dir.path(), "deep.md", "# Deep Reference");
        for (lines, want) in [(160, 11), (210, 10)] {
            assert_eq!(score(&make_lines(lines), dir.path(), &nil_bridge()), want);
        }
    }

    #[test]
    fn no_refs_short() {
        let content = "---\ndescription: x\n---\n# Short skill";
        assert_eq!(score(content, tempdir().unwrap().path(), &nil_bridge()), 12);
    }

    #[test]
    fn no_refs_medium() {
        assert_eq!(
            score(&make_lines(250), tempdir().unwrap().path(), &nil_bridge()),
            10
        );
    }

    #[test]
    fn no_refs_large() {
        assert_eq!(
            score(&make_lines(350), tempdir().unwrap().path(), &nil_bridge()),
            7
        );
    }

    #[test]
    fn no_refs_very_large() {
        assert_eq!(
            score(&make_lines(510), tempdir().unwrap().path(), &nil_bridge()),
            5
        );
    }

    #[test]
    fn token_path_with_refs() {
        assert_eq!(score_by_tokens(600, 50, 1, true).0, 15);
        assert_eq!(score_by_tokens(1000, 120, 1, true).0, 13);
        assert_eq!(score_by_tokens(1400, 160, 1, true).0, 11);
        assert_eq!(score_by_tokens(2000, 200, 1, true).0, 10);
    }

    #[test]
    fn token_path_no_refs() {
        for (tokens, want) in [(800, 12), (1500, 10), (3000, 7), (5000, 5)] {
            assert_eq!(score_by_tokens(tokens, 100, 0, false).0, want);
        }
    }
}
