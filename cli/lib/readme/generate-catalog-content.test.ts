import { describe, expect, test } from "bun:test";
import { generateCatalogContent } from "./generate-catalog-content";

describe("generateCatalogContent", () => {
  test("returns string starting with # Tile Catalog header", async () => {
    const result = await generateCatalogContent([], []);
    expect(result).toContain("# Tile Catalog");
  });

  test("returns non-empty string", async () => {
    expect((await generateCatalogContent([], [])).length).toBeGreaterThan(0);
  });
});
