import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

type Difference = { exit: number; stdout: string; reason: string };
type Expectations = {
  header: string;
  okLabel: string;
  yamlSeparator: string;
  differences: Record<string, Difference>;
};
type Run = { stdout: string; stderr: string; exit: number };

const PREFIX = "__PREFIX__";
const HEADER_ONLY = "__HEADER__";

// Runs the TypeScript validator from the fixtures folder, so the relative paths match the recording.
const run = (script: string, cwd: string, args: string[]): Run => {
  const result = Bun.spawnSync(["bun", "--no-install", script, ...args], {
    cwd,
  });
  return {
    stdout: result.stdout.toString(),
    stderr: result.stderr.toString(),
    exit: result.exitCode,
  };
};

const recorded = (fixtures: string, name: string): Run => {
  const read = (ext: string): string =>
    readFileSync(join(fixtures, "expected", `${name}.${ext}`), "utf8");
  return {
    stdout: read("stdout"),
    stderr: read("stderr"),
    exit: Number(read("exit")),
  };
};

// Compares a TypeScript validator with the recorded python output for every invocation of a fixtures folder. A case
// listed in expectations.json differs on purpose: its expected output is there, with the reason.
export const registerGoldenTests = (script: string, fixtures: string): void => {
  const expectations = JSON.parse(
    readFileSync(join(fixtures, "expectations.json"), "utf8"),
  ) as Expectations;
  const invocations = readFileSync(join(fixtures, "invocations.txt"), "utf8")
    .split("\n")
    .filter(Boolean)
    .map((line) => {
      const [name, args] = line.split("|");
      return { name, args: args ? args.split(" ") : [] };
    });

  describe(`${script.split("/").pop()} against the recorded python output`, () => {
    for (const { name, args } of invocations) {
      test(name, () => {
        const actual = run(script, fixtures, args);
        const difference = expectations.differences[name];
        if (!difference) {
          const python = recorded(fixtures, name);
          expect(actual.exit).toBe(python.exit);
          expect(actual.stdout).toBe(python.stdout);
          expect(actual.stderr).toBe(python.stderr);
          return;
        }
        expect(actual.exit).toBe(difference.exit);
        expect(actual.stderr).toBe("");
        if (difference.stdout === HEADER_ONLY) {
          expect(actual.stdout.split("\n")[0]).toBe(expectations.header);
        } else if (difference.stdout.startsWith(PREFIX)) {
          const lines = actual.stdout.split("\n");
          expect(lines[0]).toBe(expectations.header);
          expect(lines[1]).toStartWith(
            `  ${difference.stdout.slice(PREFIX.length)}`,
          );
        } else {
          expect(actual.stdout).toBe(difference.stdout);
        }
      });
    }

    test("every difference is an invocation, has a reason, and is in the README", () => {
      const names = new Set(invocations.map((i) => i.name));
      const readme = readFileSync(join(fixtures, "README.md"), "utf8");
      for (const [name, difference] of Object.entries(
        expectations.differences,
      )) {
        expect(names.has(name)).toBe(true);
        expect(difference.reason.length).toBeGreaterThan(0);
        expect(readme).toContain(`\`${name}\``);
      }
    });
  });
};
