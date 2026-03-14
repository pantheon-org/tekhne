import { agents } from "./agents";
import type { AgentType } from "./types";

/**
 * Check if an agent uses the universal .agents/skills directory.
 */
export const isUniversalAgent = (type: AgentType): boolean => {
  return agents[type].skillsDir === ".agents/skills";
};
