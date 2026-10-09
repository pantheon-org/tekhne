import { describe, expect, test } from "bun:test";
import {
  chmodSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";

const SCRIPT = join(import.meta.dir, "record-run.sh");

// A stand-in for the real `claude` command. It records how it was called and
// prints a tiny stream, so these tests never call a model.
const stub = (exitCode = 0): { bin: string; log: string } => {
  const dir = mkdtempSync(join(tmpdir(), "record-run-stub-"));
  const log = join(dir, "calls.log");
  const body = `#!/usr/bin/env bash
{ echo "pwd=$(pwd)"; echo "argc=$#"; for a in "$@"; do echo "arg=$a"; done; } > "${log}"
echo '{"type":"result","subtype":"success","is_error":false,"result":"stub output"}'
exit ${exitCode}
`;
  writeFileSync(join(dir, "claude"), body);
  chmodSync(join(dir, "claude"), 0o755);
  return { bin: dir, log };
};

const makeFixture = (): string => {
  const dir = mkdtempSync(join(tmpdir(), "record-run-fixture-"));
  mkdirSync(join(dir, "skills", "demo"), { recursive: true });
  writeFileSync(
    join(dir, "skills", "demo", "SKILL.md"),
    "# a skill under audit\n",
  );
  return dir;
};

const run = (
  args: string[],
  s: { bin: string },
): { code: number; out: string; err: string } => {
  const p = Bun.spawnSync(["bash", SCRIPT, ...args], {
    env: { ...process.env, PATH: `${s.bin}:${process.env.PATH}` },
  });
  return {
    code: p.exitCode ?? -1,
    out: p.stdout.toString(),
    err: p.stderr.toString(),
  };
};

const skillFile = (): string => {
  const dir = mkdtempSync(join(tmpdir(), "record-run-skill-"));
  const f = join(dir, "SKILL.md");
  writeFileSync(f, "# the version under test\n");
  return f;
};

const baseArgs = (outFile: string, fixture: string): string[] => [
  "--skill-file",
  skillFile(),
  "--skill-name",
  "skill-quality-auditor",
  "--fixture",
  fixture,
  "--prompt",
  "Use the skill to audit skills/demo/SKILL.md.",
  "--out",
  outFile,
];

const argsOf = (log: string): string[] =>
  readFileSync(log, "utf8")
    .split("\n")
    .filter((l) => l.startsWith("arg="))
    .map((l) => l.slice(4));

describe("record-run.sh", () => {
  test("writes the stream the agent printed and names the fixture it used", () => {
    const s = stub();
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    const r = run(baseArgs(out, makeFixture()), s);
    expect(r.code).toBe(0);
    expect(readFileSync(out, "utf8")).toContain("stub output");
    expect(r.out).toMatch(/^fixture=\/.+/m);
  });

  test("runs the agent inside a fresh copy of the fixture, not the original", () => {
    const s = stub();
    const fixture = makeFixture();
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    const r = run(baseArgs(out, fixture), s);
    const used = /^fixture=(.+)$/m.exec(r.out)?.[1] ?? "";
    expect(used).not.toBe(fixture);
    // macOS reports /private/var for /var, so compare the folder name, not the full path.
    expect(readFileSync(s.log, "utf8")).toContain(basename(used));
    expect(existsSync(join(used, "skills", "demo", "SKILL.md"))).toBe(true);
  });

  test("installs the skill version under test where the agent will find it", () => {
    const s = stub();
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    const r = run(baseArgs(out, makeFixture()), s);
    const used = /^fixture=(.+)$/m.exec(r.out)?.[1] ?? "";
    expect(
      readFileSync(
        join(used, ".claude", "skills", "skill-quality-auditor", "SKILL.md"),
        "utf8",
      ),
    ).toBe("# the version under test\n");
  });

  test("commits the fixture first, so anything the agent changes shows in git status", () => {
    const s = stub();
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    const r = run(baseArgs(out, makeFixture()), s);
    const used = /^fixture=(.+)$/m.exec(r.out)?.[1] ?? "";
    const status = Bun.spawnSync(["git", "-C", used, "status", "--porcelain"]);
    expect(status.exitCode).toBe(0);
    expect(status.stdout.toString()).toBe("");
  });

  test("limits the agent: a fixed tool list, project settings only, no MCP, no stored session, a cost cap", () => {
    const s = stub();
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    run(baseArgs(out, makeFixture()), s);
    const a = argsOf(s.log);
    const has = (flag: string, value?: string): boolean => {
      const i = a.indexOf(flag);
      return i >= 0 && (value === undefined || a[i + 1] === value);
    };
    expect(a[0]).toBe("-p");
    expect(a[1]).toBe("Use the skill to audit skills/demo/SKILL.md.");
    expect(has("--output-format", "stream-json")).toBe(true);
    expect(has("--verbose")).toBe(true);
    expect(has("--setting-sources", "project")).toBe(true);
    expect(has("--strict-mcp-config")).toBe(true);
    expect(has("--no-session-persistence")).toBe(true);
    expect(has("--permission-mode", "dontAsk")).toBe(true);
    expect(has("--max-budget-usd", "0.50")).toBe(true);
    expect(has("--model", "sonnet")).toBe(true);
    const i = a.indexOf("--allowedTools");
    expect(a.slice(i + 1, i + 5)).toEqual([
      "Read",
      "Glob",
      "Grep",
      "Bash(pantheon-skill-auditor:*)",
    ]);
    // The prompt comes before the variadic tool list, or the list would swallow it.
    expect(a.indexOf("-p") + 1).toBeLessThan(i);
  });

  test("model and budget can be set", () => {
    const s = stub();
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    run(
      [...baseArgs(out, makeFixture()), "--model", "haiku", "--budget", "0.10"],
      s,
    );
    const a = argsOf(s.log);
    expect(a[a.indexOf("--model") + 1]).toBe("haiku");
    expect(a[a.indexOf("--max-budget-usd") + 1]).toBe("0.10");
  });

  test("a missing option is an error that names it", () => {
    const s = stub();
    const r = run(["--skill-name", "x"], s);
    expect(r.code).not.toBe(0);
    expect(r.err).toMatch(/--skill-file/);
  });

  test("a fixture that does not exist is an error", () => {
    const s = stub();
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    const r = run(baseArgs(out, "/no/such/fixture"), s);
    expect(r.code).not.toBe(0);
    expect(r.err).toMatch(/fixture/);
  });

  test("when the agent command fails the script fails and says so", () => {
    const s = stub(3);
    const out = join(
      mkdtempSync(join(tmpdir(), "record-run-out-")),
      "run.jsonl",
    );
    const r = run(baseArgs(out, makeFixture()), s);
    expect(r.code).not.toBe(0);
    expect(r.err).toMatch(/claude exited with 3/);
  });
});
