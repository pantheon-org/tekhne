import { join } from "node:path";
import type { AgentType } from "../types";
import { agents } from "../types";

export const resolveTargetDir = (
  agent: AgentType,
  isGlobal: boolean,
  cwd: string,
): string => {
  const config = agents[agent];
  return isGlobal
    ? (config.globalSkillsDir ?? join(cwd, config.skillsDir))
    : join(cwd, config.skillsDir);
};
