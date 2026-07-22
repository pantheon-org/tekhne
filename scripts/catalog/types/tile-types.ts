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
