import {
  existsSync,
  readlinkSync,
  realpathSync,
  symlinkSync,
  unlinkSync,
} from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { logger } from "../utils/logger";

export const createSymlink = (
  source: string,
  target: string,
  dryRun: boolean,
): boolean => {
  const resolvedSource = realpathSync.native(resolve(source));
  const relativeSource = relative(dirname(target), resolvedSource);

  if (existsSync(target)) {
    try {
      const existing = readlinkSync(target);
      // Resolve existing (may be relative or absolute) and canonicalize
      // with realpathSync to handle platform symlinks (e.g. macOS /tmp → /private/tmp)
      const resolvedExisting = realpathSync.native(
        resolve(dirname(target), existing),
      );

      if (resolvedExisting === resolvedSource) {
        logger.debug(`Already linked: ${target}`);
        return false;
      }

      if (dryRun) {
        logger.warning(`Would replace symlink: ${target} -> ${relativeSource}`);
        return true;
      }

      unlinkSync(target);
      logger.info(`Removed old symlink: ${target}`);
    } catch {
      logger.warning(`Path exists but is not a symlink: ${target}`);
      return false;
    }
  }

  if (dryRun) {
    logger.info(`Would symlink: ${target} -> ${relativeSource}`);
    return true;
  }

  try {
    symlinkSync(relativeSource, target, "dir");
    logger.success(`Linked: ${target.split("/").pop()}`);
    return true;
  } catch (error: unknown) {
    logger.error(`Failed to symlink ${source} -> ${target}: ${error}`);
    return false;
  }
};
