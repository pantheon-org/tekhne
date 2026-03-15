import { describe, expect, test } from "bun:test";
import { getUniversalAgents } from "./get-universal-agents";

describe("getUniversalAgents", () => {
  test("returns a non-empty array", () => {
    expect(getUniversalAgents().length).toBeGreaterThan(0);
  });

  test("includes opencode", () => {
    expect(getUniversalAgents()).toContain("opencode");
  });

  test("does not include claude-code (non-universal)", () => {
    expect(getUniversalAgents()).not.toContain("claude-code");
  });

  test("does not include agents with showInUniversalList: false", () => {
    const result = getUniversalAgents();
    expect(result).not.toContain("replit");
    expect(result).not.toContain("universal");
  });
});
