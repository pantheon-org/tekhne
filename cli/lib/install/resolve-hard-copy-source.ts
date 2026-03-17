import { existsSync } from "node:fs";
import { join } from "node:path";
import { isOwnedByCwd } from "./is-owned-by-cwd";

export const resolveHardCopySource = (
  targetPath: string,
  resolvedCwd: string,
): string | null => {
  const markerPath = join(targetPath, ".tekhne-source");
  if (!existsSync(markerPath)) return null;
  try {
    const sourcePath = Bun.file(markerPath).toString().trim();
    return isOwnedByCwd(sourcePath, resolvedCwd) ? sourcePath : null;
  } catch {
    return null;
  }
};
