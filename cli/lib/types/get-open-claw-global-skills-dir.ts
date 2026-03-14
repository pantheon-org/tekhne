import { existsSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

export const getOpenClawGlobalSkillsDir = (
  homeDir = homedir(),
  pathExists: (path: string) => boolean = existsSync,
): string => {
  if (pathExists(join(homeDir, ".openclaw"))) {
    return join(homeDir, ".openclaw/skills");
  }
  if (pathExists(join(homeDir, ".clawdbot"))) {
    return join(homeDir, ".clawdbot/skills");
  }
  if (pathExists(join(homeDir, ".moltbot"))) {
    return join(homeDir, ".moltbot/skills");
  }
  return join(homeDir, ".openclaw/skills");
};
