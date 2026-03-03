import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { $ } from "bun";
import { TileSchema } from "../schemas/tile.schema";
import { ShellCommandError, ValidationError } from "../utils/errors";
import { logger } from "../utils/logger";
import { exec } from "../utils/shell";

interface ManageOptions {
  workspace: string;
}

async function importSkill(skillPath: string): Promise<void> {
  logger.info(`Importing skill: ${skillPath}`);
  const { exitCode, stderr } = await exec(`tessl skill import ${skillPath}`);
  if (exitCode !== 0) {
    throw new ShellCommandError(
      `tessl skill import ${skillPath}`,
      stderr,
      exitCode,
    );
  }
  logger.success("Imported successfully");
}

async function lintSkill(skillPath: string): Promise<boolean> {
  logger.info(`Linting skill: ${skillPath}`);
  const { exitCode, stderr } = await exec(`tessl skill lint ${skillPath}`);
  if (exitCode !== 0) {
    logger.error(`Lint failed: ${stderr}`);
    return false;
  }
  logger.success("Lint passed");
  return true;
}

async function reviewSkill(
  skillPath: string,
  _workspace: string,
): Promise<boolean> {
  logger.info(`Reviewing skill: ${skillPath}`);

  const tileJsonPath = join(skillPath, "tile.json");
  if (existsSync(tileJsonPath)) {
    const rawData = await Bun.file(tileJsonPath).json();
    const tileData = TileSchema.parse(rawData);
    const skills = tileData.skills || [];

    if (skills.length > 1) {
      logger.info(`Multi-skill tile detected (${skills.length} skills)`);
      for (const skill of skills) {
        const skillDir = join(skillPath, skill.name);
        logger.info(`Reviewing skill: ${skill.name}`);
        const { exitCode, stderr } = await exec(
          `tessl skill review ${skillDir}`,
        );
        if (exitCode !== 0) {
          logger.error(`Review failed for ${skill.name}: ${stderr}`);
          return false;
        }
      }
      logger.success("All skills reviewed successfully");
      return true;
    }
  }

  const { exitCode, stderr } = await exec(`tessl skill review ${skillPath}`);
  if (exitCode !== 0) {
    logger.error(`Review failed: ${stderr}`);
    return false;
  }
  logger.success("Review passed");
  return true;
}

async function isSkillPublished(
  skillPath: string,
  workspace: string,
): Promise<boolean> {
  const tileJsonPath = join(skillPath, "tile.json");
  if (!existsSync(tileJsonPath)) {
    return false;
  }

  const rawData = await Bun.file(tileJsonPath).json();
  const tileData = TileSchema.parse(rawData);
  const tileName = tileData.name;

  const { exitCode } = await exec(`tessl search ${workspace}/${tileName}`);
  return exitCode === 0;
}

async function publishSkill(
  skillPath: string,
  workspace: string,
): Promise<void> {
  const published = await isSkillPublished(skillPath, workspace);
  if (published) {
    logger.info("Skill already published");
    return;
  }

  logger.info(`Publishing skill to ${workspace}...`);
  const { exitCode, stderr } = await exec(
    `tessl skill publish ${skillPath} --public`,
  );
  if (exitCode !== 0) {
    throw new ShellCommandError(
      `tessl skill publish ${skillPath}`,
      stderr,
      exitCode,
    );
  }
  logger.success("Published successfully");
}

async function processSkill(
  skillPath: string,
  workspace: string,
): Promise<void> {
  const tileJsonPath = join(skillPath, "tile.json");

  if (!existsSync(tileJsonPath)) {
    logger.info("No tile.json found, importing skill...");
    await importSkill(skillPath);
    return;
  }

  const lintPassed = await lintSkill(skillPath);
  if (!lintPassed) {
    throw new ValidationError(`Lint failed for ${skillPath}`);
  }

  const reviewPassed = await reviewSkill(skillPath, workspace);
  if (!reviewPassed) {
    throw new ValidationError(`Review failed for ${skillPath}`);
  }

  await publishSkill(skillPath, workspace);
}

export async function tesslManage(
  skill: string | undefined,
  options: ManageOptions,
): Promise<void> {
  if (skill) {
    logger.header(`Managing skill: ${skill}`);
    await processSkill(skill, options.workspace);
    return;
  }

  logger.header("Managing all skills");

  const skillDirs =
    await $`find skills -name "tile.json" -o -name "SKILL.md"`.text();
  const paths = skillDirs
    .trim()
    .split("\n")
    .map((p) => dirname(p))
    .filter((p, i, arr) => arr.indexOf(p) === i);

  logger.info(`Found ${paths.length} skills`);

  let processed = 0;
  let failed = 0;
  const errors: Array<{ path: string; error: Error }> = [];

  for (const skillPath of paths) {
    logger.info(`\nProcessing ${skillPath}...`);
    try {
      await processSkill(skillPath, options.workspace);
      processed++;
    } catch (error) {
      logger.error(`Failed to process ${skillPath}`);
      failed++;
      errors.push({ path: skillPath, error: error as Error });
    }
  }

  logger.header("Management Summary");
  logger.success(`Processed: ${processed}`);
  if (failed > 0) {
    logger.error(`Failed: ${failed}`);
    throw new ValidationError(`${failed} skill(s) failed processing`);
  }
}
