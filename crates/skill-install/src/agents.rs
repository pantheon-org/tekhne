//! The agent table ported from `cli/lib/types/agents.ts`.
//!
//! Each [`AgentConfig`] captures an agent's project-relative skills directory,
//! its global-skills-directory resolution, and the `detectInstalled` probe
//! paths. Directory and detection logic is expressed as data resolved against
//! an [`Environment`], mirroring the `join(...)`/`existsSync(...)` expressions
//! in the TypeScript source.

use std::path::{Path, PathBuf};

use crate::env::{open_claw_global_skills_dir, Environment};

/// A base directory an agent path is resolved relative to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    /// `os.homedir()`.
    Home,
    /// `XDG_CONFIG_HOME` or `~/.config` (`configHome`).
    ConfigHome,
    /// `CODEX_HOME` or `~/.codex` (`codexHome`).
    CodexHome,
    /// `CLAUDE_CONFIG_DIR` or `~/.claude` (`claudeHome`).
    ClaudeHome,
    /// `process.cwd()`.
    Cwd,
    /// An absolute path that ignores every base directory (e.g. `/etc/codex`).
    Absolute,
}

impl Base {
    /// Resolve `base` + `rel` against `env`. An empty `rel` yields the base
    /// directory itself; [`Base::Absolute`] yields `rel` verbatim.
    fn resolve(self, rel: &str, env: &Environment) -> PathBuf {
        if self == Base::Absolute {
            return PathBuf::from(rel);
        }
        let root: &Path = match self {
            Base::Home => &env.home,
            Base::ConfigHome => &env.config_home,
            Base::CodexHome => &env.codex_home,
            Base::ClaudeHome => &env.claude_home,
            Base::Cwd => &env.cwd,
            Base::Absolute => unreachable!(),
        };
        if rel.is_empty() {
            root.to_path_buf()
        } else {
            root.join(rel)
        }
    }
}

/// A single `existsSync` probe used by an agent's detection logic.
#[derive(Debug, Clone, Copy)]
pub struct DetectPath {
    /// The base directory the probe is relative to.
    pub base: Base,
    /// The path relative to `base` (empty means the base directory itself).
    pub rel: &'static str,
}

/// How an agent's global skills directory is resolved.
#[derive(Debug, Clone, Copy)]
pub enum GlobalDir {
    /// A fixed `base` + `rel` location.
    Fixed {
        /// The base directory.
        base: Base,
        /// The path relative to `base`.
        rel: &'static str,
    },
    /// OpenClaw's dynamic resolution (see [`open_claw_global_skills_dir`]).
    OpenClaw,
}

/// A single agent's install and detection configuration.
#[derive(Debug, Clone, Copy)]
pub struct AgentConfig {
    /// The stable agent slug (matches the `AgentType` union in `types.ts`).
    pub name: &'static str,
    /// The human-readable display name.
    pub display_name: &'static str,
    /// The project-relative skills directory.
    pub skills_dir: &'static str,
    /// How the global skills directory is resolved.
    pub global: GlobalDir,
    /// The `existsSync` probes; any hit means the agent is installed.
    pub detect: &'static [DetectPath],
    /// Whether the agent appears in the universal agents list.
    pub show_in_universal_list: bool,
}

impl AgentConfig {
    /// Resolve the agent's global skills directory against `env`.
    ///
    /// `exists` backs OpenClaw's dynamic probing; other agents ignore it.
    pub fn global_skills_dir(&self, env: &Environment, exists: &dyn Fn(&Path) -> bool) -> PathBuf {
        match self.global {
            GlobalDir::Fixed { base, rel } => base.resolve(rel, env),
            GlobalDir::OpenClaw => open_claw_global_skills_dir(&env.home, exists),
        }
    }

    /// The resolved `existsSync` probe paths for this agent, in source order.
    pub fn detect_paths(&self, env: &Environment) -> Vec<PathBuf> {
        self.detect
            .iter()
            .map(|d| d.base.resolve(d.rel, env))
            .collect()
    }

    /// Whether the agent is installed: true when any probe path exists.
    pub fn detect_installed(&self, env: &Environment, exists: &dyn Fn(&Path) -> bool) -> bool {
        self.detect_paths(env).iter().any(|p| exists(p))
    }
}

const fn probe(base: Base, rel: &'static str) -> DetectPath {
    DetectPath { base, rel }
}

const fn fixed(base: Base, rel: &'static str) -> GlobalDir {
    GlobalDir::Fixed { base, rel }
}

/// The full agent table, ported one-for-one from `agents.ts`.
pub static AGENTS: &[AgentConfig] = &[
    AgentConfig {
        name: "amp",
        display_name: "Amp",
        skills_dir: ".agents/skills",
        global: fixed(Base::ConfigHome, "agents/skills"),
        detect: &[probe(Base::ConfigHome, "amp")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "antigravity",
        display_name: "Antigravity",
        skills_dir: ".agent/skills",
        global: fixed(Base::Home, ".gemini/antigravity/skills"),
        detect: &[probe(Base::Home, ".gemini/antigravity")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "augment",
        display_name: "Augment",
        skills_dir: ".augment/skills",
        global: fixed(Base::Home, ".augment/skills"),
        detect: &[probe(Base::Home, ".augment")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "claude-code",
        display_name: "Claude Code",
        skills_dir: ".claude/skills",
        global: fixed(Base::ClaudeHome, "skills"),
        detect: &[probe(Base::ClaudeHome, "")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "openclaw",
        display_name: "OpenClaw",
        skills_dir: "skills",
        global: GlobalDir::OpenClaw,
        detect: &[
            probe(Base::Home, ".openclaw"),
            probe(Base::Home, ".clawdbot"),
            probe(Base::Home, ".moltbot"),
        ],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "cline",
        display_name: "Cline",
        skills_dir: ".agents/skills",
        global: fixed(Base::Home, ".agents/skills"),
        detect: &[probe(Base::Home, ".cline")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "codebuddy",
        display_name: "CodeBuddy",
        skills_dir: ".codebuddy/skills",
        global: fixed(Base::Home, ".codebuddy/skills"),
        detect: &[
            probe(Base::Cwd, ".codebuddy"),
            probe(Base::Home, ".codebuddy"),
        ],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "codex",
        display_name: "Codex",
        skills_dir: ".agents/skills",
        global: fixed(Base::CodexHome, "skills"),
        detect: &[
            probe(Base::CodexHome, ""),
            probe(Base::Absolute, "/etc/codex"),
        ],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "command-code",
        display_name: "Command Code",
        skills_dir: ".commandcode/skills",
        global: fixed(Base::Home, ".commandcode/skills"),
        detect: &[probe(Base::Home, ".commandcode")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "continue",
        display_name: "Continue",
        skills_dir: ".continue/skills",
        global: fixed(Base::Home, ".continue/skills"),
        detect: &[
            probe(Base::Cwd, ".continue"),
            probe(Base::Home, ".continue"),
        ],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "cortex",
        display_name: "Cortex Code",
        skills_dir: ".cortex/skills",
        global: fixed(Base::Home, ".snowflake/cortex/skills"),
        detect: &[probe(Base::Home, ".snowflake/cortex")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "crush",
        display_name: "Crush",
        skills_dir: ".crush/skills",
        global: fixed(Base::Home, ".config/crush/skills"),
        detect: &[probe(Base::Home, ".config/crush")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "cursor",
        display_name: "Cursor",
        skills_dir: ".agents/skills",
        global: fixed(Base::Home, ".cursor/skills"),
        detect: &[probe(Base::Home, ".cursor")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "droid",
        display_name: "Droid",
        skills_dir: ".factory/skills",
        global: fixed(Base::Home, ".factory/skills"),
        detect: &[probe(Base::Home, ".factory")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "gemini-cli",
        display_name: "Gemini CLI",
        skills_dir: ".agents/skills",
        global: fixed(Base::Home, ".gemini/skills"),
        detect: &[probe(Base::Home, ".gemini")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "github-copilot",
        display_name: "GitHub Copilot",
        skills_dir: ".agents/skills",
        global: fixed(Base::Home, ".copilot/skills"),
        detect: &[probe(Base::Home, ".copilot")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "goose",
        display_name: "Goose",
        skills_dir: ".goose/skills",
        global: fixed(Base::ConfigHome, "goose/skills"),
        detect: &[probe(Base::ConfigHome, "goose")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "junie",
        display_name: "Junie",
        skills_dir: ".junie/skills",
        global: fixed(Base::Home, ".junie/skills"),
        detect: &[probe(Base::Home, ".junie")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "iflow-cli",
        display_name: "iFlow CLI",
        skills_dir: ".iflow/skills",
        global: fixed(Base::Home, ".iflow/skills"),
        detect: &[probe(Base::Home, ".iflow")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "kilo",
        display_name: "Kilo Code",
        skills_dir: ".kilocode/skills",
        global: fixed(Base::Home, ".kilocode/skills"),
        detect: &[probe(Base::Home, ".kilocode")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "kimi-cli",
        display_name: "Kimi Code CLI",
        skills_dir: ".agents/skills",
        global: fixed(Base::Home, ".config/agents/skills"),
        detect: &[probe(Base::Home, ".kimi")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "kiro-cli",
        display_name: "Kiro CLI",
        skills_dir: ".kiro/skills",
        global: fixed(Base::Home, ".kiro/skills"),
        detect: &[probe(Base::Home, ".kiro")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "kode",
        display_name: "Kode",
        skills_dir: ".kode/skills",
        global: fixed(Base::Home, ".kode/skills"),
        detect: &[probe(Base::Home, ".kode")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "mcpjam",
        display_name: "MCPJam",
        skills_dir: ".mcpjam/skills",
        global: fixed(Base::Home, ".mcpjam/skills"),
        detect: &[probe(Base::Home, ".mcpjam")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "mistral-vibe",
        display_name: "Mistral Vibe",
        skills_dir: ".vibe/skills",
        global: fixed(Base::Home, ".vibe/skills"),
        detect: &[probe(Base::Home, ".vibe")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "mux",
        display_name: "Mux",
        skills_dir: ".mux/skills",
        global: fixed(Base::Home, ".mux/skills"),
        detect: &[probe(Base::Home, ".mux")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "opencode",
        display_name: "OpenCode",
        skills_dir: ".agents/skills",
        global: fixed(Base::ConfigHome, "opencode/skills"),
        detect: &[probe(Base::ConfigHome, "opencode")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "openhands",
        display_name: "OpenHands",
        skills_dir: ".openhands/skills",
        global: fixed(Base::Home, ".openhands/skills"),
        detect: &[probe(Base::Home, ".openhands")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "pi",
        display_name: "Pi",
        skills_dir: ".pi/skills",
        global: fixed(Base::Home, ".pi/agent/skills"),
        detect: &[probe(Base::Home, ".pi/agent")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "qoder",
        display_name: "Qoder",
        skills_dir: ".qoder/skills",
        global: fixed(Base::Home, ".qoder/skills"),
        detect: &[probe(Base::Home, ".qoder")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "qwen-code",
        display_name: "Qwen Code",
        skills_dir: ".qwen/skills",
        global: fixed(Base::Home, ".qwen/skills"),
        detect: &[probe(Base::Home, ".qwen")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "replit",
        display_name: "Replit",
        skills_dir: ".agents/skills",
        global: fixed(Base::ConfigHome, "agents/skills"),
        detect: &[probe(Base::Cwd, ".replit")],
        show_in_universal_list: false,
    },
    AgentConfig {
        name: "roo",
        display_name: "Roo Code",
        skills_dir: ".roo/skills",
        global: fixed(Base::Home, ".roo/skills"),
        detect: &[probe(Base::Home, ".roo")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "trae",
        display_name: "Trae",
        skills_dir: ".trae/skills",
        global: fixed(Base::Home, ".trae/skills"),
        detect: &[probe(Base::Home, ".trae")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "trae-cn",
        display_name: "Trae CN",
        skills_dir: ".trae/skills",
        global: fixed(Base::Home, ".trae-cn/skills"),
        detect: &[probe(Base::Home, ".trae-cn")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "warp",
        display_name: "Warp",
        skills_dir: ".agents/skills",
        global: fixed(Base::Home, ".agents/skills"),
        detect: &[probe(Base::Home, ".warp")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "windsurf",
        display_name: "Windsurf",
        skills_dir: ".windsurf/skills",
        global: fixed(Base::Home, ".codeium/windsurf/skills"),
        detect: &[probe(Base::Home, ".codeium/windsurf")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "zencoder",
        display_name: "Zencoder",
        skills_dir: ".zencoder/skills",
        global: fixed(Base::Home, ".zencoder/skills"),
        detect: &[probe(Base::Home, ".zencoder")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "neovate",
        display_name: "Neovate",
        skills_dir: ".neovate/skills",
        global: fixed(Base::Home, ".neovate/skills"),
        detect: &[probe(Base::Home, ".neovate")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "pochi",
        display_name: "Pochi",
        skills_dir: ".pochi/skills",
        global: fixed(Base::Home, ".pochi/skills"),
        detect: &[probe(Base::Home, ".pochi")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "adal",
        display_name: "AdaL",
        skills_dir: ".adal/skills",
        global: fixed(Base::Home, ".adal/skills"),
        detect: &[probe(Base::Home, ".adal")],
        show_in_universal_list: true,
    },
    AgentConfig {
        name: "universal",
        display_name: "Universal",
        skills_dir: ".agents/skills",
        global: fixed(Base::ConfigHome, "agents/skills"),
        detect: &[],
        show_in_universal_list: false,
    },
];

/// Every agent configuration in source order.
pub fn all() -> &'static [AgentConfig] {
    AGENTS
}

/// Look up an agent by its slug.
pub fn by_name(name: &str) -> Option<&'static AgentConfig> {
    AGENTS.iter().find(|a| a.name == name)
}

/// Every agent slug in source order.
pub fn slugs() -> Vec<&'static str> {
    AGENTS.iter().map(|a| a.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> Environment {
        Environment::resolve(
            PathBuf::from("/home/user"),
            PathBuf::from("/work/project"),
            None,
            None,
            None,
        )
    }

    #[test]
    fn table_has_forty_two_unique_agents() {
        assert_eq!(AGENTS.len(), 42);
        let mut names = slugs();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), 42);
    }

    #[test]
    fn claude_code_detects_home_itself() {
        let claude = by_name("claude-code").unwrap();
        assert_eq!(claude.detect_paths(&env()), vec![env().claude_home.clone()]);
    }

    #[test]
    fn codex_detects_absolute_etc_path() {
        let codex = by_name("codex").unwrap();
        let paths = codex.detect_paths(&env());
        assert_eq!(paths[0], env().codex_home);
        assert_eq!(paths[1], PathBuf::from("/etc/codex"));
    }

    #[test]
    fn openclaw_global_dir_is_dynamic() {
        let openclaw = by_name("openclaw").unwrap();
        let dir = openclaw.global_skills_dir(&env(), &|p: &Path| p.ends_with(".moltbot"));
        assert_eq!(dir, PathBuf::from("/home/user/.moltbot/skills"));
    }

    #[test]
    fn universal_is_never_detected() {
        let universal = by_name("universal").unwrap();
        assert!(!universal.detect_installed(&env(), &|_| true));
    }

    #[test]
    fn detect_installed_matches_any_probe() {
        let codebuddy = by_name("codebuddy").unwrap();
        let target = env().cwd.join(".codebuddy");
        assert!(codebuddy.detect_installed(&env(), &|p: &Path| p == target));
    }
}
