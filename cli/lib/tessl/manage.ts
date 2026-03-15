import { dirname } from "node:path";
import { $ } from "bun";
import { ValidationError } from "../utils/errors";
import { logger } from "../utils/logger";
import { processSkill } from "./process-skill";

interface ManageOptions {
  workspace: string;
}

export const tesslManage = async (
  skill: string | undefined,
  options: ManageOptions,
): Promise<void> => {
  if (skill) {
    logger.header(`Managing skill: ${skill}`);
    await processSkill(skill, options.workspace);
    return;
  }

  logger.header("Managing all skills");

  const skillDirs =
    await $`find skills -name "tile.json" -o -name "SKILL.md"`.text();
  const paths = skillDirs
    .trim()
    .split("\n")
    .map((p) => dirname(p))
    .filter((p, i, arr) => arr.indexOf(p) === i);

  logger.info(`Found ${paths.length} skills`);

  let processed = 0;
  let failed = 0;
  const errors: Array<{ path: string; error: Error }> = [];

  for (const skillPath of paths) {
    logger.info(`\nProcessing ${skillPath}...`);
    try {
      await processSkill(skillPath, options.workspace);
      processed++;
    } catch (error) {
      logger.error(`Failed to process ${skillPath}`);
      failed++;
      errors.push({ path: skillPath, error: error as Error });
    }
  }

  logger.header("Management Summary");
  logger.success(`Processed: ${processed}`);
  if (failed > 0) {
    logger.error(`Failed: ${failed}`);
    throw new ValidationError(`${failed} skill(s) failed processing`);
  }
};
