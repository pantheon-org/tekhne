import { describe, expect, test } from "bun:test";
import { getAgentConfig } from "./get-agent-config";

describe("getAgentConfig", () => {
  test("returns config for opencode", () => {
    const c = getAgentConfig("opencode");
    expect(c.name).toBe("opencode");
    expect(c.skillsDir).toBe(".agents/skills");
  });

  test("returns config for claude-code", () => {
    const c = getAgentConfig("claude-code");
    expect(c.name).toBe("claude-code");
    expect(c.skillsDir).toBe(".claude/skills");
  });

  test("has detectInstalled function", () => {
    const c = getAgentConfig("cursor");
    expect(typeof c.detectInstalled).toBe("function");
  });
});
