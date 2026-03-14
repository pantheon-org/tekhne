import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { FileNotFoundError } from "../utils/errors";
import { auditSkill } from "./audit-skill";

describe("auditSkill", () => {
  test("should throw FileNotFoundError when skill path does not exist", async () => {
    await expect(auditSkill("/nonexistent/skill-xyz-12345")).rejects.toThrow(
      FileNotFoundError,
    );
  });

  test("should throw FileNotFoundError when SKILL.md is missing", async () => {
    await expect(auditSkill("/tmp")).rejects.toThrow(FileNotFoundError);
  });
});

// ---------------------------------------------------------------------------
// auditSkill — evaluate script check
// ---------------------------------------------------------------------------

describe("auditSkill — evaluate script check", () => {
  let tmpDir: string;
  let originalCwd: string;

  beforeEach(() => {
    originalCwd = process.cwd();
    tmpDir = mkdtempSync(join(tmpdir(), "tekhne-audit-test-"));
    // Create a valid SKILL.md at the expected path
    mkdirSync(join(tmpDir, "skills", "test", "my-skill"), { recursive: true });
    writeFileSync(
      join(tmpDir, "skills", "test", "my-skill", "SKILL.md"),
      "---\nname: My Skill\ndescription: A test skill\n---\n# Body\n",
    );
    // chdir so resolve("skills/agentic-harness/.../evaluate.sh") points into tmpDir
    process.chdir(tmpDir);
  });

  afterEach(() => {
    process.chdir(originalCwd);
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("throws FileNotFoundError when evaluate.sh does not exist", async () => {
    // The evaluate script does not exist inside tmpDir, so auditSkill should
    // throw FileNotFoundError after finding SKILL.md but before running the script
    await expect(auditSkill("skills/test/my-skill")).rejects.toThrow(
      FileNotFoundError,
    );
  });
});
