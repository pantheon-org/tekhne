import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { buildTileSkills } from "./build-tile-skills";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "build-tile-skills-test-"));
});

afterEach(() => {
  rmSync(tmp, { recursive: true, force: true });
});

describe("buildTileSkills", () => {
  test("returns empty array when no skills defined", () => {
    expect(buildTileSkills(tmp, {})).toHaveLength(0);
  });

  test("excludes skill when SKILL.md does not exist", () => {
    const result = buildTileSkills(tmp, { "my-gen": {} });
    expect(result).toHaveLength(0);
  });

  test("includes skill when SKILL.md exists at default path", () => {
    writeFileSync(join(tmp, "SKILL.md"), "---\nname: x\n---\n");
    const result = buildTileSkills(tmp, { "my-gen": {} });
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe("my-gen");
  });

  test("resolves custom path for skill", () => {
    const subDir = join(tmp, "generator");
    mkdirSync(subDir);
    writeFileSync(join(subDir, "SKILL.md"), "---\nname: x\n---\n");
    const result = buildTileSkills(tmp, {
      "my-gen": { path: "generator/SKILL.md" },
    });
    expect(result).toHaveLength(1);
    expect(result[0].skillDir).toContain("generator");
  });
});
