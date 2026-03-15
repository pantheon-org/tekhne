import { agents } from "./agents";
import type { AgentConfig, AgentType } from "./types";

/**
 * Returns agents that use the universal .agents/skills directory.
 * These agents share a common skill location and don't need symlinks.
 * Agents with showInUniversalList: false are excluded.
 */
export const getUniversalAgents = (): AgentType[] => {
  return (Object.entries(agents) as [AgentType, AgentConfig][])
    .filter(
      ([_, config]) =>
        config.skillsDir === ".agents/skills" &&
        config.showInUniversalList !== false,
    )
    .map(([type]) => type);
};
