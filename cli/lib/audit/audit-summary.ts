import { existsSync } from "node:fs";
import { $ } from "bun";
import { logger } from "../utils/logger";

interface SkillAudit {
  path: string;
  score: number;
  grade: string;
  dimensions: Record<string, number>;
}

export async function auditSummary(): Promise<void> {
  logger.header("Generating Audit Summary");

  const auditJsonFiles =
    await $`find .context/audits -name "audit.json" -path "*/latest/*"`.text();
  const files = auditJsonFiles.trim().split("\n").filter(Boolean);

  if (files.length === 0) {
    logger.error("No audit data found");
    process.exit(1);
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

  const scores = audits.map((a) => a.score).sort((a, b) => a - b);
  const avgScore = scores.reduce((a, b) => a + b, 0) / scores.length;
  const medianScore = scores[Math.floor(scores.length / 2)];
  const minScore = Math.min(...scores);
  const maxScore = Math.max(...scores);

  const gradeRanges = {
    "A+": [114, 120],
    A: [108, 113],
    "B+": [102, 107],
    B: [96, 101],
    "C+": [90, 95],
    C: [84, 89],
    D: [72, 83],
    F: [0, 71],
  };

  const gradeCounts = Object.fromEntries(
    Object.keys(gradeRanges).map((g) => [g, 0]),
  );

  for (const audit of audits) {
    const score = audit.score;
    for (const [grade, [min, max]] of Object.entries(gradeRanges)) {
      if (score >= min && score <= max) {
        gradeCounts[grade]++;
        break;
      }
    }
  }

  const dimensionStats: Record<string, number[]> = {};
  for (const audit of audits) {
    for (const [dim, score] of Object.entries(audit.dimensions)) {
      if (!dimensionStats[dim]) dimensionStats[dim] = [];
      dimensionStats[dim].push(score);
    }
  }

  const dimensionAvgs = Object.fromEntries(
    Object.entries(dimensionStats).map(([dim, scores]) => [
      dim,
      Math.round(scores.reduce((a, b) => a + b, 0) / scores.length),
    ]),
  );

  logger.header("Overall Statistics");
  logger.info(`Total skills audited: ${audits.length}`);
  logger.info(
    `Average score: ${avgScore.toFixed(1)}/120 (${((avgScore / 120) * 100).toFixed(0)}%)`,
  );
  logger.info(`Median score: ${medianScore}/120`);
  logger.info(`Range: ${minScore} - ${maxScore}`);

  logger.header("Grade Distribution");
  for (const [grade, count] of Object.entries(gradeCounts)) {
    const percentage = Math.round((count / audits.length) * 100);
    logger.info(`${grade}: ${count} (${percentage}%)`);
  }

  logger.header("Dimensional Analysis");
  const sortedDims = Object.entries(dimensionAvgs).sort(
    ([, a], [, b]) => a - b,
  );

  for (const [dim, avg] of sortedDims) {
    const percentage = Math.round((avg / 15) * 100);
    const marker = percentage < 80 ? "⚠" : "✓";
    logger.info(`${marker} ${dim}: ${avg}/15 (${percentage}%)`);
  }

  logger.header("Top 10 Skills");
  audits
    .sort((a, b) => b.score - a.score)
    .slice(0, 10)
    .forEach((audit, i) => {
      logger.success(
        `${i + 1}. ${audit.path}: ${audit.score}/120 (${audit.grade})`,
      );
    });

  logger.header("Bottom 10 Skills");
  audits
    .sort((a, b) => a.score - b.score)
    .slice(0, 10)
    .forEach((audit, i) => {
      logger.warning(
        `${i + 1}. ${audit.path}: ${audit.score}/120 (${audit.grade})`,
      );
    });

  const reportPath = ".context/audits/summary.md";
  logger.info(`\nFull report would be written to: ${reportPath}`);
}
