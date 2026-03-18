import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const SKILLS_ROOT = resolve(process.cwd(), "../skills");

export interface TileSkillEntry {
  key: string;
  path: string;
  summary?: string;
}

export interface TileData {
  name: string;
  slug: string;
  version: string;
  summary: string;
  private: boolean;
  skills: Record<string, TileSkillEntry>;
  tilePath: string;
}

const findTileJsonFiles = (dir: string, results: string[] = []): string[] => {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) {
      findTileJsonFiles(full, results);
    } else if (entry === "tile.json") {
      results.push(full);
    }
  }
  return results;
};

const tileNameToSlug = (name: string): string => name.replace(/^[^/]+\//, "");

let _tiles: TileData[] | null = null;

export const loadTiles = (): TileData[] => {
  if (_tiles) return _tiles;

  const files = findTileJsonFiles(SKILLS_ROOT);
  const all: TileData[] = [];

  for (const file of files) {
    try {
      const raw = JSON.parse(readFileSync(file, "utf-8")) as {
        name?: string;
        version?: string;
        summary?: string;
        private?: boolean;
        skills?: Record<string, { path: string; summary?: string }>;
      };
      if (!raw.name || !raw.skills) continue;
      const tilePath = relative(SKILLS_ROOT, file).replace(/\/tile\.json$/, "");
      const skills: Record<string, TileSkillEntry> = {};
      for (const [key, val] of Object.entries(raw.skills)) {
        skills[key] = { key, path: val.path, summary: val.summary };
      }
      all.push({
        name: raw.name,
        slug: tileNameToSlug(raw.name),
        version: raw.version ?? "0.0.0",
        summary: raw.summary ?? "",
        private: raw.private ?? false,
        skills,
        tilePath,
      });
    } catch {
      // skip malformed
    }
  }

  _tiles = all;
  return all;
};

export const loadPublicTiles = (): TileData[] =>
  loadTiles().filter((t) => !t.private);

export const buildSkillToTileMap = (): Map<string, TileData> => {
  const map = new Map<string, TileData>();
  for (const tile of loadPublicTiles()) {
    for (const key of Object.keys(tile.skills)) {
      if (!map.has(key)) map.set(key, tile);
    }
  }
  return map;
};
