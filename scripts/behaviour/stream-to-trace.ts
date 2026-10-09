#!/usr/bin/env bun
// Turn a Claude Code stream-json recording into a behaviour trace
// (evals/scenario-N/traces/old.json or new.json) for the skill-quality-auditor.
//
//   bun scripts/behaviour/stream-to-trace.ts \
//     --stream run.jsonl --skill-commit <sha> [--fixture <dir>] \
//     [--harness claude-code] [--date YYYY-MM-DD] [--source recorded|hand-written]
//
// The stream comes from `claude -p --output-format stream-json --verbose`.
// Files changed are read with `git status` in the fixture folder, so commit the
// fixture before the run. Derivations follow skilldiff's src/trace-utils.ts and
// scripts/adapters.ts (MIT, commit 9271c26); see crates/skill-auditor/NOTICE.
//
// Verified so far only against a hand-written sample of the stream shape.

import { readFileSync } from "node:fs";

export type Source = "recorded" | "hand-written";

export interface ToolCall {
  tool: string;
  input: Record<string, unknown>;
}

export interface Trace {
  schema_version: 1;
  header: {
    harness: string;
    date: string;
    skill_commit: string;
    source: Source;
  };
  toolCalls: ToolCall[];
  filesChanged: string[];
  commandsRun: string[];
  output: string;
  warnings: string[];
}

export interface Options {
  skillCommit: string;
  harness?: string;
  date?: string;
  source?: Source;
  /** A git folder holding the fixture, committed before the run. */
  fixture?: string;
}

interface Block {
  type?: string;
  name?: string;
  text?: string;
  input?: Record<string, unknown>;
}

interface Event {
  type?: string;
  subtype?: string;
  is_error?: boolean;
  result?: unknown;
  message?: { content?: Block[] };
}

const parseLines = (stream: string): Event[] =>
  stream.split("\n").flatMap((raw, i) => {
    if (raw.trim() === "") return [];
    try {
      return [JSON.parse(raw) as Event];
    } catch {
      throw new Error(`stream line ${i + 1} is not valid JSON`);
    }
  });

const isShellTool = (name: string): boolean => {
  const n = name.toLowerCase();
  return ["bash", "exec_command", "shell", "command_execution"].some((f) =>
    n.includes(f),
  );
};

const commandOf = (input: Record<string, unknown>): string | undefined => {
  const c = input.command ?? input.cmd ?? input.script;
  if (typeof c === "string") return c;
  if (Array.isArray(c)) return c.join(" ");
  return undefined;
};

/** Paths the agent changed or added in the fixture, from `git status`. */
export const changedFiles = (fixture: string): string[] => {
  const p = Bun.spawnSync([
    "git",
    "-C",
    fixture,
    "status",
    "--porcelain",
    "--untracked-files=all",
  ]);
  if (p.exitCode !== 0) return [];
  return p.stdout
    .toString()
    .split("\n")
    .filter((l) => l.trim() !== "")
    .map((l) => l.slice(3))
    .map((path) =>
      path.includes(" -> ") ? (path.split(" -> ").pop() as string) : path,
    )
    .map((path) => path.replace(/^"(.*)"$/, "$1"));
};

const assistantBlocks = (events: Event[]): Block[] =>
  events
    .filter((e) => e.type === "assistant")
    .flatMap((e) => e.message?.content ?? []);

const toolCallsOf = (blocks: Block[]): ToolCall[] =>
  blocks
    .filter((b) => b.type === "tool_use" && b.name)
    .map((b) => ({ tool: b.name as string, input: b.input ?? {} }));

const textsOf = (blocks: Block[]): string[] =>
  blocks
    .filter((b) => b.type === "text" && typeof b.text === "string")
    .map((b) => b.text as string);

/** The final text and anything worth warning about, from the result event. */
const outcomeOf = (
  events: Event[],
  texts: string[],
): { output: string; warnings: string[] } => {
  const result = [...events].reverse().find((e) => e.type === "result");
  if (!result) {
    return {
      output: texts.at(-1) ?? "",
      warnings: [
        "no result event in the stream; the output is the last assistant text",
      ],
    };
  }
  const warnings: string[] = [];
  if (result.is_error) warnings.push("the run ended in an error");
  if (result.subtype === "error_max_turns")
    warnings.push("the run stopped at the turn limit");
  return {
    output: typeof result.result === "string" ? result.result : "",
    warnings,
  };
};

const commandsOf = (toolCalls: ToolCall[]): string[] =>
  toolCalls
    .filter((c) => isShellTool(c.tool))
    .map((c) => commandOf(c.input))
    .filter((c): c is string => c !== undefined);

export const streamToTrace = (stream: string, opts: Options): Trace => {
  if (!opts.skillCommit || opts.skillCommit.trim() === "") {
    throw new Error(
      "the skill commit is required: say which SKILL.md this run was recorded against",
    );
  }
  const events = parseLines(stream);
  const blocks = assistantBlocks(events);
  const toolCalls = toolCallsOf(blocks);
  const { output, warnings } = outcomeOf(events, textsOf(blocks));
  const commandsRun = commandsOf(toolCalls);

  return {
    schema_version: 1,
    header: {
      harness: opts.harness ?? "claude-code",
      date: opts.date ?? new Date().toISOString().slice(0, 10),
      skill_commit: opts.skillCommit,
      source: opts.source ?? "recorded",
    },
    toolCalls,
    filesChanged: opts.fixture ? changedFiles(opts.fixture) : [],
    commandsRun,
    output,
    warnings,
  };
};

const parseArgs = (argv: string[]): Record<string, string> => {
  const out: Record<string, string> = {};
  for (let i = 0; i < argv.length; i += 2) {
    const key = argv[i];
    if (!key?.startsWith("--") || argv[i + 1] === undefined) {
      throw new Error(
        `expected --name value pairs, got ${JSON.stringify(key)}`,
      );
    }
    out[key.slice(2)] = argv[i + 1];
  }
  return out;
};

if (import.meta.main) {
  try {
    const a = parseArgs(process.argv.slice(2));
    if (!a.stream) throw new Error("--stream <file> is required");
    if (!a["skill-commit"]) throw new Error("--skill-commit <sha> is required");
    if (a.source && a.source !== "recorded" && a.source !== "hand-written") {
      throw new Error("--source must be recorded or hand-written");
    }
    const trace = streamToTrace(readFileSync(a.stream, "utf8"), {
      skillCommit: a["skill-commit"],
      harness: a.harness,
      date: a.date,
      source: a.source as Source | undefined,
      fixture: a.fixture,
    });
    process.stdout.write(`${JSON.stringify(trace, null, 2)}\n`);
  } catch (e) {
    process.stderr.write(`error: ${(e as Error).message}\n`);
    process.exit(1);
  }
}
