import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
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
  const match = content.match(/^---\r?\n([\s\S]*?)\r?\n---/);
  return match ? match[1] : null;
}

/**
 * Validate required fields on an already-parsed frontmatter object.
 */
function checkRequiredFields(filePath: string, parsed: Frontmatter): string[] {
  const missing = REQUIRED_FIELDS.filter((f) => !parsed[f]);
  if (missing.length > 0) {
    return [
      `${filePath}: frontmatter missing required fields: ${missing.join(", ")}`,
    ];
  }
  return [];
}

/**
 * Heuristic validation when js-yaml is not available.
 */
function validateWithHeuristic(filePath: string, fm: string): string[] {
  const errors: string[] = [];
  const descLine =
    fm.split("\n").find((l) => l.startsWith("description:")) ?? "";
  const val = descLine.replace(/^description:\s*/, "");
  if (
    !val.startsWith('"') &&
    !val.startsWith("'") &&
    /[A-Z][a-z]+: /.test(val)
  ) {
    errors.push(
      `${filePath}: description contains unquoted "Key: " pattern — wrap the value in double quotes`,
    );
  }
  for (const field of REQUIRED_FIELDS) {
    if (!new RegExp(`^${field}:`, "m").test(fm)) {
      errors.push(`${filePath}: frontmatter missing required field: ${field}`);
    }
  }
  return errors;
}

/**
 * Parse and validate frontmatter using js-yaml.
 */
async function validateWithYaml(
  filePath: string,
  fm: string,
  jsyamlPath: string,
): Promise<string[]> {
  let parsed: Frontmatter;
  try {
    const yaml = await import(jsyamlPath);
    const load = yaml.load ?? yaml.default?.load;
    parsed = load(fm) as Frontmatter;
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
 * Validate a single SKILL.md file.
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

  const repoRoot = resolve(import.meta.dir, "../../..");
  const jsyamlPath = join(repoRoot, "docs/node_modules/js-yaml/index.js");

  if (existsSync(jsyamlPath)) {
    return validateWithYaml(filePath, fm, jsyamlPath);
  }
  return validateWithHeuristic(filePath, fm);
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

  const allErrors: string[] = [];

  for (const file of targets) {
    const errors = await validateFile(file);
    if (errors.length > 0) {
      for (const err of errors) {
        logger.error(err);
      }
      allErrors.push(...errors);
    }
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
