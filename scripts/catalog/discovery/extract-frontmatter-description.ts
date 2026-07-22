import { parseDescriptionValue } from "../parsing";

export interface FrontmatterResult {
  description: string;
  found: boolean;
}

export const extractFrontmatterDescription = (
  content: string,
): FrontmatterResult => {
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
