import { describe, expect, test } from "bun:test";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  checkReport,
  findSchemaPath,
  formatPyValue,
} from "../../skills/project-mgmt/plan-review/scripts/validate-review-report";

const SKILL = join(import.meta.dir, "../../skills/project-mgmt/plan-review");
const SCRIPT = join(SKILL, "scripts/validate-review-report.ts");
const FIXTURES = join(import.meta.dir, "fixtures");

type Run = { stdout: string; stderr: string; exit: number };

// Runs the TypeScript directly, from the fixtures folder so paths are relative, as the recording was made.
const run = (args: string[]): Run => {
  const result = Bun.spawnSync(["bun", "--no-install", SCRIPT, ...args], {
    cwd: FIXTURES,
  });
  return {
    stdout: result.stdout.toString(),
    stderr: result.stderr.toString(),
    exit: result.exitCode,
  };
};

const expected = (name: string): Run => ({
  stdout: readFileSync(join(FIXTURES, "expected", `${name}.stdout`), "utf8"),
  stderr: readFileSync(join(FIXTURES, "expected", `${name}.stderr`), "utf8"),
  exit: Number(
    readFileSync(join(FIXTURES, "expected", `${name}.exit`), "utf8"),
  ),
});

const invocations = readFileSync(join(FIXTURES, "invocations.txt"), "utf8")
  .split("\n")
  .filter(Boolean)
  .map((line) => {
    const [name, args] = line.split("|");
    return { name, args: args ? args.split(" ") : [] };
  });

const HEADER = "Review report validation errors:";
const HEX_ERROR =
  "report 'plan_hash' must be 64 lowercase hex characters (shasum -a 256)";

// The named differences from the python version, see fixtures/README.md. Everything else is compared exactly.
const differences: Record<string, (r: Run) => void> = {
  // D1: a file that does not exist fails instead of being skipped.
  "missing-file": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout).toBe(
      `${HEADER}\n  cases/does-not-exist.fixture: file not found\n`,
    );
  },
  "missing-and-valid": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout).toBe(
      `${HEADER}\n  cases/does-not-exist.fixture: file not found\n`,
    );
  },
  "missing-and-bad": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout).toBe(
      `${HEADER}\n  cases/does-not-exist.fixture: file not found\n  cases/hash-short.fixture: ${HEX_ERROR}\n`,
    );
  },
  // D6: bun reads YAML 1.2, python reads YAML 1.1.
  "plan-yes": (r) => {
    expect(r.exit).toBe(0);
    expect(r.stdout).toBe(
      "OK: 1 file(s) validated against review-report schema\n",
    );
  },
  "plan-date": (r) => {
    expect(r.exit).toBe(0);
    expect(r.stdout).toBe(
      "OK: 1 file(s) validated against review-report schema\n",
    );
  },
  "hash-e-notation-unquoted": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout).toBe(
      `${HEADER}\n  cases/hash-e-notation-unquoted.fixture: ${HEX_ERROR}\n`,
    );
  },
  // D7: bun cannot reproduce the parser's text, so only the status and the shape of the first line are compared.
  "invalid-yaml": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout.split("\n")[0]).toBe(HEADER);
    expect(r.stdout.split("\n")[1]).toStartWith(
      "  cases/invalid-yaml.fixture: invalid YAML",
    );
  },
  "two-documents": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout.split("\n")[0]).toBe(HEADER);
  },
  "tab-indent": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout.split("\n")[0]).toBe(HEADER);
  },
  // Python crashes with a traceback; the port reports it.
  "non-utf8-comment": (r) => {
    expect(r.exit).toBe(1);
    expect(r.stdout).toBe(
      `${HEADER}\n  cases/non-utf8-comment.fixture: not valid UTF-8\n`,
    );
  },
};

describe("validate-review-report.ts against the recorded python output", () => {
  for (const { name, args } of invocations) {
    test(name, () => {
      const actual = run(args);
      const difference = differences[name];
      if (difference) {
        difference(actual);
        return;
      }
      const recorded = expected(name);
      expect(actual.exit).toBe(recorded.exit);
      expect(actual.stdout).toBe(recorded.stdout);
      expect(actual.stderr).toBe(recorded.stderr);
    });
  }
});

describe("formatPyValue", () => {
  test("writes values the way python prints them", () => {
    expect(formatPyValue([1, 2])).toBe("[1, 2]");
    expect(formatPyValue(["technical", "legal"])).toBe(
      "['technical', 'legal']",
    );
    expect(formatPyValue(null)).toBe("None");
    expect(formatPyValue(true)).toBe("True");
    expect(formatPyValue(false)).toBe("False");
    expect(formatPyValue("x")).toBe("'x'");
    expect(formatPyValue([])).toBe("[]");
  });
});

describe("findSchemaPath", () => {
  const skill = (): string => {
    const dir = mkdtempSync(join(tmpdir(), "validate-review-report-"));
    writeFileSync(join(dir, "SKILL.md"), "# skill\n");
    mkdirSync(join(dir, "scripts"), { recursive: true });
    mkdirSync(join(dir, "assets/schemas"), { recursive: true });
    return dir;
  };

  test("walks up to SKILL.md and finds the schema by its exact name", () => {
    const dir = skill();
    const schema = join(dir, "assets/schemas/review-report.schema.json");
    writeFileSync(schema, "{}");
    writeFileSync(join(dir, "assets/schemas/other.schema.json"), "{}");
    expect(findSchemaPath(join(dir, "scripts"))).toBe(schema);
    rmSync(dir, { recursive: true, force: true });
  });

  test("fails and says so when the skill has no such schema", () => {
    const dir = skill();
    expect(() => findSchemaPath(join(dir, "scripts"))).toThrow(
      "review-report.schema.json",
    );
    rmSync(dir, { recursive: true, force: true });
  });

  test("fails and names both when the skill has two", () => {
    const dir = skill();
    writeFileSync(join(dir, "assets/schemas/review-report.schema.json"), "{}");
    mkdirSync(join(dir, "references"), { recursive: true });
    writeFileSync(join(dir, "references/review-report.schema.json"), "{}");
    expect(() => findSchemaPath(join(dir, "scripts"))).toThrow(/2 /);
    rmSync(dir, { recursive: true, force: true });
  });

  test("fails when no SKILL.md is above the start folder", () => {
    const dir = mkdtempSync(join(tmpdir(), "validate-review-report-"));
    expect(() => findSchemaPath(dir)).toThrow("SKILL.md");
    rmSync(dir, { recursive: true, force: true });
  });

  test("follows a symlinked folder to the real skill", () => {
    const dir = skill();
    const schema = join(dir, "assets/schemas/review-report.schema.json");
    writeFileSync(schema, "{}");
    const link = join(
      mkdtempSync(join(tmpdir(), "validate-review-report-")),
      "linked",
    );
    symlinkSync(join(dir, "scripts"), link);
    expect(existsSync(link)).toBe(true);
    expect(findSchemaPath(link)).toBe(schema);
    rmSync(dir, { recursive: true, force: true });
  });
});

describe("checkReport", () => {
  const schema = {
    properties: {
      report: {
        required: ["title", "plan", "lenses_run", "plan_hash", "sections"],
        properties: {
          lenses_run: {
            items: { enum: ["technical", "strategic", "risk"] },
          },
          sections: { required: ["scores"] },
        },
      },
    },
  };
  const good = {
    report: {
      title: "t",
      plan: "../plans/p.md",
      lenses_run: ["technical"],
      plan_hash: "a".repeat(64),
      sections: { scores: {} },
    },
  };

  test("returns no error for a good report", () => {
    expect(checkReport("f", good, schema)).toEqual([]);
  });

  test("says a root that is not a mapping, with the python type name", () => {
    expect(checkReport("f", [1], schema)).toEqual([
      "f: root must be a mapping (got list)",
    ]);
    expect(checkReport("f", "x", schema)).toEqual([
      "f: root must be a mapping (got str)",
    ]);
    expect(checkReport("f", null, schema)).toEqual([
      "f: root must be a mapping (got NoneType)",
    ]);
  });

  test("says a missing report key, and an empty report", () => {
    expect(checkReport("f", { other: 1 }, schema)).toEqual([
      "f: missing required top-level key 'report'",
    ]);
    expect(checkReport("f", { report: {} }, schema)[0]).toBe(
      "f: missing required top-level key 'report'",
    );
  });

  test("checks each of the three fields", () => {
    const bad = {
      report: { ...good.report, plan: "", lenses_run: [], plan_hash: "xyz" },
    };
    expect(checkReport("f", bad, schema)).toEqual([
      "f: report 'plan' must be a non-empty string",
      "f: report 'lenses_run' must be a non-empty list",
      "f: report 'plan_hash' must be 64 lowercase hex characters (shasum -a 256)",
    ]);
  });

  test("names the unknown lenses with python formatting", () => {
    const bad = { report: { ...good.report, lenses_run: ["legal"] } };
    expect(checkReport("f", bad, schema)).toEqual([
      "f: report 'lenses_run' has ['legal'], allowed ['technical', 'strategic', 'risk']",
    ]);
  });

  test("says a missing required section", () => {
    const bad = { report: { ...good.report, sections: { other: {} } } };
    expect(checkReport("f", bad, schema)).toEqual([
      "f: sections missing required field 'scores'",
    ]);
  });
});
