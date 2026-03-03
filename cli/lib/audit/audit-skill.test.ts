import { describe, expect, test } from "bun:test";
import { FileNotFoundError } from "../utils/errors";
import { auditSkill } from "./audit-skill";

describe("auditSkill", () => {
  test("should throw FileNotFoundError when skill path does not exist", async () => {
    expect(async () => {
      await auditSkill("/nonexistent/skill-xyz-12345");
    }).toThrow(FileNotFoundError);
  });

  test("should throw FileNotFoundError when SKILL.md is missing", async () => {
    expect(async () => {
      await auditSkill("/tmp");
    }).toThrow(FileNotFoundError);
  });
});
