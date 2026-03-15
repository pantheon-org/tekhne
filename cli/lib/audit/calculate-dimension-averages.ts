import type { SkillAudit } from "./collect-audit-data";

export const calculateDimensionAverages = (
  audits: SkillAudit[],
): Record<string, number> => {
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
};
