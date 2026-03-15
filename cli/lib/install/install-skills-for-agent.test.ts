import { describe, expect, test } from "bun:test";
import { homedir } from "node:os";
import { join } from "node:path";
import { AGENT_PATHS, ALWAYS_GLOBAL_AGENTS } from "./install-skills-for-agent";

describe("AGENT_PATHS", () => {
  test("opencode with isGlobal=false uses cwd/.agents/skills", () => {
    expect(AGENT_PATHS.opencode(false, "/project")).toBe(
      "/project/.agents/skills",
    );
  });

  test("opencode with isGlobal=true uses ~/.config/opencode/skills", () => {
    expect(AGENT_PATHS.opencode(true, "/project")).toBe(
      join(homedir(), ".config", "opencode", "skills"),
    );
  });

  test("cursor always uses global path regardless of isGlobal flag", () => {
    const globalPath = join(homedir(), ".config", "cursor", "skills");
    expect(AGENT_PATHS.cursor(false, "/project")).toBe(globalPath);
    expect(AGENT_PATHS.cursor(true, "/project")).toBe(globalPath);
  });

  test("claude always uses global path", () => {
    const expected = join(homedir(), ".config", "claude", "skills");
    expect(AGENT_PATHS.claude(false, "/project")).toBe(expected);
    expect(AGENT_PATHS.claude(true, "/project")).toBe(expected);
  });

  test("gemini always uses global path", () => {
    const expected = join(homedir(), ".config", "gemini", "skills");
    expect(AGENT_PATHS.gemini(false, "/project")).toBe(expected);
    expect(AGENT_PATHS.gemini(true, "/project")).toBe(expected);
  });

  test("all always-global agents are in ALWAYS_GLOBAL_AGENTS set", () => {
    for (const agent of ALWAYS_GLOBAL_AGENTS) {
      expect(AGENT_PATHS[agent]).toBeDefined();
    }
  });
});
