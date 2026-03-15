import {
  getEvalCount,
  getTileTessl,
  parseSkillDescription,
} from "../discovery";
import { formatSummary, getSkillDisplayName } from "../parsing";
import type { SkillEntry, TileEntry } from "../types";
import { buildCountLabel } from "./build-count-label";
import { buildSkillDocsRow } from "./build-skill-docs-row";

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

    const publishedCell = getTileTessl(tile);
    const versionCell = tile.version || "-";
    section += `**Published:** ${publishedCell} | **Version:** ${versionCell}\n\n`;

    section += "| Skill | Rating | Audit | Evals |\n";
    section += "| --- | --- | --- | --- |\n";

    for (const skill of tile.skills) {
      const relPath = skill.skillDir.replace(/^skills\//, "");
      const docsUrl = `/tekhne/skills/${relPath}/skill/`;
      const skillLink = `[${skill.name}](${docsUrl})`;
      const evalCount = getEvalCount(skill.skillDir);
      const evalsCell = evalCount > 0 ? String(evalCount) : "-";
      section += await buildSkillDocsRow({
        skillLink,
        auditRelPath: skill.auditRelPath,
        evalsCell,
      });
    }
  }

  for (const skill of domainUntiledSkills) {
    const displayName = getSkillDisplayName(skill.relativePath);
    const description = parseSkillDescription(`skills/${skill.relativePath}`);
    const relPath = skill.relativePath;
    const docsUrl = `/tekhne/skills/${relPath}/skill/`;
    const skillLink = `[${displayName}](${docsUrl})`;
    const evalCount = getEvalCount(`skills/${skill.relativePath}`);
    const evalsCell = evalCount > 0 ? String(evalCount) : "-";

    section += `\n### ${displayName}\n\n`;
    section += `${description}\n\n`;
    section += "| Skill | Rating | Audit | Evals |\n";
    section += "| --- | --- | --- | --- |\n";
    section += await buildSkillDocsRow({
      skillLink,
      auditRelPath: skill.relativePath,
      evalsCell,
    });
  }

  return section;
};
