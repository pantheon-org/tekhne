import { describe, expect, test } from "bun:test";
import { homedir } from "node:os";
import { join } from "node:path";
import { configHome } from "../types/agents";
import { resolveTargetDir } from "./resolve-target-dir";

describe("resolveTargetDir", () => {
  test("opencode local uses cwd/.agents/skills", () => {
    expect(resolveTargetDir("opencode", false, "/project")).toBe(
      "/project/.agents/skills",
    );
  });

  test("opencode global uses xdg config opencode/skills", () => {
    const expected = join(configHome, "opencode/skills");
    expect(resolveTargetDir("opencode", true, "/project")).toBe(expected);
  });

  test("cursor local uses cwd/.agents/skills", () => {
    expect(resolveTargetDir("cursor", false, "/project")).toBe(
      "/project/.agents/skills",
    );
  });

  test("cursor global uses home/.cursor/skills", () => {
    expect(resolveTargetDir("cursor", true, "/project")).toBe(
      join(homedir(), ".cursor/skills"),
    );
  });

  test("claude-code local uses cwd/.claude/skills", () => {
    expect(resolveTargetDir("claude-code", false, "/project")).toBe(
      "/project/.claude/skills",
    );
  });

  test("codex global uses codexHome/skills", () => {
    const codexHome =
      process.env.CODEX_HOME?.trim() ?? join(homedir(), ".codex");
    expect(resolveTargetDir("codex", true, "/project")).toBe(
      join(codexHome, "skills"),
    );
  });
});
