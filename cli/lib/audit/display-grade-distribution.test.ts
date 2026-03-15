import { describe, expect, test } from "bun:test";
import { displayGradeDistribution } from "./display-grade-distribution";

describe("displayGradeDistribution", () => {
  test("runs without throwing", () => {
    expect(() =>
      displayGradeDistribution(
        { "A+": 2, A: 1, "B+": 0, B: 3, "C+": 0, C: 0, D: 1, F: 0 },
        7,
      ),
    ).not.toThrow();
  });
});
