import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { FileNotFoundError } from "../utils/errors";
import { logger } from "../utils/logger";
import { findAllSkills, findAllTiles, findUntiledSkills } from "./discovery";
import {
  generateCatalogContent,
  generateDocsTilesPage,
  generateReadmeSummaryTables,
} from "./rendering";
import { parseReadmeSections } from "./sections";
import { showDryRunDiff } from "./show-dry-run-diff";
import type { UpdateOptions } from "./types";
import { DOMAINS } from "./types";

const TILES_PATH = "TILES.md";
const DOCS_TILES_PATH = "docs/src/content/docs/tiles.md";

export const updateReadme = async (options: UpdateOptions): Promise<void> => {
  const readmePath = "README.md";

  if (!existsSync(readmePath)) {
    throw new FileNotFoundError(readmePath);
  }

  logger.info("Finding all tiles...");
  const tiles = await findAllTiles();
  logger.info(`Found ${tiles.length} tiles`);

  logger.info("Finding all skills...");
  const allSkills = await findAllSkills();

  const untiledSkills = findUntiledSkills(allSkills, tiles);
  logger.info(`Found ${untiledSkills.length} untiled skills`);

  logger.info("Generating content...");
  const [summaryTables, catalogContent, docsTilesContent] = await Promise.all([
    generateReadmeSummaryTables(tiles, untiledSkills),
    generateCatalogContent(tiles, untiledSkills),
    generateDocsTilesPage(tiles, untiledSkills),
  ]);

  logger.info("Updating README.md...");
  const content = readFileSync(readmePath, "utf-8");
  const lines = content.split("\n");
  const domainHeaders = DOMAINS.map((d) => d.title);

  const { beforeSkills, afterSkills } = parseReadmeSections(
    lines,
    domainHeaders,
  );

  const newReadmeContent =
    `${beforeSkills.join("\n") + summaryTables}\n${afterSkills.join("\n")}`.trimEnd() +
    "\n";

  if (options.dryRun) {
    await showDryRunDiff(readmePath, newReadmeContent);
    logger.info(
      `\n=== DRY RUN - ${TILES_PATH} would be written (${catalogContent.length} chars) ===`,
    );
    logger.info(
      `=== DRY RUN - ${DOCS_TILES_PATH} would be written (${docsTilesContent.length} chars) ===`,
    );
  } else {
    writeFileSync(readmePath, newReadmeContent);
    writeFileSync(TILES_PATH, catalogContent);
    writeFileSync(DOCS_TILES_PATH, docsTilesContent);
    logger.success(
      `README.md updated with summary tables; ${TILES_PATH} and ${DOCS_TILES_PATH} written`,
    );
  }
};
