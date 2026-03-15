import { buildDomainTableSection } from "./build-domain-table-section";
import { DOMAINS } from "./domain-config";
import type { SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";

export const generateReadmeSummaryTables = async (
  tiles: TileEntry[],
  untiledSkills: SkillEntry[],
): Promise<string> => {
  let output = "";

  for (const domainInfo of DOMAINS) {
    const domainTiles = tiles.filter((t) => t.domain === domainInfo.key);
    const domainUntiledSkills = untiledSkills.filter(
      (s) => s.domain === domainInfo.key,
    );

    if (domainTiles.length === 0 && domainUntiledSkills.length === 0) continue;

    output += buildDomainTableSection(
      domainInfo,
      domainTiles,
      domainUntiledSkills,
    );
  }

  return output;
};
