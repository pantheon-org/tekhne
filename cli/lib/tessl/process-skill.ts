import { ValidationError } from "../utils/errors";
import { logger } from "../utils/logger";
import { readManifest } from "../utils/skill-manifest";
import { importSkill } from "./import-skill";
import { lintSkill } from "./lint-skill";
import { publishSkill } from "./publish-skill";
import { reviewSkill } from "./review-skill";

export const processSkill = async (
  skillPath: string,
  workspace: string,
): Promise<void> => {
  const manifest = await readManifest(skillPath);

  if (!manifest) {
    logger.info("No manifest found, importing skill...");
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
