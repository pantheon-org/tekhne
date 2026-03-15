import { Command } from "@cliffy/command";
import { syncOpencode } from "../lib/sync";
import { CLIError, logger } from "../lib/utils";

export const syncCommand = new Command()
  .description("Sync agent configurations and skill symlinks")
  .command(
    "opencode",
    new Command()
      .description(
        "Sync OpenCode configuration: clean broken symlinks, sync MCP config, and link Tessl tiles",
      )
      .option(
        "--opencode-skills",
        "Use .opencode/skills instead of .agents/skills",
        { default: false },
      )
      .option("--dry-run", "Show what would change without writing anything", {
        default: false,
      })
      .action(async (options) => {
        try {
          await syncOpencode({
            opencodeSkills: options.opencodeSkills,
            dryRun: options.dryRun,
          });
        } catch (error) {
          if (error instanceof CLIError) {
            logger.error(error.message);
            process.exit(error.exitCode);
          }
          throw error;
        }
      }),
  );
