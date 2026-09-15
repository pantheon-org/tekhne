//! `adr`: a small, deterministic Architecture Decision Record (ADR) tool.
//!
//! A decision is a markdown file whose YAML frontmatter is the single source of
//! truth for its status, author, supersede links and status history:
//! [`record`] models it, [`store`] reads and writes it, [`completeness`] scores
//! its prose, [`draft`] gathers the branch context needed to write it, and
//! [`index`] renders a catalogue derived from the records themselves.
//! [`hooks`] and [`sync`] wire the same checks into git and agent harnesses,
//! and [`skill_bundle`] with [`install_cmd`] embed and install the companion
//! `adr-creator` skill, following the A5 distribution pattern shared with
//! `skill-auditor`. Everything is a pure function of on-disk content plus an
//! injectable "today", so behaviour is fully testable without a clock.

pub mod commands;
pub mod completeness;
pub mod date;
pub mod draft;
pub mod git;
pub mod hooks;
pub mod index;
pub mod install_cmd;
pub mod record;
pub mod skill_bundle;
pub mod store;
pub mod sync;
