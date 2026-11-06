#!/usr/bin/env bun
/**
 * check-commit-types.ts
 *
 * Fails when the type list copied into the PR title gate's regex has drifted
 * from .github/commit-types.txt, the canonical list.
 *
 * The gate runs on pull_request_target and so must not read repository files:
 * the list is copied into its regex instead, and this is what keeps the copy
 * honest. Two lists that must agree is how a version string drifted before.
 *
 * Usage: bun scripts/check-commit-types.ts [types-file] [workflow-file]
 * Exit:  0 in step; 1 drift; 2 unreadable or unparseable input.
 */
import { readFileSync } from "node:fs";

export const TYPES_FILE = ".github/commit-types.txt";
export const WORKFLOW_FILE = ".github/workflows/conventional-commits.yml";

/** The canonical types: one per line, blank lines and comments ignored. */
export function parseTypes(text: string): string[] {
  return text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line !== "" && !line.startsWith("#"));
}

/** The regex held in the workflow's `pattern='...'` assignment, or null. */
export function extractPattern(workflow: string): string | null {
  return /pattern='([^']+)'/.exec(workflow)?.[1] ?? null;
}

/** The type alternation at the start of the title regex, `^(a|b|c)`, or null. */
export function typesInPattern(pattern: string): string[] | null {
  const group = /^\^\(([a-z|]+)\)/.exec(pattern)?.[1];
  return group === undefined ? null : group.split("|");
}

export type Drift = {
  /** In the canonical list but not accepted by the regex. */
  missing: string[];
  /** Accepted by the regex but not in the canonical list. */
  extra: string[];
};

export function compareTypes(canonical: string[], copied: string[]): Drift {
  return {
    missing: canonical.filter((t) => !copied.includes(t)),
    extra: copied.filter((t) => !canonical.includes(t)),
  };
}

function main(argv: string[]): number {
  const typesFile = argv[0] ?? TYPES_FILE;
  const workflowFile = argv[1] ?? WORKFLOW_FILE;

  let canonical: string[];
  let copied: string[] | null;
  try {
    canonical = parseTypes(readFileSync(typesFile, "utf8"));
    const pattern = extractPattern(readFileSync(workflowFile, "utf8"));
    copied = pattern === null ? null : typesInPattern(pattern);
  } catch (error) {
    console.error(`check-commit-types: ${(error as Error).message}`);
    return 2;
  }

  if (copied === null) {
    console.error(
      `check-commit-types: no type list found in ${workflowFile}. The regex shape changed; update this script with it.`,
    );
    return 2;
  }

  const { missing, extra } = compareTypes(canonical, copied);
  if (missing.length === 0 && extra.length === 0) {
    console.log(`check-commit-types OK: ${canonical.join(" ")}`);
    return 0;
  }
  console.error(
    `check-commit-types: the PR title gate has drifted from ${typesFile}`,
  );
  if (missing.length > 0) {
    console.error(`  rejected by the gate but canonical: ${missing.join(" ")}`);
  }
  if (extra.length > 0) {
    console.error(
      `  accepted by the gate but not canonical: ${extra.join(" ")}`,
    );
  }
  console.error("  Update whichever is wrong. Do not silence this.");
  return 1;
}

if (import.meta.main) {
  process.exit(main(process.argv.slice(2)));
}
