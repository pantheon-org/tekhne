import { DOMAINS } from "./domain-config";
import { getSkillDisplayName } from "./get-skill-display-name";
import { getTileAnchor } from "./get-tile-anchor";
import { getTileTessl } from "./get-tile-tessl";
import type { SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";

const TILES_PATH = "TILES.md";

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

    output += `\n## ${domainInfo.title} (${countLabel})\n\n`;
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
  }

  return output;
};
