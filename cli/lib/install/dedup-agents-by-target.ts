import type { AgentType } from "../types";
import { resolveTargetDir } from "./resolve-target-dir";

export const dedupAgentsByTarget = (
  agents: string[],
  isGlobal: boolean,
  cwd: string,
): string[] => {
  const seen = new Set<string>();
  return agents.filter((agent) => {
    const targetDir = resolveTargetDir(agent as AgentType, isGlobal, cwd);
    if (seen.has(targetDir)) {
      return false;
    }
    seen.add(targetDir);
    return true;
  });
};
