import { logger } from "../utils/logger";
import { calculateDimensionAverages } from "./calculate-dimension-averages";
import { calculateGradeDistribution } from "./calculate-grade-distribution";
import { calculateStatistics } from "./calculate-statistics";
import { collectAuditData } from "./collect-audit-data";
import { displayBottomSkills } from "./display-bottom-skills";
import { displayDimensionalAnalysis } from "./display-dimensional-analysis";
import { displayGradeDistribution } from "./display-grade-distribution";
import { displayStatistics } from "./display-statistics";
import { displayTopSkills } from "./display-top-skills";

export const auditSummary = async (): Promise<void> => {
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
};
