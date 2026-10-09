import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { streamToTrace } from "./stream-to-trace";

const SCRIPT = join(import.meta.dir, "stream-to-trace.ts");
const VALIDATOR = join(
  import.meta.dir,
  "../../skills/agentic-harness/skill-quality-auditor/scripts/validate-behaviour-files.sh",
);

// A hand-written sample in the shape of Claude Code's stream-json output. It is
// NOT a real recording: the converter has to be proven against a live run too.
const line = (o: unknown): string => JSON.stringify(o);
const SAMPLE = [
  line({
    type: "system",
    subtype: "init",
    session_id: "s1",
    tools: ["Read", "Bash"],
  }),
  line({
    type: "assistant",
    message: {
      content: [
        { type: "text", text: "I will read the skill first." },
        {
          type: "tool_use",
          id: "t1",
          name: "Read",
          input: { file_path: "skills/sql/SKILL.md" },
        },
      ],
    },
  }),
  line({
    type: "user",
    message: {
      content: [{ type: "tool_result", tool_use_id: "t1", content: "# SQL" }],
    },
  }),
  line({
    type: "assistant",
    message: {
      content: [
        {
          type: "tool_use",
          id: "t2",
          name: "Bash",
          input: { command: "pantheon-skill-auditor evaluate sql --json" },
        },
      ],
    },
  }),
  line({
    type: "result",
    subtype: "success",
    is_error: false,
    result: "Knowledge Delta 8/20, total 70/140.",
    num_turns: 3,
  }),
].join("\n");

const opts = { skillCommit: "abc1234", date: "2026-10-09" };

describe("streamToTrace", () => {
  test("lists the tool calls in order with their input", () => {
    const t = streamToTrace(SAMPLE, opts);
    expect(t.toolCalls.map((c) => c.tool)).toEqual(["Read", "Bash"]);
    expect(t.toolCalls[0].input).toEqual({ file_path: "skills/sql/SKILL.md" });
  });

  test("takes the commands from the shell tool calls only", () => {
    const t = streamToTrace(SAMPLE, opts);
    expect(t.commandsRun).toEqual([
      "pantheon-skill-auditor evaluate sql --json",
    ]);
  });

  test("takes the output from the result event", () => {
    const t = streamToTrace(SAMPLE, opts);
    expect(t.output).toBe("Knowledge Delta 8/20, total 70/140.");
  });

  test("writes the header from the options and says it was recorded", () => {
    const t = streamToTrace(SAMPLE, opts);
    expect(t.schema_version).toBe(1);
    expect(t.header).toEqual({
      harness: "claude-code",
      date: "2026-10-09",
      skill_commit: "abc1234",
      source: "recorded",
    });
  });

  test("a hand-written trace can be labelled as such", () => {
    const t = streamToTrace(SAMPLE, { ...opts, source: "hand-written" });
    expect(t.header.source).toBe("hand-written");
  });

  test("needs the commit of the skill it was recorded against", () => {
    expect(() => streamToTrace(SAMPLE, { skillCommit: "" })).toThrow(
      /skill commit/i,
    );
  });

  test("a failed run is recorded with a warning, not hidden", () => {
    const failed = SAMPLE.replace(
      '"subtype":"success","is_error":false',
      '"subtype":"success","is_error":true',
    );
    expect(streamToTrace(failed, opts).warnings.join(" ")).toMatch(/error/i);
  });

  test("a run that hit the turn limit says so", () => {
    const limited = SAMPLE.replace(
      '"subtype":"success"',
      '"subtype":"error_max_turns"',
    );
    expect(streamToTrace(limited, opts).warnings.join(" ")).toMatch(
      /turn limit/i,
    );
  });

  test("a stream with no result event falls back to the last assistant text and warns", () => {
    const withoutResult = SAMPLE.split("\n").slice(0, -1).join("\n");
    const t = streamToTrace(
      `${withoutResult}\n${line({ type: "assistant", message: { content: [{ type: "text", text: "Final words." }] } })}`,
      opts,
    );
    expect(t.output).toBe("Final words.");
    expect(t.warnings.join(" ")).toMatch(/no result/i);
  });

  test("a line that is not JSON is an error that names the line", () => {
    expect(() => streamToTrace(`${SAMPLE}\n{ not json`, opts)).toThrow(
      /line 6/,
    );
  });

  test("blank lines are ignored", () => {
    expect(streamToTrace(`\n${SAMPLE}\n\n`, opts).toolCalls).toHaveLength(2);
  });
});

describe("files changed", () => {
  const git = (dir: string, ...args: string[]): void => {
    const p = Bun.spawnSync(
      [
        "git",
        "-C",
        dir,
        "-c",
        "user.email=t@t",
        "-c",
        "user.name=t",
        "-c",
        "commit.gpgsign=false",
        ...args,
      ],
      {},
    );
    expect(p.exitCode).toBe(0);
  };

  test("lists what the agent changed or added in the fixture, and nothing else", () => {
    const dir = mkdtempSync(join(tmpdir(), "stream-fixture-"));
    git(dir, "init", "-q");
    writeFileSync(join(dir, "kept.md"), "same\n");
    writeFileSync(join(dir, "edited.md"), "before\n");
    git(dir, "add", ".");
    git(dir, "commit", "-q", "-m", "fixture");
    writeFileSync(join(dir, "edited.md"), "after\n");
    mkdirSync(join(dir, "sub"));
    writeFileSync(join(dir, "sub", "new.md"), "new\n");
    const t = streamToTrace(SAMPLE, { ...opts, fixture: dir });
    expect([...t.filesChanged].sort()).toEqual(["edited.md", "sub/new.md"]);
  });

  test("an untouched fixture changes nothing", () => {
    const dir = mkdtempSync(join(tmpdir(), "stream-fixture-"));
    git(dir, "init", "-q");
    writeFileSync(join(dir, "a.md"), "x\n");
    git(dir, "add", ".");
    git(dir, "commit", "-q", "-m", "fixture");
    expect(
      streamToTrace(SAMPLE, { ...opts, fixture: dir }).filesChanged,
    ).toEqual([]);
  });

  test("without a fixture the list is empty", () => {
    expect(streamToTrace(SAMPLE, opts).filesChanged).toEqual([]);
  });
});

describe("command line", () => {
  test("prints a trace that the shell validator accepts", () => {
    const hasTools = ["jq", "yq"].every(
      (t) => Bun.spawnSync(["sh", "-c", `command -v ${t}`]).exitCode === 0,
    );
    if (!hasTools) {
      console.warn("SKIPPED: jq and yq are needed to run the shell validator");
      return;
    }
    const root = mkdtempSync(join(tmpdir(), "stream-cli-"));
    mkdirSync(join(root, "traces"));
    writeFileSync(join(root, "stream.jsonl"), SAMPLE);
    writeFileSync(
      join(root, "behaviour.yaml"),
      "schema_version: 1\nname: n\nexpect:\n  output_contains: [total]\n",
    );
    const run = Bun.spawnSync([
      "bun",
      SCRIPT,
      "--stream",
      join(root, "stream.jsonl"),
      "--skill-commit",
      "abc1234",
      "--date",
      "2026-10-09",
    ]);
    expect(run.exitCode).toBe(0);
    writeFileSync(join(root, "traces", "old.json"), run.stdout.toString());
    writeFileSync(join(root, "traces", "new.json"), run.stdout.toString());
    const v = Bun.spawnSync([
      "sh",
      VALIDATOR,
      join(root, "traces", "old.json"),
    ]);
    expect(v.stderr.toString()).toBe("");
    expect(v.exitCode).toBe(0);
  });

  test("exits non-zero and says what is missing when the commit is not given", () => {
    const root = mkdtempSync(join(tmpdir(), "stream-cli-"));
    writeFileSync(join(root, "stream.jsonl"), SAMPLE);
    const run = Bun.spawnSync([
      "bun",
      SCRIPT,
      "--stream",
      join(root, "stream.jsonl"),
    ]);
    expect(run.exitCode).not.toBe(0);
    expect(run.stderr.toString()).toMatch(/skill-commit/);
  });
});
