import { describe, expect, test } from "bun:test";
import { displayDimensionalAnalysis } from "./display-dimensional-analysis";

describe("displayDimensionalAnalysis", () => {
  test("runs without throwing for normal input", () => {
    expect(() =>
      displayDimensionalAnalysis({ D1: 13, D2: 10, D3: 8 }),
    ).not.toThrow();
  });

  test("runs without throwing for empty input", () => {
    expect(() => displayDimensionalAnalysis({})).not.toThrow();
  });
});
