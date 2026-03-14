import { Command } from "@cliffy/command";
import { CLIError } from "../lib/utils/errors";
import { logger } from "../lib/utils/logger";
import { validateSkillFrontmatter } from "../lib/validate/validate-skill-frontmatter";

export const validateCommand = new Command()
  .description("Validation commands for skill files")
  .command(
    "frontmatter",
    new Command()
      .description(
        "Validate YAML frontmatter in SKILL.md files. " +
          "Checks for required fields (name, description) and valid YAML. " +
          "When no files are given, scans all skills/**/SKILL.md.",
      )
      .arguments("[...files:string]")
      .action(async (_options, ...files: string[]) => {
        try {
          await validateSkillFrontmatter(files);
        } catch (error) {
          if (error instanceof CLIError) {
            logger.error(error.message);
            process.exit(error.exitCode);
          }
          throw error;
        }
      }),
  );
