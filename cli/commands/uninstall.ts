import { Command, EnumType } from "@cliffy/command";
import { uninstallSkills } from "../lib/install";
import type { AgentType } from "../lib/types";
import { agents } from "../lib/types";
import { CLIError, logger } from "../lib/utils";

const agentType = new EnumType(Object.keys(agents) as AgentType[]);

export const uninstallCommand = new Command()
  .description("Uninstall project-owned skills from agent directories")
  .type("agent", agentType)
  .option(
    "-a, --agent <agents...:agent>",
    "Specific agents to uninstall from. Note: only opencode supports local uninstalls; all other agents always uninstall from ~/.config/",
    { default: ["opencode"] as AgentType[] },
  )
  .option(
    "-g, --global",
    "Uninstall from global agent directories (~/.config/). Only applies to opencode; other agents are always global.",
    { default: false },
  )
  .option("--dry-run", "Show what would be removed without making changes", {
    default: false,
  })
  .action(async (options) => {
    try {
      await uninstallSkills({
        agent: options.agent,
        global: options.global,
        dryRun: options.dryRun,
      });
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });
