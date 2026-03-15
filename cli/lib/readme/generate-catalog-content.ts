import { buildActiveDomains } from "./build-active-domains";
import { buildCatalogSections } from "./build-catalog-sections";
import { buildCatalogToc } from "./build-catalog-toc";
import type { SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";

export const generateCatalogContent = async (
  tiles: TileEntry[],
  untiledSkills: SkillEntry[],
): Promise<string> => {
  const activeDomains = buildActiveDomains(tiles, untiledSkills);
  const toc = buildCatalogToc(activeDomains);
  const sections = await buildCatalogSections(activeDomains);
  return `# Tile Catalog\n\nDetailed information for all tiles and skills.\n\n${toc}${sections}`;
};
