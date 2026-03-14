import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { ValidationError } from "../utils/errors";
import {
  checkRequiredFields,
  extractFrontmatter,
  validateSkillFrontmatter,
} from "./validate-skill-frontmatter";

// ---------------------------------------------------------------------------
// extractFrontmatter
// ---------------------------------------------------------------------------

describe("extractFrontmatter", () => {
  test("extracts body from standard frontmatter block", () => {
    const content = "---\nname: My Skill\ndescription: A skill\n---\n# Body";
    expect(extractFrontmatter(content)).toBe(
      "name: My Skill\ndescription: A skill",
    );
  });

  test("returns null when no opening ---", () => {
    expect(extractFrontmatter("name: My Skill\n---\n# Body")).toBeNull();
  });

  test("returns null when no closing ---", () => {
    expect(extractFrontmatter("---\nname: My Skill\n# Body")).toBeNull();
  });

  test("handles file with no trailing newline after closing ---", () => {
    const content = "---\nname: My Skill\ndescription: A skill\n---";
    expect(extractFrontmatter(content)).toBe(
      "name: My Skill\ndescription: A skill",
    );
  });

  test("handles CRLF line endings", () => {
    const content = "---\r\nname: My Skill\r\ndescription: A skill\r\n---\r\n";
    expect(extractFrontmatter(content)).toBe(
      "name: My Skill\r\ndescription: A skill",
    );
  });

  test("returns empty string for empty frontmatter block", () => {
    expect(extractFrontmatter("---\n\n---\n")).toBe("");
  });
});

// ---------------------------------------------------------------------------
// checkRequiredFields
// ---------------------------------------------------------------------------

describe("checkRequiredFields", () => {
  const fp = "skills/test/SKILL.md";

  test("returns no errors when all required fields are present", () => {
    const result = checkRequiredFields(fp, {
      name: "My Skill",
      description: "A description",
    });
    expect(result).toEqual([]);
  });

  test("returns error when name is missing", () => {
    const result = checkRequiredFields(fp, { description: "A description" });
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("name");
  });

  test("returns error when description is missing", () => {
    const result = checkRequiredFields(fp, { name: "My Skill" });
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("description");
  });

  test("returns error when name is null", () => {
    const result = checkRequiredFields(fp, {
      name: null,
      description: "A description",
    });
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("name");
  });

  test("returns error when name is empty string", () => {
    const result = checkRequiredFields(fp, {
      name: "",
      description: "A description",
    });
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("name");
  });

  test("returns error when name is whitespace-only string", () => {
    const result = checkRequiredFields(fp, {
      name: "   ",
      description: "A description",
    });
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("name");
  });

  test("returns errors for all missing fields", () => {
    const result = checkRequiredFields(fp, {});
    expect(result).toHaveLength(1);
    expect(result[0]).toContain("name");
    expect(result[0]).toContain("description");
  });

  test("includes file path in error message", () => {
    const result = checkRequiredFields(fp, {});
    expect(result[0]).toContain(fp);
  });

  test("does not flag non-string truthy values as missing", () => {
    // Unusual but valid: non-string values should be accepted for extra fields
    const result = checkRequiredFields(fp, {
      name: "My Skill",
      description: "A description",
      someNumber: 42,
    });
    expect(result).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// validateSkillFrontmatter
// ---------------------------------------------------------------------------

describe("validateSkillFrontmatter", () => {
  let tmpDir: string;

  beforeEach(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "tekhne-validate-test-"));
  });

  afterEach(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("throws ValidationError when file does not exist", async () => {
    const missingFile = join(tmpDir, "SKILL.md");
    await expect(validateSkillFrontmatter([missingFile])).rejects.toThrow(
      ValidationError,
    );
  });

  test("throws ValidationError when file is missing frontmatter delimiters", async () => {
    const filePath = join(tmpDir, "SKILL.md");
    await Bun.write(filePath, "# No frontmatter here\nJust some content.\n");
    await expect(validateSkillFrontmatter([filePath])).rejects.toThrow(
      ValidationError,
    );
  });

  test("throws ValidationError when frontmatter contains invalid YAML", async () => {
    const filePath = join(tmpDir, "SKILL.md");
    await Bun.write(filePath, "---\nkey: [unclosed bracket\n---\n# Body\n");
    await expect(validateSkillFrontmatter([filePath])).rejects.toThrow(
      ValidationError,
    );
  });

  test("throws ValidationError when frontmatter is a YAML list not a mapping", async () => {
    const filePath = join(tmpDir, "SKILL.md");
    await Bun.write(filePath, "---\n- item1\n- item2\n---\n# Body\n");
    await expect(validateSkillFrontmatter([filePath])).rejects.toThrow(
      ValidationError,
    );
  });

  test("throws ValidationError when required fields are missing", async () => {
    const filePath = join(tmpDir, "SKILL.md");
    await Bun.write(filePath, "---\nauthor: someone\n---\n# Body\n");
    await expect(validateSkillFrontmatter([filePath])).rejects.toThrow(
      ValidationError,
    );
  });

  test("resolves without error for a valid SKILL.md", async () => {
    const filePath = join(tmpDir, "SKILL.md");
    await Bun.write(
      filePath,
      "---\nname: My Skill\ndescription: A description\n---\n# Body\n",
    );
    await expect(validateSkillFrontmatter([filePath])).resolves.toBeUndefined();
  });

  test("non-SKILL.md paths are filtered out and resolves without error", async () => {
    const filePath = join(tmpDir, "README.md");
    await Bun.write(filePath, "# README\nNo frontmatter.\n");
    // README.md is filtered; zero targets triggers a warning but no error
    await expect(validateSkillFrontmatter([filePath])).resolves.toBeUndefined();
  });

  test("throws ValidationError when any file fails validation across multiple files", async () => {
    const validFile = join(tmpDir, "SKILL.md");
    await Bun.write(
      validFile,
      "---\nname: My Skill\ndescription: A description\n---\n# Body\n",
    );

    // A second SKILL.md in a nested dir that is missing frontmatter
    const subDir = join(tmpDir, "sub");
    mkdirSync(subDir);
    const invalidFile = join(subDir, "SKILL.md");
    await Bun.write(invalidFile, "# No frontmatter\n");

    await expect(
      validateSkillFrontmatter([validFile, invalidFile]),
    ).rejects.toThrow(ValidationError);
  });
});
