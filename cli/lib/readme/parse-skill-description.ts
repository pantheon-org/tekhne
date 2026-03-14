import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

interface FrontmatterResult {
  description: string;
  found: boolean;
}

const extractQuotedValue = (value: string): string | null => {
  if (value.startsWith('"') || value.startsWith("'")) {
    const extracted = value.slice(1, -1);
    return extracted.length > 0 ? extracted : null;
  }
  return null;
};

const isMultilineDescriptionStart = (value: string): boolean => {
  return value.startsWith("|") || value.startsWith(">");
};

const extractMultilineDescription = (
  lines: string[],
  startIndex: number,
): string => {
  let description = "";
  for (let i = startIndex; i < lines.length; i++) {
    const line = lines[i];
    if (line.trim() === "---") {
      break;
    }
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith("-")) {
      description += (description ? " " : "") + trimmed;
    }
  }
  return description;
};

const parseDescriptionValue = (
  value: string,
  lines: string[],
  lineIndex: number,
): string => {
  const quoted = extractQuotedValue(value);
  if (quoted !== null) {
    return quoted;
  }

  if (value.startsWith('"') || value.startsWith("'")) {
    return "";
  }

  if (isMultilineDescriptionStart(value)) {
    return extractMultilineDescription(lines, lineIndex + 1);
  }

  return value;
};

const extractFrontmatterDescription = (content: string): FrontmatterResult => {
  const lines = content.split("\n");
  let inFrontmatter = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (line.trim() === "---") {
      if (!inFrontmatter) {
        inFrontmatter = true;
        continue;
      }
      break;
    }

    if (inFrontmatter && line.startsWith("description:")) {
      const value = line.substring(12).trim();
      const description = parseDescriptionValue(value, lines, i);
      return { description, found: true };
    }
  }

  return { description: "", found: false };
};

const formatDescription = (description: string): string => {
  const cleaned = description.replace(/\|/g, "\\|");
  return cleaned.length > 80 ? `${cleaned.substring(0, 80)}...` : cleaned;
};

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
