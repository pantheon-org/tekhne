import { DOMAINS } from "./domain-config";
import { formatSummary } from "./format-summary";
import { getBadgeMarkdown } from "./get-badge-markdown";
import { getLatestAuditInfo } from "./get-latest-audit-info";
import { getSkillDisplayName } from "./get-skill-display-name";
import { parseSkillDescription } from "./parse-skill-description";
import type { SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";

export const generateDocsTilesPage = async (
  tiles: TileEntry[],
  untiledSkills: SkillEntry[],
): Promise<string> => {
  const frontmatter = `---
title: Skill Catalog
description: All Tekhne tiles and skills organized by domain.
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 3
---

Detailed information for all tiles and skills organized by domain.
`;

  let sections = "";
  let first = true;

  for (const domainInfo of DOMAINS) {
    const domainTiles = tiles.filter((t) => t.domain === domainInfo.key);
    const domainUntiledSkills = untiledSkills.filter(
      (s) => s.domain === domainInfo.key,
    );

    if (domainTiles.length === 0 && domainUntiledSkills.length === 0) continue;

    const tileCount = domainTiles.length;
    const skillCount = domainUntiledSkills.length;
    let countLabel: string;
    if (tileCount > 0 && skillCount > 0) {
      countLabel = `${tileCount} tile${tileCount !== 1 ? "s" : ""}, ${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
    } else if (tileCount > 0) {
      countLabel = `${tileCount} tile${tileCount !== 1 ? "s" : ""}`;
    } else {
      countLabel = `${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
    }

    if (!first) sections += "\n---\n";
    first = false;

    sections += `\n## ${domainInfo.title} (${countLabel})\n\n`;
    sections += `${domainInfo.description}\n`;

    // Tiled skills
    for (const tile of domainTiles) {
      sections += `\n### ${tile.shortName}\n\n`;
      sections += `${formatSummary(tile.summary)}\n\n`;
      sections += "| Skill | Rating |\n";
      sections += "| --- | --- |\n";

      for (const skill of tile.skills) {
        // Build Astro docs URL: /tekhne/skills/<relative-path>/skill/
        const relPath = skill.skillDir.replace(/^skills\//, "");
        const docsUrl = `/tekhne/skills/${relPath}/skill/`;
        const skillLink = `[${skill.name}](${docsUrl})`;
        const auditInfo = await getLatestAuditInfo(skill.auditRelPath);
        const badge = auditInfo
          ? getBadgeMarkdown(auditInfo.grade)
          : `![?](https://img.shields.io/badge/Rating-?-lightgrey)`;
        sections += `| ${skillLink} | ${badge} |\n`;
      }
    }

    // Untiled skills
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

      sections += `\n### ${displayName}\n\n`;
      sections += `${description}\n\n`;
      sections += "| Skill | Rating |\n";
      sections += "| --- | --- |\n";
      sections += `| ${skillLink} | ${badge} |\n`;
    }
  }

  return frontmatter + sections;
};
