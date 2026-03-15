import { existsSync } from "node:fs";
import { load as yamlLoad } from "js-yaml";
import { checkRequiredFields, type Frontmatter } from "./check-required-fields";
import { extractFrontmatter } from "./extract-frontmatter";

export const validateFile = async (filePath: string): Promise<string[]> => {
  if (!existsSync(filePath)) {
    return [`File not found: ${filePath}`];
  }

  const content = await Bun.file(filePath).text();
  const fm = extractFrontmatter(content);

  if (fm === null) {
    return [
      `${filePath}: cannot locate frontmatter block (missing --- delimiters)`,
    ];
  }

  let parsed: Frontmatter;
  try {
    parsed = yamlLoad(fm) as Frontmatter;
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    return [`${filePath}: invalid YAML frontmatter — ${msg}`];
  }

  if (!parsed || typeof parsed !== "object") {
    return [`${filePath}: frontmatter must be a YAML mapping`];
  }

  return checkRequiredFields(filePath, parsed);
};
