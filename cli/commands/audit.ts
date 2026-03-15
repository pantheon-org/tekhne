import { Command } from "@cliffy/command";
import { auditAll, auditSkill, auditStatus, auditSummary } from "../lib/audit";
import { CLIError, logger } from "../lib/utils";

export const auditCommand = new Command()
  .description("Skill quality audit commands")
  .command(
    "skill",
    new Command()
      .description("Audit a single skill")
      .arguments("<path:string>")
      .action(async (_options, path: string) => {
        try {
          await auditSkill(path);
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
    "all",
    new Command()
      .description("Audit all skills in the repository")
      .option("-s, --skill <path:string>", "Audit a single skill")
      .option("-f, --force", "Force re-audit even if current audit exists")
      .action(async (options) => {
        try {
          await auditAll(options);
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
    "status",
    new Command()
      .description("Check audit status and compliance for all skills")
      .action(async () => {
        try {
          await auditStatus();
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
    "summary",
    new Command()
      .description("Generate comprehensive audit summary report")
      .action(async () => {
        try {
          await auditSummary();
        } catch (error) {
          if (error instanceof CLIError) {
            logger.error(error.message);
            process.exit(error.exitCode);
          }
          throw error;
        }
      }),
  );
