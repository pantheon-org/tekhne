import { describe, expect, test } from "bun:test";
import { isSkillSectionEnd } from "./is-skill-section-end";

const headers = ["Development", "CI/CD"];

describe("isSkillSectionEnd", () => {
  test("returns true for h2 that is NOT a domain header", () => {
    expect(isSkillSectionEnd("## Contributing", headers)).toBe(true);
  });

  test("returns false for h2 that IS a domain header", () => {
    expect(isSkillSectionEnd("## Development", headers)).toBe(false);
  });

  test("returns false for non-h2 line", () => {
    expect(isSkillSectionEnd("Some text", headers)).toBe(false);
  });

  test("returns false for empty line", () => {
    expect(isSkillSectionEnd("", headers)).toBe(false);
  });
});
