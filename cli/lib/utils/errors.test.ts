import { describe, expect, test } from "bun:test";
import {
  AuditFailedError,
  CLIError,
  FileNotFoundError,
  ShellCommandError,
  ValidationError,
} from "./errors";

describe("CLIError", () => {
  test("should create base CLI error with default exit code", () => {
    const error = new CLIError("Test error");
    expect(error.message).toBe("Test error");
    expect(error.exitCode).toBe(1);
    expect(error.name).toBe("CLIError");
  });

  test("should create CLI error with custom exit code", () => {
    const error = new CLIError("Test error", 2);
    expect(error.exitCode).toBe(2);
  });
});

describe("FileNotFoundError", () => {
  test("should format file not found message", () => {
    const error = new FileNotFoundError("/path/to/file.txt");
    expect(error.message).toBe("File not found: /path/to/file.txt");
    expect(error.exitCode).toBe(1);
    expect(error.name).toBe("FileNotFoundError");
  });
});

describe("ValidationError", () => {
  test("should create validation error", () => {
    const error = new ValidationError("Invalid input");
    expect(error.message).toBe("Invalid input");
    expect(error.exitCode).toBe(1);
    expect(error.name).toBe("ValidationError");
  });
});

describe("ShellCommandError", () => {
  test("should format shell command error with stderr", () => {
    const error = new ShellCommandError(
      "npm install",
      "Package not found",
      127,
    );
    expect(error.message).toBe(
      "Command failed: npm install\nPackage not found",
    );
    expect(error.exitCode).toBe(127);
    expect(error.stderr).toBe("Package not found");
    expect(error.name).toBe("ShellCommandError");
  });
});

describe("AuditFailedError", () => {
  test("should create audit failed error with skill info", () => {
    const error = new AuditFailedError("skills/test/skill", 85);
    expect(error.message).toBe(
      "Audit failed for skills/test/skill (score: 85/120)",
    );
    expect(error.skillPath).toBe("skills/test/skill");
    expect(error.score).toBe(85);
    expect(error.exitCode).toBe(1);
    expect(error.name).toBe("AuditFailedError");
  });
});
