import { writeFileSync } from "node:fs";
import { logger } from "../utils/logger";
import { findAllSkills, findAllTiles, findUntiledSkills } from "./discovery";
import { generateDocsTilesPage, writeBadgeFiles } from "./rendering";
import { showDryRunDiff } from "./show-dry-run-diff";
import type { UpdateOptions } from "./types";

const DOCS_TILES_PATH = "docs/src/content/docs/tiles.md";

export const updateReadme = async (options: UpdateOptions): Promise<void> => {
  logger.info("Finding all tiles...");
  const tiles = await findAllTiles();
  logger.info(`Found ${tiles.length} tiles`);

  logger.info("Finding all skills...");
  const allSkills = await findAllSkills();

  const untiledSkills = findUntiledSkills(allSkills, tiles);
  logger.info(`Found ${untiledSkills.length} untiled skills`);

  logger.info("Generating badge SVG files...");
  writeBadgeFiles();

  logger.info("Generating docs tiles page...");
  const docsTilesContent = await generateDocsTilesPage(tiles, untiledSkills);

  if (options.dryRun) {
    await showDryRunDiff(DOCS_TILES_PATH, docsTilesContent);
  } else {
    writeFileSync(DOCS_TILES_PATH, docsTilesContent);
    logger.success(`${DOCS_TILES_PATH} written`);
  }
};
