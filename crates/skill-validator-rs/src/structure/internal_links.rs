//! Internal (relative) link checking (Go `structure/links.go`
//! `CheckInternalLinks`). Broken relative links are a structural issue. Path
//! resolution replicates Go's lexical `filepath.Clean`/`Join` and its
//! traversal guard. External HTTP link checking is out of scope (network).

use std::io::ErrorKind;
use std::path::Path;

use crate::links::extract_links;
use crate::types::{ResultContext, ValidationResult};

/// Validate relative links in the skill body (Go `CheckInternalLinks`). Results
/// use the "Structure" category (not "Links"), matching the Go tool.
pub fn check_internal_links(dir: &Path, body: &str) -> Vec<ValidationResult> {
    let ctx = ResultContext::with_file("Structure", "SKILL.md");
    let all_links = extract_links(body);
    if all_links.is_empty() {
        return Vec::new();
    }

    let dir_str = dir.to_string_lossy().into_owned();
    let clean_dir = clean(&dir_str);
    let prefix = format!("{clean_dir}/");

    let mut results = Vec::new();
    for link in all_links {
        if link.contains('{') {
            continue; // RFC 6570 URI template
        }
        if link.starts_with("http://") || link.starts_with("https://") {
            continue;
        }
        if link.starts_with("mailto:") || link.starts_with('#') {
            continue;
        }
        let link = link.split('#').next().unwrap_or("").to_string();
        if link.is_empty() {
            continue;
        }

        let resolved = join_clean(&dir_str, &link);
        if !resolved.starts_with(&prefix) {
            results.push(ctx.error(format!("internal link escapes skill directory: {link}")));
            continue;
        }
        if is_not_exist(&resolved) {
            results.push(ctx.error(format!("broken internal link: {link} (file not found)")));
        } else {
            results.push(ctx.pass(format!("internal link: {link} (exists)")));
        }
    }
    results
}

fn is_not_exist(path: &str) -> bool {
    match Path::new(path).metadata() {
        Ok(_) => false,
        Err(e) => e.kind() == ErrorKind::NotFound,
    }
}

/// Go `filepath.Join(a, b)` on Unix: clean(a + "/" + b).
fn join_clean(a: &str, b: &str) -> String {
    if a.is_empty() {
        return clean(b);
    }
    if b.is_empty() {
        return clean(a);
    }
    clean(&format!("{a}/{b}"))
}

/// Lexical path cleaning equivalent to Go's `filepath.Clean` (Unix semantics):
/// collapse slashes, drop `.`, resolve `..` against prior real segments.
fn clean(path: &str) -> String {
    if path.is_empty() {
        return ".".to_string();
    }
    let rooted = path.starts_with('/');
    let mut out: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => continue,
            ".." => {
                if let Some(&last) = out.last() {
                    if last != ".." {
                        out.pop();
                        continue;
                    }
                }
                if !rooted {
                    out.push("..");
                }
                // At root, ".." is dropped.
            }
            _ => out.push(seg),
        }
    }

    let joined = out.join("/");
    let result = if rooted {
        format!("/{joined}")
    } else if joined.is_empty() {
        ".".to_string()
    } else {
        joined
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_matches_go_semantics() {
        assert_eq!(clean("/a/b/../c"), "/a/c");
        assert_eq!(clean("/a/./b//c"), "/a/b/c");
        assert_eq!(clean("a/b/.."), "a");
        assert_eq!(clean("/.."), "/");
        assert_eq!(clean(""), ".");
    }

    #[test]
    fn join_clean_resolves_relative() {
        assert_eq!(
            join_clean("/skills/x", "references/g.md"),
            "/skills/x/references/g.md"
        );
        assert_eq!(join_clean("/skills/x", "../y/g.md"), "/skills/y/g.md");
    }
}
