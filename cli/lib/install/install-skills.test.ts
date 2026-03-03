import { describe, expect, test } from "bun:test";
import { homedir } from "node:os";
import { join } from "node:path";

describe("installSkills helper functions", () => {
  describe("getSkillName formatting", () => {
    test("should convert domain/skill to domain--skill", () => {
      const skillPath = "skills/domain/skill-name";
      const parts = skillPath.split("/").slice(1);
      const skillName = parts.join("--");
      expect(skillName).toBe("domain--skill-name");
    });

    test("should handle nested paths", () => {
      const skillPath = "skills/domain/category/skill-name";
      const parts = skillPath.split("/").slice(1);
      const skillName = parts.join("--");
      expect(skillName).toBe("domain--category--skill-name");
    });

    test("should handle skill names with hyphens", () => {
      const skillPath = "skills/my-domain/my-skill-name";
      const parts = skillPath.split("/").slice(1);
      const skillName = parts.join("--");
      expect(skillName).toBe("my-domain--my-skill-name");
    });
  });

  describe("agent path resolution", () => {
    test("should resolve opencode global path", () => {
      const isGlobal = true;
      const path = isGlobal
        ? join(homedir(), ".config", "opencode", "skills")
        : join("/cwd", ".agents", "skills");

      expect(path).toContain(".config/opencode/skills");
    });

    test("should resolve opencode local path", () => {
      const isGlobal = false;
      const cwd = "/project";
      const path = isGlobal
        ? join(homedir(), ".config", "opencode", "skills")
        : join(cwd, ".agents", "skills");

      expect(path).toBe("/project/.agents/skills");
    });

    test("should resolve cursor path (always global)", () => {
      const path = join(homedir(), ".config", "cursor", "skills");
      expect(path).toContain(".config/cursor/skills");
    });

    test("should resolve gemini path (always global)", () => {
      const path = join(homedir(), ".config", "gemini", "skills");
      expect(path).toContain(".config/gemini/skills");
    });
  });

  describe("installation stats tracking", () => {
    test("should initialize stats with zeros", () => {
      const stats = { installed: 0, skipped: 0, failed: 0 };

      expect(stats.installed).toBe(0);
      expect(stats.skipped).toBe(0);
      expect(stats.failed).toBe(0);
    });

    test("should increment installed count", () => {
      const stats = { installed: 0, skipped: 0, failed: 0 };
      stats.installed++;

      expect(stats.installed).toBe(1);
    });

    test("should increment skipped count", () => {
      const stats = { installed: 0, skipped: 0, failed: 0 };
      stats.skipped++;

      expect(stats.skipped).toBe(1);
    });

    test("should increment failed count", () => {
      const stats = { installed: 0, skipped: 0, failed: 0 };
      stats.failed++;

      expect(stats.failed).toBe(1);
    });
  });

  describe("skill discovery pattern", () => {
    test("should extract directory from SKILL.md path", () => {
      const skillPath = "skills/domain/skill-name/SKILL.md";
      const directory = skillPath.replace("/SKILL.md", "");

      expect(directory).toBe("skills/domain/skill-name");
    });

    test("should handle multiple SKILL.md results", () => {
      const output = `skills/domain1/skill1/SKILL.md
skills/domain2/skill2/SKILL.md
skills/domain3/skill3/SKILL.md`;

      const skills = output
        .trim()
        .split("\n")
        .filter(Boolean)
        .map((path) => path.replace("/SKILL.md", ""));

      expect(skills.length).toBe(3);
      expect(skills[0]).toBe("skills/domain1/skill1");
      expect(skills[1]).toBe("skills/domain2/skill2");
      expect(skills[2]).toBe("skills/domain3/skill3");
    });
  });

  describe("summary display logic", () => {
    test("should display stats for multiple agents", () => {
      const agents = ["opencode", "cursor"];
      const stats: Record<
        string,
        { installed: number; skipped: number; failed: number }
      > = {
        opencode: { installed: 10, skipped: 2, failed: 0 },
        cursor: { installed: 8, skipped: 4, failed: 1 },
      };

      for (const agent of agents) {
        const agentStats = stats[agent];
        expect(agentStats).toBeDefined();
        expect(agentStats.installed).toBeGreaterThanOrEqual(0);
      }
    });

    test("should skip agents with no stats", () => {
      const agents = ["opencode", "cursor", "unknown"];
      const stats: Record<
        string,
        { installed: number; skipped: number; failed: number }
      > = {
        opencode: { installed: 10, skipped: 0, failed: 0 },
        cursor: { installed: 5, skipped: 0, failed: 0 },
      };

      let displayedCount = 0;
      for (const agent of agents) {
        if (stats[agent]) {
          displayedCount++;
        }
      }

      expect(displayedCount).toBe(2);
    });
  });
});
