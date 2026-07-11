import {
  lstatSync,
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
  // Resolve the target directory through any symlinked parents (e.g.
  // ~/.claude -> ~/.config/claude). The link is physically stored in the real
  // directory, so the relative link text must be computed from there or it
  // dangles when the parent is a symlink. Fall back to the logical path when
  // the directory does not exist yet (e.g. dry-run, which skips mkdir).
  const rawTargetDir = dirname(target);
  let targetDir: string;
  try {
    targetDir = realpathSync.native(rawTargetDir);
  } catch {
    targetDir = rawTargetDir;
  }
  const relativeSource = relative(targetDir, resolvedSource);

  // lstat (not existsSync) so we detect broken/dangling symlinks too.
  let stat: ReturnType<typeof lstatSync> | null = null;
  try {
    stat = lstatSync(target);
  } catch {
    stat = null;
  }

  if (stat) {
    if (!stat.isSymbolicLink()) {
      // Merge-not-clobber: never remove a real file or directory (e.g. a
      // frozen external skill installed independently).
      logger.warning(`Path exists but is not a symlink: ${target}`);
      return false;
    }

    const existing = readlinkSync(target);
    let resolvedExisting: string | null = null;
    try {
      resolvedExisting = realpathSync.native(resolve(targetDir, existing));
    } catch {
      // Dangling link — fall through to replace it.
      resolvedExisting = null;
    }

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
