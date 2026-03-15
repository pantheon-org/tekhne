import type { SkillEntry } from "./readme-types";
import type { TileEntry } from "./tile-types";

export const findUntiledSkills = (
  allSkills: SkillEntry[],
  tiles: TileEntry[],
): SkillEntry[] => {
  const tiledSkillDirs = new Set(
    tiles.flatMap((t) => t.skills.map((s) => s.skillDir)),
  );
  return allSkills.filter(
    (skill) => !tiledSkillDirs.has(`skills/${skill.relativePath}`),
  );
};
