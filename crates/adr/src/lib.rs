//! `adr`: a small, deterministic Architecture Decision Record (ADR) tool.
//!
//! The core [`adr`] module renders and manages ADR files against the house
//! template; [`skill_bundle`] and [`install_cmd`] embed and install the
//! companion `adr-creator` skill, following the A5 distribution pattern shared
//! with `skill-auditor`. Everything is a pure function of on-disk content plus
//! an injectable "today", so behaviour is fully testable without a clock.

pub mod adr;
pub mod date;
pub mod install_cmd;
pub mod skill_bundle;
