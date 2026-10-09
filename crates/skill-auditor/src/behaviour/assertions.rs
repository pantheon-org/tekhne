//! The six assertion kinds, evaluated against one trace. Ported from skilldiff
//! (MIT), commit 9271c26, `src/assertions.ts`. Matching is partial and
//! case-insensitive, except tool names, which are compared once normalised.

use serde::Serialize;

use super::scenario::Expect;
use super::trace::{normalize_tool_name, Trace};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    FilesChanged,
    CommandsRun,
    ToolCalls,
    MustNot,
    OutputContains,
    OutputNotContains,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::FilesChanged => "files_changed",
            Kind::CommandsRun => "commands_run",
            Kind::ToolCalls => "tool_calls",
            Kind::MustNot => "must_not",
            Kind::OutputContains => "output_contains",
            Kind::OutputNotContains => "output_not_contains",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AssertionResult {
    pub kind: Kind,
    /// What was asserted.
    pub expected: String,
    pub pass: bool,
    /// What actually happened, in a few words.
    pub actual: String,
}

fn partial_match(haystacks: &[String], needle: &str) -> bool {
    let needle = needle.to_lowercase();
    haystacks.iter().any(|h| h.to_lowercase().contains(&needle))
}

fn list(items: &[String], sep: &str, empty: &str) -> String {
    if items.is_empty() {
        empty.to_string()
    } else {
        items.join(sep)
    }
}

/// Evaluate every expectation. The order is fixed: files changed, commands
/// run, tool calls, must-not (files, commands, tools), output contains, output
/// not contains.
pub fn evaluate(trace: &Trace, expect: &Expect) -> Vec<AssertionResult> {
    let mut out = Vec::new();
    let tools: Vec<String> = trace
        .tool_calls
        .iter()
        .map(|c| normalize_tool_name(&c.tool))
        .collect();

    for want in &expect.files_changed {
        out.push(AssertionResult {
            kind: Kind::FilesChanged,
            expected: want.clone(),
            pass: partial_match(&trace.files_changed, want),
            actual: format!("changed: [{}]", list(&trace.files_changed, ", ", "none")),
        });
    }
    for want in &expect.commands_run {
        out.push(AssertionResult {
            kind: Kind::CommandsRun,
            expected: want.clone(),
            pass: partial_match(&trace.commands_run, want),
            actual: format!("ran: [{}]", list(&trace.commands_run, "; ", "none")),
        });
    }
    for want in &expect.tool_calls {
        let wanted = normalize_tool_name(want);
        out.push(AssertionResult {
            kind: Kind::ToolCalls,
            expected: want.clone(),
            pass: tools.contains(&wanted),
            actual: format!("called: [{}]", list(&tools, ", ", "none")),
        });
    }

    for forbidden in &expect.must_not.files_changed {
        let violated = partial_match(&trace.files_changed, forbidden);
        out.push(AssertionResult {
            kind: Kind::MustNot,
            expected: format!("files_changed does not include {forbidden}"),
            pass: !violated,
            actual: if violated {
                format!("VIOLATED: {forbidden} was changed")
            } else {
                format!("changed: [{}]", list(&trace.files_changed, ", ", "none"))
            },
        });
    }
    for forbidden in &expect.must_not.commands_run {
        let violated = partial_match(&trace.commands_run, forbidden);
        out.push(AssertionResult {
            kind: Kind::MustNot,
            expected: format!("commands_run does not include {forbidden}"),
            pass: !violated,
            actual: if violated {
                format!("VIOLATED: ran {forbidden}")
            } else {
                format!("ran: [{}]", list(&trace.commands_run, "; ", "none"))
            },
        });
    }
    for forbidden in &expect.must_not.tool_calls {
        let violated = tools.contains(&normalize_tool_name(forbidden));
        out.push(AssertionResult {
            kind: Kind::MustNot,
            expected: format!("tool_calls does not include {forbidden}"),
            pass: !violated,
            actual: if violated {
                format!("VIOLATED: called {forbidden}")
            } else {
                "ok".to_string()
            },
        });
    }

    let output = trace.output.to_lowercase();
    for want in &expect.output_contains {
        out.push(AssertionResult {
            kind: Kind::OutputContains,
            expected: want.clone(),
            pass: output.contains(&want.to_lowercase()),
            actual: if trace.output.is_empty() {
                "(no output)".to_string()
            } else {
                format!(
                    "output: {:?}",
                    trace.output.chars().take(200).collect::<String>()
                )
            },
        });
    }
    for forbidden in &expect.output_not_contains {
        let violated = output.contains(&forbidden.to_lowercase());
        out.push(AssertionResult {
            kind: Kind::OutputNotContains,
            expected: format!("output does not include {forbidden}"),
            pass: !violated,
            actual: if violated {
                format!("VIOLATED: output contains {forbidden:?}")
            } else {
                "ok".to_string()
            },
        });
    }

    out
}
