import { Command } from "commander";
import { tesslManage } from "../lib/tessl/manage";
import { tesslPublishCheck } from "../lib/tessl/publish-check";

export const tesslCommand = new Command("tessl").description(
  "Tessl registry management commands",
);

tesslCommand
  .command("manage [skill]")
  .description("Manage skill lifecycle (import, lint, review, publish)")
  .option("-w, --workspace <name>", "Tessl workspace name", "pantheon-ai")
  .action(async (skill: string | undefined, options) => {
    await tesslManage(skill, options);
  });

tesslCommand
  .command("publish-check <tiles...>")
  .description("Validate tiles before publishing")
  .action(async (tiles: string[]) => {
    await tesslPublishCheck(tiles);
  });
