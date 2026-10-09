//! Behaviour checks: what an agent did with a skill, judged from a recorded
//! trace. Nothing here runs an agent or touches the network, and nothing here
//! changes a score; the result is a separate report beside the score.
//!
//! The assertion core is ported from skilldiff (MIT, <https://github.com/scs0209/skilldiff>),
//! commit 9271c26. See the `NOTICE` file beside this crate.

pub mod assertions;
pub mod load;
pub mod scenario;
pub mod trace;

/// Version of the `behaviour.yaml` and trace file formats, and of the report.
pub const SCHEMA_VERSION: u32 = 1;
