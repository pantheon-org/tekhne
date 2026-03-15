import { readFileSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";
import { processTileSkillLink } from "./process-tile-skill-link";

interface TileJson {
  skills: Record<string, unknown>;
}

export const syncTileSymlinks = async (
  repoRoot: string,
  tilesDir: string,
  skillsDir: string,
  dryRun: boolean,
): Promise<{ created: number; updated: number; skipped: number }> => {
  const stats = { created: 0, updated: 0, skipped: 0 };

  const globber = new Bun.Glob(`${tilesDir}/*/*/tile.json`);
  const tileJsonPaths: string[] = [];
  for await (const path of globber.scan({ cwd: repoRoot })) {
    tileJsonPaths.push(path);
  }

  for (const tileJsonRelPath of tileJsonPaths) {
    const tileJsonAbsPath = join(repoRoot, tileJsonRelPath);
    const tileDir = tileJsonRelPath.replace(/\/tile\.json$/, "");
    const target = join(repoRoot, tileDir);

    let tileJson: TileJson;
    try {
      tileJson = JSON.parse(readFileSync(tileJsonAbsPath, "utf8")) as TileJson;
    } catch (err) {
      logger.error(`Failed to parse ${tileJsonAbsPath}: ${err}`);
      continue;
    }

    const skillNames = Object.keys(tileJson.skills ?? {});

    for (const skillName of skillNames) {
      const action = processTileSkillLink(skillName, target, skillsDir, dryRun);
      if (action === "create") stats.created++;
      else if (action === "update") stats.updated++;
      else stats.skipped++;
    }
  }

  return stats;
};
