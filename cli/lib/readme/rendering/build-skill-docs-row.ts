import { getLatestAuditInfo } from "../discovery";
import { getDocsBadgeMarkdown } from "./get-docs-badge-markdown";

interface SkillRowInput {
  skillLink: string;
  auditRelPath: string;
  evalsCell: string;
}

export const buildSkillDocsRow = async ({
  skillLink,
  auditRelPath,
  evalsCell,
}: SkillRowInput): Promise<string> => {
  const auditInfo = await getLatestAuditInfo(auditRelPath);

  if (auditInfo) {
    const badge = getDocsBadgeMarkdown(auditInfo.grade);
    return `| ${skillLink} | ${badge} | ${auditInfo.date} | ${evalsCell} |\n`;
  }

  return `| ${skillLink} | ${getDocsBadgeMarkdown("?")} | - | ${evalsCell} |\n`;
};
