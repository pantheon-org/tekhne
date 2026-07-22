//! Agent harness directory and agent-name detection, ported from `agents.go`.

/// Config/data directory names used by each supported agent harness.
const HARNESS_DIRS: &[&str] = &[
    ".agents",
    ".aider",
    ".claude",
    ".codeium",
    ".codex",
    ".continue",
    ".copilot",
    ".cursor",
    ".deepagents",
    ".firebender",
    ".gemini",
    ".goose",
    ".pi",
    ".windsurf",
];

/// Human-readable agent identifiers whose presence signals a non-portable,
/// agent-specific instruction.
const AGENT_NAMES: &[&str] = &[
    "aider",
    "amp agent",
    "claude code",
    "cline",
    "codex",
    "continue.dev",
    "cursor agent",
    "deep agents",
    "firebender",
    "gemini cli",
    "github copilot",
    "goose",
    "kimi code",
    "opencode",
    "replit agent",
    "warp agent",
    "windsurf",
];

/// Return the first harness directory reference found in content (as
/// `"<dir>/"`), or an empty string (Go `findHarnessPath`).
pub fn find_harness_path(content: &str) -> String {
    for dir in HARNESS_DIRS {
        let needle = format!("{dir}/");
        if content.contains(&needle) {
            return needle;
        }
    }
    String::new()
}

/// Return the first agent-name reference found in content (case-insensitive),
/// or an empty string (Go `findAgentRef`).
pub fn find_agent_ref(content: &str) -> String {
    let lower = content.to_lowercase();
    for name in AGENT_NAMES {
        if lower.contains(name) {
            return (*name).to_string();
        }
    }
    String::new()
}
