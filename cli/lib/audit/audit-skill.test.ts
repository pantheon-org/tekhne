import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { AuditFailedError, FileNotFoundError } from "../utils/errors";
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

  test("throws AuditFailedError when evaluate.sh exits non-zero", async () => {
    // Create a failing evaluate.sh
    const scriptDir = join(
      tmpDir,
      "skills",
      "agentic-harness",
      "skill-quality-auditor",
      "scripts",
    );
    mkdirSync(scriptDir, { recursive: true });
    const scriptPath = join(scriptDir, "evaluate.sh");
    writeFileSync(scriptPath, "#!/usr/bin/env sh\nexit 1\n");

    await expect(auditSkill("skills/test/my-skill")).rejects.toThrow(
      AuditFailedError,
    );
  });

  test("completes successfully when evaluate.sh exits zero and no audit.json exists", async () => {
    // Create a passing evaluate.sh that writes nothing
    const scriptDir = join(
      tmpDir,
      "skills",
      "agentic-harness",
      "skill-quality-auditor",
      "scripts",
    );
    mkdirSync(scriptDir, { recursive: true });
    const scriptPath = join(scriptDir, "evaluate.sh");
    writeFileSync(scriptPath, "#!/usr/bin/env sh\nexit 0\n");

    // Should resolve without error (no audit.json, logs a warning)
    await expect(auditSkill("skills/test/my-skill")).resolves.toBeUndefined();
  });

  test("reads score and grade from audit.json when evaluate.sh exits zero", async () => {
    const skillPath = "skills/test/my-skill";

    // Create a passing evaluate.sh
    const scriptDir = join(
      tmpDir,
      "skills",
      "agentic-harness",
      "skill-quality-auditor",
      "scripts",
    );
    mkdirSync(scriptDir, { recursive: true });
    const scriptPath = join(scriptDir, "evaluate.sh");
    writeFileSync(scriptPath, "#!/usr/bin/env sh\nexit 0\n");

    // Create a passing audit.json at the expected path:
    // <fullPath>/.context/audits/<skillPath>/latest/audit.json
    const fullSkillPath = join(tmpDir, skillPath);
    const auditDir = join(
      fullSkillPath,
      ".context",
      "audits",
      skillPath,
      "latest",
    );
    mkdirSync(auditDir, { recursive: true });
    writeFileSync(
      join(auditDir, "audit.json"),
      JSON.stringify({ score: 120, grade: "A" }),
    );

    await expect(auditSkill(skillPath)).resolves.toBeUndefined();
  });

  test("logs warning when score is below threshold", async () => {
    const skillPath = "skills/test/my-skill";

    const scriptDir = join(
      tmpDir,
      "skills",
      "agentic-harness",
      "skill-quality-auditor",
      "scripts",
    );
    mkdirSync(scriptDir, { recursive: true });
    writeFileSync(
      join(scriptDir, "evaluate.sh"),
      "#!/usr/bin/env sh\nexit 0\n",
    );

    const fullSkillPath = join(tmpDir, skillPath);
    const auditDir = join(
      fullSkillPath,
      ".context",
      "audits",
      skillPath,
      "latest",
    );
    mkdirSync(auditDir, { recursive: true });
    writeFileSync(
      join(auditDir, "audit.json"),
      JSON.stringify({ score: 80, grade: "C" }),
    );

    // Should still resolve without error even with a low score
    await expect(auditSkill(skillPath)).resolves.toBeUndefined();
  });
});
