import { getSkillDisplayName, getTileAnchor, toGitHubAnchor } from "../parsing";
import type { CatalogDomain } from "../types";

export const buildCatalogToc = (activeDomains: CatalogDomain[]): string => {
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
  return toc;
};
