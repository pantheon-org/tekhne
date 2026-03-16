import { existsSync } from "node:fs";
import { join } from "node:path";
import { TileSchema } from "../schemas/tile.schema";
import { exec } from "../utils/exec";
import { logger } from "../utils/logger";

export const reviewSkill = async (
  skillPath: string,
  _workspace: string,
): Promise<boolean> => {
  logger.info(`Reviewing skill: ${skillPath}`);

  const tileJsonPath = join(skillPath, "tile.json");
  if (existsSync(tileJsonPath)) {
    const rawData = await Bun.file(tileJsonPath).json();
    const tileData = TileSchema.parse(rawData);
    const skillEntries = Object.entries(tileData.skills ?? {});

    if (skillEntries.length > 1) {
      logger.info(`Multi-skill tile detected (${skillEntries.length} skills)`);
      for (const [skillName, skillInfo] of skillEntries) {
        const skillDir = join(
          skillPath,
          skillInfo.path.replace(/\/SKILL\.md$/, ""),
        );
        logger.info(`Reviewing skill: ${skillName}`);
        const { exitCode, stderr } = await exec(
          `tessl skill review ${skillDir}`,
        );
        if (exitCode !== 0) {
          logger.error(`Review failed for ${skillName}: ${stderr}`);
          return false;
        }
      }
      logger.success("All skills reviewed successfully");
      return true;
    }
  }

  const { exitCode, stderr } = await exec(`tessl skill review ${skillPath}`);
  if (exitCode !== 0) {
    logger.error(`Review failed: ${stderr}`);
    return false;
  }
  logger.success("Review passed");
  return true;
};
