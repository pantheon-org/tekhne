import type { SkillAudit } from "./collect-audit-data";

export interface GradeDistribution {
  [grade: string]: number;
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

export const calculateGradeDistribution = (
  audits: SkillAudit[],
): GradeDistribution => {
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
};
