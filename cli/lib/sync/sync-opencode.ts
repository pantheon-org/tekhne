import { existsSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";
import { cleanBrokenSymlinks } from "./clean-broken-symlinks";
import { syncMcpConfig } from "./sync-mcp-config";
import { syncTileSymlinks } from "./sync-tile-symlinks";

export interface SyncOptions {
  /** Use .opencode/skills instead of .agents/skills */
  opencodeSkills: boolean;
  /** Show what would change without writing anything */
  dryRun: boolean;
}

interface SyncStats {
  brokenLinksRemoved: number;
  mcpUpdated: boolean;
  tileLinksCreated: number;
  tileLinksUpdated: number;
  tileLinksSkipped: number;
}

export const syncOpencode = async (options: SyncOptions): Promise<void> => {
  const cwd = process.cwd();
  const skillsDir = options.opencodeSkills
    ? join(cwd, ".opencode", "skills")
    : join(cwd, ".agents", "skills");
  const mcpJsonPath = join(cwd, ".mcp.json");
  const opencodeJsonPath = join(cwd, "opencode.json");
  const tilesDir = ".tessl/tiles";

  if (options.dryRun) {
    logger.header("Dry Run: Sync OpenCode");
  } else {
    logger.header("Syncing OpenCode");
  }

  logger.info(`Skills directory: ${skillsDir}`);

  // Ensure skills dir exists
  if (!options.dryRun) {
    mkdirSync(skillsDir, { recursive: true });
  } else if (!existsSync(skillsDir)) {
    logger.info(`Would create directory: ${skillsDir}`);
  }

  // Step 1: clean broken symlinks
  logger.info("\nCleaning broken symlinks...");
  const removed = cleanBrokenSymlinks(skillsDir, options.dryRun);

  // Step 2: sync MCP config
  logger.info("\nSyncing MCP config...");
  const mcpUpdated = syncMcpConfig(
    cwd,
    mcpJsonPath,
    opencodeJsonPath,
    options.dryRun,
  );

  // Step 3: sync tile symlinks
  logger.info("\nSyncing Tessl tile symlinks...");
  const tileStats = await syncTileSymlinks(
    cwd,
    tilesDir,
    skillsDir,
    options.dryRun,
  );

  const stats: SyncStats = {
    brokenLinksRemoved: removed,
    mcpUpdated,
    tileLinksCreated: tileStats.created,
    tileLinksUpdated: tileStats.updated,
    tileLinksSkipped: tileStats.skipped,
  };

  // Summary
  logger.header("Sync Summary");
  if (stats.brokenLinksRemoved > 0) {
    logger.info(`Broken symlinks removed: ${stats.brokenLinksRemoved}`);
  }
  if (stats.mcpUpdated) {
    logger.success("MCP config updated");
  } else {
    logger.debug("MCP config: no changes");
  }
  logger.info(
    `Tile symlinks — created: ${stats.tileLinksCreated}, updated: ${stats.tileLinksUpdated}, skipped: ${stats.tileLinksSkipped}`,
  );

  if (options.dryRun) {
    logger.warning("\nDry run completed. No changes made.");
  } else {
    logger.success("\nSync complete.");
  }
};
