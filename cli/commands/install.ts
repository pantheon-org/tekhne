import { Command } from "commander";
import { installSkills } from "../lib/install/install-skills";

export const installCommand = new Command("install")
  .description("Install skills to local agent directories")
  .option(
    "-a, --agent <agents...>",
    "Specific agents to install for (opencode, cursor, gemini, claude, codex)",
    ["opencode"],
  )
  .option(
    "-g, --global",
    "Install to global agent directories (~/.config/)",
    false,
  )
  .option(
    "--dry-run",
    "Show what would be installed without making changes",
    false,
  )
  .action(async (options) => {
    await installSkills(options);
  });
