import { agents } from "./agents";
import type { AgentConfig, AgentType } from "./types";

export const getAgentConfig = (type: AgentType): AgentConfig => {
  return agents[type];
};
