//! Table-parity test: proves the Rust agent table reproduces the TypeScript
//! source of truth (`cli/lib/types/agents.ts` and `types.ts`).
//!
//! The TypeScript is embedded at compile time and parsed here. Expected values
//! are derived from the TS `join(...)`/`existsSync(...)` expressions and
//! compared against the Rust table resolved under an identical synthetic
//! environment, so the two cannot silently drift.

use std::path::{Path, PathBuf};

use skill_install::{agents, Environment};

const AGENTS_TS: &str = include_str!("../../../cli/lib/types/agents.ts");
const TYPES_TS: &str = include_str!("../../../cli/lib/types/types.ts");

// Synthetic base directories used on both sides of the comparison.
const HOME: &str = "/home/user";
const CONFIG: &str = "/home/user/.config";
const CODEX: &str = "/home/user/.codex";
const CLAUDE: &str = "/home/user/.claude";
const CWD: &str = "/work/project";

fn synthetic_env() -> Environment {
    Environment::resolve(PathBuf::from(HOME), PathBuf::from(CWD), None, None, None)
}

/// Return the content between the outer parentheses whose `(` is at `open_idx`.
fn balanced(s: &str, open_idx: usize) -> &str {
    let bytes = s.as_bytes();
    let mut depth = 0usize;
    for i in open_idx..s.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return &s[open_idx + 1..i];
                }
            }
            _ => {}
        }
    }
    &s[open_idx + 1..]
}

/// Split on commas that sit at parenthesis depth zero.
fn split_top_commas(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '(' => {
                depth += 1;
                cur.push(c);
            }
            ')' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => {
                parts.push(cur.trim().to_string());
                cur.clear();
            }
            other => cur.push(other),
        }
    }
    if !cur.trim().is_empty() {
        parts.push(cur.trim().to_string());
    }
    parts
}

fn unquote(s: &str) -> String {
    s.trim().trim_matches('"').to_string()
}

/// Resolve a single `join` argument (a variable, `process.cwd()`, or literal).
fn eval_piece(piece: &str) -> String {
    let p = piece.trim();
    match p {
        "home" => HOME.to_string(),
        "configHome" => CONFIG.to_string(),
        "codexHome" => CODEX.to_string(),
        "claudeHome" => CLAUDE.to_string(),
        "process.cwd()" => CWD.to_string(),
        _ if p.starts_with('"') => unquote(p),
        _ => panic!("unhandled join piece: {p}"),
    }
}

/// Resolve a TS path expression (a `join(...)`, a bare base variable, a string
/// literal, or the OpenClaw helper) using the synthetic environment.
fn eval_expr(expr: &str) -> String {
    let e = expr.trim().trim_end_matches(',').trim();
    match e {
        "claudeHome" => return CLAUDE.to_string(),
        "codexHome" => return CODEX.to_string(),
        // No .openclaw/.clawdbot/.moltbot exist in the synthetic env, so the
        // helper falls back to its default branch.
        "getOpenClawGlobalSkillsDir()" => return format!("{HOME}/.openclaw/skills"),
        _ => {}
    }
    if e.starts_with('"') {
        return unquote(e);
    }
    if let Some(idx) = e.find('(') {
        if e[..idx].trim() == "join" {
            let inner = balanced(e, idx);
            let parts: Vec<String> = split_top_commas(inner)
                .iter()
                .map(|p| eval_piece(p))
                .collect();
            return parts.join("/");
        }
    }
    panic!("unhandled path expression: {e}");
}

/// The TS object literal for one agent, from its `name:` line to the next one.
fn agent_block(slug: &str) -> String {
    let needle = format!("name: \"{slug}\",");
    let start = AGENTS_TS
        .find(&needle)
        .unwrap_or_else(|| panic!("no block found for agent {slug}"));
    let after = start + needle.len();
    let end = AGENTS_TS[after..]
        .find("name: \"")
        .map(|i| after + i)
        .unwrap_or(AGENTS_TS.len());
    AGENTS_TS[start..end].to_string()
}

fn field_string(block: &str, field: &str) -> String {
    let key = format!("{field}: \"");
    let i = block
        .find(&key)
        .unwrap_or_else(|| panic!("missing field {field}"));
    let rest = &block[i + key.len()..];
    let end = rest.find('"').unwrap();
    rest[..end].to_string()
}

fn global_expr(block: &str) -> String {
    let key = "globalSkillsDir:";
    let i = block.find(key).expect("missing globalSkillsDir");
    let rest = &block[i + key.len()..];
    let end = rest.find('\n').unwrap();
    rest[..end].trim().trim_end_matches(',').trim().to_string()
}

/// Every `existsSync(<expr>)` argument in the block, in source order.
fn exists_args(block: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut search = 0usize;
    while let Some(rel) = block[search..].find("existsSync(") {
        let paren = search + rel + "existsSync".len();
        out.push(balanced(block, paren).trim().to_string());
        search = paren + 1;
    }
    out
}

/// Collect every double-quoted string in `s`.
fn quoted_strings(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            if let Some(rel) = s[i + 1..].find('"') {
                out.push(s[i + 1..i + 1 + rel].to_string());
                i = i + 1 + rel + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn union_slugs() -> Vec<String> {
    let start = TYPES_TS
        .find("export type AgentType =")
        .expect("AgentType union not found");
    let rest = &TYPES_TS[start..];
    let end = rest.find(';').expect("union terminator not found");
    let mut slugs = quoted_strings(&rest[..end]);
    slugs.sort();
    slugs
}

fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v
}

fn path_strings(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn slug_union_matches_table_exactly() {
    let ts_slugs = union_slugs();
    assert_eq!(ts_slugs.len(), 42, "AgentType union should list 42 agents");

    let rust_slugs = sorted(agents::slugs().iter().map(|s| s.to_string()).collect());
    assert_eq!(rust_slugs, ts_slugs, "Rust slugs must match the TS union");
    assert_eq!(agents::all().len(), 42, "Rust table should hold 42 agents");
}

#[test]
fn every_agent_directory_and_detection_matches_typescript() {
    let env = synthetic_env();
    let never_exists = |_: &Path| false;

    let mut checked = 0;
    for agent in agents::all() {
        let block = agent_block(agent.name);

        // Project-relative skills directory.
        assert_eq!(
            field_string(&block, "skillsDir"),
            agent.skills_dir,
            "skillsDir mismatch for {}",
            agent.name
        );

        // Global skills directory.
        let expected_global = eval_expr(&global_expr(&block));
        let actual_global = agent
            .global_skills_dir(&env, &never_exists)
            .to_string_lossy()
            .into_owned();
        assert_eq!(
            actual_global, expected_global,
            "globalSkillsDir mismatch for {}",
            agent.name
        );

        // detectInstalled probe paths (order-independent).
        let expected_detect = sorted(exists_args(&block).iter().map(|e| eval_expr(e)).collect());
        let actual_detect = sorted(path_strings(&agent.detect_paths(&env)));
        assert_eq!(
            actual_detect, expected_detect,
            "detectInstalled mismatch for {}",
            agent.name
        );

        // showInUniversalList: false only when explicitly set in the TS.
        let ts_hidden = block.contains("showInUniversalList: false");
        assert_eq!(
            !agent.show_in_universal_list, ts_hidden,
            "showInUniversalList mismatch for {}",
            agent.name
        );

        checked += 1;
    }
    assert_eq!(checked, 42, "expected to check all 42 agents");
}
