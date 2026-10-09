//! The `behaviour.yaml` file of an eval scenario: what an agent must and must
//! not do with the skill. The expectation kinds come from skilldiff (MIT),
//! commit 9271c26, `src/scenario.ts`; unlike its loader, this one reads every
//! kind and rejects any key it does not know.

use serde::{Deserialize, Serialize};

use super::SCHEMA_VERSION;

/// Things that must not have happened, in the same three forms as the positive
/// assertions.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MustNot {
    #[serde(default)]
    pub files_changed: Vec<String>,
    #[serde(default)]
    pub commands_run: Vec<String>,
    #[serde(default)]
    pub tool_calls: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Expect {
    /// Paths that must have been changed. Partial, case-insensitive match.
    #[serde(default)]
    pub files_changed: Vec<String>,
    /// Commands that must have been run. Partial, case-insensitive match.
    #[serde(default)]
    pub commands_run: Vec<String>,
    /// Tools that must have been used, compared by normalised name.
    #[serde(default)]
    pub tool_calls: Vec<String>,
    #[serde(default)]
    pub must_not: MustNot,
    /// Text that must appear in the final output.
    #[serde(default)]
    pub output_contains: Vec<String>,
    /// Text that must not appear in the final output.
    #[serde(default)]
    pub output_not_contains: Vec<String>,
}

impl Expect {
    fn is_empty(&self) -> bool {
        self.files_changed.is_empty()
            && self.commands_run.is_empty()
            && self.tool_calls.is_empty()
            && self.must_not.files_changed.is_empty()
            && self.must_not.commands_run.is_empty()
            && self.must_not.tool_calls.is_empty()
            && self.output_contains.is_empty()
            && self.output_not_contains.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Behaviour {
    pub schema_version: u32,
    pub name: String,
    /// The prompt given to the agent when the traces were recorded.
    #[serde(default)]
    pub prompt: Option<String>,
    /// The synthetic fixture the agent worked on, relative to the scenario.
    #[serde(default)]
    pub fixture: Option<String>,
    #[serde(default, rename = "skillPaths")]
    pub skill_paths: Vec<String>,
    pub expect: Expect,
}

impl Behaviour {
    /// Parse and check a `behaviour.yaml`.
    pub fn parse(text: &str) -> Result<Behaviour, String> {
        let value: serde_yaml_ng::Value =
            serde_yaml_ng::from_str(text).map_err(|e| format!("invalid YAML: {e}"))?;
        let behaviour: Behaviour =
            serde_yaml_ng::from_value(value).map_err(|e| format!("invalid behaviour file: {e}"))?;
        if behaviour.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "unsupported schema_version {} (this auditor reads {SCHEMA_VERSION})",
                behaviour.schema_version
            ));
        }
        if behaviour.name.trim().is_empty() {
            return Err("name must not be empty".to_string());
        }
        if behaviour.expect.is_empty() {
            return Err("expect lists no assertions".to_string());
        }
        Ok(behaviour)
    }
}
