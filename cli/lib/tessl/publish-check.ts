import { logger } from "../utils/logger";
import { checkTile } from "./check-tile";

export const tesslPublishCheck = async (tiles: string[]): Promise<void> => {
  logger.header("Tessl Publish Pre-check");

  const results = await Promise.all(tiles.map((tile) => checkTile(tile)));
  const allPassed = results.every((r) => r);

  if (!allPassed) {
    logger.error("\nSome checks failed");
    process.exit(1);
  }

  logger.success("\nAll checks passed");
};
