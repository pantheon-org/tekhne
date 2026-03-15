import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { logger } from "../utils/logger";

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

export const syncMcpConfig = (
  _cwd: string,
  mcpJsonPath: string,
  opencodeJsonPath: string,
  dryRun: boolean,
): boolean => {
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
};
