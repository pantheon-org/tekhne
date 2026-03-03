import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { AuditFailedError, FileNotFoundError } from "../utils/errors";
import { logger } from "../utils/logger";
import { exec } from "../utils/shell";

export async function auditSkill(skillPath: string): Promise<void> {
  const fullPath = resolve(skillPath);
  const skillFile = join(fullPath, "SKILL.md");

  if (!existsSync(skillFile)) {
    throw new FileNotFoundError(skillFile);
  }

  logger.info(`Auditing skill: ${skillPath}`);

  const evaluateScript = resolve(
    "skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh",
  );
  if (!existsSync(evaluateScript)) {
    throw new FileNotFoundError(evaluateScript);
  }

  const { exitCode } = await exec(
    `sh "${evaluateScript}" "${skillPath}" --json --store`,
  );

  if (exitCode !== 0) {
    throw new AuditFailedError(skillPath, 0);
  }

  const auditJsonPath = join(
    fullPath,
    ".context/audits",
    skillPath,
    "latest/audit.json",
  );
  if (existsSync(auditJsonPath)) {
    const auditData = await Bun.file(auditJsonPath).json();
    const score = auditData.score || 0;
    const grade = auditData.grade || "Unknown";

    logger.success(`Audit complete: ${score}/120 (${grade})`);

    if (score < 108) {
      logger.warning("Score below A-grade threshold (108/120)");
    }
  } else {
    logger.warning("Audit completed but could not read results");
  }
}
