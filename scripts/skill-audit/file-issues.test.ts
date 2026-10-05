import { describe, expect, test } from "bun:test";
import {
  chmodSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const SCRIPT = join(import.meta.dir, "file-issues.sh");

type Audit = {
  skill: string;
  grade: string;
  total: number;
  tier: "pass" | "warn" | "debt" | "fail";
};

const dims = {
  knowledgeDelta: 18,
  mindsetProcedures: 11,
  antiPatternQuality: 7,
  specificationCompliance: 13,
  progressiveDisclosure: 10,
  freedomCalibration: 12,
  patternRecognition: 10,
  practicalUsability: 15,
  evalValidation: 8,
};

const makeDir = (audits: Audit[]): string => {
  const dir = mkdtempSync(join(tmpdir(), "skill-audit-issues-"));
  writeFileSync(
    join(dir, "verdict.json"),
    JSON.stringify(
      audits.map((a) => ({ ...a, maxTotal: 140, dimensions: dims })),
    ),
  );
  return dir;
};

// A fake gh that logs every call, returns canned JSON for `issue list` and
// `issue view`, and "creates" issue 999.
const fakeGh = (
  dir: string,
  list = "[]",
  view = '{"body":"","comments":[]}',
) => {
  const bin = join(dir, "bin");
  mkdirSync(bin, { recursive: true });
  writeFileSync(join(dir, "list.json"), list);
  writeFileSync(join(dir, "view.json"), view);
  writeFileSync(
    join(bin, "gh"),
    `#!/usr/bin/env bash
echo "$@" >> "${dir}/calls.log"
case "$1 $2" in
  "issue list") cat "${dir}/list.json" ;;
  "issue view") cat "${dir}/view.json" ;;
  "issue create")
    while [ $# -gt 0 ]; do
      case "$1" in
        --title) printf '%s' "$2" > "${dir}/created-title.txt"; shift ;;
        --body-file) cp "$2" "${dir}/created-body.md"; shift ;;
      esac
      shift
    done
    echo "https://github.com/o/r/issues/999" ;;
  "issue comment")
    while [ $# -gt 0 ]; do
      case "$1" in --body) printf '%s' "$2" > "${dir}/comment-body.txt"; shift ;; esac
      shift
    done ;;
esac
`,
  );
  chmodSync(join(bin, "gh"), 0o755);
};

const run = (dir: string, extra: Record<string, string> = {}) =>
  Bun.spawnSync(["bash", SCRIPT, join(dir, "verdict.json")], {
    env: {
      PATH: `${join(dir, "bin")}:${process.env.PATH ?? ""}`,
      GITHUB_REPOSITORY: "o/r",
      PR_NUMBER: "42",
      RUN_URL: "https://github.com/o/r/actions/runs/7",
      TRACKED_FILE: join(dir, "tracked.md"),
      ...extra,
    },
  });

const calls = (dir: string): string =>
  existsSync(join(dir, "calls.log"))
    ? readFileSync(join(dir, "calls.log"), "utf8")
    : "";

const tracked = (dir: string): string =>
  existsSync(join(dir, "tracked.md"))
    ? readFileSync(join(dir, "tracked.md"), "utf8")
    : "";

const below: Audit = {
  skill: "software-engineering/bridge",
  grade: "C",
  total: 100,
  tier: "debt",
};
const fine: Audit = {
  skill: "agentic-harness/pin",
  grade: "B+",
  total: 120,
  tier: "warn",
};

describe("file-issues.sh", () => {
  test("does nothing when no skill is known debt", () => {
    const dir = makeDir([fine]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(calls(dir)).not.toContain("issue create");
    expect(tracked(dir)).toBe("");
  });

  test("files nothing for a skill that blocks the pull request", () => {
    const dir = makeDir([{ ...below, tier: "fail" }]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(calls(dir)).not.toContain("issue create");
    expect(tracked(dir)).toBe("");
  });

  test("files one issue for a skill below B and links the PR", () => {
    const dir = makeDir([below, fine]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    const created = calls(dir)
      .split("\n")
      .filter((l) => l.startsWith("issue create"));
    expect(created.length).toBe(1);
    expect(readFileSync(join(dir, "created-title.txt"), "utf8")).toBe(
      "Raise software-engineering/bridge to grade B (100/140, grade C)",
    );
    const body = readFileSync(join(dir, "created-body.md"), "utf8");
    expect(body).toContain(
      "<!-- skill-audit-below-b: software-engineering/bridge -->",
    );
    expect(body).toContain("#42");
    expect(body).toContain("https://github.com/o/r/actions/runs/7");
    expect(body).toContain("D3 Anti-Pattern Quality");
    expect(body).toContain("did not lower");
    expect(tracked(dir)).toContain("`software-engineering/bridge`: #999");
  });

  test("reuses an open issue found by its marker and comments once on the PR", () => {
    const dir = makeDir([below]);
    fakeGh(
      dir,
      JSON.stringify([
        {
          number: 12,
          title: "Anything",
          body: "x\n<!-- skill-audit-below-b: software-engineering/bridge -->",
        },
      ]),
    );
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(calls(dir)).not.toContain("issue create");
    expect(calls(dir)).toContain("issue comment 12");
    expect(readFileSync(join(dir, "comment-body.txt"), "utf8")).toContain(
      "#42",
    );
    expect(tracked(dir)).toContain("`software-engineering/bridge`: #12");
  });

  test("reuses an issue filed by hand, matched by its title", () => {
    const dir = makeDir([below]);
    fakeGh(
      dir,
      JSON.stringify([
        {
          number: 371,
          title:
            "Raise software-engineering/bridge to grade B (100/140, grade C)",
          body: "no marker here",
        },
      ]),
    );
    run(dir);
    expect(calls(dir)).not.toContain("issue create");
    expect(tracked(dir)).toContain("#371");
  });

  test("does not match a different skill whose name starts the same way", () => {
    const dir = makeDir([below]);
    fakeGh(
      dir,
      JSON.stringify([
        {
          number: 5,
          title:
            "Raise software-engineering/bridge-extra to grade B (1/140, grade C)",
          body: "",
        },
      ]),
    );
    run(dir);
    expect(calls(dir)).toContain("issue create");
  });

  test("does not comment again when the issue already mentions the PR", () => {
    const dir = makeDir([below]);
    fakeGh(
      dir,
      JSON.stringify([
        {
          number: 12,
          title: "x",
          body: "<!-- skill-audit-below-b: software-engineering/bridge -->",
        },
      ]),
      JSON.stringify({ body: "", comments: [{ body: "Also blocking #42" }] }),
    );
    run(dir);
    expect(calls(dir)).not.toContain("issue comment");
    expect(tracked(dir)).toContain("#12");
  });

  test("treats #4 as different from #42 when checking for an existing link", () => {
    const dir = makeDir([below]);
    fakeGh(
      dir,
      JSON.stringify([
        {
          number: 12,
          title: "x",
          body: "<!-- skill-audit-below-b: software-engineering/bridge -->",
        },
      ]),
      JSON.stringify({ body: "", comments: [{ body: "Also blocking #421" }] }),
    );
    run(dir);
    expect(calls(dir)).toContain("issue comment 12");
  });

  test("skips a skill name that is not a plain path", () => {
    const dir = makeDir([
      { skill: "x/$(rm -rf)", grade: "C", total: 100, tier: "debt" },
      below,
    ]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    const created = calls(dir)
      .split("\n")
      .filter((l) => l.startsWith("issue create"));
    expect(created.length).toBe(1);
    expect(tracked(dir)).not.toContain("rm -rf");
  });

  test("DRY_RUN lists what it would do without creating or commenting", () => {
    const dir = makeDir([below]);
    fakeGh(dir);
    const r = run(dir, { DRY_RUN: "1" });
    expect(r.exitCode).toBe(0);
    expect(calls(dir)).not.toContain("issue create");
    expect(calls(dir)).not.toContain("issue comment");
    expect(r.stdout.toString()).toContain("would create");
  });

  test("keeps going when one issue cannot be created, then exits non-zero", () => {
    const dir = makeDir([
      below,
      {
        skill: "agentic-harness/pick-model",
        grade: "C",
        total: 100,
        tier: "debt",
      },
    ]);
    fakeGh(dir);
    // Make the first create fail by pointing gh at a failing variant.
    writeFileSync(
      join(dir, "bin", "gh"),
      `#!/usr/bin/env bash
echo "$@" >> "${dir}/calls.log"
case "$1 $2" in
  "issue list") echo "[]" ;;
  "issue create") echo "boom" >&2; exit 1 ;;
esac
`,
    );
    const r = run(dir);
    expect(r.exitCode).not.toBe(0);
    const attempts = calls(dir)
      .split("\n")
      .filter((l) => l.startsWith("issue create"));
    expect(attempts.length).toBe(2);
  });
});
