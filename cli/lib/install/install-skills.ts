import {
  existsSync,
  mkdirSync,
  readlinkSync,
  symlinkSync,
  unlinkSync,
} from "node:fs";
import { homedir } from "node:os";
import { join, resolve } from "node:path";
import { $ } from "bun";
import { logger } from "../utils/logger";

interface InstallOptions {
  agent: string[];
  global: boolean;
  dryRun: boolean;
}

const AGENT_PATHS: Record<string, (isGlobal: boolean, cwd: string) => string> =
  {
    opencode: (isGlobal: boolean, cwd: string) =>
      isGlobal
        ? join(homedir(), ".config", "opencode", "skills")
        : join(cwd, ".agents", "skills"),
    cursor: (_isGlobal: boolean) =>
      join(homedir(), ".config", "cursor", "skills"),
    gemini: (_isGlobal: boolean) =>
      join(homedir(), ".config", "gemini", "skills"),
    claude: (_isGlobal: boolean) =>
      join(homedir(), ".config", "claude", "skills"),
    codex: (_isGlobal: boolean) =>
      join(homedir(), ".config", "codex", "skills"),
  };

async function findSkills(): Promise<string[]> {
  const output = await $`find skills -name "SKILL.md" -type f`.text();
  return output
    .trim()
    .split("\n")
    .filter(Boolean)
    .map((path) => path.replace("/SKILL.md", ""));
}

function getSkillName(skillPath: string): string {
  const parts = skillPath.split("/").slice(1);
  return parts.join("--");
}

function ensureDirectory(dir: string, dryRun: boolean): void {
  if (dryRun) {
    if (!existsSync(dir)) {
      logger.info(`Would create directory: ${dir}`);
    }
    return;
  }

  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
    logger.success(`Created directory: ${dir}`);
  }
}

function createSymlink(
  source: string,
  target: string,
  dryRun: boolean,
): boolean {
  if (existsSync(target)) {
    try {
      const existing = readlinkSync(target);
      const resolvedSource = resolve(source);

      if (existing === resolvedSource) {
        logger.debug(`Already linked: ${target}`);
        return false;
      }

      if (dryRun) {
        logger.warning(`Would replace symlink: ${target} -> ${source}`);
        return true;
      }

      unlinkSync(target);
      logger.info(`Removed old symlink: ${target}`);
    } catch {
      logger.warning(`Path exists but is not a symlink: ${target}`);
      return false;
    }
  }

  if (dryRun) {
    logger.info(`Would symlink: ${target} -> ${source}`);
    return true;
  }

  try {
    symlinkSync(resolve(source), target, "dir");
    logger.success(`Linked: ${target.split("/").pop()}`);
    return true;
  } catch (error: unknown) {
    logger.error(`Failed to symlink ${source} -> ${target}: ${error}`);
    return false;
  }
}

export async function installSkills(options: InstallOptions): Promise<void> {
  const cwd = process.cwd();

  if (options.dryRun) {
    logger.header("Dry Run: Install Skills");
  } else {
    logger.header("Installing Skills");
  }

  const skills = await findSkills();
  logger.info(`Found ${skills.length} skills`);

  const agents = options.agent;
  logger.info(`Target agents: ${agents.join(", ")}`);
  logger.info(`Mode: ${options.global ? "global" : "local"}`);

  const stats: Record<
    string,
    { installed: number; skipped: number; failed: number }
  > = {};

  for (const agent of agents) {
    if (!AGENT_PATHS[agent]) {
      logger.error(`Unknown agent: ${agent}`);
      continue;
    }

    logger.header(`\nInstalling for ${agent}`);

    const targetDir = AGENT_PATHS[agent](options.global, cwd);
    logger.info(`Target directory: ${targetDir}`);

    ensureDirectory(targetDir, options.dryRun);

    stats[agent] = { installed: 0, skipped: 0, failed: 0 };

    for (const skillPath of skills) {
      const skillName = getSkillName(skillPath);
      const source = join(cwd, skillPath);
      const target = join(targetDir, skillName);

      const result = createSymlink(source, target, options.dryRun);

      if (result) {
        stats[agent].installed++;
      } else if (existsSync(target)) {
        stats[agent].skipped++;
      } else {
        stats[agent].failed++;
      }
    }
  }

  logger.header("\nInstallation Summary");

  for (const agent of agents) {
    if (!stats[agent]) continue;

    const { installed, skipped, failed } = stats[agent];
    logger.info(`\n${agent}:`);
    if (installed > 0) logger.success(`  Installed: ${installed}`);
    if (skipped > 0) logger.debug(`  Skipped: ${skipped}`);
    if (failed > 0) logger.error(`  Failed: ${failed}`);
  }

  if (options.dryRun) {
    logger.warning("\nDry run completed. No changes made.");
  }
}
