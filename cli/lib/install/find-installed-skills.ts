import { existsSync, readdirSync, realpathSync } from "node:fs";
import { resolve } from "node:path";
import { logger } from "../utils/logger";
import { classifyEntry } from "./classify-entry";

export interface InstalledSkill {
  name: string;
  targetPath: string;
  sourcePath: string;
  isSymlink: boolean;
}

export const findInstalledSkills = (
  targetDir: string,
  cwd: string,
): InstalledSkill[] => {
  if (!existsSync(targetDir)) {
    return [];
  }

  const resolvedCwd = realpathSync.native(resolve(cwd));

  let entries: string[];
  try {
    entries = readdirSync(targetDir);
  } catch {
    logger.warning(`Cannot read directory: ${targetDir}`);
    return [];
  }

  return entries
    .map((entry) => classifyEntry(entry, targetDir, resolvedCwd))
    .filter((skill): skill is InstalledSkill => skill !== null);
};
