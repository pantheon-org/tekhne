import { existsSync } from "node:fs";
import { join } from "node:path";
import type { AgentType } from "../types";
import { logger } from "../utils/logger";
import { createSymlink } from "./create-symlink";
import { ensureDirectory } from "./ensure-directory";
import { getSkillName } from "./get-skill-name";
import type { InstallOptions } from "./install-skills";
import { resolveTargetDir } from "./resolve-target-dir";

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

  const targetDir = resolveTargetDir(agent as AgentType, options.global, cwd);
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
