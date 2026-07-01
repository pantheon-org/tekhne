import { existsSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";
import { readManifest } from "../utils/skill-manifest";

export const validateSkillFiles = async (
  tilePath: string,
): Promise<boolean> => {
  const manifest = await readManifest(tilePath);
  if (!manifest) {
    logger.error("No manifest found");
    return false;
  }

  let allValid = true;

  for (const skillPathRel of manifest.skills) {
    const skillMdPath = join(tilePath, skillPathRel);
    if (!existsSync(skillMdPath)) {
      logger.error(`Missing SKILL.md: ${skillPathRel}`);
      allValid = false;
    }
  }

  return allValid;
};
