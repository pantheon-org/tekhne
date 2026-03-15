import { describe, expect, test } from "bun:test";
import { displayTopSkills } from "./display-top-skills";

describe("displayTopSkills", () => {
  test("runs without throwing for empty list", () => {
    expect(() => displayTopSkills([])).not.toThrow();
  });

  test("runs without throwing for multiple audits", () => {
    const audits = Array.from({ length: 12 }, (_, i) => ({
      path: `skill-${i}`,
      score: i * 10,
      grade: "A",
      dimensions: {},
    }));
    expect(() => displayTopSkills(audits)).not.toThrow();
  });
});
