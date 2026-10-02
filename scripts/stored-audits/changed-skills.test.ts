import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const SCRIPT = join(import.meta.dir, "changed-skills.sh");

const makeRepo = (skills: string[]): string => {
  const root = mkdtempSync(join(tmpdir(), "changed-skills-"));
  for (const s of skills) {
    mkdirSync(join(root, "skills", s), { recursive: true });
    writeFileSync(join(root, "skills", s, "SKILL.md"), "# skill\n");
  }
  return root;
};

const run = (
  root: string,
  paths: string[],
): { code: number; out: string[] } => {
  const proc = Bun.spawnSync(["bash", SCRIPT, root], {
    stdin: new TextEncoder().encode(paths.join("\n") + "\n"),
  });
  const text = proc.stdout.toString();
  return { code: proc.exitCode ?? -1, out: text.split("\n").filter(Boolean) };
};

describe("changed-skills.sh", () => {
  test("maps any file inside a skill to that skill's key", () => {
    const root = makeRepo(["ci-cd/helm/validator"]);
    const { code, out } = run(root, [
      "skills/ci-cd/helm/validator/SKILL.md",
      "skills/ci-cd/helm/validator/evals/scenario-1/task.md",
      "skills/ci-cd/helm/validator/references/guide.md",
      "skills/ci-cd/helm/validator/scripts/run.sh",
    ]);
    expect(code).toBe(0);
    expect(out).toEqual(["ci-cd/helm/validator"]);
  });

  test("ignores stored audit files, so storing an audit does not retrigger itself", () => {
    const root = makeRepo(["ci-cd/helm/validator"]);
    const { out } = run(root, [
      "skills/ci-cd/helm/validator/.audits/2026-10-02/audit.json",
      "skills/ci-cd/helm/validator/.audits/2026-10-02/Analysis.md",
    ]);
    expect(out).toEqual([]);
  });

  test("a change to a skill plus its audit still reports the skill once", () => {
    const root = makeRepo(["a/one"]);
    const { out } = run(root, [
      "skills/a/one/evals/x.md",
      "skills/a/one/.audits/2026-10-02/audit.json",
    ]);
    expect(out).toEqual(["a/one"]);
  });

  test("ignores paths outside skills/", () => {
    const root = makeRepo(["a/one"]);
    const { out } = run(root, [
      "README.md",
      "crates/skill-auditor/src/main.rs",
      "docs/src/content/docs/tiles.md",
    ]);
    expect(out).toEqual([]);
  });

  test("de-duplicates and sorts", () => {
    const root = makeRepo(["b/two", "a/one"]);
    const { out } = run(root, [
      "skills/b/two/SKILL.md",
      "skills/a/one/SKILL.md",
      "skills/b/two/evals/x.md",
    ]);
    expect(out).toEqual(["a/one", "b/two"]);
  });

  test("skips a path whose skill no longer exists (deleted skill)", () => {
    const root = makeRepo(["a/one"]);
    const { out } = run(root, [
      "skills/gone/skill/SKILL.md",
      "skills/gone/skill/evals/x.md",
    ]);
    expect(out).toEqual([]);
  });

  test("handles a skill nested below a folder that is not itself a skill", () => {
    const root = makeRepo([
      "documentation/astro-starlight/skills/starlight-base",
    ]);
    const { out } = run(root, [
      "skills/documentation/astro-starlight/skills/starlight-base/evals/scenario-1/task.md",
    ]);
    expect(out).toEqual([
      "documentation/astro-starlight/skills/starlight-base",
    ]);
  });

  test("picks the nearest SKILL.md when folders nest", () => {
    const root = makeRepo(["parent", "parent/child"]);
    const { out } = run(root, [
      "skills/parent/child/evals/x.md",
      "skills/parent/notes.md",
    ]);
    expect(out).toEqual(["parent", "parent/child"]);
  });

  test("empty input is not an error", () => {
    const root = makeRepo(["a/one"]);
    const proc = Bun.spawnSync(["bash", SCRIPT, root], {
      stdin: new TextEncoder().encode(""),
    });
    expect(proc.exitCode).toBe(0);
    expect(proc.stdout.toString()).toBe("");
  });
});
