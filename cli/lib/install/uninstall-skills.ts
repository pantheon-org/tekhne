import type { AgentType } from "../types";
import { agents } from "../types";
import { CLIError } from "../utils/errors";
import { logger } from "../utils/logger";
import { dedupAgentsByTarget } from "./dedup-agents-by-target";
import { displayUninstallSummary } from "./display-uninstall-summary";
import { uninstallSkillsForAgent } from "./uninstall-skills-for-agent";

export interface UninstallOptions {
  agent: string[];
  global: boolean;
  dryRun: boolean;
}

export const uninstallSkills = async (
  options: UninstallOptions,
): Promise<void> => {
  const cwd = process.cwd();

  const unknownAgents = options.agent.filter((a) => !agents[a as AgentType]);
  if (unknownAgents.length > 0) {
    throw new CLIError(
      `Unknown agent(s): ${unknownAgents.join(", ")}. Valid agents: ${Object.keys(agents).join(", ")}`,
      1,
    );
  }

  if (options.dryRun) {
    logger.header("Dry Run: Uninstall Skills");
  } else {
    logger.header("Uninstalling Skills");
  }

  const selectedAgents = dedupAgentsByTarget(
    options.agent,
    options.global,
    cwd,
  );
  logger.info(`Target agents: ${selectedAgents.join(", ")}`);
  logger.info(`Mode: ${options.global ? "global" : "local"}`);

  const stats: Record<
    string,
    { removed: number; skipped: number; failed: number }
  > = {};

  for (const agent of selectedAgents) {
    stats[agent] = uninstallSkillsForAgent(agent, cwd, options);
  }

  displayUninstallSummary(selectedAgents, stats, options.dryRun);
};
