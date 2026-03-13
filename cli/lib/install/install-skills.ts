import {
  existsSync,
  mkdirSync,
  readlinkSync,
  realpathSync,
  symlinkSync,
  unlinkSync,
} from "node:fs";
import { homedir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import checkbox from "@inquirer/checkbox";
import { $ } from "bun";
import { CLIError } from "../utils/errors";
import { logger } from "../utils/logger";

export interface InstallOptions {
  agent: string[];
  global: boolean;
  dryRun: boolean;
  skillDomain?: string[];
  skillSubdomain?: string[];
  interactive?: boolean;
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

// Agents that are always global (--global flag has no effect)
const ALWAYS_GLOBAL_AGENTS = new Set(["cursor", "gemini", "claude", "codex"]);

export async function findSkills(cwd: string): Promise<string[]> {
  if (!existsSync(join(cwd, "skills"))) {
    throw new CLIError(
      "skills/ directory not found. Run this command from the repository root.",
      1,
    );
  }

  const output = await $`find skills -name "SKILL.md" -type f`.cwd(cwd).text();
  return output
    .trim()
    .split("\n")
    .filter(Boolean)
    .map((path) => path.replace("/SKILL.md", ""));
}

export function getSkillName(skillPath: string): string {
  const parts = skillPath.split("/").slice(1);
  return parts.join("--");
}

export function filterSkills(
  skills: string[],
  options: Pick<InstallOptions, "skillDomain" | "skillSubdomain">,
): string[] {
  const { skillDomain, skillSubdomain } = options;
  const hasDomainFilter = skillDomain && skillDomain.length > 0;
  const hasSubdomainFilter = skillSubdomain && skillSubdomain.length > 0;

  if (!hasDomainFilter && !hasSubdomainFilter) {
    return skills;
  }

  return skills.filter((skillPath) => {
    // skillPath format: "skills/<domain>/<subdomain>[/leaf]"
    const parts = skillPath.split("/");
    const domain = parts[1];
    const subdomain = parts[2];

    const domainMatch = hasDomainFilter ? skillDomain?.includes(domain) : true;
    const subdomainMatch = hasSubdomainFilter
      ? skillSubdomain?.includes(subdomain)
      : true;

    return domainMatch && subdomainMatch;
  });
}

export async function selectSkillsInteractively(
  skills: string[],
): Promise<string[]> {
  if (!process.stdin.isTTY) {
    logger.warning(
      "--interactive ignored: stdin is not a TTY. Installing all skills.",
    );
    return skills;
  }

  const choices = skills.map((skillPath) => ({
    name: getSkillName(skillPath),
    value: skillPath,
    checked: true,
  }));

  const selected = await checkbox({
    message: "Select skills to install (space to toggle, enter to confirm):",
    choices,
  });

  if (selected.length === 0) {
    logger.info("No skills selected. Aborting.");
    return [];
  }

  return selected;
}

export function ensureDirectory(dir: string, dryRun: boolean): void {
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

export function createSymlink(
  source: string,
  target: string,
  dryRun: boolean,
): boolean {
  const resolvedSource = realpathSync.native(resolve(source));
  const relativeSource = relative(dirname(target), resolvedSource);

  if (existsSync(target)) {
    try {
      const existing = readlinkSync(target);
      // Resolve existing (may be relative or absolute) and canonicalize
      // with realpathSync to handle platform symlinks (e.g. macOS /tmp → /private/tmp)
      const resolvedExisting = realpathSync.native(
        resolve(dirname(target), existing),
      );

      if (resolvedExisting === resolvedSource) {
        logger.debug(`Already linked: ${target}`);
        return false;
      }

      if (dryRun) {
        logger.warning(`Would replace symlink: ${target} -> ${relativeSource}`);
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
    logger.info(`Would symlink: ${target} -> ${relativeSource}`);
    return true;
  }

  try {
    symlinkSync(relativeSource, target, "dir");
    logger.success(`Linked: ${target.split("/").pop()}`);
    return true;
  } catch (error: unknown) {
    logger.error(`Failed to symlink ${source} -> ${target}: ${error}`);
    return false;
  }
}

interface AgentStats {
  installed: number;
  skipped: number;
  failed: number;
}

function installSkillsForAgent(
  agent: string,
  skills: string[],
  cwd: string,
  options: InstallOptions,
): AgentStats {
  const stats: AgentStats = { installed: 0, skipped: 0, failed: 0 };

  logger.header(`\nInstalling for ${agent}`);

  if (options.global && ALWAYS_GLOBAL_AGENTS.has(agent)) {
    logger.warning(
      `--global has no effect for ${agent}: it always installs to ~/.config/${agent}/skills`,
    );
  }

  const targetDir = AGENT_PATHS[agent](options.global, cwd);
  logger.info(`Target directory: ${targetDir}`);

  ensureDirectory(targetDir, options.dryRun);

  for (const skillPath of skills) {
    const skillName = getSkillName(skillPath);
    const source = join(cwd, skillPath);
    const target = join(targetDir, skillName);

    const result = createSymlink(source, target, options.dryRun);

    if (result) {
      stats.installed++;
    } else if (existsSync(target)) {
      stats.skipped++;
    } else {
      stats.failed++;
    }
  }

  return stats;
}

function displaySummary(
  agents: string[],
  stats: Record<string, AgentStats>,
  dryRun: boolean,
): void {
  logger.header("\nInstallation Summary");

  for (const agent of agents) {
    if (!stats[agent]) continue;

    const { installed, skipped, failed } = stats[agent];
    logger.info(`\n${agent}:`);
    if (installed > 0) logger.success(`  Installed: ${installed}`);
    if (skipped > 0) logger.debug(`  Skipped: ${skipped}`);
    if (failed > 0) logger.error(`  Failed: ${failed}`);
  }

  if (dryRun) {
    logger.warning("\nDry run completed. No changes made.");
  }
}

export async function installSkills(options: InstallOptions): Promise<void> {
  const cwd = process.cwd();

  // Validate agents upfront before doing any work
  const unknownAgents = options.agent.filter((a) => !AGENT_PATHS[a]);
  if (unknownAgents.length > 0) {
    throw new CLIError(
      `Unknown agent(s): ${unknownAgents.join(", ")}. Valid agents: ${Object.keys(AGENT_PATHS).join(", ")}`,
      1,
    );
  }

  if (options.dryRun) {
    logger.header("Dry Run: Install Skills");
  } else {
    logger.header("Installing Skills");
  }

  let skills = await findSkills(cwd);

  // Apply domain/subdomain filters
  const filtered = filterSkills(skills, options);
  if (filtered.length < skills.length) {
    logger.info(
      `Excluded ${skills.length - filtered.length} skill(s) via domain/subdomain filters`,
    );
  }
  skills = filtered;

  // Apply interactive selection (ignored if not TTY)
  if (options.interactive) {
    skills = await selectSkillsInteractively(skills);
    if (skills.length === 0) {
      return;
    }
  }

  logger.info(`Found ${skills.length} skills`);

  const agents = options.agent;
  logger.info(`Target agents: ${agents.join(", ")}`);
  logger.info(`Mode: ${options.global ? "global" : "local"}`);

  const stats: Record<string, AgentStats> = {};

  for (const agent of agents) {
    stats[agent] = installSkillsForAgent(agent, skills, cwd, options);
  }

  displaySummary(agents, stats, options.dryRun);
}
