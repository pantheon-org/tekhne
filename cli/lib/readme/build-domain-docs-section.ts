import { buildCountLabel } from "./build-count-label";
import { formatSummary } from "./format-summary";
import { getBadgeMarkdown } from "./get-badge-markdown";
import { getLatestAuditInfo } from "./get-latest-audit-info";
import { getSkillDisplayName } from "./get-skill-display-name";
import { parseSkillDescription } from "./parse-skill-description";
import type { SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";

export const buildDomainDocsSection = async (
  domainInfo: { key: string; title: string; description: string },
  domainTiles: TileEntry[],
  domainUntiledSkills: SkillEntry[],
  isFirst: boolean,
): Promise<string> => {
  const countLabel = buildCountLabel(
    domainTiles.length,
    domainUntiledSkills.length,
  );

  let section = "";
  if (!isFirst) section += "\n---\n";

  section += `\n## ${domainInfo.title} (${countLabel})\n\n`;
  section += `${domainInfo.description}\n`;

  for (const tile of domainTiles) {
    section += `\n### ${tile.shortName}\n\n`;
    section += `${formatSummary(tile.summary)}\n\n`;
    section += "| Skill | Rating |\n";
    section += "| --- | --- |\n";

    for (const skill of tile.skills) {
      const relPath = skill.skillDir.replace(/^skills\//, "");
      const docsUrl = `/tekhne/skills/${relPath}/skill/`;
      const skillLink = `[${skill.name}](${docsUrl})`;
      const auditInfo = await getLatestAuditInfo(skill.auditRelPath);
      const badge = auditInfo
        ? getBadgeMarkdown(auditInfo.grade)
        : `![?](https://img.shields.io/badge/Rating-?-lightgrey)`;
      section += `| ${skillLink} | ${badge} |\n`;
    }
  }

  for (const skill of domainUntiledSkills) {
    const displayName = getSkillDisplayName(skill.relativePath);
    const description = parseSkillDescription(`skills/${skill.relativePath}`);
    const relPath = skill.relativePath;
    const docsUrl = `/tekhne/skills/${relPath}/skill/`;
    const skillLink = `[${displayName}](${docsUrl})`;
    const auditInfo = await getLatestAuditInfo(skill.relativePath);
    const badge = auditInfo
      ? getBadgeMarkdown(auditInfo.grade)
      : `![?](https://img.shields.io/badge/Rating-?-lightgrey)`;

    section += `\n### ${displayName}\n\n`;
    section += `${description}\n\n`;
    section += "| Skill | Rating |\n";
    section += "| --- | --- |\n";
    section += `| ${skillLink} | ${badge} |\n`;
  }

  return section;
};
