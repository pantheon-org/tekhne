import { Command } from "commander";
import { auditAll } from "../lib/audit/audit-all";
import { auditSkill } from "../lib/audit/audit-skill";
import { auditStatus } from "../lib/audit/audit-status";
import { auditSummary } from "../lib/audit/audit-summary";

export const auditCommand = new Command("audit").description(
  "Skill quality audit commands",
);

auditCommand
  .command("skill <path>")
  .description("Audit a single skill")
  .action(async (path: string) => {
    await auditSkill(path);
  });

auditCommand
  .command("all")
  .description("Audit all skills in the repository")
  .option("-s, --skill <path>", "Audit a single skill")
  .option("-f, --force", "Force re-audit even if current audit exists")
  .action(async (options) => {
    await auditAll(options);
  });

auditCommand
  .command("status")
  .description("Check audit status and compliance for all skills")
  .action(async () => {
    await auditStatus();
  });

auditCommand
  .command("summary")
  .description("Generate comprehensive audit summary report")
  .action(async () => {
    await auditSummary();
  });
