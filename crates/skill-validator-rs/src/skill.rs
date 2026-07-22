//! SKILL.md parsing: frontmatter extraction and body separation (Go `skill`
//! package). Frontmatter is parsed twice, mirroring the Go tool: once into
//! typed fields and once into a raw value for unrecognised-field and metadata
//! checks. YAML is handled by `serde_yaml_ng` (libyaml), whose scalar
//! coercion matches `gopkg.in/yaml.v3`'s lenient behaviour.

use std::collections::BTreeSet;
use std::path::Path;

use serde_yaml_ng::Value;

/// The `allowed-tools` frontmatter field, which may be a space-delimited
/// string or a YAML sequence (Go `skill.AllowedTools`).
#[derive(Debug, Clone, Default)]
pub struct AllowedTools {
    /// The normalised, space-delimited value.
    pub value: String,
    /// True when the original YAML used a sequence.
    pub was_list: bool,
}

impl AllowedTools {
    /// Whether no value was supplied.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

/// The typed frontmatter fields the validator reads (Go `skill.Frontmatter`).
#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    /// The `name` field.
    pub name: String,
    /// The `description` field.
    pub description: String,
    /// The optional `license` field.
    pub license: String,
    /// The optional `compatibility` field.
    pub compatibility: String,
    /// The optional `allowed-tools` field.
    pub allowed_tools: AllowedTools,
}

/// A parsed skill package (Go `skill.Skill`).
#[derive(Debug, Clone)]
pub struct Skill {
    /// The skill directory.
    pub dir: String,
    /// Typed frontmatter fields.
    pub frontmatter: Frontmatter,
    /// Raw parsed frontmatter mapping (or `Null` when absent/empty).
    pub raw_frontmatter: Value,
    /// The SKILL.md body (frontmatter stripped).
    pub body: String,
    /// The complete SKILL.md content.
    pub raw_content: String,
}

/// Frontmatter field names defined by the spec; others trigger a warning.
const KNOWN_FIELDS: &[&str] = &[
    "name",
    "description",
    "license",
    "compatibility",
    "metadata",
    "allowed-tools",
];

impl Skill {
    /// Read and parse `<dir>/SKILL.md` (Go `skill.Load`).
    pub fn load(dir: &Path) -> Result<Skill, String> {
        let path = dir.join("SKILL.md");
        let content =
            std::fs::read_to_string(&path).map_err(|e| format!("reading SKILL.md: {e}"))?;

        let (fm, body) = split_frontmatter(&content)?;

        let mut skill = Skill {
            dir: dir.to_string_lossy().into_owned(),
            frontmatter: Frontmatter::default(),
            raw_frontmatter: Value::Null,
            body,
            raw_content: content,
        };

        if !fm.is_empty() {
            let raw: Value = serde_yaml_ng::from_str(&fm)
                .map_err(|e| format!("parsing frontmatter YAML: {e}"))?;
            skill.frontmatter = parse_frontmatter(&raw)?;
            skill.raw_frontmatter = raw;
        }

        Ok(skill)
    }

    /// Frontmatter field names not recognised by the spec, sorted for
    /// deterministic output (Go `Skill.UnrecognizedFields`, which returns
    /// them in Go map-iteration order).
    pub fn unrecognized_fields(&self) -> Vec<String> {
        let mut unknown = BTreeSet::new();
        if let Value::Mapping(map) = &self.raw_frontmatter {
            for key in map.keys() {
                if let Some(k) = key.as_str() {
                    if !KNOWN_FIELDS.contains(&k) {
                        unknown.insert(k.to_string());
                    }
                }
            }
        }
        unknown.into_iter().collect()
    }
}

/// Coerce a scalar YAML value to a string the way `yaml.v3` does when
/// decoding into a `string` field (numbers/bools stringify, null -> empty).
fn scalar_to_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::Null => Some(String::new()),
        _ => None,
    }
}

/// Build the typed [`Frontmatter`] from the raw mapping. Returns an error for
/// type shapes the Go typed decode would reject (e.g. a mapping where a string
/// is expected, or an `allowed-tools` mapping).
fn parse_frontmatter(raw: &Value) -> Result<Frontmatter, String> {
    let map = match raw {
        Value::Mapping(m) => m,
        // A non-mapping top-level document leaves all fields empty, matching a
        // typed decode into an all-optional struct.
        _ => return Ok(Frontmatter::default()),
    };

    let string_field = |field: &str| -> Result<String, String> {
        match map.get(Value::String(field.to_string())) {
            None => Ok(String::new()),
            Some(v) => scalar_to_string(v)
                .ok_or_else(|| format!("parsing frontmatter YAML: field {field} is not a scalar")),
        }
    };

    let mut fm = Frontmatter {
        name: string_field("name")?,
        description: string_field("description")?,
        license: string_field("license")?,
        compatibility: string_field("compatibility")?,
        allowed_tools: AllowedTools::default(),
    };

    if let Some(v) = map.get(Value::String("allowed-tools".to_string())) {
        fm.allowed_tools = parse_allowed_tools(v)?;
    }

    Ok(fm)
}

fn parse_allowed_tools(v: &Value) -> Result<AllowedTools, String> {
    match v {
        Value::Sequence(items) => {
            let mut parts = Vec::with_capacity(items.len());
            for item in items {
                match scalar_to_string(item) {
                    Some(s) => parts.push(s),
                    None => return Err("decoding allowed-tools list: non-scalar item".to_string()),
                }
            }
            Ok(AllowedTools {
                value: parts.join(" "),
                was_list: true,
            })
        }
        Value::String(s) => Ok(AllowedTools {
            value: s.clone(),
            was_list: false,
        }),
        Value::Bool(_) | Value::Number(_) | Value::Null => Ok(AllowedTools {
            value: scalar_to_string(v).unwrap_or_default(),
            was_list: false,
        }),
        _ => Err("allowed-tools must be a string or list".to_string()),
    }
}

/// Separate YAML frontmatter (between `---` delimiters) from the body
/// (Go `skill.splitFrontmatter`). Operates on bytes; all delimiters are ASCII.
pub fn split_frontmatter(content: &str) -> Result<(String, String), String> {
    if !content.starts_with("---") {
        return Ok((String::new(), content.to_string()));
    }

    let bytes = content.as_bytes();
    let mut rest = &bytes[3..];

    // Skip the newline after the opening ---.
    if !rest.is_empty() && rest[0] == b'\n' {
        rest = &rest[1..];
    } else if rest.len() > 1 && rest[0] == b'\r' && rest[1] == b'\n' {
        rest = &rest[2..];
    }

    // Empty frontmatter: closing --- immediately.
    if rest.starts_with(b"---") {
        let mut body = &rest[3..];
        if !body.is_empty() && body[0] == b'\n' {
            body = &body[1..];
        } else if body.len() > 1 && body[0] == b'\r' && body[1] == b'\n' {
            body = &body[2..];
        }
        return Ok((String::new(), bytes_to_string(body)));
    }

    // Find the closing "\n---".
    match find_subslice(rest, b"\n---") {
        None => Err("unterminated frontmatter: missing closing ---".to_string()),
        Some(idx) => {
            let before = &rest[..idx];
            let after = &rest[idx + 4..];
            let frontmatter = trim_right_cr(before);
            let mut body = after;
            if !body.is_empty() && body[0] == b'\n' {
                body = &body[1..];
            } else if body.len() > 1 && body[0] == b'\r' && body[1] == b'\n' {
                body = &body[2..];
            }
            Ok((bytes_to_string(frontmatter), bytes_to_string(body)))
        }
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

fn trim_right_cr(b: &[u8]) -> &[u8] {
    let mut end = b.len();
    while end > 0 && b[end - 1] == b'\r' {
        end -= 1;
    }
    &b[..end]
}

fn bytes_to_string(b: &[u8]) -> String {
    // Content originated from a valid UTF-8 &str sliced on ASCII boundaries.
    String::from_utf8_lossy(b).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_frontmatter_returns_whole_body() {
        let (fm, body) = split_frontmatter("# Title\nbody").unwrap();
        assert_eq!(fm, "");
        assert_eq!(body, "# Title\nbody");
    }

    #[test]
    fn splits_frontmatter_and_body() {
        let (fm, body) = split_frontmatter("---\nname: x\n---\n# Body\n").unwrap();
        assert_eq!(fm, "name: x");
        assert_eq!(body, "# Body\n");
    }

    #[test]
    fn empty_frontmatter() {
        let (fm, body) = split_frontmatter("---\n---\n# Body").unwrap();
        assert_eq!(fm, "");
        assert_eq!(body, "# Body");
    }

    #[test]
    fn unterminated_frontmatter_errors() {
        assert!(split_frontmatter("---\nname: x\n").is_err());
    }

    #[test]
    fn allowed_tools_list_joins_with_space() {
        let raw: Value = serde_yaml_ng::from_str("allowed-tools:\n  - Bash\n  - Read\n").unwrap();
        let fm = parse_frontmatter(&raw).unwrap();
        assert_eq!(fm.allowed_tools.value, "Bash Read");
        assert!(fm.allowed_tools.was_list);
    }

    #[test]
    fn allowed_tools_string_kept() {
        let raw: Value = serde_yaml_ng::from_str("allowed-tools: Bash, Read, Write\n").unwrap();
        let fm = parse_frontmatter(&raw).unwrap();
        assert_eq!(fm.allowed_tools.value, "Bash, Read, Write");
        assert!(!fm.allowed_tools.was_list);
    }
}
