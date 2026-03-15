import { lstatSync, readlinkSync, symlinkSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";

export type LinkAction = "skip" | "create" | "update";

export const processTileSkillLink = (
  skillName: string,
  target: string,
  skillsDir: string,
  dryRun: boolean,
): LinkAction => {
  const linkPath = join(skillsDir, `tessl__${skillName}`);

  let linkStat: ReturnType<typeof lstatSync> | null = null;
  try {
    linkStat = lstatSync(linkPath);
  } catch {
    // does not exist — will create below
  }

  if (linkStat !== null) {
    if (!linkStat.isSymbolicLink()) {
      logger.warning(
        `warning: ${linkPath} exists and is not a symlink, skipping`,
      );
      return "skip";
    }

    const existing = readlinkSync(linkPath);
    if (existing === target) {
      logger.debug(`skip: ${linkPath} -> ${target} (already correct)`);
      return "skip";
    }

    if (dryRun) {
      logger.info(`Would update: ${linkPath} -> ${target} (was ${existing})`);
      return "update";
    }

    unlinkSync(linkPath);
    logger.info(`update: ${linkPath} -> ${target} (was ${existing})`);
    symlinkSync(target, linkPath);
    return "update";
  }

  if (dryRun) {
    logger.info(`Would create: ${linkPath} -> ${target}`);
    return "create";
  }

  symlinkSync(target, linkPath);
  logger.success(`create: ${linkPath} -> ${target}`);
  return "create";
};
