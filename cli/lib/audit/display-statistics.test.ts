import { describe, expect, test } from "bun:test";
import { displayStatistics } from "./display-statistics";

describe("displayStatistics", () => {
  test("runs without throwing", () => {
    expect(() =>
      displayStatistics(
        { avgScore: 90, medianScore: 88, minScore: 70, maxScore: 110 },
        5,
      ),
    ).not.toThrow();
  });
});
