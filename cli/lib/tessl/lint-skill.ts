import { exec } from "../utils/exec";
import { logger } from "../utils/logger";

export const lintSkill = async (skillPath: string): Promise<boolean> => {
  logger.info(`Linting skill: ${skillPath}`);
  const { exitCode, stderr } = await exec(`tessl skill lint ${skillPath}`);
  if (exitCode !== 0) {
    logger.error(`Lint failed: ${stderr}`);
    return false;
  }
  logger.success("Lint passed");
  return true;
};
