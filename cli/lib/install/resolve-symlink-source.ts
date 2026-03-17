import { readlinkSync, realpathSync } from "node:fs";
import { resolve } from "node:path";
import { isOwnedByCwd } from "./is-owned-by-cwd";

export const resolveSymlinkSource = (
  targetPath: string,
  targetDir: string,
  resolvedCwd: string,
): string | null => {
  try {
    const link = readlinkSync(targetPath);
    const resolvedLink = realpathSync.native(resolve(targetDir, link));
    return isOwnedByCwd(resolvedLink, resolvedCwd) ? resolvedLink : null;
  } catch {
    return null;
  }
};
