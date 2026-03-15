import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { getOpenClawGlobalSkillsDir } from "./get-open-claw-global-skills-dir";

describe("getOpenClawGlobalSkillsDir", () => {
  test("returns .openclaw/skills when .openclaw exists", () => {
    const result = getOpenClawGlobalSkillsDir("/home/user", (p) =>
      p.endsWith(".openclaw"),
    );
    expect(result).toBe(join("/home/user", ".openclaw/skills"));
  });

  test("returns .clawdbot/skills when .clawdbot exists", () => {
    const result = getOpenClawGlobalSkillsDir("/home/user", (p) =>
      p.endsWith(".clawdbot"),
    );
    expect(result).toBe(join("/home/user", ".clawdbot/skills"));
  });

  test("returns .moltbot/skills when .moltbot exists", () => {
    const result = getOpenClawGlobalSkillsDir("/home/user", (p) =>
      p.endsWith(".moltbot"),
    );
    expect(result).toBe(join("/home/user", ".moltbot/skills"));
  });

  test("falls back to .openclaw/skills when nothing exists", () => {
    const result = getOpenClawGlobalSkillsDir("/home/user", () => false);
    expect(result).toBe(join("/home/user", ".openclaw/skills"));
  });
});
