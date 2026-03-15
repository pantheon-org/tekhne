import { describe, expect, test } from "bun:test";
import { displayBottomSkills } from "./display-bottom-skills";

describe("displayBottomSkills", () => {
  test("runs without throwing for empty list", () => {
    expect(() => displayBottomSkills([])).not.toThrow();
  });

  test("runs without throwing for multiple audits", () => {
    const audits = Array.from({ length: 12 }, (_, i) => ({
      path: `skill-${i}`,
      score: i * 10,
      grade: "B",
      dimensions: {},
    }));
    expect(() => displayBottomSkills(audits)).not.toThrow();
  });
});
