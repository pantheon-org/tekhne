//! `skill-validator-rs`: the standalone offline validator gate. The CLI itself
//! lives in [`skill_validator_rs::cli`] so `pantheon-skill-auditor` can embed
//! it; this binary is kept as a thin wrapper for existing invocations.

use std::process::ExitCode;

fn main() -> ExitCode {
    skill_validator_rs::cli::run("skill-validator-rs", std::env::args_os())
}
