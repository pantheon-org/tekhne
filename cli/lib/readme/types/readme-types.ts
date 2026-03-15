import type { TileEntry } from "./tile-types";

export interface UpdateOptions {
  dryRun?: boolean;
}

export interface SkillEntry {
  domain: string;
  relativePath: string;
}

export interface CatalogDomain {
  heading: string;
  description: string;
  tiles: TileEntry[];
  untiledSkills: SkillEntry[];
}

export interface ReadmeSections {
  beforeSkills: string[];
  afterSkills: string[];
}
