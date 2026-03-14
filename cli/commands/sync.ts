import { Command } from "commander";
import { syncOpencode } from "../lib/sync/sync-opencode";
import { CLIError } from "../lib/utils/errors";
import { logger } from "../lib/utils/logger";

const syncOpencodeCommand = new Command("opencode")
  .description(
    "Sync OpenCode configuration: clean broken symlinks, sync MCP config, and link Tessl tiles",
  )
  .option(
    "--opencode-skills",
    "Use .opencode/skills instead of .agents/skills",
    false,
  )
  .option("--dry-run", "Show what would change without writing anything", false)
  .action(async (options) => {
    try {
      await syncOpencode({
        opencodeSkills: options.opencodeSkills as boolean,
        dryRun: options.dryRun as boolean,
      });
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });

export const syncCommand = new Command("sync")
  .description("Sync agent configurations and skill symlinks")
  .addCommand(syncOpencodeCommand);
