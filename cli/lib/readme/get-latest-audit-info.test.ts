import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getLatestAuditInfo } from "./get-latest-audit-info";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "audit-info-test-"));
});

afterEach(() => {
  Bun.spawnSync(["rm", "-rf", tmp]);
});

// Helper: create a fake audit directory structure under tmp/.context/audits/<skill>/<date>/
const makeAuditDir = (skillPath: string, date: string, grade: string) => {
  const dir = join(tmp, ".context", "audits", skillPath, date);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "audit.json"), JSON.stringify({ grade }));
  return dir;
};

describe("getLatestAuditInfo", () => {
  test("returns null when audit base dir does not exist", async () => {
    // Change cwd to tmp so the relative path resolves there
    const orig = process.cwd();
    process.chdir(tmp);
    try {
      expect(await getLatestAuditInfo("no/such/skill")).toBeNull();
    } finally {
      process.chdir(orig);
    }
  });

  test("returns the most recent audit when multiple dates exist", async () => {
    makeAuditDir("my/skill", "2026-01-01", "B");
    makeAuditDir("my/skill", "2026-03-15", "A");

    const orig = process.cwd();
    process.chdir(tmp);
    try {
      const info = await getLatestAuditInfo("my/skill");
      expect(info?.date).toBe("2026-03-15");
      expect(info?.grade).toBe("A");
    } finally {
      process.chdir(orig);
    }
  });

  test("falls back to grade '?' when audit.json has no grade field", async () => {
    const dir = join(tmp, ".context", "audits", "my/skill", "2026-03-15");
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "audit.json"), JSON.stringify({}));

    const orig = process.cwd();
    process.chdir(tmp);
    try {
      const info = await getLatestAuditInfo("my/skill");
      expect(info?.grade).toBe("?");
    } finally {
      process.chdir(orig);
    }
  });

  test("skips directories without audit.json and returns null if none found", async () => {
    const dir = join(tmp, ".context", "audits", "my/skill", "2026-03-15");
    mkdirSync(dir, { recursive: true }); // no audit.json

    const orig = process.cwd();
    process.chdir(tmp);
    try {
      expect(await getLatestAuditInfo("my/skill")).toBeNull();
    } finally {
      process.chdir(orig);
    }
  });

  test("skips date dirs with corrupt audit.json and falls back to null", async () => {
    const dir = join(tmp, ".context", "audits", "my/skill", "2026-03-15");
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "audit.json"), "not-valid-json{{{{");

    const orig = process.cwd();
    process.chdir(tmp);
    try {
      expect(await getLatestAuditInfo("my/skill")).toBeNull();
    } finally {
      process.chdir(orig);
    }
  });
});
