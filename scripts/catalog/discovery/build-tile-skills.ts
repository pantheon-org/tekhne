import { existsSync } from "node:fs";
import { join } from "node:path";
import type { TileSkillEntry } from "../types";
import { resolveSkillDir } from "./resolve-skill-dir";

export const buildTileSkills = (
  tileDir: string,
  skills: string[],
): TileSkillEntry[] =>
  skills
    .map((skillPath) => {
      const skillDir = resolveSkillDir(tileDir, skillPath);
      const auditRelPath = skillDir.replace(/^skills\//, "");
      const name = skillDir.split("/").pop() || "skill";
      return { name, skillDir, auditRelPath };
    })
    .filter((s) => existsSync(join(s.skillDir, "SKILL.md")));
