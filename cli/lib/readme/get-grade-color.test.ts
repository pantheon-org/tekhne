import { describe, expect, test } from "bun:test";
import { getGradeColor } from "./get-grade-color";

describe("getGradeColor", () => {
  test.each([
    ["A+", "brightgreen"],
    ["A", "green"],
    ["B+", "yellowgreen"],
    ["B", "yellow"],
    ["C+", "orange"],
    ["C", "red"],
    ["D", "red"],
    ["F", "purple"],
  ])("grade %s → %s", (grade, color) => {
    expect(getGradeColor(grade)).toBe(color);
  });

  test("unknown grade falls back to lightgrey", () => {
    expect(getGradeColor("Z")).toBe("lightgrey");
    expect(getGradeColor("")).toBe("lightgrey");
  });
});
