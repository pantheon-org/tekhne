import { existsSync } from "node:fs";
import { join } from "node:path";
import { $ } from "bun";
import { logger } from "../utils/logger";

interface AuditAllOptions {
  skill?: string;
  force?: boolean;
}

export const auditAll = async (options: AuditAllOptions): Promise<void> => {
  if (options.skill) {
    logger.info(`Auditing single skill: ${options.skill}`);
    const { auditSkill } = await import("./audit-skill");
    await auditSkill(options.skill);
    return;
  }

  logger.header("Auditing all skills");

  const skillsOutput = await $`find skills -name "SKILL.md" -type f`.text();
  const skillFiles = skillsOutput.trim().split("\n").filter(Boolean);

  logger.info(`Found ${skillFiles.length} skills`);

  const today = new Date().toISOString().split("T")[0];
  let audited = 0;
  let skipped = 0;
  let failed = 0;

  for (const skillFile of skillFiles) {
    const skillPath = skillFile.replace("/SKILL.md", "");
    const auditDir = join(skillPath, ".context/audits", skillPath);
    const latestLink = join(auditDir, "latest");
    const auditJson = join(latestLink, "audit.json");

    let shouldAudit = options.force || !existsSync(auditJson);

    if (!shouldAudit && existsSync(auditJson)) {
      const auditData = await Bun.file(auditJson).json();
      const auditDate = auditData.timestamp?.split("T")[0];
      shouldAudit = auditDate !== today;
    }

    if (!shouldAudit) {
      logger.debug(`Skipping ${skillPath} (already audited today)`);
      skipped++;
      continue;
    }

    logger.info(`Auditing ${skillPath}...`);
    const { auditSkill } = await import("./audit-skill");
    try {
      await auditSkill(skillPath);
      audited++;
    } catch (_error) {
      logger.error(`Failed to audit ${skillPath}`);
      failed++;
    }
  }

  logger.header("Audit Summary");
  logger.success(`Audited: ${audited}`);
  logger.info(`Skipped: ${skipped}`);
  if (failed > 0) {
    logger.error(`Failed: ${failed}`);
  }
};
