import { describe, expect, test } from "bun:test";
import { buildCountLabel } from "./build-count-label";

describe("buildCountLabel", () => {
  test("returns both tiles and skills when both are positive", () => {
    expect(buildCountLabel(2, 3)).toBe("2 tiles, 3 skills");
  });

  test("uses singular 'tile' when tileCount is 1", () => {
    expect(buildCountLabel(1, 3)).toBe("1 tile, 3 skills");
  });

  test("uses singular 'skill' when skillCount is 1", () => {
    expect(buildCountLabel(2, 1)).toBe("2 tiles, 1 skill");
  });

  test("uses singular for both when both are 1", () => {
    expect(buildCountLabel(1, 1)).toBe("1 tile, 1 skill");
  });

  test("returns tiles-only label when skillCount is 0", () => {
    expect(buildCountLabel(3, 0)).toBe("3 tiles");
  });

  test("uses singular 'tile' in tiles-only branch", () => {
    expect(buildCountLabel(1, 0)).toBe("1 tile");
  });

  test("returns skills-only label when tileCount is 0", () => {
    expect(buildCountLabel(0, 4)).toBe("4 skills");
  });

  test("uses singular 'skill' in skills-only branch", () => {
    expect(buildCountLabel(0, 1)).toBe("1 skill");
  });

  test("returns '0 skills' when both counts are 0", () => {
    // tileCount 0 falls through to skills-only branch
    expect(buildCountLabel(0, 0)).toBe("0 skills");
  });
});
