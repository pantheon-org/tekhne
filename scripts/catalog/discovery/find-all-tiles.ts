import { dirname } from "node:path";
import { $ } from "bun";
import { parseShortName } from "../parsing";
import type { TileEntry } from "../types";
import { buildTileSkills } from "./build-tile-skills";
import { isChildTile } from "./is-child-tile";
import { parsePublishedStatus } from "./parse-published-status";

export const findAllTiles = async (): Promise<TileEntry[]> => {
  const output =
    await $`find skills -name "plugin.json" -path "*/.tessl-plugin/*" -o -name "tile.json" -type f`.text();
  const files = output.trim().split("\n").filter(Boolean);

  const tileDirs = new Set(files.map((f) => dirname(f)));
  const tiles: TileEntry[] = [];

  for (const file of files) {
    try {
      const rawData = (await Bun.file(file).json()) as Record<string, unknown>;
      const tileDir = dirname(file);

      if (isChildTile(tileDir, tileDirs, rawData.private === true)) continue;

      const tileRelDir = tileDir.replace(/^skills\//, "");
      const domain = tileRelDir.split("/")[0];

      const skillsArr: string[] = [];
      const rawSkills = rawData.skills;
      if (Array.isArray(rawSkills)) {
        for (const entry of rawSkills) {
          if (typeof entry === "string") skillsArr.push(entry);
          else if (entry && typeof entry === "object" && "path" in entry) {
            skillsArr.push((entry as { path: string }).path);
          }
        }
      } else if (rawSkills && typeof rawSkills === "object") {
        for (const def of Object.values(rawSkills) as Array<
          Record<string, unknown>
        >) {
          if (def?.path && typeof def.path === "string")
            skillsArr.push(def.path);
        }
      }

      const skills = buildTileSkills(tileDir, skillsArr);
      if (skills.length === 0) continue;

      const fullName = (rawData.name as string) || "";
      const summary =
        (rawData.description as string) ?? (rawData.summary as string) ?? "";

      tiles.push({
        tileDir,
        domain,
        shortName: parseShortName(fullName),
        fullName,
        version: (rawData.version as string) || "",
        summary,
        isPublic: parsePublishedStatus(rawData) === "public",
        publishedStatus: parsePublishedStatus(rawData),
        skills,
      });
    } catch {
      // skip invalid manifest
    }
  }

  return tiles;
};
