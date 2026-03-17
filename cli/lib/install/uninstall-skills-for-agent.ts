import { rmSync, unlinkSync } from "node:fs";
import type { AgentType } from "../types";
import { logger } from "../utils/logger";
import { findInstalledSkills } from "./find-installed-skills";
import { resolveTargetDir } from "./resolve-target-dir";
import type { UninstallOptions } from "./uninstall-skills";

interface AgentStats {
  removed: number;
  skipped: number;
  failed: number;
}

export const uninstallSkillsForAgent = (
  agent: string,
  cwd: string,
  options: UninstallOptions,
): AgentStats => {
  const stats: AgentStats = { removed: 0, skipped: 0, failed: 0 };

  logger.header(`\nUninstalling for ${agent}`);

  const targetDir = resolveTargetDir(agent as AgentType, options.global, cwd);
  logger.info(`Target directory: ${targetDir}`);

  const installed = findInstalledSkills(targetDir, cwd);

  if (installed.length === 0) {
    logger.info("No project-owned skills found");
    return stats;
  }

  for (const skill of installed) {
    if (options.dryRun) {
      logger.info(
        `Would remove: ${skill.name} (${skill.isSymlink ? "symlink" : "copy"})`,
      );
      stats.removed++;
      continue;
    }

    try {
      if (skill.isSymlink) {
        unlinkSync(skill.targetPath);
      } else {
        rmSync(skill.targetPath, { recursive: true, force: true });
      }
      logger.success(`Removed: ${skill.name}`);
      stats.removed++;
    } catch (error: unknown) {
      logger.error(`Failed to remove ${skill.name}: ${error}`);
      stats.failed++;
    }
  }

  return stats;
};
