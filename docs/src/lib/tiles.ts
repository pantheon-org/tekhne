import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { basename, dirname, join, relative, resolve, sep } from "node:path";

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

const PLUGIN_DIR = ".tessl-plugin";

const findTileManifests = (dir: string, results: string[] = []): string[] => {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) {
      findTileManifests(full, results);
    } else if (
      entry === "tile.json" ||
      (entry === "plugin.json" && basename(dir) === PLUGIN_DIR)
    ) {
      results.push(full);
    }
  }
  return results;
};

/**
 * The tile directory holding a manifest, relative to SKILLS_ROOT and always
 * "/"-separated. Compared and sliced with path calls rather than string
 * suffixes, because join()/relative() emit "\" on Windows and a hardcoded "/"
 * would match nothing there.
 */
const manifestToTilePath = (file: string): string => {
  const dir = dirname(file);
  const tileDir = basename(dir) === PLUGIN_DIR ? dirname(dir) : dir;
  return relative(SKILLS_ROOT, tileDir).split(sep).join("/");
};

/**
 * A skill's key is its `name` frontmatter, which for a nested skill is a
 * consolidated name ("github-actions-generator") rather than its directory
 * name ("generator"). The docs pages are keyed the same way, so read it from
 * source and fall back to the directory name only when it cannot be found.
 */
const skillKeyAt = (skillPath: string): string | null => {
  const file = join(SKILLS_ROOT, skillPath, "SKILL.md");
  if (!existsSync(file)) return null;
  const frontmatter = readFileSync(file, "utf-8").match(
    /^---\n([\s\S]*?)\n---/,
  );
  return frontmatter?.[1].match(/^name:\s*(.+)$/m)?.[1].trim() || null;
};

/**
 * `skills` is a record of {path, summary} in a legacy tile.json, and an array
 * of paths relative to the tile in a .tessl-plugin/plugin.json. A relative
 * path may be a subdirectory ("generator"), or a bare "SKILL.md" meaning the
 * tile directory is itself the skill.
 */
const entryPath = (entry: unknown): string =>
  typeof entry === "string"
    ? entry
    : ((entry as { path?: string })?.path ?? "");

const skillsFromArray = (
  raw: unknown[],
  tilePath: string,
): Record<string, TileSkillEntry> => {
  const skills: Record<string, TileSkillEntry> = {};
  for (const entry of raw) {
    const rawPath = entryPath(entry);
    if (!rawPath) continue;
    const relDir = rawPath.replace(/(^|\/)SKILL\.md$/, "");
    const skillPath = relDir ? `${tilePath}/${relDir}` : tilePath;
    const key = skillKeyAt(skillPath) ?? skillPath.split("/").pop();
    if (key) skills[key] = { key, path: skillPath };
  }
  return skills;
};

const skillsFromRecord = (raw: object): Record<string, TileSkillEntry> => {
  const skills: Record<string, TileSkillEntry> = {};
  for (const [key, val] of Object.entries(
    raw as Record<string, { path: string; summary?: string }>,
  )) {
    skills[key] = { key, path: val.path, summary: val.summary };
  }
  return skills;
};

const normaliseSkills = (
  raw: unknown,
  tilePath: string,
): Record<string, TileSkillEntry> => {
  if (Array.isArray(raw)) return skillsFromArray(raw, tilePath);
  if (raw && typeof raw === "object") return skillsFromRecord(raw);
  return {};
};

const tileNameToSlug = (name: string): string => name.replace(/^[^/]+\//, "");

let _tiles: TileData[] | null = null;

export const loadTiles = (): TileData[] => {
  if (_tiles) return _tiles;

  const files = findTileManifests(SKILLS_ROOT);
  const all: TileData[] = [];

  for (const file of files) {
    try {
      const raw = JSON.parse(readFileSync(file, "utf-8")) as {
        name?: string;
        version?: string;
        summary?: string;
        description?: string;
        private?: boolean;
        skills?: unknown;
      };
      if (!raw.name || !raw.skills) continue;
      const tilePath = manifestToTilePath(file);
      const skills = normaliseSkills(raw.skills, tilePath);
      if (Object.keys(skills).length === 0) continue;
      all.push({
        name: raw.name,
        slug: tileNameToSlug(raw.name),
        version: raw.version ?? "0.0.0",
        summary: raw.summary ?? raw.description ?? "",
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
