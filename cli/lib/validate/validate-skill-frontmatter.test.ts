import { describe, expect, test } from "bun:test";
import {
  checkRequiredFields,
  extractFrontmatter,
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
