import { describe, expect, test } from "bun:test";
import { generateDocsTilesPage } from "./generate-docs-tiles-page";
import type { TileEntry } from "./tile-types";

const makeTile = (domain: string): TileEntry => ({
  tileDir: `skills/${domain}/a`,
  domain,
  shortName: "my-tile",
  fullName: "org/my-tile",
  version: "0.1.0",
  summary: "A summary",
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

describe("generateDocsTilesPage", () => {
  test("includes frontmatter with title", async () => {
    const result = await generateDocsTilesPage([], []);
    expect(result).toContain("title: Skill Catalog");
  });

  test("includes domain section when tile exists for that domain", async () => {
    const result = await generateDocsTilesPage([makeTile("ci-cd")], []);
    expect(result).toContain("my-tile");
  });

  test("includes untiled skill section when domain matches", async () => {
    const result = await generateDocsTilesPage(
      [],
      [{ domain: "ci-cd", relativePath: "ci-cd/my-skill" }],
    );
    expect(result).toContain("my-skill");
  });

  test("includes separator between multiple domains", async () => {
    const result = await generateDocsTilesPage(
      [makeTile("ci-cd"), makeTile("infrastructure")],
      [],
    );
    expect(result).toContain("---");
  });
});
