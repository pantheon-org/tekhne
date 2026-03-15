import { describe, expect, test } from "bun:test";
import { findUntiledSkills } from "./find-untiled-skills";

const makeTile = (skillDirs: string[]) => ({
  tileDir: "skills/a",
  domain: "a",
  shortName: "tile",
  fullName: "org/tile",
  version: "0.1.0",
  summary: "",
  isPublic: false,
  publishedStatus: "unpublished" as const,
  skills: skillDirs.map((dir) => ({
    name: "s",
    skillDir: dir,
    auditRelPath: dir,
  })),
});

describe("findUntiledSkills", () => {
  test("returns all skills when no tiles", () => {
    const skills = [
      { domain: "dev", relativePath: "dev/bun" },
      { domain: "ci", relativePath: "ci/gh" },
    ];
    expect(findUntiledSkills(skills, [])).toHaveLength(2);
  });

  test("excludes skills covered by a tile", () => {
    const skills = [
      { domain: "dev", relativePath: "dev/bun" },
      { domain: "ci", relativePath: "ci/gh" },
    ];
    const tiles = [makeTile(["skills/dev/bun"])];
    const result = findUntiledSkills(skills, tiles);
    expect(result).toHaveLength(1);
    expect(result[0].relativePath).toBe("ci/gh");
  });

  test("returns empty array when all skills are tiled", () => {
    const skills = [{ domain: "dev", relativePath: "dev/bun" }];
    const tiles = [makeTile(["skills/dev/bun"])];
    expect(findUntiledSkills(skills, tiles)).toHaveLength(0);
  });
});
