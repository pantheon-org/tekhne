import { describe, expect, test } from "bun:test";
import { getTileTessl } from "./get-tile-tessl";

const baseTile = {
  tileDir: "skills/a",
  domain: "a",
  shortName: "my-tile",
  version: "0.1.0",
  summary: "",
  skills: [],
};

describe("getTileTessl", () => {
  test("returns '-' for non-public tile", () => {
    expect(
      getTileTessl({
        ...baseTile,
        isPublic: false,
        fullName: "",
        publishedStatus: "private",
      }),
    ).toBe("-");
  });

  test("returns link for public tile with fullName", () => {
    const result = getTileTessl({
      ...baseTile,
      isPublic: true,
      fullName: "org/my-tile",
      publishedStatus: "public",
    });
    expect(result).toContain("tessl.io");
    expect(result).toContain("org/my-tile");
  });

  test("returns 'Public' for public tile without fullName", () => {
    const result = getTileTessl({
      ...baseTile,
      isPublic: true,
      fullName: "",
      publishedStatus: "public",
    });
    expect(result).toBe("Public");
  });
});
