import { describe, expect, test } from "bun:test";
import { isSkillSectionStart } from "./is-skill-section-start";

const headers = ["Development", "CI/CD"];

describe("isSkillSectionStart", () => {
  test("returns true for matching domain header", () => {
    expect(isSkillSectionStart("## Development", headers)).toBe(true);
  });

  test("returns true for second matching header", () => {
    expect(isSkillSectionStart("## CI/CD", headers)).toBe(true);
  });

  test("returns false for non-matching h2", () => {
    expect(isSkillSectionStart("## Other", headers)).toBe(false);
  });

  test("returns false for h3 matching header", () => {
    expect(isSkillSectionStart("### Development", headers)).toBe(false);
  });

  test("returns false for empty line", () => {
    expect(isSkillSectionStart("", headers)).toBe(false);
  });
});
