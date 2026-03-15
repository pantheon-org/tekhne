import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { validateTileExists } from "./validate-tile-exists";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "validate-tile-test-"));
});

afterEach(() => {
  rmSync(tmp, { recursive: true, force: true });
});

describe("validateTileExists", () => {
  test("returns false when tilePath does not exist", () => {
    expect(validateTileExists(join(tmp, "nonexistent"))).toBe(false);
  });

  test("returns false when tile.json is missing", () => {
    const dir = join(tmp, "my-tile");
    mkdirSync(dir);
    expect(validateTileExists(dir)).toBe(false);
  });

  test("returns true when tilePath and tile.json exist", () => {
    const dir = join(tmp, "my-tile");
    mkdirSync(dir);
    writeFileSync(join(dir, "tile.json"), "{}");
    expect(validateTileExists(dir)).toBe(true);
  });
});
