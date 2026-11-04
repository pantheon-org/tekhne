import { describe, expect, test } from "bun:test";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  annotation,
  countCodeLines,
  findLongBlocks,
  parseArgs,
} from "./check-inline-scripts";

const SCRIPT = join(import.meta.dir, "check-inline-scripts.ts");

const workflow = (run: string, name = "Do the thing") =>
  `jobs:\n  a:\n    steps:\n      - name: ${name}\n        run: |\n${run
    .split("\n")
    .map((l) => `          ${l}`)
    .join("\n")}\n`;

const lines = (n: number) =>
  Array.from({ length: n }, (_, i) => `echo ${i}`).join("\n");

describe("countCodeLines", () => {
  test("ignores blank lines and comments", () => {
    expect(countCodeLines("# c\n\n  # indented\necho a\n\necho b")).toBe(2);
  });

  test("counts a trailing command after a comment-only first line", () => {
    expect(countCodeLines("set -e\n# note\nls")).toBe(2);
  });
});

describe("findLongBlocks", () => {
  test("a block at the limit passes", () => {
    expect(findLongBlocks(workflow(lines(15)), "w.yml")).toEqual([]);
  });

  test("a block over the limit is reported with its step name and size", () => {
    expect(findLongBlocks(workflow(lines(16)), "w.yml")).toEqual([
      { file: "w.yml", step: "Do the thing", lines: 16 },
    ]);
  });

  test("comments and blank lines do not push a block over", () => {
    const run = `${lines(15)}\n# one\n\n# two`;
    expect(findLongBlocks(workflow(run), "w.yml")).toEqual([]);
  });

  test("an inline-ok comment opts the block out", () => {
    const run = `# inline-ok: generated\n${lines(40)}`;
    expect(findLongBlocks(workflow(run), "w.yml")).toEqual([]);
  });

  test("an exempt block does not exempt its neighbour", () => {
    const text = `jobs:
  a:
    steps:
      - name: exempt
        run: |
          # inline-ok: because
${lines(20)
  .split("\n")
  .map((l) => `          ${l}`)
  .join("\n")}
      - name: long
        run: |
${lines(20)
  .split("\n")
  .map((l) => `          ${l}`)
  .join("\n")}
`;
    expect(findLongBlocks(text, "w.yml").map((f) => f.step)).toEqual(["long"]);
  });

  test("finds run blocks in a composite action", () => {
    const text = `runs:\n  using: composite\n  steps:\n    - shell: bash\n      run: |\n${lines(
      16,
    )
      .split("\n")
      .map((l) => `        ${l}`)
      .join("\n")}\n`;
    const found = findLongBlocks(text, "action.yml");
    expect(found).toHaveLength(1);
    expect(found[0].step).toBe("echo 0");
  });

  test("honours a custom limit", () => {
    expect(findLongBlocks(workflow(lines(6)), "w.yml", 5)).toHaveLength(1);
  });

  test("ignores a run value that is not a string", () => {
    expect(findLongBlocks("on:\n  push:\n    run: [1, 2]\n", "w.yml")).toEqual(
      [],
    );
  });
});

describe("annotation", () => {
  const f = { file: "w.yml", step: "s", lines: 20 };
  test("is a warning by default and an error when strict", () => {
    expect(annotation(f, 15, false)).toStartWith("::warning file=w.yml::");
    expect(annotation(f, 15, true)).toStartWith("::error file=w.yml::");
  });
});

describe("parseArgs", () => {
  test("reads flags and files", () => {
    expect(parseArgs(["--strict", "--max", "10", "a.yml"])).toEqual({
      strict: true,
      max: 10,
      files: ["a.yml"],
    });
  });

  test("rejects a bad --max", () => {
    expect(() => parseArgs(["--max", "x"])).toThrow();
  });
});

describe("cli", () => {
  const run = (args: string[], text: string) => {
    const dir = mkdtempSync(join(tmpdir(), "inline-"));
    const file = join(dir, "w.yml");
    writeFileSync(file, text);
    const proc = Bun.spawnSync(["bun", SCRIPT, ...args, file]);
    return { code: proc.exitCode, out: proc.stdout.toString() };
  };

  test("warn-only exits 0 and prints a warning", () => {
    const r = run([], workflow(lines(30)));
    expect(r.code).toBe(0);
    expect(r.out).toContain("::warning");
    expect(r.out).toContain("warn-only");
  });

  test("--strict exits 1 and prints an error", () => {
    const r = run(["--strict"], workflow(lines(30)));
    expect(r.code).toBe(1);
    expect(r.out).toContain("::error");
  });

  test("a clean file exits 0 even in --strict", () => {
    expect(run(["--strict"], workflow(lines(3))).code).toBe(0);
  });

  test("unparseable input exits 2, never a silent pass", () => {
    expect(run([], "a: [unclosed\n").code).toBe(2);
  });
});
