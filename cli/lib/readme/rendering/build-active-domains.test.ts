import { describe, expect, test } from "bun:test";
import type { SkillEntry, TileEntry } from "../types";
import { buildActiveDomains } from "./build-active-domains";

const makeTile = (domain: string): TileEntry => ({
  tileDir: `skills/${domain}/some-tool`,
  domain,
  shortName: "some-tool",
  fullName: `org/some-tool`,
  version: "1.0.0",
  summary: "A summary",
  isPublic: true,
  publishedStatus: "public",
  skills: [],
});

const makeSkill = (domain: string): SkillEntry => ({
  domain,
  relativePath: `${domain}/some-skill`,
});

describe("buildActiveDomains", () => {
  test("returns empty array when no tiles or skills match any domain", () => {
    const result = buildActiveDomains([], []);
    expect(result).toEqual([]);
  });

  test("includes only domains that have tiles or skills", () => {
    const tiles = [makeTile("ci-cd")];
    const result = buildActiveDomains(tiles, []);
    expect(result).toHaveLength(1);
    expect(result[0].heading).toContain("CI/CD");
  });

  test("heading includes count label for tiles only", () => {
    const tiles = [makeTile("infrastructure"), makeTile("infrastructure")];
    const result = buildActiveDomains(tiles, []);
    expect(result[0].heading).toContain("2 tiles");
  });

  test("heading includes count label for skills only", () => {
    const skills = [makeSkill("documentation")];
    const result = buildActiveDomains([], skills);
    expect(result[0].heading).toContain("1 skill");
  });

  test("heading includes combined count when both tiles and skills exist", () => {
    const tiles = [makeTile("development")];
    const skills = [makeSkill("development")];
    const result = buildActiveDomains(tiles, skills);
    expect(result[0].heading).toContain("1 tile, 1 skill");
  });

  test("domain description is populated from DOMAINS config", () => {
    const tiles = [makeTile("testing")];
    const result = buildActiveDomains(tiles, []);
    expect(result[0].description).toBe("Testing methodologies & quality");
  });

  test("tiles and untiledSkills are correctly assigned to domain entry", () => {
    const tiles = [makeTile("ci-cd")];
    const skills = [makeSkill("ci-cd")];
    const result = buildActiveDomains(tiles, skills);
    expect(result[0].tiles).toHaveLength(1);
    expect(result[0].untiledSkills).toHaveLength(1);
  });

  test("preserves domain order from DOMAINS config", () => {
    const tiles = [makeTile("development"), makeTile("ci-cd")];
    const result = buildActiveDomains(tiles, []);
    // ci-cd comes before development in DOMAINS config
    expect(result[0].heading).toContain("CI/CD");
    expect(result[1].heading).toContain("Development");
  });
});
