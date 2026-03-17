import { lstatSync } from "node:fs";
import { join } from "node:path";
import type { InstalledSkill } from "./find-installed-skills";
import { resolveHardCopySource } from "./resolve-hard-copy-source";
import { resolveSymlinkSource } from "./resolve-symlink-source";

export const classifyEntry = (
  entry: string,
  targetDir: string,
  resolvedCwd: string,
): InstalledSkill | null => {
  const targetPath = join(targetDir, entry);

  let stat: ReturnType<typeof lstatSync>;
  try {
    stat = lstatSync(targetPath);
  } catch {
    return null;
  }

  if (stat.isSymbolicLink()) {
    const sourcePath = resolveSymlinkSource(targetPath, targetDir, resolvedCwd);
    if (!sourcePath) return null;
    return { name: entry, targetPath, sourcePath, isSymlink: true };
  }

  if (stat.isDirectory()) {
    const sourcePath = resolveHardCopySource(targetPath, resolvedCwd);
    if (!sourcePath) return null;
    return { name: entry, targetPath, sourcePath, isSymlink: false };
  }

  return null;
};
