import { describe, expect, test } from "bun:test";
import {
  mkdtempSync,
  readFileSync,
  realpathSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const SKILLS = join(import.meta.dir, "../../skills/project-mgmt");
const SCRIPTS = join(import.meta.dir, "..");
const BASH = Bun.which("bash") ?? "bash";

type Wrapper = {
  name: string;
  skill: string;
  fixtures: string;
};

const WRAPPERS: Wrapper[] = [
  {
    name: "validate-plan-scaffold",
    skill: "plan-create",
    fixtures: "plan-create/fixtures",
  },
  {
    name: "validate-model-selection",
    skill: "plan-review",
    fixtures: "plan-review/fixtures-model-selection",
  },
  {
    name: "validate-tech-evaluation",
    skill: "tech-evaluation",
    fixtures: "tech-evaluation/fixtures",
  },
];

const wrapperPath = (w: Wrapper): string =>
  join(SKILLS, w.skill, "scripts", `${w.name}.sh`);

const realTemp = (prefix: string): string =>
  realpathSync(mkdtempSync(join(tmpdir(), prefix)));

// A folder holding only a fake `bun` that prints `version` for --version and otherwise says it ran.
const stubBun = (version: string): string => {
  const dir = realTemp("validator-wrapper-stub-");
  writeFileSync(
    join(dir, "bun"),
    `#!/bin/sh\nif [ "$1" = "--version" ]; then echo '${version}'; else echo "stub bun ran: $*"; fi\n`,
    { mode: 0o755 },
  );
  return dir;
};

const run = (
  w: Wrapper,
  args: string[],
  options: { cwd?: string; env?: Record<string, string> } = {},
) => {
  const result = Bun.spawnSync([BASH, wrapperPath(w), ...args], {
    cwd: options.cwd,
    env: options.env ?? process.env,
  });
  return {
    stdout: result.stdout.toString(),
    stderr: result.stderr.toString(),
    exit: result.exitCode,
  };
};

for (const w of WRAPPERS) {
  describe(`${w.name}.sh`, () => {
    test("gives the recorded verdict for the valid fixture", () => {
      const fixtures = join(SCRIPTS, w.fixtures);
      const actual = run(w, ["cases/valid.fixture"], { cwd: fixtures });
      const recorded = readFileSync(
        join(fixtures, "expected/valid.stdout"),
        "utf8",
      );
      expect(actual.exit).toBe(0);
      expect(actual.stdout).toBe(recorded);
    });

    for (const version of [
      "1.3.10",
      "1.3.11",
      "1.4.2",
      "1.10.0",
      "2.0.0",
      "1.4.0-canary.20260101+abc",
    ]) {
      test(`runs bun ${version}`, () => {
        const dir = stubBun(version);
        const result = run(w, ["x"], { env: { PATH: dir } });
        expect(result.exit).toBe(0);
        expect(result.stdout).toContain("stub bun ran: --no-install");
        expect(result.stdout).toContain(`${w.name}.ts x`);
        rmSync(dir, { recursive: true, force: true });
      });
    }

    for (const version of ["1.2.20", "1.2.21", "1.3.9", "0.9.0", "abc", ""]) {
      test(`refuses bun "${version}" and names the version needed`, () => {
        const dir = stubBun(version);
        const result = run(w, ["x"], { env: { PATH: dir } });
        expect(result.exit).toBe(1);
        expect(result.stderr).toContain(
          `${w.name}.sh needs bun 1.3.10 or later`,
        );
        expect(result.stdout).not.toContain("stub bun ran");
        rmSync(dir, { recursive: true, force: true });
      });
    }

    test("says bun is missing when there is none on PATH", () => {
      const dir = realTemp("validator-wrapper-empty-");
      const result = run(w, ["x"], { env: { PATH: dir } });
      expect(result.exit).toBe(1);
      expect(result.stderr).toContain("needs bun 1.3.10 or later");
      expect(result.stderr).toContain("not on PATH");
      rmSync(dir, { recursive: true, force: true });
    });

    test("finds the script through a symlinked skill folder", () => {
      const base = realTemp("validator-wrapper-link-");
      symlinkSync(join(SKILLS, w.skill), join(base, w.skill));
      const fixtures = join(SCRIPTS, w.fixtures);
      const result = Bun.spawnSync(
        [
          BASH,
          join(base, w.skill, "scripts", `${w.name}.sh`),
          "cases/valid.fixture",
        ],
        { cwd: fixtures },
      );
      expect(result.exitCode).toBe(0);
      expect(result.stdout.toString()).toContain("OK: 1 file(s)");
      rmSync(base, { recursive: true, force: true });
    });
  });
}
