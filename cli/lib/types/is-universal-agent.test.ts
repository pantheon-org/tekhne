import { describe, expect, test } from "bun:test";
import { isUniversalAgent } from "./is-universal-agent";

describe("isUniversalAgent", () => {
  test("opencode is universal (uses .agents/skills)", () => {
    expect(isUniversalAgent("opencode")).toBe(true);
  });

  test("claude-code is NOT universal (uses .claude/skills)", () => {
    expect(isUniversalAgent("claude-code")).toBe(false);
  });

  test("cursor is universal (uses .agents/skills)", () => {
    expect(isUniversalAgent("cursor")).toBe(true);
  });
});
