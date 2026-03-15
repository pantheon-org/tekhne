import { buildCountLabel } from "./build-count-label";
import { getSkillDisplayName } from "./get-skill-display-name";
import { getTileAnchor } from "./get-tile-anchor";
import { getTileTessl } from "./get-tile-tessl";
import type { SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";

const TILES_PATH = "TILES.md";

export const buildDomainTableSection = (
  domainInfo: { key: string; title: string; description: string },
  domainTiles: TileEntry[],
  domainUntiledSkills: SkillEntry[],
): string => {
  const countLabel = buildCountLabel(
    domainTiles.length,
    domainUntiledSkills.length,
  );

  let output = `\n## ${domainInfo.title} (${countLabel})\n\n`;
  output += `${domainInfo.description}\n\n`;
  output += "| Tile | Skills | Published | Version |\n";
  output += "| --- | --- | --- | --- |\n";

  for (const tile of domainTiles) {
    const anchor = getTileAnchor(tile.shortName);
    const tileLink = `[${tile.shortName}](${TILES_PATH}#${anchor})`;
    const tileSkillCount = tile.skills.length;
    const publishedCell =
      tile.publishedStatus === "public"
        ? getTileTessl(tile)
        : tile.publishedStatus === "private"
          ? "Private"
          : "-";
    const versionCell = tile.version || "-";
    output += `| ${tileLink} | ${tileSkillCount} | ${publishedCell} | ${versionCell} |\n`;
  }

  for (const skill of domainUntiledSkills) {
    const displayName = getSkillDisplayName(skill.relativePath);
    const anchor = `${getTileAnchor(displayName)}-no-tile`;
    const skillLink = `[${displayName}](${TILES_PATH}#${anchor}) _(no tile)_`;
    output += `| ${skillLink} | 1 | - | - |\n`;
  }

  return output;
};
