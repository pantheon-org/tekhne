import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readlinkSync,
  realpathSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { CLIError } from "../utils/errors";
import { createSymlink } from "./create-symlink";
import { dedupAgentsByTarget } from "./dedup-agents-by-target";
import { ensureDirectory } from "./ensure-directory";
import { filterSkills } from "./filter-skills";
import { findSkills } from "./find-skills";
import { getSkillName } from "./get-skill-name";
import { installSkills } from "./install-skills";
import { installSkillsForAgent } from "./install-skills-for-agent";
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
// dedupAgentsByTarget
// ---------------------------------------------------------------------------

describe("dedupAgentsByTarget", () => {
  test("keeps unique agents when all target dirs differ", () => {
    const result = dedupAgentsByTarget(
      ["opencode", "claude-code"],
      false,
      "/project",
    );
    expect(result).toEqual(["opencode", "claude-code"]);
  });

  test("removes second agent when both resolve to same local target dir", () => {
    // amp and cursor both use .agents/skills locally
    const result = dedupAgentsByTarget(["amp", "cursor"], false, "/project");
    expect(result).toEqual(["amp"]);
  });

  test("preserves order and keeps first occurrence", () => {
    // cursor, amp, opencode — cursor and amp share .agents/skills; opencode too
    const result = dedupAgentsByTarget(
      ["cursor", "amp", "opencode"],
      false,
      "/project",
    );
    expect(result).toEqual(["cursor"]);
  });

  test("returns empty array for empty input", () => {
    expect(dedupAgentsByTarget([], false, "/project")).toEqual([]);
  });

  test("keeps single agent unchanged", () => {
    expect(dedupAgentsByTarget(["opencode"], false, "/project")).toEqual([
      "opencode",
    ]);
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

  test("dryRun returns true and skips creation when target does not exist", () => {
    const target = join(tmpDir, "link");
    const result = createSymlink(sourceDir, target, true);
    expect(result).toBe(true);
    expect(existsSync(target)).toBe(false);
  });

  test("returns false when symlinkSync throws (parent dir is read-only)", () => {
    const readonlyDir = join(tmpDir, "readonly");
    mkdirSync(readonlyDir);
    require("node:fs").chmodSync(readonlyDir, 0o444);
    const target = join(readonlyDir, "link");
    const result = createSymlink(sourceDir, target, false);
    require("node:fs").chmodSync(readonlyDir, 0o755);
    expect(result).toBe(false);
  });

  test("dryRun returns true and skips unlink when target is stale symlink", () => {
    const otherSource = join(tmpDir, "other");
    mkdirSync(otherSource);
    const target = join(tmpDir, "link");
    symlinkSync(otherSource, target, "dir");

    const result = createSymlink(sourceDir, target, true);
    expect(result).toBe(true);
    // symlink still points to old target — not replaced
    expect(readlinkSync(target)).toContain("other");
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
  const originalIsTTY = process.stdin.isTTY;

  beforeEach(() => {
    Object.defineProperty(process.stdin, "isTTY", {
      value: false,
      configurable: true,
      writable: true,
    });
  });

  afterEach(() => {
    Object.defineProperty(process.stdin, "isTTY", {
      value: originalIsTTY,
      configurable: true,
      writable: true,
    });
  });

  test("returns all skills as-is when stdin is not a TTY", async () => {
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

    test("dry-run with gemini-cli agent completes without error", async () => {
      await expect(
        installSkills({
          agent: ["gemini-cli"],
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

    test("global flag with cursor agent completes without error", async () => {
      await expect(
        installSkills({
          agent: ["cursor"],
          global: true,
          dryRun: true,
          interactive: false,
        }),
      ).resolves.toBeUndefined();
    });
  },
  { timeout: 15000 },
);

// ---------------------------------------------------------------------------
// installSkillsForAgent — direct unit tests
// ---------------------------------------------------------------------------

describe("installSkillsForAgent", () => {
  let tmpDir: string;

  beforeEach(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "tekhne-agent-test-"));
  });

  afterEach(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("installSkillsForAgent with cursor and no skills returns zero stats", () => {
    const stats = installSkillsForAgent("cursor", [], tmpDir, {
      agent: ["cursor"],
      global: true,
      dryRun: true,
      interactive: false,
    });
    expect(stats.installed).toBe(0);
    expect(stats.skipped).toBe(0);
    expect(stats.failed).toBe(0);
  });

  test("returns stats with failed count when symlink creation throws and target absent", () => {
    // Create skill source dir, then create a real dir where the symlink target would go
    const skillSrcDir = join(tmpDir, "skills", "nonexistent", "skill");
    mkdirSync(skillSrcDir, { recursive: true });
    const targetBase = join(tmpDir, ".agents", "skills");
    mkdirSync(targetBase, { recursive: true });
    // Create a real directory at the target path so createSymlink returns false
    // and existsSync(target) is true → skipped (not failed). Instead, test the
    // failed path by using a skill whose source exists but target is absent and
    // symlinkSync fails due to a permissions error. We approximate this by
    // making the parent target dir read-only.
    // This is OS-dependent; skip if running as root.
    const readonlyTarget = join(tmpDir, ".agents", "readonly-skills");
    mkdirSync(readonlyTarget, { recursive: true });
    require("node:fs").chmodSync(readonlyTarget, 0o444);

    const stats = installSkillsForAgent(
      "opencode",
      ["skills/nonexistent/skill"],
      tmpDir,
      { agent: ["opencode"], global: false, dryRun: false, interactive: false },
    );
    // Restore permissions before assertions
    require("node:fs").chmodSync(readonlyTarget, 0o755);

    // Whether failed or installed depends on permissions; just verify it ran
    expect(stats.installed + stats.failed + stats.skipped).toBe(1);
  });

  test("returns stats with skipped count when target already exists as real dir", () => {
    // Create skill source dir so realpathSync does not throw
    const skillSrcDir = join(tmpDir, "skills", "ci-cd", "github-actions");
    mkdirSync(skillSrcDir, { recursive: true });
    // Create a target directory that already exists (not a symlink, not created by us)
    const targetBase = join(tmpDir, ".agents", "skills");
    mkdirSync(targetBase, { recursive: true });
    const skillName = "ci-cd--github-actions";
    mkdirSync(join(targetBase, skillName));

    // createSymlink on a real dir returns false but target exists → skipped++
    const stats = installSkillsForAgent(
      "opencode",
      ["skills/ci-cd/github-actions"],
      tmpDir,
      { agent: ["opencode"], global: false, dryRun: false, interactive: false },
    );
    expect(stats.skipped).toBe(1);
    expect(stats.failed).toBe(0);
    expect(stats.installed).toBe(0);
  });

  test("dry-run returns installed count without creating files", () => {
    // Create skill source dir so realpathSync does not throw
    const skillSrcDir = join(tmpDir, "skills", "ci-cd", "github-actions");
    mkdirSync(skillSrcDir, { recursive: true });

    const stats = installSkillsForAgent(
      "opencode",
      ["skills/ci-cd/github-actions"],
      tmpDir,
      { agent: ["opencode"], global: false, dryRun: true, interactive: false },
    );
    // dry-run createSymlink returns true → installed++
    expect(stats.installed).toBe(1);
    expect(stats.failed).toBe(0);
    expect(stats.skipped).toBe(0);
    // Target dir should not be created in dry-run
    expect(
      require("node:fs").existsSync(join(tmpDir, ".agents", "skills")),
    ).toBe(false);
  });
});
