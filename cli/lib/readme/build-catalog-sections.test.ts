import { describe, expect, test } from "bun:test";
import { buildCatalogSections } from "./build-catalog-sections";
import type { CatalogDomain } from "./readme-types";

const makeDomain = (
  heading: string,
  description: string,
  tiles: CatalogDomain["tiles"] = [],
  untiledSkills: CatalogDomain["untiledSkills"] = [],
): CatalogDomain => ({
  heading,
  description,
  tiles,
  untiledSkills,
});

describe("buildCatalogSections", () => {
  test("returns empty string when no domains provided", async () => {
    const result = await buildCatalogSections([]);
    expect(result).toBe("");
  });

  test("includes domain heading as H2", async () => {
    const domain = makeDomain("CI/CD (0 tiles)", "CI/CD description");
    const result = await buildCatalogSections([domain]);
    expect(result).toContain("## CI/CD (0 tiles)");
  });

  test("includes domain description", async () => {
    const domain = makeDomain("Testing (0 tiles)", "Testing methodologies");
    const result = await buildCatalogSections([domain]);
    expect(result).toContain("Testing methodologies");
  });

  test("produces sections for multiple domains", async () => {
    const d1 = makeDomain("CI/CD (0 tiles)", "CI desc");
    const d2 = makeDomain("Infrastructure (0 tiles)", "Infra desc");
    const result = await buildCatalogSections([d1, d2]);
    expect(result).toContain("## CI/CD");
    expect(result).toContain("## Infrastructure");
  });

  test("domain with tiles generates tile section content", async () => {
    const tile = {
      tileDir: "skills/ci-cd/github-actions",
      domain: "ci-cd",
      shortName: "github-actions",
      fullName: "org/github-actions",
      version: "1.0.0",
      summary: "GitHub Actions tile",
      isPublic: true,
      publishedStatus: "public" as const,
      skills: [],
    };
    const domain = makeDomain("CI/CD (1 tile)", "CI desc", [tile]);
    const result = await buildCatalogSections([domain]);
    // generateTileSection should produce the tile name
    expect(result).toContain("github-actions");
  });
});
