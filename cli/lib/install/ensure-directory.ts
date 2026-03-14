import { existsSync, mkdirSync } from "node:fs";
import { logger } from "../utils/logger";

export const ensureDirectory = (dir: string, dryRun: boolean): void => {
  if (dryRun) {
    if (!existsSync(dir)) {
      logger.info(`Would create directory: ${dir}`);
    }
    return;
  }

  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
    logger.success(`Created directory: ${dir}`);
  }
};
