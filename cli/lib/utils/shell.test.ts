import { describe, expect, test } from "bun:test";
import { exec, execOrThrow } from "./shell";

describe("shell utilities", () => {
  describe("exec", () => {
    test("should execute command and return result", async () => {
      const result = await exec("echo 'hello world'");
      expect(result.exitCode).toBe(0);
      expect(result.stdout).toContain("hello world");
      expect(result.stderr).toBe("");
    });

    test("should capture exit code on failure", async () => {
      const result = await exec("exit 42");
      expect(result.exitCode).toBe(42);
    });

    test("should capture stderr on command error", async () => {
      const result = await exec("ls /nonexistent-directory-12345");
      expect(result.exitCode).not.toBe(0);
      expect(result.stderr.length).toBeGreaterThan(0);
    });
  });

  describe("execOrThrow", () => {
    test("should execute successful command without throwing", async () => {
      const result = await execOrThrow("echo 'success'");
      expect(result.includes("success")).toBe(true);
    });

    test("should throw on command failure", async () => {
      expect(async () => {
        await execOrThrow("exit 1");
      }).toThrow();
    });

    test("should throw on non-existent command", async () => {
      expect(async () => {
        await execOrThrow("nonexistent-command-xyz");
      }).toThrow();
    });
  });
});
