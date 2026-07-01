import { existsSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";

export const validateTileExists = (tilePath: string): boolean => {
  const pluginJson = join(tilePath, ".tessl-plugin", "plugin.json");
  const tileJson = join(tilePath, "tile.json");

  if (!existsSync(pluginJson) && !existsSync(tileJson)) {
    logger.error("Missing skill manifest (no plugin.json or tile.json)");
    return false;
  }

  return true;
};
