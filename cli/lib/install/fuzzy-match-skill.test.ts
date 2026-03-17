import { describe, expect, test } from "bun:test";
import { fuzzyMatchSkill } from "./fuzzy-match-skill";

describe("fuzzyMatchSkill", () => {
  test("exact match", () => {
    expect(fuzzyMatchSkill("opencode", "opencode")).toBe(true);
  });

  test("substring match", () => {
    expect(
      fuzzyMatchSkill(
        "agentic-harness--opencode-toolkit--configure",
        "opencode",
      ),
    ).toBe(true);
  });

  test("subsequence match across separators", () => {
    expect(
      fuzzyMatchSkill(
        "agentic-harness--opencode-toolkit--build-tool",
        "opnbld",
      ),
    ).toBe(true);
  });

  test("case insensitive", () => {
    expect(fuzzyMatchSkill("Agentic-Harness--OpenCode", "opencode")).toBe(true);
    expect(fuzzyMatchSkill("opencode", "OPENCODE")).toBe(true);
  });

  test("empty pattern matches everything", () => {
    expect(fuzzyMatchSkill("anything", "")).toBe(true);
    expect(fuzzyMatchSkill("", "")).toBe(true);
  });

  test("pattern longer than name does not match", () => {
    expect(fuzzyMatchSkill("ab", "abc")).toBe(false);
  });

  test("characters out of order do not match", () => {
    expect(fuzzyMatchSkill("abc", "bac")).toBe(false);
  });

  test("non-matching pattern", () => {
    expect(
      fuzzyMatchSkill("development--typescript-advanced", "opencode"),
    ).toBe(false);
  });

  test("partial prefix match", () => {
    expect(
      fuzzyMatchSkill(
        "agentic-harness--opencode-toolkit--configure",
        "configure",
      ),
    ).toBe(true);
  });

  test("single character match", () => {
    expect(fuzzyMatchSkill("abc", "a")).toBe(true);
    expect(fuzzyMatchSkill("abc", "z")).toBe(false);
  });
});
