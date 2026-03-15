import { buildDomainDocsSection } from "./build-domain-docs-section";
import { DOMAINS } from "./domain-config";
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

    sections += await buildDomainDocsSection(
      domainInfo,
      domainTiles,
      domainUntiledSkills,
      first,
    );
    first = false;
  }

  return frontmatter + sections;
};
