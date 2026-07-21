//! Structured output helpers for tool results.

use serde::Serialize;

use crate::error::{Error, Result};

/// The output format a tool should emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Format {
    /// Human-readable text (the default for interactive use).
    #[default]
    Text,
    /// Machine-readable JSON (for `--json` / CI consumers).
    Json,
}

impl Format {
    /// Parse a format from a CLI flag value, defaulting to [`Format::Text`].
    pub fn parse(value: &str) -> Result<Self> {
        match value.to_ascii_lowercase().as_str() {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            other => Err(Error::Config(format!("unknown output format: {other}"))),
        }
    }
}

/// Serialize a value to a compact JSON string.
pub fn to_json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|source| Error::json("output", source))
}

/// Serialize a value to a pretty (2-space indented) JSON string.
pub fn to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string_pretty(value).map_err(|source| Error::json("output", source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Result_ {
        grade: &'static str,
        total: u32,
    }

    #[test]
    fn json_is_compact() {
        let out = to_json(&Result_ {
            grade: "B",
            total: 112,
        })
        .unwrap();
        assert_eq!(out, r#"{"grade":"B","total":112}"#);
    }

    #[test]
    fn json_pretty_is_indented() {
        let out = to_json_pretty(&Result_ {
            grade: "B",
            total: 112,
        })
        .unwrap();
        assert!(out.contains("\n  \"grade\""));
    }

    #[test]
    fn format_parse_and_default() {
        assert_eq!(Format::parse("json").unwrap(), Format::Json);
        assert_eq!(Format::parse("TEXT").unwrap(), Format::Text);
        assert_eq!(Format::default(), Format::Text);
        assert!(Format::parse("xml").is_err());
    }
}
