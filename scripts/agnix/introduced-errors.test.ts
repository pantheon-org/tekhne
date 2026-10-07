import { describe, expect, test } from "bun:test";
import { existsSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const SCRIPT = join(import.meta.dir, "introduced-errors.sh");

type Diagnostic = {
  level: "error" | "warning";
  rule: string;
  file: string;
  line: number;
  message: string;
};

const err = (rule: string, file: string, line = 1): Diagnostic => ({
  level: "error",
  rule,
  file,
  line,
  message: "m",
});

const warning = (rule: string, file: string): Diagnostic => ({
  level: "warning",
  rule,
  file,
  line: 1,
  message: "m",
});

const json = (diagnostics: Diagnostic[], filesChecked = 2146): string =>
  JSON.stringify({
    version: "0.56.6",
    files_checked: filesChecked,
    diagnostics,
    summary: {
      errors: diagnostics.filter((d) => d.level === "error").length,
      warnings: diagnostics.filter((d) => d.level === "warning").length,
      info: 0,
    },
  });

const setup = (head: string, base: string): string => {
  const dir = mkdtempSync(join(tmpdir(), "agnix-gate-"));
  writeFileSync(join(dir, "head.json"), head);
  writeFileSync(join(dir, "base.json"), base);
  return dir;
};

const run = (dir: string) =>
  Bun.spawnSync(
    ["bash", SCRIPT, join(dir, "head.json"), join(dir, "base.json")],
    {
      env: {
        PATH: process.env.PATH ?? "",
        INTRODUCED_FILE: join(dir, "introduced.txt"),
      },
    },
  );

const introduced = (dir: string): string =>
  existsSync(join(dir, "introduced.txt"))
    ? readFileSync(join(dir, "introduced.txt"), "utf8")
    : "";

const star = "skills/agentic-harness/professional-honesty/SKILL.md";

describe("agnix introduced-errors.sh", () => {
  test("passes when the head has the same errors as the base", () => {
    const dir = setup(
      json([err("CC-SK-008", star)]),
      json([err("CC-SK-008", star)]),
    );
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(introduced(dir)).toBe("");
  });

  test("fails and lists an error that is only in the head", () => {
    const dir = setup(
      json([err("CC-SK-008", star), err("CC-SK-002", "skills/a/SKILL.md")]),
      json([err("CC-SK-008", star)]),
    );
    const r = run(dir);
    expect(r.exitCode).toBe(1);
    expect(introduced(dir)).toBe("CC-SK-002\tskills/a/SKILL.md\n");
  });

  test("passes when an error was fixed", () => {
    const dir = setup(json([]), json([err("CC-SK-008", star)]));
    expect(run(dir).exitCode).toBe(0);
    expect(introduced(dir)).toBe("");
  });

  test("ignores warnings", () => {
    const dir = setup(json([warning("CC-SK-017", star)]), json([]));
    expect(run(dir).exitCode).toBe(0);
    expect(introduced(dir)).toBe("");
  });

  test("compares an absolute runner path with a relative one", () => {
    const dir = setup(
      json([err("CC-SK-008", `/home/runner/work/r/r/${star}`)]),
      json([err("CC-SK-008", star)]),
    );
    expect(run(dir).exitCode).toBe(0);
  });

  test("does not count a further instance of an error the base already has", () => {
    const dir = setup(
      json([err("CC-SK-008", star, 4), err("CC-SK-008", star, 9)]),
      json([err("CC-SK-008", star, 4)]),
    );
    expect(run(dir).exitCode).toBe(0);
  });

  test("does not fail, and says so, when the base output is unusable", () => {
    const dir = setup(json([err("CC-SK-008", star)]), "not json");
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(r.stdout.toString()).toContain("::warning::");
    expect(introduced(dir)).toBe("");
  });

  test("does not fail, and says so, when the head output is unusable", () => {
    const dir = setup("not json", json([]));
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(r.stdout.toString()).toContain("::warning::");
  });
});
