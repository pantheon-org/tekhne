import { agents } from "./agents";
import type { AgentType } from "./types";

export const detectInstalledAgents = async (): Promise<AgentType[]> => {
  const results = await Promise.all(
    Object.entries(agents).map(async ([type, config]) => ({
      type: type as AgentType,
      installed: await config.detectInstalled(),
    })),
  );
  return results.filter((r) => r.installed).map((r) => r.type);
};
