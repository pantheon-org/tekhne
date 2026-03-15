import {
  getEvalCount,
  getLatestAuditInfo,
  parseSkillDescription,
} from "../discovery";
import { getSkillDisplayName } from "../parsing";
import type { SkillEntry } from "../types";
import { getAuditLink } from "./get-audit-link";
import { getBadgeMarkdown } from "./get-badge-markdown";

export const generateUntiledSkillSection = async (
  skill: SkillEntry,
): Promise<string> => {
  const displayName = getSkillDisplayName(skill.relativePath);
  const description = parseSkillDescription(`skills/${skill.relativePath}`);
  const auditInfo = await getLatestAuditInfo(skill.relativePath);
  const skillDir = `skills/${skill.relativePath}`;
  const evalCount = getEvalCount(skillDir);
  const evalsCell = evalCount > 0 ? String(evalCount) : "-";

  let output = `\n### ${displayName} _(no tile)_\n\n${description}\n\n`;
  output += "| Skill | Rating | Audit | Evals |\n";
  output += "| --- | --- | --- | --- |\n";

  const skillLink = `[${displayName}](${skillDir}/SKILL.md)`;
  if (auditInfo) {
    const badge = getBadgeMarkdown(auditInfo.grade);
    const auditLink = getAuditLink(auditInfo.date, auditInfo.path);
    output += `| ${skillLink} | ${badge} | ${auditLink} | ${evalsCell} |\n`;
  } else {
    output += `| ${skillLink} | ${getBadgeMarkdown("?")} | - | ${evalsCell} |\n`;
  }

  return output;
};
