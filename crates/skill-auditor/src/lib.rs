//! `skill-auditor`: deterministic 9-dimension skill quality scorer and report
//! writer, ported from the Go `tools/skill-auditor`. The scoring path is a pure
//! function of on-disk content plus the `skill-validator-rs` analysis; no LLM or
//! network access is involved anywhere.

pub mod aggregation;
pub mod duplication;
pub mod install_cmd;
pub mod prune;
pub mod reporter;
pub mod scorer;
pub mod skill_bundle;
