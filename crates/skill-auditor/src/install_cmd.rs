//! Orchestration for the `skill install` subcommand.
//!
//! Decides which agents to target (detected agents by default, an explicit
//! `--agent` set, or every known agent with `--all`), resolves each target
//! directory, and installs the already-materialised skill into it using the
//! shared `skill-install` crate. The logic is separated from `main` so agent
//! selection and installation can be unit-tested against a temporary
//! filesystem.

use std::path::{Path, PathBuf};

use common::{Error, Result};
use skill_install::{
    agents::{all, by_name, AgentConfig},
    env::Environment,
    install::{install_skill, resolve_target_dir, uninstall_skill, InstallMode},
};

/// How the target agents are chosen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// Install into every agent detected on this machine (the default).
    Detected,
    /// Install into the named agents, whether or not they are detected.
    Explicit(Vec<String>),
    /// Install into every agent in the universal agents list.
    All,
}

/// Options controlling a `skill install` run.
#[derive(Debug, Clone)]
pub struct InstallOptions {
    /// Which agents to target.
    pub selection: Selection,
    /// Install into global skills directories rather than project-local ones.
    pub global: bool,
    /// Whether to copy or symlink the skill into each target.
    pub mode: InstallMode,
    /// Report actions without writing anything.
    pub dry_run: bool,
}

/// What happened for a single agent.
#[derive(Debug, Clone)]
pub struct AgentOutcome {
    /// The agent slug.
    pub agent: String,
    /// The resolved skills directory for the agent.
    pub target_dir: PathBuf,
    /// The installed skill path, when a write happened (or would in a dry run).
    pub installed_path: Option<PathBuf>,
    /// Whether the agent was detected as installed on this machine.
    pub detected: bool,
    /// An error message if the install failed for this agent.
    pub error: Option<String>,
}

/// The result of a `skill install` run.
#[derive(Debug, Clone)]
pub struct InstallReport {
    /// Per-agent outcomes, in selection order after de-duplication.
    pub outcomes: Vec<AgentOutcome>,
    /// Agents that were requested but are not installed on this machine.
    pub missing: Vec<String>,
}

/// Resolve the ordered, de-duplicated set of agents to install into.
///
/// `Explicit` selections validate every slug up front and error on unknown
/// names. Duplicate target directories are collapsed, mirroring the TypeScript
/// installer's `dedupAgentsByTarget`.
pub fn select_agents(
    opts: &InstallOptions,
    env: &Environment,
    exists: &dyn Fn(&Path) -> bool,
) -> Result<Vec<&'static AgentConfig>> {
    let candidates: Vec<&'static AgentConfig> = match &opts.selection {
        Selection::Detected => all()
            .iter()
            .filter(|a| a.detect_installed(env, exists))
            .collect(),
        Selection::All => all().iter().filter(|a| a.show_in_universal_list).collect(),
        Selection::Explicit(names) => {
            let mut resolved = Vec::with_capacity(names.len());
            for name in names {
                let agent = by_name(name).ok_or_else(|| {
                    Error::Config(format!(
                        "unknown agent {name:?}; run `skill-auditor skill install --list-agents`"
                    ))
                })?;
                resolved.push(agent);
            }
            resolved
        }
    };

    Ok(dedup_by_target(candidates, opts.global, env, exists))
}

/// Collapse agents that resolve to the same target directory, keeping the
/// first occurrence (ports `dedupAgentsByTarget`).
fn dedup_by_target(
    agents: Vec<&'static AgentConfig>,
    global: bool,
    env: &Environment,
    exists: &dyn Fn(&Path) -> bool,
) -> Vec<&'static AgentConfig> {
    let mut seen: Vec<PathBuf> = Vec::new();
    let mut kept = Vec::new();
    for agent in agents {
        let dir = resolve_target_dir(agent, global, env, exists);
        if seen.contains(&dir) {
            continue;
        }
        seen.push(dir);
        kept.push(agent);
    }
    kept
}

/// Install the materialised skill at `source_skill_dir` into every selected
/// agent directory.
///
/// `source_skill_dir` must already contain the skill tree (see
/// [`crate::skill_bundle::materialise`]). Missing-but-requested agents are
/// recorded in [`InstallReport::missing`] and still installed into, so the
/// caller can inform the user without silently skipping.
pub fn run_install(
    opts: &InstallOptions,
    env: &Environment,
    exists: &dyn Fn(&Path) -> bool,
    source_skill_dir: &Path,
) -> Result<InstallReport> {
    let agents = select_agents(opts, env, exists)?;
    let skill_name = source_skill_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string();

    let mut outcomes = Vec::with_capacity(agents.len());
    let mut missing = Vec::new();

    for agent in agents {
        let target_dir = resolve_target_dir(agent, opts.global, env, exists);
        let detected = agent.detect_installed(env, exists);
        if !detected {
            missing.push(agent.name.to_string());
        }

        if opts.dry_run {
            outcomes.push(AgentOutcome {
                agent: agent.name.to_string(),
                installed_path: Some(target_dir.join(&skill_name)),
                target_dir,
                detected,
                error: None,
            });
            continue;
        }

        match install_one(source_skill_dir, &target_dir, &skill_name, opts.mode) {
            Ok(installed) => outcomes.push(AgentOutcome {
                agent: agent.name.to_string(),
                target_dir,
                installed_path: Some(installed),
                detected,
                error: None,
            }),
            Err(e) => outcomes.push(AgentOutcome {
                agent: agent.name.to_string(),
                target_dir,
                installed_path: None,
                detected,
                error: Some(e.to_string()),
            }),
        }
    }

    Ok(InstallReport { outcomes, missing })
}

/// Install into one target, first clearing a stale symlink so symlink installs
/// are idempotent (a plain copy overwrites its files in place).
fn install_one(
    source: &Path,
    target_dir: &Path,
    skill_name: &str,
    mode: InstallMode,
) -> Result<PathBuf> {
    if mode == InstallMode::Symlink {
        let target = target_dir.join(skill_name);
        if let Ok(meta) = std::fs::symlink_metadata(&target) {
            if meta.file_type().is_symlink() {
                std::fs::remove_file(&target).map_err(|e| Error::io(&target, e))?;
            }
        }
    }
    install_skill(source, target_dir, mode)
}

/// Options controlling a `skill uninstall` run.
#[derive(Debug, Clone)]
pub struct UninstallOptions {
    /// Which agents to target.
    pub selection: Selection,
    /// Operate on global skills directories rather than project-local ones.
    pub global: bool,
    /// Report actions without removing anything.
    pub dry_run: bool,
}

/// What happened for a single agent during an uninstall run.
#[derive(Debug, Clone)]
pub struct UninstallOutcome {
    /// The agent slug.
    pub agent: String,
    /// The resolved skills directory for the agent.
    pub target_dir: PathBuf,
    /// The skill path that was (or would be) removed.
    pub target_path: PathBuf,
    /// Whether a skill was actually present and removed.
    pub removed: bool,
    /// Whether the agent was detected as installed on this machine.
    pub detected: bool,
    /// An error message if the removal failed for this agent.
    pub error: Option<String>,
}

/// The result of a `skill uninstall` run.
#[derive(Debug, Clone)]
pub struct UninstallReport {
    /// Per-agent outcomes, in selection order after de-duplication.
    pub outcomes: Vec<UninstallOutcome>,
    /// Agents that were requested but are not installed on this machine.
    pub missing: Vec<String>,
}

/// Remove the bundled skill named `skill_name` from every selected agent
/// directory.
///
/// Agent selection mirrors [`run_install`]. Idempotent per agent: an absent
/// skill is reported with `removed = false` rather than as an error.
pub fn run_uninstall(
    opts: &UninstallOptions,
    env: &Environment,
    exists: &dyn Fn(&Path) -> bool,
    skill_name: &str,
) -> Result<UninstallReport> {
    // Reuse the install selection logic; the placement mode is irrelevant here.
    let install_opts = InstallOptions {
        selection: opts.selection.clone(),
        global: opts.global,
        mode: InstallMode::Copy,
        dry_run: opts.dry_run,
    };
    let agents = select_agents(&install_opts, env, exists)?;

    let mut outcomes = Vec::with_capacity(agents.len());
    let mut missing = Vec::new();

    for agent in agents {
        let target_dir = resolve_target_dir(agent, opts.global, env, exists);
        let target_path = target_dir.join(skill_name);
        let detected = agent.detect_installed(env, exists);
        if !detected {
            missing.push(agent.name.to_string());
        }

        if opts.dry_run {
            outcomes.push(UninstallOutcome {
                agent: agent.name.to_string(),
                removed: exists(&target_path),
                target_dir,
                target_path,
                detected,
                error: None,
            });
            continue;
        }

        match uninstall_skill(&target_dir, skill_name) {
            Ok(removed) => outcomes.push(UninstallOutcome {
                agent: agent.name.to_string(),
                target_dir,
                target_path,
                removed,
                detected,
                error: None,
            }),
            Err(e) => outcomes.push(UninstallOutcome {
                agent: agent.name.to_string(),
                target_dir,
                target_path,
                removed: false,
                detected,
                error: Some(e.to_string()),
            }),
        }
    }

    Ok(UninstallReport { outcomes, missing })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_with(home: &Path, cwd: &Path) -> Environment {
        Environment::resolve(home.to_path_buf(), cwd.to_path_buf(), None, None, None)
    }

    fn fixture_skill(root: &Path) -> PathBuf {
        let skill = root.join("skill-quality-auditor");
        std::fs::create_dir_all(skill.join("references")).unwrap();
        std::fs::write(skill.join("SKILL.md"), "# skill").unwrap();
        std::fs::write(skill.join("references/ref.md"), "ref").unwrap();
        skill
    }

    #[test]
    fn explicit_unknown_agent_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_with(tmp.path(), tmp.path());
        let opts = InstallOptions {
            selection: Selection::Explicit(vec!["not-an-agent".into()]),
            global: true,
            mode: InstallMode::Copy,
            dry_run: false,
        };
        let err = select_agents(&opts, &env, &|_| false).unwrap_err();
        assert!(err.to_string().contains("unknown agent"));
    }

    #[test]
    fn explicit_agent_resolves_regardless_of_detection() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_with(tmp.path(), tmp.path());
        let opts = InstallOptions {
            selection: Selection::Explicit(vec!["claude-code".into()]),
            global: true,
            mode: InstallMode::Copy,
            dry_run: false,
        };
        let agents = select_agents(&opts, &env, &|_| false).unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "claude-code");
    }

    #[test]
    fn detected_selection_filters_to_installed() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path();
        let env = env_with(home, home);
        // Pretend only the cursor agent is installed (~/.cursor exists).
        let cursor_marker = home.join(".cursor");
        let exists = move |p: &Path| p == cursor_marker;
        let opts = InstallOptions {
            selection: Selection::Detected,
            global: true,
            mode: InstallMode::Copy,
            dry_run: false,
        };
        let agents = select_agents(&opts, &env, &exists).unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "cursor");
    }

    #[test]
    fn copy_mode_writes_files_into_target() {
        let tmp = tempfile::tempdir().unwrap();
        let source = fixture_skill(&tmp.path().join("bundle"));
        let home = tmp.path().join("home");
        std::fs::create_dir_all(&home).unwrap();
        let env = env_with(&home, &home);

        let opts = InstallOptions {
            selection: Selection::Explicit(vec!["claude-code".into()]),
            global: true,
            mode: InstallMode::Copy,
            dry_run: false,
        };
        let report = run_install(&opts, &env, &|_| false, &source).unwrap();

        assert_eq!(report.outcomes.len(), 1);
        let outcome = &report.outcomes[0];
        assert!(outcome.error.is_none());
        let installed = outcome.installed_path.as_ref().unwrap();
        assert!(installed.join("SKILL.md").is_file());
        assert!(installed.join("references/ref.md").is_file());
        assert!(!installed
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink());
        // claude-code was not detected, so it is flagged as missing.
        assert_eq!(report.missing, vec!["claude-code".to_string()]);
    }

    #[test]
    fn symlink_mode_links_target_to_source() {
        let tmp = tempfile::tempdir().unwrap();
        let source = fixture_skill(&tmp.path().join("bundle"));
        let home = tmp.path().join("home");
        std::fs::create_dir_all(&home).unwrap();
        let env = env_with(&home, &home);

        let opts = InstallOptions {
            selection: Selection::Explicit(vec!["claude-code".into()]),
            global: true,
            mode: InstallMode::Symlink,
            dry_run: false,
        };
        let report = run_install(&opts, &env, &|_| false, &source).unwrap();
        let installed = report.outcomes[0].installed_path.as_ref().unwrap();

        assert!(installed
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(
            std::fs::read_to_string(installed.join("SKILL.md")).unwrap(),
            "# skill"
        );

        // Re-running is idempotent: the stale symlink is replaced, not errored.
        let again = run_install(&opts, &env, &|_| false, &source).unwrap();
        assert!(again.outcomes[0].error.is_none());
    }

    #[test]
    fn dry_run_writes_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let source = fixture_skill(&tmp.path().join("bundle"));
        let home = tmp.path().join("home");
        std::fs::create_dir_all(&home).unwrap();
        let env = env_with(&home, &home);

        let opts = InstallOptions {
            selection: Selection::Explicit(vec!["claude-code".into()]),
            global: true,
            mode: InstallMode::Copy,
            dry_run: true,
        };
        let report = run_install(&opts, &env, &|_| false, &source).unwrap();
        let planned = report.outcomes[0].installed_path.as_ref().unwrap();
        assert!(!planned.exists(), "dry run must not write the skill");
    }

    #[test]
    fn run_uninstall_removes_installed_skill() {
        let tmp = tempfile::tempdir().unwrap();
        let source = fixture_skill(&tmp.path().join("bundle"));
        let home = tmp.path().join("home");
        std::fs::create_dir_all(&home).unwrap();
        let env = env_with(&home, &home);
        let skill_name = source.file_name().unwrap().to_str().unwrap().to_string();

        let install_opts = InstallOptions {
            selection: Selection::Explicit(vec!["claude-code".into()]),
            global: true,
            mode: InstallMode::Copy,
            dry_run: false,
        };
        let installed = run_install(&install_opts, &env, &|_| false, &source).unwrap();
        let path = installed.outcomes[0].installed_path.clone().unwrap();
        assert!(path.exists());

        let opts = UninstallOptions {
            selection: Selection::Explicit(vec!["claude-code".into()]),
            global: true,
            dry_run: false,
        };
        let report = run_uninstall(&opts, &env, &|p: &Path| p.exists(), &skill_name).unwrap();
        assert_eq!(report.outcomes.len(), 1);
        assert!(report.outcomes[0].removed);
        assert!(!path.exists(), "uninstall should remove the skill");

        // Idempotent: a second uninstall removes nothing and does not error.
        let again = run_uninstall(&opts, &env, &|p: &Path| p.exists(), &skill_name).unwrap();
        assert!(!again.outcomes[0].removed);
        assert!(again.outcomes[0].error.is_none());
    }
}
