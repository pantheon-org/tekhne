import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { resolveSkillDir } from "./resolve-skill-dir";

describe("resolveSkillDir", () => {
  test("returns tileDir unchanged when skillPath is SKILL.md", () => {
    expect(resolveSkillDir("skills/a/b", "SKILL.md")).toBe("skills/a/b");
  });

  test("appends subdirectory when skillPath is subdir/SKILL.md", () => {
    expect(resolveSkillDir("skills/a/b", "generator/SKILL.md")).toBe(
      join("skills/a/b", "generator"),
    );
  });

  test("handles bare subdirectory path without SKILL.md suffix", () => {
    expect(resolveSkillDir("skills/a/b", "validator")).toBe(
      join("skills/a/b", "validator"),
    );
  });
});
