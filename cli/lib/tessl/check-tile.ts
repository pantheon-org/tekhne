import { logger } from "../utils/logger";
import { runLintCheck } from "./run-lint-check";
import { validateSkillFiles } from "./validate-skill-files";
import { validateTileExists } from "./validate-tile-exists";

export const checkTile = async (tilePath: string): Promise<boolean> => {
  logger.info(`\nChecking ${tilePath}...`);

  if (!validateTileExists(tilePath)) {
    return false;
  }

  const skillsValid = await validateSkillFiles(tilePath);
  const lintPassed = await runLintCheck(tilePath);

  return skillsValid && lintPassed;
};
