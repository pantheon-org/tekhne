import { Command } from "@cliffy/command";
import { updateReadme } from "../lib/readme";
import { CLIError, logger } from "../lib/utils";

export const readmeCommand = new Command()
  .description("README.md maintenance commands")
  .command(
    "update",
    new Command()
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
      }),
  );
