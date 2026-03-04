import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { $ } from "bun";

export interface TileSkillEntry {
  name: string; // skill key from tile.json, e.g., "makefile-generator"
  skillDir: string; // from repo root, e.g., "skills/development/scripting/makefile/generator"
  auditRelPath: string; // e.g., "development/scripting/makefile/generator"
}

export interface TileEntry {
  tileDir: string; // e.g., "skills/development/scripting/makefile"
  domain: string; // e.g., "development"
  shortName: string; // e.g., "makefile-toolkit"
  fullName: string; // e.g., "pantheon-ai/makefile-toolkit"
  summary: string;
  isPublic: boolean;
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

  const tiles: TileEntry[] = [];

  for (const file of files) {
    try {
      const tileData = await Bun.file(file).json();
      const tileDir = dirname(file);
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

      tiles.push({
        tileDir,
        domain,
        shortName,
        fullName,
        summary: tileData.summary || "",
        isPublic: tileData.private === false,
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
    return `[Public](https://tessl.io/registry/skills/pantheon-ai/${tile.fullName})`;
  }
  return "Public";
}
