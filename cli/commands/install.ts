import { Command, EnumType } from "@cliffy/command";
import { installSkills } from "../lib/install";
import type { AgentType } from "../lib/types";
import { agents } from "../lib/types";
import { CLIError, logger } from "../lib/utils";

const agentType = new EnumType(Object.keys(agents) as AgentType[]);

export const installCommand = new Command()
  .description("Install skills to local agent directories")
  .type("agent", agentType)
  .option(
    "-a, --agent <agents...:agent>",
    "Specific agents to install for. Note: only opencode supports local installs; all other agents always install to ~/.config/",
    { default: ["opencode"] as AgentType[] },
  )
  .option(
    "-g, --global",
    "Install to global agent directories (~/.config/). Only applies to opencode; other agents are always global.",
    { default: false },
  )
  .option("--dry-run", "Show what would be installed without making changes", {
    default: false,
  })
  .option(
    "--skill-domain <domains...:string>",
    "Only install skills from these top-level domains (e.g. ci-cd infrastructure)",
  )
  .option(
    "--skill-subdomain <subdomains...:string>",
    "Only install skills from these subdomains (e.g. github-actions terraform)",
  )
  .option(
    "-i, --interactive",
    "Interactively select which skills to install (ignored when stdin is not a TTY)",
    { default: false },
  )
  .action(async (options) => {
    try {
      await installSkills({
        agent: options.agent,
        global: options.global,
        dryRun: options.dryRun,
        skillDomain: options.skillDomain,
        skillSubdomain: options.skillSubdomain,
        interactive: options.interactive,
      });
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });
