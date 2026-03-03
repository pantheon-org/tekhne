import { existsSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

interface AuditInfo {
  date: string;
  path: string;
  grade: string;
}

export async function getLatestAuditInfo(
  skillRelativePath: string,
): Promise<AuditInfo | null> {
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
}

export function getGradeColor(grade: string): string {
  const colorMap: Record<string, string> = {
    "A+": "brightgreen",
    A: "green",
    "B+": "yellowgreen",
    B: "yellow",
    "C+": "orange",
    C: "red",
    D: "red",
    F: "purple",
  };
  return colorMap[grade] || "lightgrey";
}

export function getBadgeMarkdown(grade: string): string {
  const color = getGradeColor(grade);
  return `![${grade}](https://img.shields.io/badge/Rating-${grade}-${color})`;
}

export function getAuditLink(date: string, path: string): string {
  return `[${date}](${path})`;
}
