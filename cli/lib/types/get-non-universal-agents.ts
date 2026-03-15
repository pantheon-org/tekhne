import { agents } from "./agents";
import type { AgentConfig, AgentType } from "./types";

/**
 * Returns agents that use agent-specific skill directories (not universal).
 * These agents need symlinks from the canonical .agents/skills location.
 */
export const getNonUniversalAgents = (): AgentType[] => {
  return (Object.entries(agents) as [AgentType, AgentConfig][])
    .filter(([_, config]) => config.skillsDir !== ".agents/skills")
    .map(([type]) => type);
};
