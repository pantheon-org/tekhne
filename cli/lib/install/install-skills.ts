import { CLIError } from "../utils/errors";
import { logger } from "../utils/logger";
import { displaySummary } from "./display-summary";
import { filterSkills } from "./filter-skills";
import { findSkills } from "./find-skills";
import { AGENT_PATHS, installSkillsForAgent } from "./install-skills-for-agent";
import { selectSkillsInteractively } from "./select-skills-interactively";

export interface InstallOptions {
  agent: string[];
  global: boolean;
  dryRun: boolean;
  skillDomain?: string[];
  skillSubdomain?: string[];
  interactive?: boolean;
}

export const installSkills = async (options: InstallOptions): Promise<void> => {
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

  const stats: Record<
    string,
    { installed: number; skipped: number; failed: number }
  > = {};

  for (const agent of agents) {
    stats[agent] = installSkillsForAgent(agent, skills, cwd, options);
  }

  displaySummary(agents, stats, options.dryRun);
};
