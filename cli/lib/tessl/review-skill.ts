import { dirname, join } from "node:path";
import { exec } from "../utils/exec";
import { logger } from "../utils/logger";
import { readManifest } from "../utils/skill-manifest";

export const reviewSkill = async (
  skillPath: string,
  _workspace: string,
): Promise<boolean> => {
  logger.info(`Reviewing skill: ${skillPath}`);

  const manifest = await readManifest(skillPath);
  if (manifest && manifest.skills.length > 1) {
    logger.info(
      `Multi-skill plugin detected (${manifest.skills.length} skills)`,
    );
    for (const skillPathRel of manifest.skills) {
      const skillFullPath = join(skillPath, dirname(skillPathRel));
      logger.info(`Reviewing skill: ${skillPathRel}`);
      const { exitCode, stderr } = await exec(
        `tessl review run ${skillFullPath}`,
      );
      if (exitCode !== 0) {
        logger.error(`Review failed for ${skillPathRel}: ${stderr}`);
        return false;
      }
    }
    logger.success("All skills reviewed successfully");
    return true;
  }

  const { exitCode, stderr } = await exec(`tessl review run ${skillPath}`);
  if (exitCode !== 0) {
    logger.error(`Review failed: ${stderr}`);
    return false;
  }
  logger.success("Review passed");
  return true;
};
