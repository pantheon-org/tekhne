import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { validateFile } from "./validate-file";

let tmpDir: string;

beforeEach(() => {
  tmpDir = mkdtempSync(join(tmpdir(), "validate-file-test-"));
});

afterEach(() => {
  rmSync(tmpDir, { recursive: true, force: true });
});

describe("validateFile", () => {
  test("returns error when file does not exist", async () => {
    const errors = await validateFile(join(tmpDir, "missing.md"));
    expect(errors).toHaveLength(1);
    expect(errors[0]).toContain("File not found");
  });

  test("returns error when frontmatter delimiters are absent", async () => {
    const p = join(tmpDir, "SKILL.md");
    writeFileSync(p, "# No frontmatter\n");
    const errors = await validateFile(p);
    expect(errors[0]).toContain("missing --- delimiters");
  });

  test("returns error for invalid YAML inside frontmatter", async () => {
    const p = join(tmpDir, "SKILL.md");
    writeFileSync(p, "---\nkey: [unclosed\n---\n");
    const errors = await validateFile(p);
    expect(errors[0]).toContain("invalid YAML frontmatter");
  });

  test("returns error when frontmatter is a YAML list not a mapping", async () => {
    const p = join(tmpDir, "SKILL.md");
    writeFileSync(p, "---\n- item\n---\n");
    const errors = await validateFile(p);
    expect(errors[0]).toContain("frontmatter must be a YAML mapping");
  });

  test("returns empty array for a valid file", async () => {
    const p = join(tmpDir, "SKILL.md");
    writeFileSync(p, "---\nname: my-skill\ndescription: A skill\n---\n");
    expect(await validateFile(p)).toHaveLength(0);
  });
});
