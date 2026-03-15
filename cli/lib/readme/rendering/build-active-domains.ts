import type { CatalogDomain, SkillEntry, TileEntry } from "../types";
import { DOMAINS } from "../types";
import { buildCountLabel } from "./build-count-label";

export const buildActiveDomains = (
  tiles: TileEntry[],
  untiledSkills: SkillEntry[],
): CatalogDomain[] => {
  const activeDomains: CatalogDomain[] = [];

  for (const domainInfo of DOMAINS) {
    const domainTiles = tiles.filter((t) => t.domain === domainInfo.key);
    const domainUntiledSkills = untiledSkills.filter(
      (s) => s.domain === domainInfo.key,
    );

    if (domainTiles.length === 0 && domainUntiledSkills.length === 0) continue;

    const countLabel = buildCountLabel(
      domainTiles.length,
      domainUntiledSkills.length,
    );

    activeDomains.push({
      heading: `${domainInfo.title} (${countLabel})`,
      description: domainInfo.description,
      tiles: domainTiles,
      untiledSkills: domainUntiledSkills,
    });
  }

  return activeDomains;
};
