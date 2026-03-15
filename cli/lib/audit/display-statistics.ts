import { logger } from "../utils/logger";
import type { StatsSummary } from "./calculate-statistics";

export const displayStatistics = (
  stats: StatsSummary,
  totalSkills: number,
): void => {
  logger.header("Overall Statistics");
  logger.info(`Total skills audited: ${totalSkills}`);
  logger.info(
    `Average score: ${stats.avgScore.toFixed(1)}/120 (${((stats.avgScore / 120) * 100).toFixed(0)}%)`,
  );
  logger.info(`Median score: ${stats.medianScore}/120`);
  logger.info(`Range: ${stats.minScore} - ${stats.maxScore}`);
};
