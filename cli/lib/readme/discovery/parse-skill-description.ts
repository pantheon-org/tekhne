import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { formatDescription } from "../parsing";
import { extractFrontmatterDescription } from "./extract-frontmatter-description";

export const parseSkillDescription = (skillPath: string): string => {
  const skillFile = join(skillPath, "SKILL.md");
  if (!existsSync(skillFile)) {
    return "-";
  }

  const content = readFileSync(skillFile, "utf-8");
  const result = extractFrontmatterDescription(content);

  if (!result.found || !result.description) {
    return "-";
  }

  return formatDescription(result.description);
};
