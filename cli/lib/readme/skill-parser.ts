import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

export function parseSkillDescription(skillPath: string): string {
  const skillFile = join(skillPath, "SKILL.md");
  if (!existsSync(skillFile)) {
    return "-";
  }

  const content = readFileSync(skillFile, "utf-8");
  const lines = content.split("\n");

  let inFrontmatter = false;
  let inDescription = false;
  let description = "";

  for (const line of lines) {
    if (line.trim() === "---") {
      if (!inFrontmatter) {
        inFrontmatter = true;
        continue;
      }
      break;
    }

    if (inFrontmatter) {
      if (line.startsWith("description:")) {
        const value = line.substring(12).trim();
        if (value.startsWith("|") || value.startsWith(">")) {
          inDescription = true;
          continue;
        }
        if (value.startsWith('"') || value.startsWith("'")) {
          description = value.slice(1, -1);
          break;
        }
        description = value;
        break;
      }
      if (inDescription) {
        const trimmed = line.trim();
        if (trimmed && !trimmed.startsWith("-")) {
          description += (description ? " " : "") + trimmed;
        }
      }
    }
  }

  if (!description) {
    return "-";
  }

  const cleaned = description.replace(/\|/g, "\\|");
  return cleaned.length > 80 ? `${cleaned.substring(0, 80)}...` : cleaned;
}

export function getSkillDisplayName(skillRelativePath: string): string {
  const parts = skillRelativePath.split("/");
  const pathWithoutDomain = parts.slice(1).join("/");

  if (pathWithoutDomain.includes("/")) {
    return pathWithoutDomain.replace(/\//g, "-");
  }

  return pathWithoutDomain;
}
