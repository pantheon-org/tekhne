import { getSkillDisplayName } from "./get-skill-display-name";
import { getTileAnchor } from "./get-tile-anchor";
import type { CatalogDomain } from "./readme-types";
import { toGitHubAnchor } from "./to-github-anchor";

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
