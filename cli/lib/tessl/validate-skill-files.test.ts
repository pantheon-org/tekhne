import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { validateSkillFiles } from "./validate-skill-files";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "validate-skill-files-test-"));
});

afterEach(() => {
  rmSync(tmp, { recursive: true, force: true });
});

const writeTile = (dir: string, skills: Record<string, { path: string }>) => {
  writeFileSync(
    join(dir, "tile.json"),
    JSON.stringify({
      name: "org/my-tile",
      version: "1.0.0",
      summary: "A tile",
      skills,
    }),
  );
};

describe("validateSkillFiles", () => {
  test("returns true when no skills listed", async () => {
    writeTile(tmp, {});
    expect(await validateSkillFiles(tmp)).toBe(true);
  });

  test("returns true when all skill SKILL.md files exist", async () => {
    const skillDir = join(tmp, "my-skill");
    mkdirSync(skillDir);
    writeFileSync(join(skillDir, "SKILL.md"), "---\nname: x\n---\n");
    writeTile(tmp, { "my-skill": { path: "my-skill/SKILL.md" } });
    expect(await validateSkillFiles(tmp)).toBe(true);
  });

  test("returns false when a skill SKILL.md is missing", async () => {
    writeTile(tmp, { "missing-skill": { path: "missing-skill/SKILL.md" } });
    expect(await validateSkillFiles(tmp)).toBe(false);
  });
});
