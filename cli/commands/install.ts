import { Command } from "commander";
import { installSkills } from "../lib/install/install-skills";
import { CLIError } from "../lib/utils/errors";
import { logger } from "../lib/utils/logger";

export const installCommand = new Command("install")
  .description("Install skills to local agent directories")
  .option(
    "-a, --agent <agents...>",
    "Specific agents to install for (opencode, cursor, gemini, claude, codex). Note: only opencode supports local installs; all other agents always install to ~/.config/",
    ["opencode"],
  )
  .option(
    "-g, --global",
    "Install to global agent directories (~/.config/). Only applies to opencode; other agents are always global.",
    false,
  )
  .option(
    "--dry-run",
    "Show what would be installed without making changes",
    false,
  )
  .option(
    "--skill-domain <domains...>",
    "Only install skills from these top-level domains (e.g. ci-cd infrastructure)",
  )
  .option(
    "--skill-subdomain <subdomains...>",
    "Only install skills from these subdomains (e.g. github-actions terraform)",
  )
  .option(
    "-i, --interactive",
    "Interactively select which skills to install (ignored when stdin is not a TTY)",
    false,
  )
  .action(async (options) => {
    try {
      await installSkills(options);
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });
