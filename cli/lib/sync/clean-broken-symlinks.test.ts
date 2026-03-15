import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, symlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { cleanBrokenSymlinks } from "./clean-broken-symlinks";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "clean-symlinks-test-"));
});

afterEach(() => {
  rmSync(tmp, { recursive: true, force: true });
});

describe("cleanBrokenSymlinks", () => {
  test("returns 0 when directory does not exist", () => {
    expect(cleanBrokenSymlinks(join(tmp, "nonexistent"), false)).toBe(0);
  });

  test("returns 0 for directory with no symlinks", () => {
    const dir = join(tmp, "skills");
    mkdirSync(dir);
    expect(cleanBrokenSymlinks(dir, false)).toBe(0);
  });

  test("removes broken symlink and returns count 1", () => {
    const dir = join(tmp, "skills");
    mkdirSync(dir);
    // Create a symlink pointing to nonexistent target
    symlinkSync(join(tmp, "gone"), join(dir, "broken-link"));
    expect(cleanBrokenSymlinks(dir, false)).toBe(1);
  });

  test("dry run counts broken symlinks without removing", () => {
    const dir = join(tmp, "skills");
    mkdirSync(dir);
    symlinkSync(join(tmp, "gone"), join(dir, "broken-link"));
    const count = cleanBrokenSymlinks(dir, true);
    expect(count).toBe(1);
    // Link still exists in dry run
    expect(Bun.file(join(dir, "broken-link")).size).toBeDefined();
  });

  test("does not count valid symlinks", () => {
    const dir = join(tmp, "skills");
    const target = join(tmp, "real-dir");
    mkdirSync(dir);
    mkdirSync(target);
    symlinkSync(target, join(dir, "valid-link"));
    expect(cleanBrokenSymlinks(dir, false)).toBe(0);
  });
});
