import { existsSync } from "node:fs";
import { isAbsolute, resolve } from "node:path";
import { load as yamlLoad } from "js-yaml";
import { ValidationError } from "../utils/errors";
import { logger } from "../utils/logger";
import { checkRequiredFields, type Frontmatter } from "./check-required-fields";
import { extractFrontmatter } from "./extract-frontmatter";

/**
 * Parse and validate frontmatter for a single file.
 * Returns an array of error messages (empty means valid).
 */
const validateFile = async (filePath: string): Promise<string[]> => {
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

const collectSkillFiles = async (repoRoot: string): Promise<string[]> => {
  const glob = new Bun.Glob("skills/**/SKILL.md");
  const files: string[] = [];
  for await (const file of glob.scan({ cwd: repoRoot, absolute: true })) {
    files.push(file);
  }
  return files.sort();
};

/**
 * Validate SKILL.md frontmatter for one or more files.
 * When no files are provided, scans all SKILL.md files under skills/.
 * File paths may be relative (e.g. from lefthook) or absolute.
 */
export const validateSkillFrontmatter = async (
  files: string[],
): Promise<void> => {
  const repoRoot = resolve(import.meta.dir, "../../..");
  const targets: string[] =
    files.length > 0
      ? files
          .filter((f) => f.endsWith("SKILL.md"))
          // Resolve relative paths (e.g. passed by lefthook) to absolute.
          .map((f) => (isAbsolute(f) ? f : resolve(process.cwd(), f)))
      : await collectSkillFiles(repoRoot);

  if (targets.length === 0) {
    logger.warning("No SKILL.md files found to validate.");
    return;
  }

  logger.info(
    `Validating frontmatter in ${targets.length} SKILL.md file(s)...`,
  );

  const results = await Promise.all(targets.map(validateFile));
  const allErrors = results.flat();

  for (const err of allErrors) {
    logger.error(err);
  }

  if (allErrors.length > 0) {
    throw new ValidationError(
      `Frontmatter validation failed: ${allErrors.length} error(s) found.`,
    );
  }

  logger.success(
    `All ${targets.length} SKILL.md file(s) have valid frontmatter.`,
  );
};
