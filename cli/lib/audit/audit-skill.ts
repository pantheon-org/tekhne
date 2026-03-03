import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { logger } from "../utils/logger";
import { exec } from "../utils/shell";

export async function auditSkill(skillPath: string): Promise<void> {
  const fullPath = resolve(skillPath);
  const skillFile = join(fullPath, "SKILL.md");

  if (!existsSync(skillFile)) {
    logger.error(`No SKILL.md found at ${fullPath}`);
    process.exit(1);
  }

  logger.info(`Auditing skill: ${skillPath}`);

  const evaluateScript = resolve(
    "skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh",
  );
  if (!existsSync(evaluateScript)) {
    logger.error(
      "Audit script not found: skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh",
    );
    process.exit(1);
  }

  const { exitCode, stderr } = await exec(
    `sh "${evaluateScript}" "${skillPath}" --json --store`,
  );

  if (exitCode !== 0) {
    logger.error("Audit failed");
    console.error(stderr);
    process.exit(1);
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
