import { ShellCommandError } from "../utils/errors";
import { exec } from "../utils/exec";
import { logger } from "../utils/logger";
import { isSkillPublished } from "./is-skill-published";

export const publishSkill = async (
  skillPath: string,
  workspace: string,
): Promise<void> => {
  const published = await isSkillPublished(skillPath, workspace);
  if (published) {
    logger.info("Skill already published");
    return;
  }

  logger.info(`Publishing skill to ${workspace}...`);
  const { exitCode, stderr } = await exec(
    `tessl skill publish ${skillPath} --public`,
  );
  if (exitCode !== 0) {
    throw new ShellCommandError(
      `tessl skill publish ${skillPath}`,
      stderr,
      exitCode,
    );
  }
  logger.success("Published successfully");
};
