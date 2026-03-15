import { describe, expect, test } from "bun:test";
import { isChildTile } from "./is-child-tile";

describe("isChildTile", () => {
  const tileDirs = new Set([
    "skills/infrastructure/terraform",
    "skills/infrastructure",
  ]);

  test("returns true when parent dir is in tileDirs and tile is private", () => {
    // tileDir = "skills/infrastructure/terraform/generator"
    // dirname = "skills/infrastructure/terraform" → in tileDirs, private = true
    expect(
      isChildTile("skills/infrastructure/terraform/generator", tileDirs, {
        private: true,
      }),
    ).toBe(true);
  });

  test("returns false when parent dir is in tileDirs but tile is not private", () => {
    expect(
      isChildTile("skills/infrastructure/terraform/generator", tileDirs, {
        private: false,
      }),
    ).toBe(false);
  });

  test("returns false when parent dir is NOT in tileDirs even if private", () => {
    // tileDir = "skills/ci-cd/github-actions"
    // dirname = "skills/ci-cd" → not in tileDirs
    expect(
      isChildTile("skills/ci-cd/github-actions", tileDirs, { private: true }),
    ).toBe(false);
  });

  test("returns false when tileData.private is undefined", () => {
    expect(
      isChildTile("skills/infrastructure/terraform/generator", tileDirs, {}),
    ).toBe(false);
  });

  test("returns false when tileDirs is empty", () => {
    expect(
      isChildTile("skills/infrastructure/terraform/generator", new Set(), {
        private: true,
      }),
    ).toBe(false);
  });
});
