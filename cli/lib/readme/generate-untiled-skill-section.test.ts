import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { generateUntiledSkillSection } from "./generate-untiled-skill-section";
import type { SkillEntry } from "./readme-types";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "untiled-skill-test-"));
});

afterEach(() => {
  Bun.spawnSync(["rm", "-rf", tmp]);
});

const makeSkill = (relativePath: string): SkillEntry => ({
  relativePath,
  domain: relativePath.split("/")[0],
});

describe("generateUntiledSkillSection", () => {
  test("renders section heading and unknown-rating badge when no audit exists", async () => {
    const skill = makeSkill("development/my-skill");
    const orig = process.cwd();
    process.chdir(tmp);
    try {
      const output = await generateUntiledSkillSection(skill);
      expect(output).toContain("### my-skill _(no tile)_");
      expect(output).toContain("Rating-?-lightgrey");
      expect(output).toContain("| - |"); // no audit link, no evals
    } finally {
      process.chdir(orig);
    }
  });

  test("renders badge and audit link when audit exists", async () => {
    // Set up fake SKILL.md and audit
    const skillDir = join(tmp, "skills", "development", "my-skill");
    mkdirSync(skillDir, { recursive: true });
    writeFileSync(
      join(skillDir, "SKILL.md"),
      `---\nname: my-skill\ndescription: A test skill\n---\n`,
    );
    const auditDir = join(
      tmp,
      ".context",
      "audits",
      "development",
      "my-skill",
      "2026-03-15",
    );
    mkdirSync(auditDir, { recursive: true });
    writeFileSync(join(auditDir, "audit.json"), JSON.stringify({ grade: "A" }));

    const skill = makeSkill("development/my-skill");
    const orig = process.cwd();
    process.chdir(tmp);
    try {
      const output = await generateUntiledSkillSection(skill);
      expect(output).toContain("Rating-A-green");
      expect(output).toContain("2026-03-15");
    } finally {
      process.chdir(orig);
    }
  });
});
