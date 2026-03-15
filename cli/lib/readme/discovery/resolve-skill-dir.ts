import { join } from "node:path";

export const resolveSkillDir = (tileDir: string, skillPath: string): string => {
  const withoutSkillMd = skillPath
    .replace(/\/SKILL\.md$/, "")
    .replace(/^SKILL\.md$/, "");
  return withoutSkillMd ? join(tileDir, withoutSkillMd) : tileDir;
};
