import { logger } from "../utils/logger";

export const displayDimensionalAnalysis = (
  dimensionAvgs: Record<string, number>,
): void => {
  logger.header("Dimensional Analysis");
  const sortedDims = Object.entries(dimensionAvgs).sort(
    ([, a], [, b]) => a - b,
  );

  for (const [dim, avg] of sortedDims) {
    const percentage = Math.round((avg / 15) * 100);
    const marker = percentage < 80 ? "⚠" : "✓";
    logger.info(`${marker} ${dim}: ${avg}/15 (${percentage}%)`);
  }
};
