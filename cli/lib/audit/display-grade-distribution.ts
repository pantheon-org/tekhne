import { logger } from "../utils/logger";
import type { GradeDistribution } from "./calculate-grade-distribution";

export const displayGradeDistribution = (
  gradeCounts: GradeDistribution,
  totalSkills: number,
): void => {
  logger.header("Grade Distribution");
  for (const [grade, count] of Object.entries(gradeCounts)) {
    const percentage = Math.round((count / totalSkills) * 100);
    logger.info(`${grade}: ${count} (${percentage}%)`);
  }
};
