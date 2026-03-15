import { dirname } from "node:path";
import { $ } from "bun";
import { parseShortName } from "../parsing";
import type { TileEntry } from "../types";
import { buildTileSkills } from "./build-tile-skills";
import { isChildTile } from "./is-child-tile";
import { parsePublishedStatus } from "./parse-published-status";

export const findAllTiles = async (): Promise<TileEntry[]> => {
  const output = await $`find skills -name "tile.json" -type f`.text();
  const files = output.trim().split("\n").filter(Boolean);

  // Build a set of all tile directories to detect parent/child relationships.
  // A child tile is one whose parent directory also contains a tile.json.
  const tileDirs = new Set(files.map((f) => dirname(f)));

  const tiles: TileEntry[] = [];

  for (const file of files) {
    try {
      const tileData = await Bun.file(file).json();
      const tileDir = dirname(file);

      // Skip private child tiles: if the parent directory also has a tile.json
      // AND this tile is private, it is a sub-skill component of a consolidated
      // parent tile and should not appear as a top-level entry.
      if (isChildTile(tileDir, tileDirs, tileData)) continue;

      const tileRelDir = tileDir.replace(/^skills\//, "");
      const domain = tileRelDir.split("/")[0];

      const skillsObj: Record<string, { path?: string }> =
        tileData.skills || {};
      const skills = buildTileSkills(tileDir, skillsObj);

      if (skills.length === 0) continue;

      const fullName = tileData.name || "";
      const shortName = parseShortName(fullName);
      const publishedStatus = parsePublishedStatus(tileData);
      const isPublic = publishedStatus === "public";

      tiles.push({
        tileDir,
        domain,
        shortName,
        fullName,
        version: tileData.version || "",
        summary: tileData.summary || "",
        isPublic,
        publishedStatus,
        skills,
      });
    } catch {
      // skip invalid tile.json
    }
  }

  return tiles;
};
