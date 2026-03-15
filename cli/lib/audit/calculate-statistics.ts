import type { SkillAudit } from "./collect-audit-data";

export interface StatsSummary {
  avgScore: number;
  medianScore: number;
  minScore: number;
  maxScore: number;
}

export const calculateStatistics = (audits: SkillAudit[]): StatsSummary => {
  const scores = audits.map((a) => a.score).sort((a, b) => a - b);
  const avgScore = scores.reduce((a, b) => a + b, 0) / scores.length;
  const medianScore = scores[Math.floor(scores.length / 2)];
  const minScore = Math.min(...scores);
  const maxScore = Math.max(...scores);

  return { avgScore, medianScore, minScore, maxScore };
};
