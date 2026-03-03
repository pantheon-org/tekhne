import { Command } from "commander";
import { updateReadme } from "../lib/readme/update-readme";
import { CLIError } from "../lib/utils/errors";
import { logger } from "../lib/utils/logger";

export const readmeCommand = new Command("readme").description(
  "README.md maintenance commands",
);

readmeCommand
  .command("update")
  .description("Update skill tables in README.md")
  .option("--dry-run", "Show changes without applying")
  .action(async (options) => {
    try {
      await updateReadme(options);
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });
