import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { $ } from "bun";

export interface TileSkillEntry {
  name: string; // skill key from tile.json, e.g., "makefile-generator"
  skillDir: string; // from repo root, e.g., "skills/development/scripting/makefile/generator"
  auditRelPath: string; // e.g., "development/scripting/makefile/generator"
}

export type PublishedStatus = "public" | "private" | "unpublished";

export interface TileEntry {
  tileDir: string; // e.g., "skills/development/scripting/makefile"
  domain: string; // e.g., "development"
  shortName: string; // e.g., "makefile-toolkit"
  fullName: string; // e.g., "pantheon-ai/makefile-toolkit"
  version: string; // e.g., "0.1.0"
  summary: string;
  isPublic: boolean;
  publishedStatus: PublishedStatus;
  skills: TileSkillEntry[];
}

function resolveSkillDir(tileDir: string, skillPath: string): string {
  const withoutSkillMd = skillPath
    .replace(/\/SKILL\.md$/, "")
    .replace(/^SKILL\.md$/, "");
  return withoutSkillMd ? join(tileDir, withoutSkillMd) : tileDir;
}

export async function findAllTiles(): Promise<TileEntry[]> {
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
      if (tileDirs.has(dirname(tileDir)) && tileData.private === true) continue;

      const tileRelDir = tileDir.replace(/^skills\//, "");
      const domain = tileRelDir.split("/")[0];

      const skillsObj: Record<string, { path?: string }> =
        tileData.skills || {};

      const skills: TileSkillEntry[] = Object.entries(skillsObj)
        .map(([name, def]) => {
          const skillMdPath = def.path || "SKILL.md";
          const skillDir = resolveSkillDir(tileDir, skillMdPath);
          const auditRelPath = skillDir.replace(/^skills\//, "");
          return { name, skillDir, auditRelPath };
        })
        .filter((s) => existsSync(join(s.skillDir, "SKILL.md")));

      if (skills.length === 0) continue;

      const fullName = tileData.name || "";
      const shortName = fullName.includes("/")
        ? fullName.split("/").slice(1).join("/")
        : fullName;

      const isPublic = tileData.private === false;
      const publishedStatus: PublishedStatus =
        tileData.private === false
          ? "public"
          : tileData.private === true
            ? "private"
            : "unpublished";

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
}

export function getTileTessl(tile: TileEntry): string {
  if (!tile.isPublic) return "-";
  if (tile.fullName) {
    return `[Public](https://tessl.io/registry/skills/${tile.fullName})`;
  }
  return "Public";
}
