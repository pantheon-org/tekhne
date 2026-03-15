import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { getSkillDisplayName } from "../parsing";
import { parseSkillDescription } from "./parse-skill-description";

const TEST_DIR = "/tmp/skill-parser-test";

beforeEach(() => {
  if (existsSync(TEST_DIR)) {
    rmSync(TEST_DIR, { recursive: true });
  }
  mkdirSync(TEST_DIR, { recursive: true });
});

afterEach(() => {
  if (existsSync(TEST_DIR)) {
    rmSync(TEST_DIR, { recursive: true });
  }
});

describe("parseSkillDescription", () => {
  test("should return '-' when SKILL.md does not exist", () => {
    const result = parseSkillDescription("/nonexistent/path");
    expect(result).toBe("-");
  });

  test("should parse simple quoted description", () => {
    const skillPath = join(TEST_DIR, "skill1");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: "This is a simple description"
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("This is a simple description");
  });

  test("should parse single-quoted description", () => {
    const skillPath = join(TEST_DIR, "skill2");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: 'Single quoted description'
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("Single quoted description");
  });

  test("should parse unquoted description", () => {
    const skillPath = join(TEST_DIR, "skill3");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: Unquoted description text
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("Unquoted description text");
  });

  test("should parse multiline description with pipe syntax", () => {
    const skillPath = join(TEST_DIR, "skill4");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: |
  This is a multiline
  description that spans
  multiple lines
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe(
      "This is a multiline description that spans multiple lines",
    );
  });

  test("should parse multiline description with > syntax", () => {
    const skillPath = join(TEST_DIR, "skill5");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: >
  Folded multiline
  description text
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("Folded multiline description text");
  });

  test("should truncate descriptions longer than 80 characters", () => {
    const skillPath = join(TEST_DIR, "skill6");
    mkdirSync(skillPath);
    const longDescription =
      "This is a very long description that exceeds the eighty character limit and should be truncated with ellipsis";
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: "${longDescription}"
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result.length).toBe(83);
    expect(result.endsWith("...")).toBe(true);
    expect(result).toBe(`${longDescription.substring(0, 80)}...`);
  });

  test("should escape pipe characters in description", () => {
    const skillPath = join(TEST_DIR, "skill7");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: "Description with | pipe character"
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("Description with \\| pipe character");
  });

  test("should return '-' when description is empty", () => {
    const skillPath = join(TEST_DIR, "skill8");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
description: ""
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("-");
  });

  test("should return '-' when no description field in frontmatter", () => {
    const skillPath = join(TEST_DIR, "skill9");
    mkdirSync(skillPath);
    writeFileSync(
      join(skillPath, "SKILL.md"),
      `---
title: "Some Skill"
version: "1.0.0"
---`,
    );

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("-");
  });

  test("should handle SKILL.md without frontmatter", () => {
    const skillPath = join(TEST_DIR, "skill10");
    mkdirSync(skillPath);
    writeFileSync(join(skillPath, "SKILL.md"), "# Some Skill\n\nContent here");

    const result = parseSkillDescription(skillPath);
    expect(result).toBe("-");
  });
});

describe("getSkillDisplayName", () => {
  test("should format single-level path", () => {
    const result = getSkillDisplayName("domain/skill-name");
    expect(result).toBe("skill-name");
  });

  test("should format nested path with hyphens", () => {
    const result = getSkillDisplayName("domain/category/skill-name");
    expect(result).toBe("category-skill-name");
  });

  test("should format deeply nested path", () => {
    const result = getSkillDisplayName("domain/cat1/cat2/skill");
    expect(result).toBe("cat1-cat2-skill");
  });
});
