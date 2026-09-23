//! The tag taxonomy: a controlled vocabulary of facets and aliases that the
//! `lint` pass checks corpus tags against.
//!
//! This is a Rust port of the journal CLI's `taxonomy-schema.ts`,
//! `load-taxonomy.ts`, and `normalise-tag.ts`. The taxonomy is *data*, not
//! code: a `taxonomy.json` is discovered at the journal root at runtime, and a
//! generic, org-agnostic default is embedded in the binary (via the skill
//! bundle) as the fallback. The same binary is therefore safe to distribute
//! publicly while still honouring a private vocabulary when run inside a repo
//! that ships its own `taxonomy.json`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use common::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::skill_bundle;

/// The relative path of the embedded generic taxonomy inside the skill bundle.
const DEFAULT_TAXONOMY_REL: &str = "assets/taxonomy.default.json";

/// The conventional file name discovered at a journal root.
const TAXONOMY_FILE: &str = "taxonomy.json";

/// An overrides-only controlled vocabulary. The effective canonical tag set is
/// computed from tag frequency at lint time; this struct holds only what
/// frequency cannot infer.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Taxonomy {
    /// Optional self-reference to the JSON Schema; ignored at runtime.
    #[serde(rename = "$schema", default)]
    pub schema: Option<String>,
    /// Minimum occurrence count before an unfaceted tag is flagged.
    pub threshold: u32,
    /// Non-canonical spellings mapped onto their canonical form.
    pub aliases: BTreeMap<String, String>,
    /// Named groups of canonical tags (e.g. `type`, `tech`, `topic`).
    pub facets: BTreeMap<String, Vec<String>>,
    /// Tags kept in the canonical set regardless of frequency.
    #[serde(default)]
    pub pin: Vec<String>,
    /// Tags excluded from the canonical set and from lint flagging.
    #[serde(default)]
    pub suppress: Vec<String>,
    /// Regular expression identifying issue-tracker key tags.
    #[serde(rename = "ticketPattern")]
    pub ticket_pattern: String,
    /// A consuming project's own marker for an append-only taxonomy policy
    /// (e.g. a git commit sha the taxonomy must only grow since). Opaque to
    /// this crate: never read or validated, only preserved on round-trip, so
    /// a project-specific append-only-taxonomy mechanism built on top of this
    /// file doesn't have to fork the schema to add its own marker.
    #[serde(
        rename = "appendOnlyBaseline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub append_only_baseline: Option<String>,
}

/// Where a resolved taxonomy came from, for reporting to the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaxonomySource {
    /// An explicit `--taxonomy <path>`.
    Explicit(PathBuf),
    /// A `taxonomy.json` discovered at the journal root.
    Root(PathBuf),
    /// The generic default embedded in the binary.
    EmbeddedDefault,
}

impl std::fmt::Display for TaxonomySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaxonomySource::Explicit(p) => write!(f, "{}", p.display()),
            TaxonomySource::Root(p) => write!(f, "{}", p.display()),
            TaxonomySource::EmbeddedDefault => write!(f, "embedded default taxonomy"),
        }
    }
}

impl Taxonomy {
    /// Parse and validate a taxonomy from a JSON string. `context` names what
    /// is being parsed, for error messages.
    pub fn from_json(json: &str, context: &str) -> Result<Self> {
        let taxonomy: Taxonomy =
            serde_json::from_str(json).map_err(|e| Error::json(context.to_string(), e))?;
        taxonomy.validate(context)?;
        Ok(taxonomy)
    }

    /// Read and validate a taxonomy from a file.
    pub fn from_path(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
        Self::from_json(&raw, &path.display().to_string())
    }

    /// The generic default embedded in the binary.
    pub fn embedded_default() -> Result<Self> {
        let bytes = skill_bundle::embedded_file(DEFAULT_TAXONOMY_REL).ok_or_else(|| {
            Error::Config(format!(
                "no embedded default taxonomy at {DEFAULT_TAXONOMY_REL}"
            ))
        })?;
        let json = std::str::from_utf8(bytes)
            .map_err(|e| Error::Config(format!("embedded default taxonomy is not UTF-8: {e}")))?;
        Self::from_json(json, "embedded default taxonomy")
    }

    /// Resolve a taxonomy per the discovery order:
    /// explicit path, then `<root>/taxonomy.json`, then the embedded default.
    pub fn resolve(root: &Path, explicit: Option<&Path>) -> Result<(Self, TaxonomySource)> {
        if let Some(path) = explicit {
            return Ok((
                Self::from_path(path)?,
                TaxonomySource::Explicit(path.to_path_buf()),
            ));
        }
        let root_file = root.join(TAXONOMY_FILE);
        if root_file.is_file() {
            return Ok((
                Self::from_path(&root_file)?,
                TaxonomySource::Root(root_file),
            ));
        }
        Ok((Self::embedded_default()?, TaxonomySource::EmbeddedDefault))
    }

    /// Fail loudly on a structurally valid but semantically broken config: a
    /// non-positive threshold or an uncompilable ticket pattern.
    fn validate(&self, context: &str) -> Result<()> {
        if self.threshold < 1 {
            return Err(Error::Config(format!(
                "{context}: threshold must be a positive integer"
            )));
        }
        Regex::new(&self.ticket_pattern).map_err(|e| {
            Error::Config(format!(
                "{context}: ticketPattern is not a valid regex: {e}"
            ))
        })?;
        Ok(())
    }

    /// Compile the ticket-key pattern. Safe to unwrap after [`Taxonomy::validate`],
    /// but returns a `Result` so callers need not assume it was validated.
    pub fn ticket_regex(&self) -> Result<Regex> {
        Regex::new(&self.ticket_pattern)
            .map_err(|e| Error::Config(format!("ticketPattern is not a valid regex: {e}")))
    }

    /// True when `tag` appears in any facet's canonical list.
    pub fn is_faceted(&self, tag: &str) -> bool {
        self.facets
            .values()
            .any(|list| list.iter().any(|t| t == tag))
    }

    /// Promote `tag` into `facet`'s canonical list, keeping it sorted and
    /// deduplicated. Errors if `facet` does not already exist (a typo'd facet
    /// name should not silently create a new one) or if `tag` is already
    /// faceted anywhere (would create a duplicate membership, not a move).
    pub fn add_tag_to_facet(&mut self, tag: &str, facet: &str) -> Result<()> {
        // Normalize first: trims, lower-cases, and resolves aliases, so
        // " AWS-Lambda " or a known alias can't bypass the already-faceted
        // check or land in the taxonomy in a form lint would never produce.
        let tag = normalise_tag(tag, &self.aliases);
        if self.is_faceted(&tag) {
            return Err(Error::Config(format!(
                "tag \"{tag}\" is already faceted; remove it from its current facet first if you meant to move it"
            )));
        }
        if !self.facets.contains_key(facet) {
            let known: Vec<&str> = self.facets.keys().map(String::as_str).collect();
            return Err(Error::Config(format!(
                "no such facet \"{facet}\" (known facets: {})",
                known.join(", ")
            )));
        }
        let list = self.facets.get_mut(facet).expect("checked above");
        list.push(tag);
        list.sort();
        list.dedup();
        Ok(())
    }

    /// Serialize back to the on-disk JSON shape, pretty-printed with a
    /// trailing newline to match the embedded default's formatting.
    pub fn to_json_pretty(&self) -> Result<String> {
        let mut json =
            serde_json::to_string_pretty(self).map_err(|e| Error::json("taxonomy", e))?;
        json.push('\n');
        Ok(json)
    }

    /// Write this taxonomy to `path`, creating parent directories if needed.
    pub fn write_to_path(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
            }
        }
        let json = self.to_json_pretty()?;
        std::fs::write(path, json).map_err(|e| Error::io(path, e))
    }
}

/// Lower-case and trim a tag, then resolve it through the alias map so
/// near-duplicate variants collapse onto a single canonical spelling.
pub fn normalise_tag(tag: &str, aliases: &BTreeMap<String, String>) -> String {
    let lowered = tag.trim().to_lowercase();
    aliases.get(&lowered).cloned().unwrap_or(lowered)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = r#"{
        "threshold": 3,
        "aliases": { "teams": "ms-teams" },
        "facets": { "type": ["troubleshooting", "general"], "tech": ["aws-lambda"] },
        "pin": [],
        "suppress": [],
        "ticketPattern": "^[a-z]+-?[0-9]+$"
    }"#;

    #[test]
    fn parses_minimal() {
        let tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        assert_eq!(tax.threshold, 3);
        assert_eq!(
            tax.aliases.get("teams").map(String::as_str),
            Some("ms-teams")
        );
        assert!(tax.is_faceted("aws-lambda"));
        assert!(!tax.is_faceted("nonexistent"));
    }

    #[test]
    fn rejects_unknown_fields() {
        let json =
            r#"{ "threshold": 1, "aliases": {}, "facets": {}, "ticketPattern": "x", "bogus": 1 }"#;
        assert!(Taxonomy::from_json(json, "test").is_err());
    }

    #[test]
    fn rejects_zero_threshold() {
        let json = r#"{ "threshold": 0, "aliases": {}, "facets": {}, "ticketPattern": "x" }"#;
        assert!(Taxonomy::from_json(json, "test").is_err());
    }

    #[test]
    fn rejects_bad_ticket_pattern() {
        let json = r#"{ "threshold": 1, "aliases": {}, "facets": {}, "ticketPattern": "[" }"#;
        assert!(Taxonomy::from_json(json, "test").is_err());
    }

    #[test]
    fn pin_and_suppress_default_to_empty() {
        let json =
            r#"{ "threshold": 1, "aliases": {}, "facets": {}, "ticketPattern": "^x-[0-9]+$" }"#;
        let tax = Taxonomy::from_json(json, "test").unwrap();
        assert!(tax.pin.is_empty());
        assert!(tax.suppress.is_empty());
    }

    #[test]
    fn normalise_lowercases_and_aliases() {
        let tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        assert_eq!(normalise_tag("  Teams ", &tax.aliases), "ms-teams");
        assert_eq!(normalise_tag("AWS-Lambda", &tax.aliases), "aws-lambda");
        assert_eq!(normalise_tag("unmapped", &tax.aliases), "unmapped");
    }

    #[test]
    fn add_tag_to_facet_appends_sorted() {
        let mut tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        tax.add_tag_to_facet("api-gateway", "tech").unwrap();
        assert_eq!(
            tax.facets.get("tech"),
            Some(&vec!["api-gateway".to_string(), "aws-lambda".to_string()])
        );
    }

    #[test]
    fn add_tag_to_facet_normalizes_whitespace_and_case() {
        let mut tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        tax.add_tag_to_facet(" API-Gateway ", "tech").unwrap();
        assert_eq!(
            tax.facets.get("tech"),
            Some(&vec!["api-gateway".to_string(), "aws-lambda".to_string()])
        );
    }

    #[test]
    fn add_tag_to_facet_resolves_aliases_before_checking() {
        // MINIMAL aliases "teams" -> "ms-teams"; faceting the alias spelling
        // must not bypass the already-faceted check against the canonical form.
        let mut tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        tax.add_tag_to_facet("ms-teams", "tech").unwrap();
        let err = tax.add_tag_to_facet("Teams", "tech").unwrap_err();
        assert!(err.to_string().contains("already faceted"));
    }

    #[test]
    fn add_tag_to_facet_rejects_unknown_facet() {
        let mut tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        let err = tax
            .add_tag_to_facet("api-gateway", "nonexistent")
            .unwrap_err();
        assert!(err.to_string().contains("no such facet"));
    }

    #[test]
    fn add_tag_to_facet_rejects_already_faceted() {
        let mut tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        let err = tax.add_tag_to_facet("aws-lambda", "type").unwrap_err();
        assert!(err.to_string().contains("already faceted"));
    }

    #[test]
    fn add_tag_to_facet_dedups() {
        let mut tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        // Faceting "aws-lambda" again onto "tech" (where it already lives) is
        // rejected by the already-faceted guard, not silently deduped away.
        assert!(tax.add_tag_to_facet("aws-lambda", "tech").is_err());
    }

    #[test]
    fn to_json_pretty_round_trips() {
        let tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        let json = tax.to_json_pretty().unwrap();
        assert!(json.ends_with('\n'));
        let reparsed = Taxonomy::from_json(&json, "round-trip").unwrap();
        assert_eq!(reparsed.threshold, tax.threshold);
        assert_eq!(reparsed.facets, tax.facets);
    }

    #[test]
    fn write_to_path_creates_parent_dirs() {
        let dir = std::env::temp_dir().join(format!(
            "pantheon-journal-taxonomy-test-{}",
            std::process::id()
        ));
        let path = dir.join("nested").join("taxonomy.json");
        let tax = Taxonomy::from_json(MINIMAL, "test").unwrap();
        tax.write_to_path(&path).unwrap();
        let reloaded = Taxonomy::from_path(&path).unwrap();
        assert_eq!(reloaded.threshold, tax.threshold);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn append_only_baseline_round_trips_but_is_never_required() {
        let with_baseline = r#"{
            "threshold": 3,
            "aliases": {},
            "facets": { "type": ["general"] },
            "pin": [],
            "suppress": [],
            "ticketPattern": "^[a-z]+-?[0-9]+$",
            "appendOnlyBaseline": "30d80d4"
        }"#;
        let tax = Taxonomy::from_json(with_baseline, "test").unwrap();
        assert_eq!(tax.append_only_baseline.as_deref(), Some("30d80d4"));
        let json = tax.to_json_pretty().unwrap();
        assert!(json.contains("\"appendOnlyBaseline\": \"30d80d4\""));

        // Absent entirely (the common case) must still parse and round-trip
        // without emitting the key.
        let without = Taxonomy::from_json(MINIMAL, "test").unwrap();
        assert_eq!(without.append_only_baseline, None);
        assert!(!without
            .to_json_pretty()
            .unwrap()
            .contains("appendOnlyBaseline"));
    }

    #[test]
    fn embedded_default_is_valid() {
        let tax = Taxonomy::embedded_default().unwrap();
        assert!(tax.threshold >= 1);
        assert!(tax.facets.contains_key("type"));
        // The default must stay org-agnostic: aliases empty, only the type facet.
        assert!(tax.aliases.is_empty());
        assert!(tax.ticket_regex().is_ok());
    }
}
