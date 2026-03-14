import {
  existsSync,
  lstatSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  readlinkSync,
  symlinkSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";

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

// ---------------------------------------------------------------------------
// Step 1: Remove broken symlinks from the skills directory
// ---------------------------------------------------------------------------
function cleanBrokenSymlinks(skillsDir: string, dryRun: boolean): number {
  let removed = 0;

  if (!existsSync(skillsDir)) {
    return removed;
  }

  const entries = readdirSync(skillsDir);
  for (const entry of entries) {
    const linkPath = join(skillsDir, entry);
    try {
      const stat = lstatSync(linkPath);
      if (stat.isSymbolicLink() && !existsSync(linkPath)) {
        if (dryRun) {
          logger.info(`Would remove broken symlink: ${linkPath}`);
        } else {
          unlinkSync(linkPath);
          logger.info(`unlink: ${linkPath} (broken symlink)`);
        }
        removed++;
      }
    } catch {
      // lstatSync can throw on very broken paths — skip silently
    }
  }

  return removed;
}

// ---------------------------------------------------------------------------
// Step 2: Sync .mcp.json → opencode.json mcp section
// ---------------------------------------------------------------------------
interface McpServer {
  command: string;
  args?: string[];
  [key: string]: unknown;
}

interface McpJson {
  mcpServers: Record<string, McpServer>;
}

interface OpenCodeMcpEntry {
  type: "local";
  command: string[];
}

interface OpenCodeJson {
  $schema?: string;
  mcp?: Record<string, OpenCodeMcpEntry>;
  [key: string]: unknown;
}

function syncMcpConfig(
  _cwd: string,
  mcpJsonPath: string,
  opencodeJsonPath: string,
  dryRun: boolean,
): boolean {
  if (!existsSync(mcpJsonPath)) {
    return false;
  }

  let mcpJson: McpJson;
  try {
    mcpJson = JSON.parse(readFileSync(mcpJsonPath, "utf8")) as McpJson;
  } catch (err) {
    logger.error(`Failed to parse ${mcpJsonPath}: ${err}`);
    return false;
  }

  // Transform mcpServers → opencode mcp format:
  //   { command, args? } → { type: "local", command: [command, ...args] }
  const mcp: Record<string, OpenCodeMcpEntry> = {};
  for (const [key, value] of Object.entries(mcpJson.mcpServers ?? {})) {
    mcp[key] = {
      type: "local",
      command: [value.command, ...(value.args ?? [])],
    };
  }

  let existing: OpenCodeJson = { $schema: "https://opencode.ai/config.json" };
  if (existsSync(opencodeJsonPath)) {
    try {
      existing = JSON.parse(
        readFileSync(opencodeJsonPath, "utf8"),
      ) as OpenCodeJson;
    } catch (err) {
      logger.error(`Failed to parse ${opencodeJsonPath}: ${err}`);
      return false;
    }
  }

  const updated: OpenCodeJson = {
    ...existing,
    $schema: "https://opencode.ai/config.json",
    mcp,
  };

  const updatedStr = JSON.stringify(updated, null, 2);
  const existingStr = JSON.stringify(existing, null, 2);

  if (updatedStr === existingStr) {
    logger.debug(`skip: ${opencodeJsonPath} mcp section already up to date`);
    return false;
  }

  if (dryRun) {
    logger.info(
      `Would update: ${opencodeJsonPath} with mcp servers from ${mcpJsonPath}`,
    );
    return true;
  }

  writeFileSync(opencodeJsonPath, `${updatedStr}\n`, "utf8");
  logger.success(
    `updated: ${opencodeJsonPath} with mcp servers from ${mcpJsonPath}`,
  );
  return true;
}

// ---------------------------------------------------------------------------
// Step 3: Create/update Tessl tile symlinks
// ---------------------------------------------------------------------------
interface TileJson {
  skills: Record<string, unknown>;
}

type LinkAction = "skip" | "create" | "update";

function processTileSkillLink(
  skillName: string,
  target: string,
  skillsDir: string,
  dryRun: boolean,
): LinkAction {
  const linkPath = join(skillsDir, `tessl__${skillName}`);

  let linkStat: ReturnType<typeof lstatSync> | null = null;
  try {
    linkStat = lstatSync(linkPath);
  } catch {
    // does not exist — will create below
  }

  if (linkStat !== null) {
    if (!linkStat.isSymbolicLink()) {
      logger.warning(
        `warning: ${linkPath} exists and is not a symlink, skipping`,
      );
      return "skip";
    }

    const existing = readlinkSync(linkPath);
    if (existing === target) {
      logger.debug(`skip: ${linkPath} -> ${target} (already correct)`);
      return "skip";
    }

    if (dryRun) {
      logger.info(`Would update: ${linkPath} -> ${target} (was ${existing})`);
      return "update";
    }

    unlinkSync(linkPath);
    logger.info(`update: ${linkPath} -> ${target} (was ${existing})`);
    symlinkSync(target, linkPath);
    return "update";
  }

  if (dryRun) {
    logger.info(`Would create: ${linkPath} -> ${target}`);
    return "create";
  }

  symlinkSync(target, linkPath);
  logger.success(`create: ${linkPath} -> ${target}`);
  return "create";
}

async function syncTileSymlinks(
  repoRoot: string,
  tilesDir: string,
  skillsDir: string,
  dryRun: boolean,
): Promise<{ created: number; updated: number; skipped: number }> {
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
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------
export async function syncOpencode(options: SyncOptions): Promise<void> {
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
}
