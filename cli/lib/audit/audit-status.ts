import { existsSync, readlinkSync, statSync } from "node:fs";
import { join } from "node:path";
import { $ } from "bun";
import { logger } from "../utils/logger";

export async function auditStatus(): Promise<void> {
  logger.header("Audit Status Check");

  const skillsOutput = await $`find skills -name "SKILL.md" -type f`.text();
  const skillFiles = skillsOutput.trim().split("\n").filter(Boolean);

  let compliant = 0;
  let outdated = 0;
  let missing = 0;

  for (const skillFile of skillFiles) {
    const skillPath = skillFile.replace("/SKILL.md", "");
    const auditDir = join(skillPath, ".context/audits", skillPath);
    const latestLink = join(auditDir, "latest");
    const auditJson = join(latestLink, "audit.json");

    if (!existsSync(latestLink)) {
      logger.error(`✗ ${skillPath} - No audit found`);
      missing++;
      continue;
    }

    try {
      const _target = readlinkSync(latestLink);
      if (!existsSync(auditJson)) {
        logger.error(`✗ ${skillPath} - Broken symlink`);
        missing++;
        continue;
      }
    } catch {
      logger.error(`✗ ${skillPath} - Invalid symlink`);
      missing++;
      continue;
    }

    const stats = statSync(auditJson);
    const age = (Date.now() - stats.mtimeMs) / (1000 * 60 * 60 * 24);

    if (age > 30) {
      const auditData = await Bun.file(auditJson).json();
      const score = auditData.score || 0;
      logger.warning(
        `⚠ ${skillPath} - Outdated (${Math.floor(age)} days old, score: ${score}/120)`,
      );
      outdated++;
    } else {
      const auditData = await Bun.file(auditJson).json();
      const score = auditData.score || 0;
      logger.success(`✓ ${skillPath} - Compliant (score: ${score}/120)`);
      compliant++;
    }
  }

  logger.header("Status Summary");
  logger.success(`Compliant: ${compliant}`);
  if (outdated > 0) logger.warning(`Outdated: ${outdated}`);
  if (missing > 0) logger.error(`Missing: ${missing}`);

  const total = compliant + outdated + missing;
  const percentage = Math.round((compliant / total) * 100);
  logger.info(`Compliance rate: ${percentage}%`);
}
