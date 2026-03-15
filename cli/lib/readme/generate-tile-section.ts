import { formatSummary } from "./format-summary";
import { getAuditLink } from "./get-audit-link";
import { getBadgeMarkdown } from "./get-badge-markdown";
import { getEvalCount } from "./get-eval-count";
import { getLatestAuditInfo } from "./get-latest-audit-info";
import type { TileEntry } from "./tile-types";

export const generateTileSection = async (tile: TileEntry): Promise<string> => {
  const tileLink = `[${tile.shortName}](${tile.tileDir})`;
  const description = formatSummary(tile.summary);

  let output = `\n### ${tileLink}\n\n`;
  output += `${description}\n\n`;
  output += "| Skill | Rating | Audit | Evals |\n";
  output += "| --- | --- | --- | --- |\n";

  for (const skill of tile.skills) {
    const skillLink = `[${skill.name}](${skill.skillDir}/SKILL.md)`;
    const auditInfo = await getLatestAuditInfo(skill.auditRelPath);
    const evalCount = getEvalCount(skill.skillDir);
    const evalsCell = evalCount > 0 ? String(evalCount) : "-";

    if (auditInfo) {
      const badge = getBadgeMarkdown(auditInfo.grade);
      const auditLink = getAuditLink(auditInfo.date, auditInfo.path);
      output += `| ${skillLink} | ${badge} | ${auditLink} | ${evalsCell} |\n`;
    } else {
      output += `| ${skillLink} | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | ${evalsCell} |\n`;
    }
  }

  return output;
};
