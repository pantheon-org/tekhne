import { describe, expect, test } from "bun:test";
import type { TileEntry } from "../types";
import { generateReadmeSummaryTables } from "./generate-readme-summary-tables";

const makeTile = (domain: string): TileEntry => ({
  tileDir: `skills/${domain}/a`,
  domain,
  shortName: "my-tile",
  fullName: "org/my-tile",
  version: "0.1.0",
  summary: "A tile",
  isPublic: false,
  publishedStatus: "unpublished",
  skills: [
    {
      name: "skill-a",
      skillDir: `skills/${domain}/a`,
      auditRelPath: `${domain}/a`,
    },
  ],
});

describe("generateReadmeSummaryTables", () => {
  test("returns empty string for no tiles and no untiled skills", async () => {
    expect(await generateReadmeSummaryTables([], [])).toBe("");
  });

  test("returns non-empty string for known domain tile", async () => {
    const result = await generateReadmeSummaryTables([makeTile("ci-cd")], []);
    expect(result.length).toBeGreaterThan(0);
    expect(result).toContain("my-tile");
  });

  test("includes untiled skill rows", async () => {
    const result = await generateReadmeSummaryTables(
      [],
      [{ domain: "ci-cd", relativePath: "ci-cd/my-skill" }],
    );
    expect(result).toContain("no tile");
  });

  test("skips domains with no tiles or skills", async () => {
    const result = await generateReadmeSummaryTables([makeTile("ci-cd")], []);
    expect(result).not.toContain("## Infrastructure");
  });
});
