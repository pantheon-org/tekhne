import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { collectSkillFiles } from "./collect-skill-files";

describe("collectSkillFiles", () => {
  let tmpDir: string;

  beforeEach(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "tekhne-collect-test-"));
  });

  afterEach(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("returns empty array when no SKILL.md files exist", async () => {
    mkdirSync(join(tmpDir, "skills"), { recursive: true });
    const result = await collectSkillFiles(tmpDir);
    expect(result).toEqual([]);
  });

  test("returns empty array when skills directory does not exist", async () => {
    const result = await collectSkillFiles(tmpDir);
    expect(result).toEqual([]);
  });

  test("returns absolute paths for SKILL.md files", async () => {
    mkdirSync(join(tmpDir, "skills", "ci-cd", "github-actions"), {
      recursive: true,
    });
    writeFileSync(
      join(tmpDir, "skills", "ci-cd", "github-actions", "SKILL.md"),
      "---\nname: Test\n---\n",
    );

    const result = await collectSkillFiles(tmpDir);
    expect(result).toHaveLength(1);
    expect(result[0]).toBe(
      join(tmpDir, "skills", "ci-cd", "github-actions", "SKILL.md"),
    );
  });

  test("returns sorted paths when multiple SKILL.md files exist", async () => {
    mkdirSync(join(tmpDir, "skills", "z-domain", "skill"), { recursive: true });
    mkdirSync(join(tmpDir, "skills", "a-domain", "skill"), { recursive: true });
    mkdirSync(join(tmpDir, "skills", "m-domain", "skill"), { recursive: true });
    writeFileSync(join(tmpDir, "skills", "z-domain", "skill", "SKILL.md"), "");
    writeFileSync(join(tmpDir, "skills", "a-domain", "skill", "SKILL.md"), "");
    writeFileSync(join(tmpDir, "skills", "m-domain", "skill", "SKILL.md"), "");

    const result = await collectSkillFiles(tmpDir);
    expect(result).toHaveLength(3);
    // Must be sorted
    expect(result).toEqual([...result].sort());
  });

  test("does not include non-SKILL.md files", async () => {
    mkdirSync(join(tmpDir, "skills", "ci-cd", "github-actions"), {
      recursive: true,
    });
    writeFileSync(
      join(tmpDir, "skills", "ci-cd", "github-actions", "SKILL.md"),
      "",
    );
    writeFileSync(
      join(tmpDir, "skills", "ci-cd", "github-actions", "README.md"),
      "",
    );
    writeFileSync(
      join(tmpDir, "skills", "ci-cd", "github-actions", "notes.txt"),
      "",
    );

    const result = await collectSkillFiles(tmpDir);
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("SKILL.md");
  });

  test("matches SKILL.md at any depth under skills/", async () => {
    mkdirSync(join(tmpDir, "skills", "domain", "tool", "generator"), {
      recursive: true,
    });
    writeFileSync(
      join(tmpDir, "skills", "domain", "tool", "generator", "SKILL.md"),
      "",
    );

    const result = await collectSkillFiles(tmpDir);
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("generator");
  });
});
