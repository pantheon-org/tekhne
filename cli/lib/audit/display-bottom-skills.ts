import { logger } from "../utils/logger";
import type { SkillAudit } from "./collect-audit-data";

export const displayBottomSkills = (audits: SkillAudit[]): void => {
  logger.header("Bottom 10 Skills");
  audits
    .sort((a, b) => a.score - b.score)
    .slice(0, 10)
    .forEach((audit, i) => {
      logger.warning(
        `${i + 1}. ${audit.path}: ${audit.score}/120 (${audit.grade})`,
      );
    });
};
