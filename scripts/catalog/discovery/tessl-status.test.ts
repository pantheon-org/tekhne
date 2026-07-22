import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getTesslStatus } from "./tessl-status";

let tmp: string;
let orig: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "tessl-status-test-"));
  orig = process.cwd();
  process.chdir(tmp);
});

afterEach(() => {
  process.chdir(orig);
  rmSync(tmp, { recursive: true, force: true });
});

describe("getTesslStatus", () => {
  test("returns '-' when tile.json does not exist", async () => {
    expect(await getTesslStatus("dev/bun")).toBe("-");
  });

  test("returns 'Public' link for public tile with name", async () => {
    const dir = join(tmp, "skills", "dev", "bun");
    mkdirSync(dir, { recursive: true });
    writeFileSync(
      join(dir, "tile.json"),
      JSON.stringify({ private: false, name: "org/bun-toolkit" }),
    );
    const result = await getTesslStatus("dev/bun");
    expect(result).toContain("tessl.io");
  });

  test("returns 'Public' for public tile without name", async () => {
    const dir = join(tmp, "skills", "dev", "bun");
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "tile.json"), JSON.stringify({ private: false }));
    expect(await getTesslStatus("dev/bun")).toBe("Public");
  });

  test("returns 'Private' for private tile", async () => {
    const dir = join(tmp, "skills", "dev", "bun");
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "tile.json"), JSON.stringify({ private: true }));
    expect(await getTesslStatus("dev/bun")).toBe("Private");
  });

  test("returns 'Configured' for corrupt tile.json", async () => {
    const dir = join(tmp, "skills", "dev", "bun");
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "tile.json"), "not-json{{{{");
    expect(await getTesslStatus("dev/bun")).toBe("Configured");
  });
});
