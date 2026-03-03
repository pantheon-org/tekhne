import { existsSync } from "node:fs";
import { join } from "node:path";
import { TileSchema } from "../schemas/tile.schema";
import { logger } from "../utils/logger";
import { exec } from "../utils/shell";

function validateTileExists(tilePath: string): boolean {
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
}

async function validateSkillFiles(tilePath: string): Promise<boolean> {
  const tileJsonPath = join(tilePath, "tile.json");
  const rawData = await Bun.file(tileJsonPath).json();
  const tileData = TileSchema.parse(rawData);
  const skillsList = tileData.skills || [];

  let allValid = true;

  for (const skill of skillsList) {
    const skillMdPath = join(tilePath, skill.name, "SKILL.md");
    if (!existsSync(skillMdPath)) {
      logger.error(`Missing SKILL.md for skill: ${skill.name}`);
      allValid = false;
    }
  }

  return allValid;
}

async function runLintCheck(tilePath: string): Promise<boolean> {
  logger.info("Running tessl skill lint...");
  const { exitCode, stdout, stderr } = await exec(
    `tessl skill lint ${tilePath}`,
  );

  if (exitCode !== 0) {
    logger.error("Lint failed");
    console.error(stderr);
    return false;
  }

  logger.success("Lint passed");

  if (stdout.includes("warning") || stdout.includes("orphaned")) {
    logger.warning("Warnings detected (not blocking)");
    console.log(stdout);
  }

  return true;
}

async function checkTile(tilePath: string): Promise<boolean> {
  logger.info(`\nChecking ${tilePath}...`);

  if (!validateTileExists(tilePath)) {
    return false;
  }

  const skillsValid = await validateSkillFiles(tilePath);
  const lintPassed = await runLintCheck(tilePath);

  return skillsValid && lintPassed;
}

export async function tesslPublishCheck(tiles: string[]): Promise<void> {
  logger.header("Tessl Publish Pre-check");

  const results = await Promise.all(tiles.map((tile) => checkTile(tile)));
  const allPassed = results.every((r) => r);

  if (!allPassed) {
    logger.error("\nSome checks failed");
    process.exit(1);
  }

  logger.success("\nAll checks passed");
}
