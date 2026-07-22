import { describe, expect, test } from "bun:test";
import { getBadgeFilename } from "./get-badge-filename";

describe("getBadgeFilename", () => {
  test.each([
    ["A+", "A-plus"],
    ["B+", "B-plus"],
    ["C+", "C-plus"],
    ["A", "A"],
    ["B", "B"],
    ["C", "C"],
    ["D", "D"],
    ["F", "F"],
    ["?", "unknown"],
  ])("grade %s → %s", (grade, expected) => {
    expect(getBadgeFilename(grade)).toBe(expected);
  });
});
