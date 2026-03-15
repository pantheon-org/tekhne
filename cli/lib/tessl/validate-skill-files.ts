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
  const skillsList = tileData.skills || [];

  let allValid = true;

  for (const skill of skillsList) {
    const skillMdPath = join(tilePath, skill.name, "SKILL.md");
    if (!existsSync(skillMdPath)) {
      logger.error(`Missing SKILL.md for skill: ${skill.name}`);
      allValid = false;
    }
  }

  return allValid;
};
