import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

const SCRIPT = join(import.meta.dir, "check-no-cache-files.sh");

// Only the variables the script needs are passed to the child process.
const childEnv = { PATH: Bun.env.PATH ?? "/usr/bin:/bin" };

// A throwaway repo root where the named files exist on disk.
const makeRoot = (files: string[]): string => {
  const root = mkdtempSync(join(tmpdir(), "no-cache-"));
  for (const f of files) {
    mkdirSync(join(root, dirname(f)), { recursive: true });
    writeFileSync(join(root, f), "x\n");
  }
  return root;
};

const run = (root: string, paths: string[]) => {
  const proc = Bun.spawnSync(["bash", SCRIPT, "--root", root, ...paths], {
    env: childEnv,
  });
  return {
    code: proc.exitCode ?? -1,
    out: proc.stdout.toString() + proc.stderr.toString(),
  };
};

const runTracked = (root: string) => {
  Bun.spawnSync(["git", "init", "-q"], { cwd: root });
  Bun.spawnSync(["git", "add", "-A", "-f"], { cwd: root });
  const proc = Bun.spawnSync(["bash", SCRIPT, "--root", root, "--tracked"], {
    env: childEnv,
  });
  return { code: proc.exitCode ?? -1, out: proc.stdout.toString() };
};

describe("check-no-cache-files.sh", () => {
  test("passes ordinary files", () => {
    const paths = ["skills/a/SKILL.md", "scripts/x.sh", "README.md"];
    const r = run(makeRoot(paths), paths);
    expect(r.code).toBe(0);
    expect(r.out).toBe("");
  });

  test("rejects a terragrunt cache file and names it", () => {
    const p = "skills/i/tg/test/dev/vpc/.terragrunt-cache/abc/def/main.tf";
    const r = run(makeRoot([p]), [p]);
    expect(r.code).toBe(1);
    expect(r.out).toContain(".terragrunt-cache");
  });

  for (const dir of [
    ".terragrunt-cache",
    ".terraform",
    "__pycache__",
    ".pytest_cache",
    "node_modules",
    ".cache",
    ".venv",
  ]) {
    test(`rejects a file inside ${dir}/`, () => {
      const p = `skills/a/b/${dir}/inner/file.txt`;
      expect(run(makeRoot([p]), [p]).code).toBe(1);
    });
  }

  test("matches whole folder names only, so a committed lock file is allowed", () => {
    const paths = [
      "skills/i/tf/.terraform.lock.hcl",
      "docs/terragrunt-cache-notes.md",
      "skills/a/cache/readme.md",
      "skills/a/my.cache.json",
    ];
    expect(run(makeRoot(paths), paths).code).toBe(0);
  });

  test("ignores a path that no longer exists, so deleting cache files is not blocked", () => {
    const root = makeRoot(["README.md"]);
    expect(run(root, ["skills/i/tg/.terragrunt-cache/abc/main.tf"]).code).toBe(
      0,
    );
  });

  test("exempts the golden-corpus test fixtures", () => {
    const p =
      "crates/skill-validator-rs/tests/golden-corpus/fixtures/x/node_modules/y.js";
    expect(run(makeRoot([p]), [p]).code).toBe(0);
  });

  test("prints the folder once and the fix", () => {
    const paths = [
      "skills/i/tg/.terragrunt-cache/a/1.tf",
      "skills/i/tg/.terragrunt-cache/a/2.tf",
      "skills/i/tg/.terragrunt-cache/b/3.tf",
    ];
    const r = run(makeRoot(paths), paths);
    expect(r.code).toBe(1);
    const lines = r.out
      .split("\n")
      .filter((l) => l.includes("skills/i/tg/.terragrunt-cache"));
    expect(lines.length).toBe(1);
    expect(r.out).toContain("--cached");
    expect(r.out).toContain(".gitignore");
  });

  test("--tracked checks every file git tracks", () => {
    const bad = runTracked(
      makeRoot(["README.md", "skills/i/tg/.terragrunt-cache/a/1.tf"]),
    );
    expect(bad.code).toBe(1);
    const good = runTracked(makeRoot(["README.md"]));
    expect(good.code).toBe(0);
  });
});
