import { logger } from "../utils/logger";
import type { SkillAudit } from "./collect-audit-data";

export const displayTopSkills = (audits: SkillAudit[]): void => {
  logger.header("Top 10 Skills");
  audits
    .sort((a, b) => b.score - a.score)
    .slice(0, 10)
    .forEach((audit, i) => {
      logger.success(
        `${i + 1}. ${audit.path}: ${audit.score}/120 (${audit.grade})`,
      );
    });
};
