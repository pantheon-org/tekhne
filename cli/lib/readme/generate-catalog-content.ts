import { DOMAINS } from "./domain-config";
import { generateTileSection } from "./generate-tile-section";
import { generateUntiledSkillSection } from "./generate-untiled-skill-section";
import { getSkillDisplayName } from "./get-skill-display-name";
import { getTileAnchor } from "./get-tile-anchor";
import type { CatalogDomain, SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";
import { toGitHubAnchor } from "./to-github-anchor";

export const generateCatalogContent = async (
  tiles: TileEntry[],
  untiledSkills: SkillEntry[],
): Promise<string> => {
  // Collect active domains first so we can build ToC and sections together
  const activeDomains: CatalogDomain[] = [];

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

    activeDomains.push({
      heading: `${domainInfo.title} (${countLabel})`,
      description: domainInfo.description,
      tiles: domainTiles,
      untiledSkills: domainUntiledSkills,
    });
  }

  // Build ToC
  let toc = "## Contents\n\n";
  for (const domain of activeDomains) {
    const domainAnchor = toGitHubAnchor(domain.heading);
    toc += `- [${domain.heading}](#${domainAnchor})\n`;
    for (const tile of domain.tiles) {
      const tileAnchor = getTileAnchor(tile.shortName);
      toc += `  - [${tile.shortName}](#${tileAnchor})\n`;
    }
    for (const skill of domain.untiledSkills) {
      const displayName = getSkillDisplayName(skill.relativePath);
      const anchor = `${getTileAnchor(displayName)}-no-tile`;
      toc += `  - [${displayName} _(no tile)_](#${anchor})\n`;
    }
  }

  // Build sections
  let sections = "";
  for (const domain of activeDomains) {
    sections += `\n## ${domain.heading}\n\n`;
    sections += `${domain.description}\n`;

    for (const tile of domain.tiles) {
      sections += await generateTileSection(tile);
    }

    for (const skill of domain.untiledSkills) {
      sections += await generateUntiledSkillSection(skill);
    }
  }

  return `# Tile Catalog\n\nDetailed information for all tiles and skills.\n\n${toc}${sections}`;
};
