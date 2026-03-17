import type { AgentType } from "../types";
import { agents } from "../types";
import { CLIError } from "../utils/errors";
import { logger } from "../utils/logger";
import { dedupAgentsByTarget } from "./dedup-agents-by-target";
import { displayUninstallSummary } from "./display-uninstall-summary";
import { findInstalledSkills } from "./find-installed-skills";
import { resolveTargetDir } from "./resolve-target-dir";
import { selectSkillsToUninstallInteractively } from "./select-skills-to-uninstall-interactively";
import { uninstallSkillsForAgent } from "./uninstall-skills-for-agent";

export interface UninstallOptions {
  agent: string[];
  global: boolean;
  dryRun: boolean;
  interactive?: boolean;
  filter?: string;
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

  let selectedNames: Set<string> | undefined;

  if (options.interactive) {
    const allInstalled = selectedAgents.flatMap((agent) => {
      const targetDir = resolveTargetDir(
        agent as AgentType,
        options.global,
        cwd,
      );
      return findInstalledSkills(targetDir, cwd);
    });

    const uniqueInstalled = [
      ...new Map(allInstalled.map((s) => [s.name, s])).values(),
    ];

    const chosen = await selectSkillsToUninstallInteractively(uniqueInstalled);
    if (chosen.length === 0) {
      return;
    }
    selectedNames = new Set(chosen.map((s) => s.name));
  }

  const stats: Record<
    string,
    { removed: number; skipped: number; failed: number }
  > = {};

  for (const agent of selectedAgents) {
    stats[agent] = uninstallSkillsForAgent(agent, cwd, options, selectedNames);
  }

  displayUninstallSummary(selectedAgents, stats, options.dryRun);
};
