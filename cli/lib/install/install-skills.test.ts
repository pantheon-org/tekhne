import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import {
  mkdirSync,
  mkdtempSync,
  realpathSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { CLIError } from "../utils/errors";
import { createSymlink } from "./create-symlink";
import { ensureDirectory } from "./ensure-directory";
import { filterSkills } from "./filter-skills";
import { findSkills } from "./find-skills";
import { getSkillName } from "./get-skill-name";
import { installSkills } from "./install-skills";
import { selectSkillsInteractively } from "./select-skills-interactively";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function makeTempDir(): string {
  return mkdtempSync(join(tmpdir(), "tekhne-install-test-"));
}

function scaffold(base: string, paths: string[]): void {
  for (const p of paths) {
    const full = join(base, p);
    mkdirSync(full.replace(/\/[^/]+$/, ""), { recursive: true });
    writeFileSync(full, "");
  }
}

// ---------------------------------------------------------------------------
// getSkillName
// ---------------------------------------------------------------------------

describe("getSkillName", () => {
  test("converts domain/skill to domain--skill", () => {
    expect(getSkillName("skills/domain/skill-name")).toBe("domain--skill-name");
  });

  test("handles nested paths", () => {
    expect(getSkillName("skills/domain/category/skill-name")).toBe(
      "domain--category--skill-name",
    );
  });

  test("preserves hyphens", () => {
    expect(getSkillName("skills/my-domain/my-skill-name")).toBe(
      "my-domain--my-skill-name",
    );
  });
});

// ---------------------------------------------------------------------------
// filterSkills
// ---------------------------------------------------------------------------

describe("filterSkills", () => {
  const skills = [
    "skills/ci-cd/github-actions/generator",
    "skills/ci-cd/gitlab-ci/validator",
    "skills/infrastructure/terraform/generator",
    "skills/infrastructure/k8s",
    "skills/documentation/markdown-authoring",
  ];

  test("returns all skills when no filters are set", () => {
    expect(filterSkills(skills, {})).toEqual(skills);
  });

  test("filters by domain", () => {
    const result = filterSkills(skills, { skillDomain: ["ci-cd"] });
    expect(result).toEqual([
      "skills/ci-cd/github-actions/generator",
      "skills/ci-cd/gitlab-ci/validator",
    ]);
  });

  test("filters by multiple domains (union)", () => {
    const result = filterSkills(skills, {
      skillDomain: ["ci-cd", "documentation"],
    });
    expect(result).toHaveLength(3);
  });

  test("filters by subdomain", () => {
    const result = filterSkills(skills, {
      skillSubdomain: ["terraform"],
    });
    expect(result).toEqual(["skills/infrastructure/terraform/generator"]);
  });

  test("intersection when both domain and subdomain are set", () => {
    // domain=ci-cd AND subdomain=github-actions → only the github-actions skill
    const result = filterSkills(skills, {
      skillDomain: ["ci-cd"],
      skillSubdomain: ["github-actions"],
    });
    expect(result).toEqual(["skills/ci-cd/github-actions/generator"]);
  });

  test("intersection excludes skills matching only one filter", () => {
    // domain=infrastructure AND subdomain=github-actions → no intersection
    const result = filterSkills(skills, {
      skillDomain: ["infrastructure"],
      skillSubdomain: ["github-actions"],
    });
    expect(result).toEqual([]);
  });

  test("empty domain array treated as no domain filter", () => {
    const result = filterSkills(skills, { skillDomain: [] });
    expect(result).toEqual(skills);
  });
});

// ---------------------------------------------------------------------------
// findSkills
// ---------------------------------------------------------------------------

describe(
  "findSkills",
  () => {
    let tmpDir: string;

    beforeEach(() => {
      tmpDir = makeTempDir();
    });

    afterEach(() => {
      rmSync(tmpDir, { recursive: true, force: true });
    });

    test("throws CLIError when skills/ directory does not exist", async () => {
      const err = await findSkills(tmpDir).catch((e) => e);
      expect(err).toBeInstanceOf(CLIError);
      expect((err as CLIError).message).toContain(
        "skills/ directory not found",
      );
      expect((err as CLIError).exitCode).toBe(1);
    });

    test("returns skill paths when SKILL.md files exist", async () => {
      scaffold(tmpDir, [
        "skills/ci-cd/github-actions/SKILL.md",
        "skills/infrastructure/terraform/SKILL.md",
      ]);

      const skills = await findSkills(tmpDir);
      expect(skills).toContain("skills/ci-cd/github-actions");
      expect(skills).toContain("skills/infrastructure/terraform");
      expect(skills).toHaveLength(2);
    });

    test("strips /SKILL.md suffix from each path", async () => {
      scaffold(tmpDir, ["skills/domain/skill/SKILL.md"]);

      const skills = await findSkills(tmpDir);
      expect(skills).toHaveLength(1);
      expect(skills[0]).not.toContain("SKILL.md");
      expect(skills[0]).toBe("skills/domain/skill");
    });

    test("returns empty array when skills/ exists but has no SKILL.md files", async () => {
      mkdirSync(join(tmpDir, "skills"));
      const skills = await findSkills(tmpDir);
      expect(skills).toEqual([]);
    });
  },
  { timeout: 10000 },
);

// ---------------------------------------------------------------------------
// ensureDirectory
// ---------------------------------------------------------------------------

describe("ensureDirectory", () => {
  let tmpDir: string;

  beforeEach(() => {
    tmpDir = makeTempDir();
  });

  afterEach(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("creates directory in live mode", () => {
    const dir = join(tmpDir, "new", "nested");
    ensureDirectory(dir, false);
    expect(require("node:fs").existsSync(dir)).toBe(true);
    // Intermediate parent must also be created (mkdir -p behaviour)
    expect(require("node:fs").existsSync(join(tmpDir, "new"))).toBe(true);
  });

  test("does not create directory in dry-run mode", () => {
    const dir = join(tmpDir, "should-not-exist");
    ensureDirectory(dir, true);
    expect(require("node:fs").existsSync(dir)).toBe(false);
  });

  test("does not throw if directory already exists", () => {
    const dir = join(tmpDir, "existing");
    mkdirSync(dir);
    expect(() => ensureDirectory(dir, false)).not.toThrow();
  });
});

// ---------------------------------------------------------------------------
// createSymlink
// ---------------------------------------------------------------------------

describe("createSymlink", () => {
  let tmpDir: string;
  let sourceDir: string;

  beforeEach(() => {
    tmpDir = makeTempDir();
    sourceDir = join(tmpDir, "source");
    mkdirSync(sourceDir);
  });

  afterEach(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("creates a relative symlink in live mode", () => {
    const target = join(tmpDir, "link");
    const result = createSymlink(sourceDir, target, false);

    expect(result).toBe(true);
    expect(require("node:fs").existsSync(target)).toBe(true);

    const linkDest = require("node:fs").readlinkSync(target);
    // Should be relative, not absolute
    expect(linkDest).not.toMatch(/^\//);
    // And must resolve to the correct source directory
    expect(
      realpathSync.native(
        resolve(require("node:path").dirname(target), linkDest),
      ),
    ).toBe(realpathSync.native(resolve(sourceDir)));
  });

  test("returns true but does not create symlink in dry-run mode", () => {
    const target = join(tmpDir, "link");
    const result = createSymlink(sourceDir, target, true);

    expect(result).toBe(true); // dry-run still signals "would install"
    expect(require("node:fs").existsSync(target)).toBe(false);
  });

  test("returns false when target is already linked to same source", () => {
    const target = join(tmpDir, "link");
    createSymlink(sourceDir, target, false);

    const result = createSymlink(sourceDir, target, false);
    expect(result).toBe(false);
  });

  test("replaces stale symlink pointing to a different source", () => {
    const otherSource = join(tmpDir, "other");
    mkdirSync(otherSource);
    const target = join(tmpDir, "link");

    // Create initial symlink pointing to otherSource
    symlinkSync(otherSource, target, "dir");

    // Now replace with sourceDir
    const result = createSymlink(sourceDir, target, false);
    expect(result).toBe(true);

    const linkDest = require("node:fs").readlinkSync(target);
    // Resolved destination should point to sourceDir (use realpathSync to
    // handle platform-level symlinks such as macOS /tmp → /private/tmp)
    expect(
      realpathSync.native(
        resolve(require("node:path").dirname(target), linkDest),
      ),
    ).toBe(realpathSync.native(resolve(sourceDir)));
  });

  test("returns false and does not throw when target is a real directory", () => {
    const target = join(tmpDir, "realdir");
    mkdirSync(target);

    const result = createSymlink(sourceDir, target, false);
    expect(result).toBe(false);
  });
});

// ---------------------------------------------------------------------------
// installSkills — unknown agent validation
// ---------------------------------------------------------------------------

describe("installSkills", () => {
  test("throws CLIError for unknown agent", async () => {
    const err = await installSkills({
      agent: ["unknown-agent"],
      global: false,
      dryRun: true,
      interactive: false,
    }).catch((e) => e);
    expect(err).toBeInstanceOf(CLIError);
    expect((err as CLIError).message).toContain("Unknown agent(s)");
    expect((err as CLIError).exitCode).toBe(1);
  });

  test("throws CLIError listing all unknown agents", async () => {
    const err = await installSkills({
      agent: ["bad1", "bad2"],
      global: false,
      dryRun: true,
      interactive: false,
    }).catch((e) => e);
    expect(err).toBeInstanceOf(CLIError);
    expect((err as CLIError).message).toContain("bad1");
    expect((err as CLIError).message).toContain("bad2");
  });
});

// ---------------------------------------------------------------------------
// selectSkillsInteractively
// ---------------------------------------------------------------------------

describe("selectSkillsInteractively", () => {
  test("returns all skills as-is when stdin is not a TTY", async () => {
    // In bun:test, process.stdin.isTTY is falsy so the function bypasses prompting
    const skills = [
      "skills/ci-cd/github-actions",
      "skills/infrastructure/terraform",
    ];
    const result = await selectSkillsInteractively(skills);
    expect(result).toEqual(skills);
  });

  test("returns empty array when empty array is passed", async () => {
    const result = await selectSkillsInteractively([]);
    expect(result).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// installSkills — full dry-run flow
// ---------------------------------------------------------------------------

describe(
  "installSkills — full dry-run flow",
  () => {
    let tmpDir: string;
    let originalCwd: string;

    beforeEach(() => {
      originalCwd = process.cwd();
      tmpDir = mkdtempSync(join(tmpdir(), "tekhne-install-flow-test-"));
      // Create a skills directory with one SKILL.md
      mkdirSync(join(tmpDir, "skills", "ci-cd", "github-actions"), {
        recursive: true,
      });
      writeFileSync(
        join(tmpDir, "skills", "ci-cd", "github-actions", "SKILL.md"),
        "---\nname: GitHub Actions\ndescription: CI skill\n---\n",
      );
      process.chdir(tmpDir);
    });

    afterEach(() => {
      process.chdir(originalCwd);
      rmSync(tmpDir, { recursive: true, force: true });
    });

    test("dry-run with opencode agent completes without error", async () => {
      await expect(
        installSkills({
          agent: ["opencode"],
          global: false,
          dryRun: true,
          interactive: false,
        }),
      ).resolves.toBeUndefined();
    });

    test("dry-run with cursor agent completes without error", async () => {
      await expect(
        installSkills({
          agent: ["cursor"],
          global: false,
          dryRun: true,
          interactive: false,
        }),
      ).resolves.toBeUndefined();
    });

    test("dry-run with gemini agent completes without error", async () => {
      await expect(
        installSkills({
          agent: ["gemini"],
          global: false,
          dryRun: true,
          interactive: false,
        }),
      ).resolves.toBeUndefined();
    });

    test("dry-run with domain filter installs only matching skills", async () => {
      // Add a second skill in a different domain
      mkdirSync(join(tmpDir, "skills", "infrastructure", "terraform"), {
        recursive: true,
      });
      writeFileSync(
        join(tmpDir, "skills", "infrastructure", "terraform", "SKILL.md"),
        "---\nname: Terraform\ndescription: IaC skill\n---\n",
      );

      // Should complete without error; domain filter limits scope
      await expect(
        installSkills({
          agent: ["opencode"],
          global: false,
          dryRun: true,
          interactive: false,
          skillDomain: ["ci-cd"],
        }),
      ).resolves.toBeUndefined();
    });

    test("non-dry-run creates symlinks in .agents/skills for opencode", async () => {
      await installSkills({
        agent: ["opencode"],
        global: false,
        dryRun: false,
        interactive: false,
      });

      const targetDir = join(tmpDir, ".agents", "skills");
      const linkPath = join(targetDir, "ci-cd--github-actions");
      expect(require("node:fs").existsSync(linkPath)).toBe(true);
    });
  },
  { timeout: 15000 },
);
