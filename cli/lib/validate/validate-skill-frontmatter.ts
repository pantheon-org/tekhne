import { isAbsolute, resolve } from "node:path";
import { ValidationError } from "../utils/errors";
import { logger } from "../utils/logger";
import { collectSkillFiles } from "./collect-skill-files";
import { validateFile } from "./validate-file";

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
