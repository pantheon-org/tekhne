import { Command } from "commander";
import { updateReadme } from "../lib/readme/update-readme";

export const readmeCommand = new Command("readme").description(
  "README.md maintenance commands",
);

readmeCommand
  .command("update")
  .description("Update skill tables in README.md")
  .option("--dry-run", "Show changes without applying")
  .action(async (options) => {
    await updateReadme(options);
  });
