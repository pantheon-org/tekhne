import { existsSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { logger } from "../utils/logger";
import { createSymlink } from "./create-symlink";
import { ensureDirectory } from "./ensure-directory";
import { getSkillName } from "./get-skill-name";
import type { InstallOptions } from "./install-skills";

export const AGENT_PATHS: Record<
  string,
  (isGlobal: boolean, cwd: string) => string
> = {
  opencode: (isGlobal: boolean, cwd: string) =>
    isGlobal
      ? join(homedir(), ".config", "opencode", "skills")
      : join(cwd, ".agents", "skills"),
  cursor: (_isGlobal: boolean) =>
    join(homedir(), ".config", "cursor", "skills"),
  gemini: (_isGlobal: boolean) =>
    join(homedir(), ".config", "gemini", "skills"),
  claude: (_isGlobal: boolean) =>
    join(homedir(), ".config", "claude", "skills"),
  codex: (_isGlobal: boolean) => join(homedir(), ".config", "codex", "skills"),
};

// Agents that are always global (--global flag has no effect)
export const ALWAYS_GLOBAL_AGENTS = new Set([
  "cursor",
  "gemini",
  "claude",
  "codex",
]);

interface AgentStats {
  installed: number;
  skipped: number;
  failed: number;
}

export const installSkillsForAgent = (
  agent: string,
  skills: string[],
  cwd: string,
  options: InstallOptions,
): AgentStats => {
  const stats: AgentStats = { installed: 0, skipped: 0, failed: 0 };

  logger.header(`\nInstalling for ${agent}`);

  if (options.global && ALWAYS_GLOBAL_AGENTS.has(agent)) {
    logger.warning(
      `--global has no effect for ${agent}: it always installs to ~/.config/${agent}/skills`,
    );
  }

  const targetDir = AGENT_PATHS[agent](options.global, cwd);
  logger.info(`Target directory: ${targetDir}`);

  ensureDirectory(targetDir, options.dryRun);

  for (const skillPath of skills) {
    const skillName = getSkillName(skillPath);
    const source = join(cwd, skillPath);
    const target = join(targetDir, skillName);

    const result = createSymlink(source, target, options.dryRun);

    if (result) {
      stats.installed++;
    } else if (existsSync(target)) {
      stats.skipped++;
    } else {
      stats.failed++;
    }
  }

  return stats;
};
