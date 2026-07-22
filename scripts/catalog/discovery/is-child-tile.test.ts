import { describe, expect, test } from "bun:test";
import { isChildTile } from "./is-child-tile";

describe("isChildTile", () => {
  const tileDirs = new Set([
    "skills/infrastructure/terraform",
    "skills/infrastructure",
  ]);

  test("returns true when parent dir is in tileDirs and tile is private", () => {
    expect(
      isChildTile("skills/infrastructure/terraform/generator", tileDirs, true),
    ).toBe(true);
  });

  test("returns false when parent dir is in tileDirs but tile is not private", () => {
    expect(
      isChildTile("skills/infrastructure/terraform/generator", tileDirs, false),
    ).toBe(false);
  });

  test("returns false when parent dir is NOT in tileDirs even if private", () => {
    expect(isChildTile("skills/ci-cd/github-actions", tileDirs, true)).toBe(
      false,
    );
  });

  test("returns false when tile is not private", () => {
    expect(
      isChildTile("skills/infrastructure/terraform/generator", tileDirs, false),
    ).toBe(false);
  });

  test("returns false when tileDirs is empty", () => {
    expect(
      isChildTile("skills/infrastructure/terraform/generator", new Set(), true),
    ).toBe(false);
  });
});
