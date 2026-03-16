import { existsSync } from "node:fs";
import { join } from "node:path";
import { TileSchema } from "../schemas/tile.schema";
import { logger } from "../utils/logger";

export const validateSkillFiles = async (
  tilePath: string,
): Promise<boolean> => {
  const tileJsonPath = join(tilePath, "tile.json");
  const rawData = await Bun.file(tileJsonPath).json();
  const tileData = TileSchema.parse(rawData);
  const skillsMap = tileData.skills ?? {};

  let allValid = true;

  for (const [skillName, skillInfo] of Object.entries(skillsMap)) {
    const skillMdPath = join(tilePath, skillInfo.path);
    if (!existsSync(skillMdPath)) {
      logger.error(
        `Missing SKILL.md for skill "${skillName}": ${skillInfo.path}`,
      );
      allValid = false;
    }
  }

  return allValid;
};
