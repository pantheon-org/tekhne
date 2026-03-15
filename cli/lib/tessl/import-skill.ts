import { ShellCommandError } from "../utils/errors";
import { exec } from "../utils/exec";
import { logger } from "../utils/logger";

export const importSkill = async (skillPath: string): Promise<void> => {
  logger.info(`Importing skill: ${skillPath}`);
  const { exitCode, stderr } = await exec(`tessl skill import ${skillPath}`);
  if (exitCode !== 0) {
    throw new ShellCommandError(
      `tessl skill import ${skillPath}`,
      stderr,
      exitCode,
    );
  }
  logger.success("Imported successfully");
};
