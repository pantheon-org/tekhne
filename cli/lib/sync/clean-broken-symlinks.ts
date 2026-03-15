import { existsSync, lstatSync, readdirSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";

export const cleanBrokenSymlinks = (
  skillsDir: string,
  dryRun: boolean,
): number => {
  let removed = 0;

  if (!existsSync(skillsDir)) {
    return removed;
  }

  const entries = readdirSync(skillsDir);
  for (const entry of entries) {
    const linkPath = join(skillsDir, entry);
    try {
      const stat = lstatSync(linkPath);
      if (stat.isSymbolicLink() && !existsSync(linkPath)) {
        if (dryRun) {
          logger.info(`Would remove broken symlink: ${linkPath}`);
        } else {
          unlinkSync(linkPath);
          logger.info(`unlink: ${linkPath} (broken symlink)`);
        }
        removed++;
      }
    } catch {
      // lstatSync can throw on very broken paths — skip silently
    }
  }

  return removed;
};
