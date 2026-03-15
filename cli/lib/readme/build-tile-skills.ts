import { existsSync } from "node:fs";
import { join } from "node:path";
import { resolveSkillDir } from "./resolve-skill-dir";
import type { TileSkillEntry } from "./tile-types";

export const buildTileSkills = (
  tileDir: string,
  skillsObj: Record<string, { path?: string }>,
): TileSkillEntry[] =>
  Object.entries(skillsObj)
    .map(([name, def]) => {
      const skillMdPath = def.path || "SKILL.md";
      const skillDir = resolveSkillDir(tileDir, skillMdPath);
      const auditRelPath = skillDir.replace(/^skills\//, "");
      return { name, skillDir, auditRelPath };
    })
    .filter((s) => existsSync(join(s.skillDir, "SKILL.md")));
