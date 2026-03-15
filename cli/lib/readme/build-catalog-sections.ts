import { generateTileSection } from "./generate-tile-section";
import { generateUntiledSkillSection } from "./generate-untiled-skill-section";
import type { CatalogDomain } from "./readme-types";

export const buildCatalogSections = async (
  activeDomains: CatalogDomain[],
): Promise<string> => {
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
  return sections;
};
