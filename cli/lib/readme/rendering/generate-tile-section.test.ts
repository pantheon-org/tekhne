import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { TileEntry } from "../types";
import { generateTileSection } from "./generate-tile-section";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "tile-section-test-"));
});

afterEach(() => {
  Bun.spawnSync(["rm", "-rf", tmp]);
});

const makeTile = (overrides: Partial<TileEntry> = {}): TileEntry => ({
  tileDir: "skills/ci-cd/github-actions",
  domain: "ci-cd",
  shortName: "github-actions-toolkit",
  fullName: "pantheon-ai/github-actions-toolkit",
  version: "1.0.0",
  summary: "GitHub Actions helpers",
  isPublic: true,
  publishedStatus: "public",
  skills: [],
  ...overrides,
});

describe("generateTileSection", () => {
  test("renders heading and description with no skills", async () => {
    const orig = process.cwd();
    process.chdir(tmp);
    try {
      const output = await generateTileSection(makeTile());
      expect(output).toContain("### [github-actions-toolkit]");
      expect(output).toContain("GitHub Actions helpers");
      expect(output).toContain("| Skill | Rating | Audit | Evals |");
    } finally {
      process.chdir(orig);
    }
  });

  test("renders unknown-rating row when skill has no audit", async () => {
    const tile = makeTile({
      skills: [
        {
          name: "my-generator",
          skillDir: "skills/ci-cd/github-actions/generator",
          auditRelPath: "ci-cd/github-actions/generator",
        },
      ],
    });
    const orig = process.cwd();
    process.chdir(tmp);
    try {
      const output = await generateTileSection(tile);
      expect(output).toContain("[my-generator]");
      expect(output).toContain("unknown.svg");
      expect(output).toContain("| - |");
    } finally {
      process.chdir(orig);
    }
  });

  test("renders badge and audit link when skill has audit", async () => {
    const auditDir = join(
      tmp,
      ".context",
      "audits",
      "ci-cd",
      "github-actions",
      "generator",
      "2026-03-15",
    );
    mkdirSync(auditDir, { recursive: true });
    writeFileSync(
      join(auditDir, "audit.json"),
      JSON.stringify({ grade: "B+" }),
    );

    const tile = makeTile({
      skills: [
        {
          name: "my-generator",
          skillDir: "skills/ci-cd/github-actions/generator",
          auditRelPath: "ci-cd/github-actions/generator",
        },
      ],
    });
    const orig = process.cwd();
    process.chdir(tmp);
    try {
      const output = await generateTileSection(tile);
      expect(output).toContain("B-plus.svg");
      expect(output).toContain("2026-03-15");
    } finally {
      process.chdir(orig);
    }
  });
});
