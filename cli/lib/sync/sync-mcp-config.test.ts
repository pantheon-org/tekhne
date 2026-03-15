import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { syncMcpConfig } from "./sync-mcp-config";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "sync-mcp-test-"));
});

afterEach(() => {
  rmSync(tmp, { recursive: true, force: true });
});

const mcpJson = (
  servers: Record<string, { command: string; args?: string[] }>,
) => JSON.stringify({ mcpServers: servers });

describe("syncMcpConfig", () => {
  test("returns false when mcp.json does not exist", () => {
    expect(
      syncMcpConfig(
        tmp,
        join(tmp, ".mcp.json"),
        join(tmp, "opencode.json"),
        false,
      ),
    ).toBe(false);
  });

  test("returns false for corrupt mcp.json", () => {
    const mcpPath = join(tmp, ".mcp.json");
    writeFileSync(mcpPath, "not-json{{{{");
    expect(syncMcpConfig(tmp, mcpPath, join(tmp, "opencode.json"), false)).toBe(
      false,
    );
  });

  test("writes opencode.json with transformed mcp entries", () => {
    const mcpPath = join(tmp, ".mcp.json");
    const ocPath = join(tmp, "opencode.json");
    writeFileSync(
      mcpPath,
      mcpJson({ myTool: { command: "npx", args: ["my-pkg"] } }),
    );
    syncMcpConfig(tmp, mcpPath, ocPath, false);
    const written = JSON.parse(require("node:fs").readFileSync(ocPath, "utf8"));
    expect(written.mcp.myTool.type).toBe("local");
    expect(written.mcp.myTool.command).toEqual(["npx", "my-pkg"]);
  });

  test("dry run returns true without writing", () => {
    const mcpPath = join(tmp, ".mcp.json");
    const ocPath = join(tmp, "opencode.json");
    writeFileSync(mcpPath, mcpJson({ myTool: { command: "node" } }));
    const result = syncMcpConfig(tmp, mcpPath, ocPath, true);
    expect(result).toBe(true);
    expect(Bun.file(ocPath).size).toBe(0); // not written
  });

  test("returns false when content is already up to date", () => {
    const mcpPath = join(tmp, ".mcp.json");
    const ocPath = join(tmp, "opencode.json");
    writeFileSync(mcpPath, mcpJson({ myTool: { command: "node" } }));
    // Write once
    syncMcpConfig(tmp, mcpPath, ocPath, false);
    // Second call should detect no changes
    expect(syncMcpConfig(tmp, mcpPath, ocPath, false)).toBe(false);
  });

  test("returns false for corrupt opencode.json", () => {
    const mcpPath = join(tmp, ".mcp.json");
    const ocPath = join(tmp, "opencode.json");
    writeFileSync(mcpPath, mcpJson({ myTool: { command: "node" } }));
    writeFileSync(ocPath, "bad-json{{{{");
    expect(syncMcpConfig(tmp, mcpPath, ocPath, false)).toBe(false);
  });
});
