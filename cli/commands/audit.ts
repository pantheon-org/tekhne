import { Command } from "commander";
import { auditAll } from "../lib/audit/audit-all";
import { auditSkill } from "../lib/audit/audit-skill";
import { auditStatus } from "../lib/audit/audit-status";
import { auditSummary } from "../lib/audit/audit-summary";
import { CLIError } from "../lib/utils/errors";
import { logger } from "../lib/utils/logger";

export const auditCommand = new Command("audit").description(
  "Skill quality audit commands",
);

auditCommand
  .command("skill <path>")
  .description("Audit a single skill")
  .action(async (path: string) => {
    try {
      await auditSkill(path);
    } catch (error) {
      if (error instanceof CLIError) {
        logger.error(error.message);
        process.exit(error.exitCode);
      }
      throw error;
    }
  });

auditCommand
  .command("all")
  .description("Audit all skills in the repository")
  .option("-s, --skill <path>", "Audit a single skill")
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
  });

auditCommand
  .command("status")
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
  });

auditCommand
  .command("summary")
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
  });
