import { existsSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";

export const validateTileExists = (tilePath: string): boolean => {
  if (!existsSync(tilePath)) {
    logger.error(`Tile not found: ${tilePath}`);
    return false;
  }

  const tileJsonPath = join(tilePath, "tile.json");
  if (!existsSync(tileJsonPath)) {
    logger.error("Missing tile.json");
    return false;
  }

  return true;
};
