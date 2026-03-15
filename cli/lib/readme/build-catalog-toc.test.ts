import { describe, expect, test } from "bun:test";
import { buildCatalogToc } from "./build-catalog-toc";
import type { CatalogDomain } from "./readme-types";
import type { TileEntry } from "./tile-types";

const makeTile = (shortName: string): TileEntry => ({
  tileDir: `skills/ci-cd/${shortName}`,
  domain: "ci-cd",
  shortName,
  fullName: `org/${shortName}`,
  version: "1.0.0",
  summary: "A summary",
  isPublic: true,
  publishedStatus: "public",
  skills: [],
});

const makeDomain = (
  heading: string,
  tiles: TileEntry[] = [],
  untiledSkills: { domain: string; relativePath: string }[] = [],
): CatalogDomain => ({
  heading,
  description: "A description",
  tiles,
  untiledSkills,
});

describe("buildCatalogToc", () => {
  test("returns ToC header line for empty domains list", () => {
    const toc = buildCatalogToc([]);
    expect(toc).toBe("## Contents\n\n");
  });

  test("includes domain heading anchor link", () => {
    const domain = makeDomain("CI/CD (2 tiles)");
    const toc = buildCatalogToc([domain]);
    expect(toc).toContain("- [CI/CD (2 tiles)]");
    expect(toc).toContain("#");
  });

  test("includes tile sub-links under domain", () => {
    const tile = makeTile("github-actions");
    const domain = makeDomain("CI/CD (1 tile)", [tile]);
    const toc = buildCatalogToc([domain]);
    expect(toc).toContain("  - [github-actions]");
  });

  test("includes untiled skill sub-links with (no tile) annotation", () => {
    const skill = { domain: "ci-cd", relativePath: "ci-cd/my-skill" };
    const domain = makeDomain("CI/CD (1 skill)", [], [skill]);
    const toc = buildCatalogToc([domain]);
    expect(toc).toContain("_(no tile)_");
    expect(toc).toContain("-no-tile");
  });

  test("produces multiple domain entries in order", () => {
    const d1 = makeDomain("CI/CD (1 tile)", [makeTile("github-actions")]);
    const d2 = makeDomain("Infrastructure (1 tile)", [makeTile("terraform")]);
    const toc = buildCatalogToc([d1, d2]);
    const idx1 = toc.indexOf("CI/CD");
    const idx2 = toc.indexOf("Infrastructure");
    expect(idx1).toBeLessThan(idx2);
  });
});
