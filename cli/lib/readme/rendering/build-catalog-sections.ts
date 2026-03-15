import type { CatalogDomain } from "../types";
import { generateTileSection } from "./generate-tile-section";
import { generateUntiledSkillSection } from "./generate-untiled-skill-section";

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
