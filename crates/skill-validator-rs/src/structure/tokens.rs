//! Token counting and size thresholds (Go `structure/tokens.go`). Token counts
//! come from `tiktoken-rs` `o200k_base()` via `encode_ordinary`, which is
//! bit-exact against the Go tool's `tiktoken-go` `O200kBase.Encode` (proven by
//! the S1 token-parity gate). Directory iteration is sorted to match Go's
//! `os.ReadDir` / `filepath.Walk` lexical ordering.

use std::path::Path;
use std::sync::LazyLock;

use tiktoken_rs::CoreBPE;

use super::fsutil::{ext_lower, is_binary_ext, read_dir_sorted, read_file, rel_slash, walk_files};
use super::Options;
use crate::types::{ResultContext, TokenCount, ValidationResult};
use crate::util::format_number;

const REF_FILE_SOFT_LIMIT: usize = 10_000;
const REF_FILE_HARD_LIMIT: usize = 25_000;
const REF_TOTAL_SOFT_LIMIT: usize = 25_000;
const REF_TOTAL_HARD_LIMIT: usize = 50_000;
const OTHER_TOTAL_SOFT_LIMIT: usize = 25_000;
const OTHER_TOTAL_HARD_LIMIT: usize = 100_000;

static ENCODER: LazyLock<CoreBPE> =
    LazyLock::new(|| tiktoken_rs::o200k_base().expect("load o200k_base"));

/// Count `o200k_base` tokens in `text` (Go `enc.Encode(text)` length).
pub fn count_tokens(text: &str) -> usize {
    ENCODER.encode_ordinary(text).len()
}

const TEXT_ASSET_EXTENSIONS: &[&str] = &[
    ".md", ".tex", ".py", ".yaml", ".yml", ".tsx", ".ts", ".jsx", ".sty", ".mplstyle", ".ipynb",
];

const STANDARD_ROOT_FILES: &[&str] = &["skill.md"];
const STANDARD_DIRS: &[&str] = &["references", "scripts", "assets"];

/// Count tokens across SKILL.md body, references, root files (flat layouts),
/// non-standard files, and text assets (Go `structure.CheckTokens`).
pub fn check_tokens(
    dir: &Path,
    body: &str,
    opts: &Options,
) -> (Vec<ValidationResult>, Vec<TokenCount>, Vec<TokenCount>) {
    let ctx = ResultContext::new("Tokens");
    let mut results = Vec::new();
    let mut counts = Vec::new();

    // SKILL.md body.
    let body_count = count_tokens(body);
    counts.push(TokenCount {
        file: "SKILL.md body".to_string(),
        tokens: body_count,
    });
    if body_count > 5000 {
        results.push(ctx.warn_file(
            "SKILL.md",
            format!("SKILL.md body is {body_count} tokens (spec recommends < 5000)"),
        ));
    }
    let line_count = body.matches('\n').count() + 1;
    if line_count > 500 {
        results.push(ctx.warn_file(
            "SKILL.md",
            format!("SKILL.md body is {line_count} lines (spec recommends < 500)"),
        ));
    }

    // references/
    let mut ref_total = 0usize;
    let refs_dir = dir.join("references");
    for e in read_dir_sorted(&refs_dir) {
        if e.is_dir || e.name.starts_with('.') {
            continue;
        }
        let rel = format!("references/{}", e.name);
        let Some(data) = read_file(&e.path) else {
            results.push(ctx.warn_file(&rel, format!("could not read {rel}")));
            continue;
        };
        let file_tokens = count_tokens(&data);
        counts.push(TokenCount {
            file: rel.clone(),
            tokens: file_tokens,
        });
        ref_total += file_tokens;
        push_ref_file_limit(&ctx, &mut results, &rel, file_tokens);
    }

    // Flat layouts: root-level text files count as standard content.
    if opts.allow_flat_layouts {
        for rc in count_root_files(dir) {
            let tokens = rc.tokens;
            let file = rc.file.clone();
            counts.push(rc);
            ref_total += tokens;
            push_ref_file_limit(&ctx, &mut results, &file, tokens);
        }
    }

    // Aggregate reference limits.
    if ref_total > REF_TOTAL_HARD_LIMIT {
        results.push(ctx.error(format!(
            "total reference files: {ref_total} tokens — this will consume 25-40% of a typical \
             context window; reduce content or split into a skill with fewer references"
        )));
    } else if ref_total > REF_TOTAL_SOFT_LIMIT {
        results.push(ctx.warn(format!(
            "total reference files: {ref_total} tokens — agents may load multiple references \
             in one session, consider whether all this content is essential"
        )));
    }

    // Non-standard ("other") files.
    let other_counts = count_other_files(dir, opts);
    let other_total: usize = other_counts.iter().map(|c| c.tokens).sum();
    if other_total > OTHER_TOTAL_HARD_LIMIT {
        results.push(ctx.error(format!(
            "non-standard files total {other_total} tokens — if an agent loads these, \
             they will consume most of the context window and severely degrade performance; \
             move essential content into references/ or remove unnecessary files"
        )));
    } else if other_total > OTHER_TOTAL_SOFT_LIMIT {
        results.push(ctx.warn(format!(
            "non-standard files total {other_total} tokens — if an agent loads these, \
             they could consume a significant portion of the context window; \
             consider moving essential content into references/ or removing unnecessary files"
        )));
    }

    // Text assets, appended last.
    counts.extend(count_asset_files(dir));

    (results, counts, other_counts)
}

fn push_ref_file_limit(
    ctx: &ResultContext,
    results: &mut Vec<ValidationResult>,
    rel: &str,
    file_tokens: usize,
) {
    if file_tokens > REF_FILE_HARD_LIMIT {
        results.push(ctx.error_file(
            rel,
            format!(
                "{rel} is {file_tokens} tokens — this will consume 12-20% of a typical context \
                 window and meaningfully degrade agent performance; split into smaller focused files"
            ),
        ));
    } else if file_tokens > REF_FILE_SOFT_LIMIT {
        results.push(ctx.warn_file(
            rel,
            format!(
                "{rel} is {file_tokens} tokens — consider splitting into smaller focused files \
                 so agents load only what they need"
            ),
        ));
    }
}

fn count_asset_files(dir: &Path) -> Vec<TokenCount> {
    let assets_dir = dir.join("assets");
    let mut files = Vec::new();
    walk_files(&assets_dir, &mut files);
    let mut counts = Vec::new();
    for path in files {
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if !TEXT_ASSET_EXTENSIONS.contains(&ext_lower(&name).as_str()) {
            continue;
        }
        let Some(data) = read_file(&path) else { continue };
        counts.push(TokenCount {
            file: rel_slash(dir, &path),
            tokens: count_tokens(&data),
        });
    }
    counts
}

fn count_other_files(dir: &Path, opts: &Options) -> Vec<TokenCount> {
    let mut counts = Vec::new();
    for e in read_dir_sorted(dir) {
        if e.name.starts_with('.') {
            continue;
        }
        if e.is_dir {
            if STANDARD_DIRS.contains(&e.name.to_lowercase().as_str()) {
                continue;
            }
            let mut files = Vec::new();
            walk_files(&e.path, &mut files);
            for path in files {
                let fname = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                if is_binary_ext(&fname) {
                    continue;
                }
                let Some(data) = read_file(&path) else { continue };
                counts.push(TokenCount {
                    file: rel_slash(dir, &path),
                    tokens: count_tokens(&data),
                });
            }
        } else {
            if STANDARD_ROOT_FILES.contains(&e.name.to_lowercase().as_str()) || opts.allow_flat_layouts {
                continue;
            }
            if is_binary_ext(&e.name) {
                continue;
            }
            let Some(data) = read_file(&e.path) else { continue };
            counts.push(TokenCount {
                file: e.name.clone(),
                tokens: count_tokens(&data),
            });
        }
    }
    counts
}

fn count_root_files(dir: &Path) -> Vec<TokenCount> {
    let mut counts = Vec::new();
    for e in read_dir_sorted(dir) {
        if e.is_dir || e.name.starts_with('.') {
            continue;
        }
        if STANDARD_ROOT_FILES.contains(&e.name.to_lowercase().as_str()) {
            continue;
        }
        if is_binary_ext(&e.name) {
            continue;
        }
        let Some(data) = read_file(&e.path) else { continue };
        counts.push(TokenCount {
            file: e.name.clone(),
            tokens: count_tokens(&data),
        });
    }
    counts
}

/// Skill-ratio holistic check (Go `structure.checkSkillRatio`).
pub fn check_skill_ratio(standard: &[TokenCount], other: &[TokenCount]) -> Vec<ValidationResult> {
    let ctx = ResultContext::new("Overall");
    let standard_total: usize = standard.iter().map(|c| c.tokens).sum();
    let other_total: usize = other.iter().map(|c| c.tokens).sum();

    if other_total > 25_000 && standard_total > 0 && other_total > standard_total * 10 {
        return vec![ctx.error(format!(
            "this content doesn't appear to be structured as a skill — \
             there are {} tokens of non-standard content but only {} tokens in the \
             standard skill structure (SKILL.md + references). This ratio suggests a \
             build pipeline issue or content that belongs in a different format, not a skill. \
             Per the spec, a skill should contain a focused SKILL.md with optional references, \
             scripts, and assets.",
            format_number(other_total as i64),
            format_number(standard_total as i64),
        ))];
    }
    Vec::new()
}
