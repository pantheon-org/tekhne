import { Command } from "commander";
import { tesslManage } from "../lib/tessl/manage";
import { tesslPublishCheck } from "../lib/tessl/publish-check";
import { CLIError } from "../lib/utils/errors";
import { logger } from "../lib/utils/logger";

export const tesslCommand = new Command("tessl").description(
  "Tessl registry management commands",
);

tesslCommand
  .command("manage [skill]")
  .description("Manage skill lifecycle (import, lint, review, publish)")
  .option("-w, --workspace <name>", "Tessl workspace name", "pantheon-ai")
  .action(async (skill: string | undefined, options) => {
    try {
      await tesslManage(skill, options);
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });

tesslCommand
  .command("publish-check <tiles...>")
  .description("Validate tiles before publishing")
  .action(async (tiles: string[]) => {
    try {
      await tesslPublishCheck(tiles);
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });
