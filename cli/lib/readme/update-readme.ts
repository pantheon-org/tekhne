import { readFileSync, writeFileSync } from "node:fs";
import { logger } from "../utils/logger";
import { findAllSkills, findAllTiles, findUntiledSkills } from "./discovery";
import {
  buildReadmeCatalogLine,
  generateDocsTilesPage,
  writeBadgeFiles,
} from "./rendering";
import { showDryRunDiff } from "./show-dry-run-diff";
import type { UpdateOptions } from "./types";

const README_PATH = "README.md";
const DOCS_TILES_PATH = "docs/src/content/docs/tiles.md";
const CATALOG_STATS_MARKER = "<!-- skill-catalog-stats -->";

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

  const totalSkills = tiles.reduce((sum, t) => sum + t.skills.length, 0);
  const catalogLine = buildReadmeCatalogLine(tiles.length, totalSkills);

  const readmeContent = readFileSync(README_PATH, "utf-8");
  const readmeLines = readmeContent.split("\n");
  const markerIndex = readmeLines.findIndex((l) =>
    l.includes(CATALOG_STATS_MARKER),
  );

  let newReadmeContent = readmeContent;
  if (markerIndex !== -1 && markerIndex + 1 < readmeLines.length) {
    readmeLines[markerIndex + 1] = catalogLine;
    newReadmeContent = readmeLines.join("\n");
  }

  logger.info("Generating docs tiles page...");
  const docsTilesContent = await generateDocsTilesPage(tiles, untiledSkills);

  if (options.dryRun) {
    await showDryRunDiff(README_PATH, newReadmeContent);
    await showDryRunDiff(DOCS_TILES_PATH, docsTilesContent);
  } else {
    writeFileSync(README_PATH, newReadmeContent);
    writeFileSync(DOCS_TILES_PATH, docsTilesContent);
    logger.success(`${README_PATH} and ${DOCS_TILES_PATH} written`);
  }
};
