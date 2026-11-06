import { describe, expect, test } from "bun:test";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  compareTypes,
  extractPattern,
  parseTypes,
  TYPES_FILE,
  typesInPattern,
  WORKFLOW_FILE,
} from "./check-commit-types";

const SCRIPT = join(import.meta.dir, "check-commit-types.ts");
const ROOT = join(import.meta.dir, "..");

const realTypes = parseTypes(readFileSync(join(ROOT, TYPES_FILE), "utf8"));
const realPattern = extractPattern(
  readFileSync(join(ROOT, WORKFLOW_FILE), "utf8"),
) as string;

const workflowWith = (types: string) =>
  `run: |\n  pattern='^(${types})(\\([[:alnum:]]+\\))?!?: [[:lower:]]'\n`;

function runCli(types: string, workflow: string) {
  const dir = mkdtempSync(join(tmpdir(), "commit-types-"));
  const typesPath = join(dir, "types.txt");
  const workflowPath = join(dir, "workflow.yml");
  writeFileSync(typesPath, types);
  writeFileSync(workflowPath, workflow);
  const result = Bun.spawnSync(["bun", SCRIPT, typesPath, workflowPath]);
  return {
    code: result.exitCode,
    out: result.stdout.toString() + result.stderr.toString(),
  };
}

describe("parseTypes", () => {
  test("ignores comments and blank lines", () => {
    expect(parseTypes("# note\n\nfeat\n  fix  \n# fix2\n")).toEqual([
      "feat",
      "fix",
    ]);
  });
});

describe("extractPattern / typesInPattern", () => {
  test("pulls the alternation out of the pattern assignment", () => {
    const pattern = extractPattern(workflowWith("feat|fix"));
    expect(pattern).not.toBeNull();
    expect(typesInPattern(pattern as string)).toEqual(["feat", "fix"]);
  });

  test("returns null when there is no pattern", () => {
    expect(extractPattern("run: echo hi")).toBeNull();
  });

  test("returns null when the regex no longer starts with a type group", () => {
    expect(typesInPattern("^[a-z]+: ")).toBeNull();
  });
});

describe("compareTypes", () => {
  test("no drift", () => {
    expect(compareTypes(["a", "b"], ["b", "a"])).toEqual({
      missing: [],
      extra: [],
    });
  });

  test("reports both directions", () => {
    expect(compareTypes(["a", "b"], ["b", "c"])).toEqual({
      missing: ["a"],
      extra: ["c"],
    });
  });
});

describe("the CLI", () => {
  test("exits 0 when the lists agree", () => {
    const { code } = runCli("feat\nfix\n", workflowWith("feat|fix"));
    expect(code).toBe(0);
  });

  test("exits 1 and names the drifted types", () => {
    const { code, out } = runCli("feat\nfix\n", workflowWith("feat|security"));
    expect(code).toBe(1);
    expect(out).toContain("fix");
    expect(out).toContain("security");
  });

  test("exits 2 when the workflow has no pattern", () => {
    expect(runCli("feat\n", "run: echo hi").code).toBe(2);
  });

  test("exits 2 when a file is missing", () => {
    const result = Bun.spawnSync([
      "bun",
      SCRIPT,
      "/nonexistent",
      "/nonexistent",
    ]);
    expect(result.exitCode).toBe(2);
  });
});

describe("the repository's own files", () => {
  test("the gate and the canonical list agree today", () => {
    const copied = typesInPattern(realPattern);
    expect(copied).not.toBeNull();
    expect(compareTypes(realTypes, copied as string[])).toEqual({
      missing: [],
      extra: [],
    });
  });
});

/**
 * Runs the workflow's real bash regex, not a JavaScript translation of it: the
 * POSIX classes in it mean what bash says they mean.
 */
const accepts = (title: string) =>
  Bun.spawnSync(["bash", "-c", '[[ "$TITLE" =~ $PATTERN ]]'], {
    env: { ...process.env, TITLE: title, PATTERN: realPattern },
  }).exitCode === 0;

describe("the title regex", () => {
  const accepted = [
    // Real titles from this repository.
    "fix: stop the parity gates rotting on skill edits",
    "feat(context-index): add check-wave-evidence.sh sibling",
    "ci: give every job a timeout",
    // Dependabot.
    "chore(deps): bump @astrojs/mdx from 7.0.8 to 8.0.0 in /docs",
    "chore(deps-dev): bump tessl from 0.92.0 to 0.99.0",
    "ci(deps): bump the actions-minor-patch group across 1 directory with 3 updates",
    // Breaking changes.
    "feat!: drop the Go validator",
    "refactor(skill-auditor)!: rename the scoring entry point",
    // release-please's own pull request titles. If any of these fail, every
    // release pull request is blocked by the gate.
    "chore(main): release 1.2.3",
    "chore: release main",
    "chore(skills/agentic-harness/agents-md): release 1.0.1",
    // Every canonical type is accepted.
    ...realTypes.map((type) => `${type}: a lowercase summary`),
  ];

  const rejected = [
    "Add blocks/blocked-by dependency links", // a real merged title, no type
    "fix stop the parity gates rotting", // no colon
    "fix: Stop the parity gates rotting", // summary starts uppercase
    "FEAT: shouting", // type is not lowercase
    "security: patch the thing", // release-please ignores this type
    "bench: measure it", // not a release-please default
    "deps: bump a thing", // not a type
    "feat:", // empty summary
    "feat: ", // whitespace-only summary
    "feat: a summary ", // trailing space
    "feat(): empty scope",
    ": no type at all",
  ];

  test.each(accepted)("accepts %p", (title) => {
    expect(accepts(title)).toBe(true);
  });

  test.each(rejected)("rejects %p", (title) => {
    expect(accepts(title)).toBe(false);
  });
});
