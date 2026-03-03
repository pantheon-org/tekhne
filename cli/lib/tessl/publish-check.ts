import { existsSync } from "node:fs";
import { join } from "node:path";
import { logger } from "../utils/logger";
import { exec } from "../utils/shell";

export async function tesslPublishCheck(tiles: string[]): Promise<void> {
  logger.header("Tessl Publish Pre-check");

  let allPassed = true;

  for (const tilePath of tiles) {
    logger.info(`\nChecking ${tilePath}...`);

    if (!existsSync(tilePath)) {
      logger.error(`Tile not found: ${tilePath}`);
      allPassed = false;
      continue;
    }

    const tileJsonPath = join(tilePath, "tile.json");
    if (!existsSync(tileJsonPath)) {
      logger.error("Missing tile.json");
      allPassed = false;
      continue;
    }

    const tileData = await Bun.file(tileJsonPath).json();
    const skillsList = tileData.skills || [];

    for (const skill of skillsList) {
      const skillMdPath = join(tilePath, skill, "SKILL.md");
      if (!existsSync(skillMdPath)) {
        logger.error(`Missing SKILL.md for skill: ${skill}`);
        allPassed = false;
      }
    }

    logger.info("Running tessl skill lint...");
    const { exitCode, stdout, stderr } = await exec(
      `tessl skill lint ${tilePath}`,
    );

    if (exitCode !== 0) {
      logger.error("Lint failed");
      console.error(stderr);
      allPassed = false;
    } else {
      logger.success("Lint passed");

      if (stdout.includes("warning") || stdout.includes("orphaned")) {
        logger.warning("Warnings detected (not blocking)");
        console.log(stdout);
      }
    }
  }

  if (!allPassed) {
    logger.error("\nSome checks failed");
    process.exit(1);
  }

  logger.success("\nAll checks passed");
}
