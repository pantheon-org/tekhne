import { readFileSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";
import { processTileSkillLink } from "./process-tile-skill-link";

interface PluginJson {
  skills: Array<string | { path: string }>;
}

export const syncTileSymlinks = async (
  repoRoot: string,
  tilesDir: string,
  skillsDir: string,
  dryRun: boolean,
): Promise<{ created: number; updated: number; skipped: number }> => {
  const stats = { created: 0, updated: 0, skipped: 0 };

  const globber = new Bun.Glob(`${tilesDir}/*/*/.tessl-plugin/plugin.json`);
  const manifestPaths: string[] = [];
  for await (const path of globber.scan({ cwd: repoRoot })) {
    manifestPaths.push(path);
  }

  for (const manifestRelPath of manifestPaths) {
    const tileDir = manifestRelPath.replace(
      /\/\.tessl-plugin\/plugin\.json$/,
      "",
    );
    const target = join(repoRoot, tileDir);

    let pluginJson: PluginJson;
    try {
      pluginJson = JSON.parse(
        readFileSync(join(repoRoot, manifestRelPath), "utf8"),
      ) as PluginJson;
    } catch (err) {
      logger.error(`Failed to parse ${manifestRelPath}: ${err}`);
      continue;
    }

    const skillPaths: string[] = [];
    if (Array.isArray(pluginJson.skills)) {
      for (const entry of pluginJson.skills) {
        if (typeof entry === "string") skillPaths.push(entry);
        else if (entry?.path) skillPaths.push(entry.path);
      }
    }

    for (const skillPath of skillPaths) {
      const action = processTileSkillLink(skillPath, target, skillsDir, dryRun);
      if (action === "create") stats.created++;
      else if (action === "update") stats.updated++;
      else stats.skipped++;
    }
  }

  return stats;
};
