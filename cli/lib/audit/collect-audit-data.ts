import { existsSync } from "node:fs";
import { $ } from "bun";
import { logger } from "../utils/logger";

export interface SkillAudit {
  path: string;
  score: number;
  grade: string;
  dimensions: Record<string, number>;
}

export const collectAuditData = async (): Promise<SkillAudit[]> => {
  if (!existsSync(".context/audits")) {
    return [];
  }

  const auditJsonFiles =
    await $`find -L .context/audits -name "audit.json" -path "*/latest/*"`.text();
  const files = auditJsonFiles.trim().split("\n").filter(Boolean);

  if (files.length === 0) {
    return [];
  }

  const audits: SkillAudit[] = [];

  for (const file of files) {
    if (!existsSync(file)) continue;

    try {
      const data = await Bun.file(file).json();
      const pathMatch = file.match(/\.context\/audits\/(.+?)\/latest/);
      if (!pathMatch) continue;

      audits.push({
        path: pathMatch[1],
        score: data.score || 0,
        grade: data.grade || "F",
        dimensions: data.dimensions || {},
      });
    } catch (_error) {
      logger.warning(`Failed to parse ${file}`);
    }
  }

  return audits;
};
