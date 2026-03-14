import { Command } from "@cliffy/command";
import { tesslManage, tesslPublishCheck } from "../lib/tessl";
import { CLIError, logger } from "../lib/utils";

export const tesslCommand = new Command()
  .description("Tessl registry management commands")
  .command(
    "manage",
    new Command()
      .description("Manage skill lifecycle (import, lint, review, publish)")
      .arguments("[skill:string]")
      .option("-w, --workspace <name:string>", "Tessl workspace name", {
        default: "pantheon-ai",
      })
      .action(async (options, skill?: string) => {
        try {
          await tesslManage(skill, options);
        } catch (error) {
          if (error instanceof CLIError) {
            logger.error(error.message);
            process.exit(error.exitCode);
          }
          throw error;
        }
      }),
  )
  .command(
    "publish-check",
    new Command()
      .description("Validate tiles before publishing")
      .arguments("<tiles...:string>")
      .action(async (_options, ...tiles: string[]) => {
        try {
          await tesslPublishCheck(tiles);
        } catch (error) {
          if (error instanceof CLIError) {
            logger.error(error.message);
            process.exit(error.exitCode);
          }
          throw error;
        }
      }),
  );
