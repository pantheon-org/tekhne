import { writeFileSync } from "node:fs";
import { $ } from "bun";
import { logger } from "../utils/logger";

export const showDryRunDiff = async (
  readmePath: string,
  newContent: string,
): Promise<void> => {
  logger.info("=== DRY RUN - Changes that would be made ===\n");

  const tmpFile = "/tmp/readme-new.md";
  writeFileSync(tmpFile, newContent);

  try {
    const diff = await $`diff -u ${readmePath} ${tmpFile}`.text();
    console.log(diff);
  } catch (error: unknown) {
    const err = error as { stdout?: Buffer };
    if (err.stdout) {
      console.log(err.stdout.toString());
    }
  }

  logger.info("\nTo apply changes, run without --dry-run");
};
