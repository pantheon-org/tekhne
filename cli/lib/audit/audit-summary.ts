import { existsSync } from "node:fs";
import { $ } from "bun";
import { logger } from "../utils/logger";

interface SkillAudit {
  path: string;
  score: number;
  grade: string;
  dimensions: Record<string, number>;
}

interface GradeDistribution {
  [grade: string]: number;
}

interface StatsSummary {
  avgScore: number;
  medianScore: number;
  minScore: number;
  maxScore: number;
}

const GRADE_RANGES = {
  "A+": [114, 120],
  A: [108, 113],
  "B+": [102, 107],
  B: [96, 101],
  "C+": [90, 95],
  C: [84, 89],
  D: [72, 83],
  F: [0, 71],
} as const;

async function collectAuditData(): Promise<SkillAudit[]> {
  const auditJsonFiles =
    await $`find .context/audits -name "audit.json" -path "*/latest/*"`.text();
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
}

function calculateStatistics(audits: SkillAudit[]): StatsSummary {
  const scores = audits.map((a) => a.score).sort((a, b) => a - b);
  const avgScore = scores.reduce((a, b) => a + b, 0) / scores.length;
  const medianScore = scores[Math.floor(scores.length / 2)];
  const minScore = Math.min(...scores);
  const maxScore = Math.max(...scores);

  return { avgScore, medianScore, minScore, maxScore };
}

function calculateGradeDistribution(audits: SkillAudit[]): GradeDistribution {
  const gradeCounts = Object.fromEntries(
    Object.keys(GRADE_RANGES).map((g) => [g, 0]),
  );

  for (const audit of audits) {
    const score = audit.score;
    for (const [grade, [min, max]] of Object.entries(GRADE_RANGES)) {
      if (score >= min && score <= max) {
        gradeCounts[grade]++;
        break;
      }
    }
  }

  return gradeCounts;
}

function calculateDimensionAverages(
  audits: SkillAudit[],
): Record<string, number> {
  const dimensionStats: Record<string, number[]> = {};

  for (const audit of audits) {
    for (const [dim, score] of Object.entries(audit.dimensions)) {
      if (!dimensionStats[dim]) dimensionStats[dim] = [];
      dimensionStats[dim].push(score);
    }
  }

  return Object.fromEntries(
    Object.entries(dimensionStats).map(([dim, scores]) => [
      dim,
      Math.round(scores.reduce((a, b) => a + b, 0) / scores.length),
    ]),
  );
}

function displayStatistics(stats: StatsSummary, totalSkills: number): void {
  logger.header("Overall Statistics");
  logger.info(`Total skills audited: ${totalSkills}`);
  logger.info(
    `Average score: ${stats.avgScore.toFixed(1)}/120 (${((stats.avgScore / 120) * 100).toFixed(0)}%)`,
  );
  logger.info(`Median score: ${stats.medianScore}/120`);
  logger.info(`Range: ${stats.minScore} - ${stats.maxScore}`);
}

function displayGradeDistribution(
  gradeCounts: GradeDistribution,
  totalSkills: number,
): void {
  logger.header("Grade Distribution");
  for (const [grade, count] of Object.entries(gradeCounts)) {
    const percentage = Math.round((count / totalSkills) * 100);
    logger.info(`${grade}: ${count} (${percentage}%)`);
  }
}

function displayDimensionalAnalysis(
  dimensionAvgs: Record<string, number>,
): void {
  logger.header("Dimensional Analysis");
  const sortedDims = Object.entries(dimensionAvgs).sort(
    ([, a], [, b]) => a - b,
  );

  for (const [dim, avg] of sortedDims) {
    const percentage = Math.round((avg / 15) * 100);
    const marker = percentage < 80 ? "⚠" : "✓";
    logger.info(`${marker} ${dim}: ${avg}/15 (${percentage}%)`);
  }
}

function displayTopSkills(audits: SkillAudit[]): void {
  logger.header("Top 10 Skills");
  audits
    .sort((a, b) => b.score - a.score)
    .slice(0, 10)
    .forEach((audit, i) => {
      logger.success(
        `${i + 1}. ${audit.path}: ${audit.score}/120 (${audit.grade})`,
      );
    });
}

function displayBottomSkills(audits: SkillAudit[]): void {
  logger.header("Bottom 10 Skills");
  audits
    .sort((a, b) => a.score - b.score)
    .slice(0, 10)
    .forEach((audit, i) => {
      logger.warning(
        `${i + 1}. ${audit.path}: ${audit.score}/120 (${audit.grade})`,
      );
    });
}

export async function auditSummary(): Promise<void> {
  logger.header("Generating Audit Summary");

  const audits = await collectAuditData();

  if (audits.length === 0) {
    logger.error("No audit data found");
    process.exit(1);
  }

  const stats = calculateStatistics(audits);
  const gradeCounts = calculateGradeDistribution(audits);
  const dimensionAvgs = calculateDimensionAverages(audits);

  displayStatistics(stats, audits.length);
  displayGradeDistribution(gradeCounts, audits.length);
  displayDimensionalAnalysis(dimensionAvgs);
  displayTopSkills(audits);
  displayBottomSkills(audits);

  const reportPath = ".context/audits/summary.md";
  logger.info(`\nFull report would be written to: ${reportPath}`);
}
