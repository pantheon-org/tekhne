import { existsSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

interface AuditInfo {
  date: string;
  path: string;
  grade: string;
}

export const getLatestAuditInfo = async (
  skillRelativePath: string,
): Promise<AuditInfo | null> => {
  const auditBase = join(".context/audits", skillRelativePath);

  if (!existsSync(auditBase)) {
    return null;
  }

  const dates = readdirSync(auditBase)
    .filter(
      (f) =>
        /^\d{4}-\d{2}-\d{2}$/.test(f) &&
        statSync(join(auditBase, f)).isDirectory(),
    )
    .sort()
    .reverse();

  for (const date of dates) {
    const auditJsonPath = join(auditBase, date, "audit.json");
    if (existsSync(auditJsonPath)) {
      try {
        const auditData = await Bun.file(auditJsonPath).json();
        const grade = auditData.grade || "?";
        return { date, path: auditJsonPath, grade };
      } catch {}
    }
  }

  return null;
};
