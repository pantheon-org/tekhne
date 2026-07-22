//! `skill-validator-rs`: deterministic, offline skill validation (structure
//! metrics, content analysis, token counts) consumed by the Rust auditor.
//!
//! Ported from `github.com/agent-ecosystem/skill-validator` per the S1 spike
//! (`.context/plans/skill-validator-port-spec.md`). Only the offline,
//! deterministic surface is covered; the LLM scoring path and the external
//! (network) link group are out of scope. The CLI/binary is deferred to A5b.
//!
//! The public entry points mirror the two Go calls the auditor bridge makes:
//! [`structure::validate`] (directory, frontmatter, tokens, markdown, internal
//! links, orphans) plus the content/contamination analysers in [`content`] and
//! [`contamination`].

pub mod artifacts;
pub mod contamination;
pub mod content;
pub mod links;
pub mod skill;
pub mod structure;
pub mod types;
pub mod util;

pub use contamination::analyze as analyze_contamination;
pub use content::analyze as analyze_content;
pub use skill::Skill;
pub use structure::{validate, validate_multi, Options};
pub use types::{
    ContaminationReport, ContentReport, Level, MultiReport, Report, TokenCount, ValidationResult,
};
