//! Base-directory resolution for agent detection.
//!
//! Ports the module-level path constants from `cli/lib/types/agents.ts`
//! (`home`, `configHome`, `codexHome`, `claudeHome`) plus the
//! `getOpenClawGlobalSkillsDir` helper. Resolution is expressed as pure
//! functions so the environment-override and XDG semantics can be unit-tested
//! without touching the process environment.

use std::path::{Path, PathBuf};

use common::{Error, Result};

/// The resolved base directories an agent's install paths are computed from.
///
/// Mirrors the module-level constants in `agents.ts`. `cwd` backs the
/// `process.cwd()` checks a few agents perform in `detectInstalled`.
#[derive(Debug, Clone)]
pub struct Environment {
    /// The user's home directory (`os.homedir()`).
    pub home: PathBuf,
    /// The XDG config home (`XDG_CONFIG_HOME` or `~/.config`).
    pub config_home: PathBuf,
    /// The Codex home (`CODEX_HOME` or `~/.codex`).
    pub codex_home: PathBuf,
    /// The Claude Code home (`CLAUDE_CONFIG_DIR` or `~/.claude`).
    pub claude_home: PathBuf,
    /// The current working directory (`process.cwd()`).
    pub cwd: PathBuf,
}

impl Environment {
    /// Resolve every base directory from the live process environment.
    ///
    /// Reads `XDG_CONFIG_HOME`, `CODEX_HOME`, and `CLAUDE_CONFIG_DIR`, matching
    /// the override precedence in `agents.ts`.
    pub fn from_env() -> Result<Self> {
        let home = std::env::home_dir()
            .filter(|p| !p.as_os_str().is_empty())
            .ok_or_else(|| Error::Config("could not determine home directory".into()))?;
        let cwd = std::env::current_dir().map_err(|source| Error::io(".", source))?;
        Ok(Self::resolve(
            home,
            cwd,
            std::env::var("XDG_CONFIG_HOME").ok().as_deref(),
            std::env::var("CODEX_HOME").ok().as_deref(),
            std::env::var("CLAUDE_CONFIG_DIR").ok().as_deref(),
        ))
    }

    /// Resolve base directories from explicit inputs (used by `from_env` and
    /// exercised directly in tests).
    pub fn resolve(
        home: PathBuf,
        cwd: PathBuf,
        xdg_config_home: Option<&str>,
        codex_home: Option<&str>,
        claude_config_dir: Option<&str>,
    ) -> Self {
        let config_home = resolve_config_home(&home, xdg_config_home);
        let codex_home = resolve_home_override(&home, codex_home, ".codex");
        let claude_home = resolve_home_override(&home, claude_config_dir, ".claude");
        Self {
            home,
            config_home,
            codex_home,
            claude_home,
            cwd,
        }
    }
}

/// Resolve the XDG config home.
///
/// Matches `xdgConfig ?? join(home, ".config")` where `xdgConfig` itself is
/// `XDG_CONFIG_HOME || join(home, ".config")`: an unset or empty value falls
/// back to `~/.config`, and (unlike the trimmed overrides) the value is used
/// verbatim otherwise.
pub fn resolve_config_home(home: &Path, xdg_config_home: Option<&str>) -> PathBuf {
    match xdg_config_home {
        Some(value) if !value.is_empty() => PathBuf::from(value),
        _ => home.join(".config"),
    }
}

/// Resolve a trimmed home override such as `CODEX_HOME` or `CLAUDE_CONFIG_DIR`.
///
/// Matches `process.env.VAR?.trim() || join(home, fallback)`: the value is
/// trimmed, and an unset or whitespace-only value falls back to
/// `home/<fallback>`.
pub fn resolve_home_override(home: &Path, raw: Option<&str>, fallback: &str) -> PathBuf {
    match raw.map(str::trim).filter(|v| !v.is_empty()) {
        Some(value) => PathBuf::from(value),
        None => home.join(fallback),
    }
}

/// Resolve OpenClaw's global skills directory.
///
/// Ports `getOpenClawGlobalSkillsDir`: prefer `.openclaw`, then `.clawdbot`,
/// then `.moltbot` if present, otherwise fall back to `.openclaw/skills`.
pub fn open_claw_global_skills_dir(home: &Path, exists: &dyn Fn(&Path) -> bool) -> PathBuf {
    for name in [".openclaw", ".clawdbot", ".moltbot"] {
        if exists(&home.join(name)) {
            return home.join(name).join("skills");
        }
    }
    home.join(".openclaw").join("skills")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> PathBuf {
        PathBuf::from("/home/user")
    }

    #[test]
    fn config_home_defaults_to_dot_config() {
        assert_eq!(resolve_config_home(&home(), None), home().join(".config"));
    }

    #[test]
    fn config_home_uses_xdg_when_set() {
        assert_eq!(
            resolve_config_home(&home(), Some("/xdg/cfg")),
            PathBuf::from("/xdg/cfg")
        );
    }

    #[test]
    fn config_home_empty_xdg_falls_back() {
        assert_eq!(
            resolve_config_home(&home(), Some("")),
            home().join(".config")
        );
    }

    #[test]
    fn home_override_defaults_to_fallback() {
        assert_eq!(
            resolve_home_override(&home(), None, ".codex"),
            home().join(".codex")
        );
    }

    #[test]
    fn home_override_uses_and_trims_value() {
        assert_eq!(
            resolve_home_override(&home(), Some("  /custom/codex  "), ".codex"),
            PathBuf::from("/custom/codex")
        );
    }

    #[test]
    fn home_override_whitespace_only_falls_back() {
        assert_eq!(
            resolve_home_override(&home(), Some("   "), ".claude"),
            home().join(".claude")
        );
    }

    #[test]
    fn open_claw_prefers_clawdbot_when_present() {
        let dir = open_claw_global_skills_dir(&home(), &|p: &Path| p.ends_with(".clawdbot"));
        assert_eq!(dir, home().join(".clawdbot").join("skills"));
    }

    #[test]
    fn open_claw_falls_back_to_openclaw() {
        let dir = open_claw_global_skills_dir(&home(), &|_: &Path| false);
        assert_eq!(dir, home().join(".openclaw").join("skills"));
    }

    #[test]
    fn resolve_wires_up_all_bases() {
        let env =
            Environment::resolve(home(), PathBuf::from("/work"), None, Some("/x/codex"), None);
        assert_eq!(env.config_home, home().join(".config"));
        assert_eq!(env.codex_home, PathBuf::from("/x/codex"));
        assert_eq!(env.claude_home, home().join(".claude"));
        assert_eq!(env.cwd, PathBuf::from("/work"));
    }
}
