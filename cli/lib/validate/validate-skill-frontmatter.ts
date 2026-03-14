import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { load as yamlLoad } from "js-yaml";
import { ValidationError } from "../utils/errors";
import { logger } from "../utils/logger";

const REQUIRED_FIELDS = ["name", "description"] as const;

interface Frontmatter {
  name?: unknown;
  description?: unknown;
  [key: string]: unknown;
}

/**
 * Extract raw YAML frontmatter string from a Markdown file.
 * Returns null if no frontmatter block is found.
 */
function extractFrontmatter(content: string): string | null {
  // Match opening ---, capture body, then closing --- which may be followed by
  // a newline or appear at end-of-file (no trailing newline required).
  const match = content.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  return match ? match[1] : null;
}

/**
 * Validate required fields on an already-parsed frontmatter object.
 * Uses an explicit null/undefined check rather than falsy to avoid
 * false positives on empty strings or zero values.
 */
function checkRequiredFields(filePath: string, parsed: Frontmatter): string[] {
  const missing = REQUIRED_FIELDS.filter(
    (f) => parsed[f] == null || parsed[f] === "",
  );
  if (missing.length > 0) {
    return [
      `${filePath}: frontmatter missing required fields: ${missing.join(", ")}`,
    ];
  }
  return [];
}

/**
 * Parse and validate frontmatter for a single file.
 * Returns an array of error messages (empty means valid).
 */
async function validateFile(filePath: string): Promise<string[]> {
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
}

/**
 * Validate SKILL.md frontmatter for one or more files.
 * When no files are provided, scans all SKILL.md files under skills/.
 */
export async function validateSkillFrontmatter(files: string[]): Promise<void> {
  const repoRoot = resolve(import.meta.dir, "../../..");
  const targets: string[] =
    files.length > 0
      ? files.filter((f) => f.endsWith("SKILL.md"))
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
}

async function collectSkillFiles(repoRoot: string): Promise<string[]> {
  const glob = new Bun.Glob("skills/**/SKILL.md");
  const files: string[] = [];
  for await (const file of glob.scan({ cwd: repoRoot, absolute: true })) {
    files.push(file);
  }
  return files.sort();
}
