//! A recorded run of an agent with a skill: what the agent did, not what it
//! said it would do. Shape and tool-name normalisation ported from skilldiff
//! (MIT), commit 9271c26, `src/assertions.ts` and `src/trace-utils.ts`.

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::SCHEMA_VERSION;

/// Map a harness's tool name to a common one, because harnesses name the same
/// tool differently. The groups are tried in order and the first match wins, as
/// in skilldiff; a name is matched by containing the fragment, not by equality.
pub fn normalize_tool_name(tool: &str) -> String {
    const GROUPS: [(&str, &[&str]); 6] = [
        ("read", &["read", "view_file", "open_file"]),
        (
            "write",
            &[
                "write",
                "create_file",
                "new_file",
                "edit",
                "str_replace",
                "update_file",
                "multiedit",
            ],
        ),
        (
            "bash",
            &["bash", "exec_command", "shell", "command_execution"],
        ),
        ("glob", &["glob", "ls_dir", "list_dir", "find"]),
        ("grep", &["grep", "search"]),
        ("spawn", &["spawn", "task", "subagent"]),
    ];
    let lower = tool.to_lowercase();
    for (name, fragments) in GROUPS {
        if fragments.iter().any(|f| lower.contains(f)) {
            return name.to_string();
        }
    }
    lower
}

/// Whether a trace came from a real run or was written by a person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Source {
    Recorded,
    HandWritten,
}

impl Source {
    pub fn is_recorded(self) -> bool {
        self == Source::Recorded
    }

    pub fn label(self) -> &'static str {
        match self {
            Source::Recorded => "recorded",
            Source::HandWritten => "hand-written",
        }
    }
}

/// Where a trace came from, so a stale one can be told apart from a fresh one.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub harness: String,
    /// `YYYY-MM-DD`.
    pub date: String,
    /// Commit of the skill's `SKILL.md` the run was recorded against.
    pub skill_commit: String,
    pub source: Source,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCall {
    pub tool: String,
    #[serde(default)]
    pub input: Value,
}

/// Everything observable from one run.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trace {
    pub schema_version: u32,
    pub header: Header,
    #[serde(rename = "toolCalls")]
    pub tool_calls: Vec<ToolCall>,
    /// Paths changed or created, relative to the fixture root.
    #[serde(rename = "filesChanged")]
    pub files_changed: Vec<String>,
    /// Full command strings that were executed.
    #[serde(rename = "commandsRun")]
    pub commands_run: Vec<String>,
    /// The agent's final text.
    pub output: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl Trace {
    /// Parse and check a trace file. Unknown fields are an error, so a typo in
    /// a field name cannot silently turn an assertion into a pass.
    pub fn parse(text: &str) -> Result<Trace, String> {
        let trace: Trace = serde_json::from_str(text).map_err(|e| {
            if e.is_syntax() || e.is_eof() {
                format!("invalid JSON: {e}")
            } else {
                format!("invalid trace: {e}")
            }
        })?;
        if trace.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "unsupported schema_version {} (this auditor reads {SCHEMA_VERSION})",
                trace.schema_version
            ));
        }
        trace.header.check()?;
        Ok(trace)
    }
}

impl Header {
    fn check(&self) -> Result<(), String> {
        if self.harness.trim().is_empty() {
            return Err("header.harness must not be empty".to_string());
        }
        let date = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("static pattern");
        if !date.is_match(&self.date) {
            return Err(format!(
                "header.date must be YYYY-MM-DD, got {:?}",
                self.date
            ));
        }
        if self.skill_commit.trim().is_empty() {
            return Err("header.skill_commit must not be empty".to_string());
        }
        Ok(())
    }
}
