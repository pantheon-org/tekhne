import { existsSync } from "node:fs";
import { join } from "node:path";
import { ValidationError } from "../utils/errors";
import { logger } from "../utils/logger";
import { importSkill } from "./import-skill";
import { lintSkill } from "./lint-skill";
import { publishSkill } from "./publish-skill";
import { reviewSkill } from "./review-skill";

export const processSkill = async (
  skillPath: string,
  workspace: string,
): Promise<void> => {
  const tileJsonPath = join(skillPath, "tile.json");

  if (!existsSync(tileJsonPath)) {
    logger.info("No tile.json found, importing skill...");
    await importSkill(skillPath);
    return;
  }

  const lintPassed = await lintSkill(skillPath);
  if (!lintPassed) {
    throw new ValidationError(`Lint failed for ${skillPath}`);
  }

  const reviewPassed = await reviewSkill(skillPath, workspace);
  if (!reviewPassed) {
    throw new ValidationError(`Review failed for ${skillPath}`);
  }

  await publishSkill(skillPath, workspace);
};
