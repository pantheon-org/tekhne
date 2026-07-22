#!/usr/bin/env bun
// Standalone skill-catalog generator (relocated from cli/lib/readme in B2a).
//
// Regenerates the README skill-catalog line, docs/src/content/docs/tiles.md,
// and the grade badge SVGs from ./skills — with no dependency on the TS cli/.
// Run: `bun scripts/catalog/update.ts [--dry-run]`.

import { logger } from "./logger";
import { updateReadme } from "./update-readme";

const dryRun = process.argv.includes("--dry-run");

try {
  await updateReadme({ dryRun });
} catch (error) {
  logger.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
