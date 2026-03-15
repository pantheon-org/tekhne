import { describe, expect, test } from "bun:test";
import { getNonUniversalAgents } from "./get-non-universal-agents";

describe("getNonUniversalAgents", () => {
  test("returns a non-empty array", () => {
    expect(getNonUniversalAgents().length).toBeGreaterThan(0);
  });

  test("includes claude-code", () => {
    expect(getNonUniversalAgents()).toContain("claude-code");
  });

  test("does not include opencode (universal)", () => {
    expect(getNonUniversalAgents()).not.toContain("opencode");
  });
});
