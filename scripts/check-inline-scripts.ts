#!/usr/bin/env bun
/**
 * check-inline-scripts.ts
 *
 * Flags a `run:` block in a workflow or composite action that holds more than
 * MAX lines of code (default 15). Longer logic belongs in a script, where it can
 * be run locally, shellchecked and tested; the step then calls it.
 *
 *   - Blank lines and comment lines do not count.
 *   - A block opts out with a line `# inline-ok: <reason>` inside it.
 *
 * Warn-only by default: findings are `::warning::` annotations and the exit code
 * is 0. Pass --strict to make them `::error::` annotations and exit 1.
 *
 * Usage: bun scripts/check-inline-scripts.ts [--strict] [--max N] [file ...]
 *        With no files, checks .github/workflows/*.{yml,yaml} and
 *        .github/actions/* /action.{yml,yaml}.
 * Exit:  0 clean (or warn-only); 1 findings in --strict; 2 unreadable input.
 */
import { readFileSync } from "node:fs";

export const DEFAULT_MAX = 15;

const OPT_OUT = /^\s*#\s*inline-ok:/m;

export type Finding = {
  file: string;
  step: string;
  lines: number;
};

/** Lines that are neither blank nor comments. */
export function countCodeLines(script: string): number {
  return script.split("\n").filter((line) => !/^\s*(#.*)?$/.test(line)).length;
}

type Block = { step: string; script: string };

/** Every string `run:` value in a parsed workflow or action, with its step name. */
export function collectRunBlocks(node: unknown): Block[] {
  if (Array.isArray(node)) {
    return node.flatMap(collectRunBlocks);
  }
  if (node === null || typeof node !== "object") {
    return [];
  }
  const map = node as Record<string, unknown>;
  const own: Block[] = [];
  if (typeof map.run === "string") {
    const first = map.run.split("\n").find((l) => !/^\s*(#.*)?$/.test(l));
    const step =
      typeof map.name === "string"
        ? map.name
        : (first ?? "").trim().slice(0, 60);
    own.push({ step, script: map.run });
  }
  return own.concat(Object.values(map).flatMap(collectRunBlocks));
}

export function findLongBlocks(
  yamlText: string,
  file: string,
  max = DEFAULT_MAX,
): Finding[] {
  const parsed: unknown = Bun.YAML.parse(yamlText);
  return collectRunBlocks(parsed)
    .filter((b) => !OPT_OUT.test(b.script))
    .map((b) => ({ file, step: b.step, lines: countCodeLines(b.script) }))
    .filter((f) => f.lines > max);
}

export function annotation(f: Finding, max: number, strict: boolean): string {
  const level = strict ? "error" : "warning";
  return `::${level} file=${f.file}::Step "${f.step}" has ${f.lines} lines of code in run: (limit ${max}). Move it to a script and call it, or add '# inline-ok: <reason>' inside the block.`;
}

async function defaultFiles(): Promise<string[]> {
  const patterns = [
    ".github/workflows/*.{yml,yaml}",
    ".github/actions/*/action.{yml,yaml}",
  ];
  const found: string[] = [];
  for (const pattern of patterns) {
    for await (const path of new Bun.Glob(pattern).scan(".")) {
      found.push(path);
    }
  }
  return found.sort();
}

type Options = { strict: boolean; max: number; files: string[] };

export function parseArgs(argv: string[]): Options {
  const options: Options = { strict: false, max: DEFAULT_MAX, files: [] };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === "--strict") {
      options.strict = true;
    } else if (arg === "--max") {
      options.max = Number(argv[++i]);
    } else {
      options.files.push(arg);
    }
  }
  if (!Number.isInteger(options.max) || options.max < 1) {
    throw new Error("--max needs a positive integer");
  }
  return options;
}

async function main(): Promise<number> {
  const options = parseArgs(process.argv.slice(2));
  const files = options.files.length > 0 ? options.files : await defaultFiles();
  const findings: Finding[] = [];
  for (const file of files) {
    try {
      findings.push(
        ...findLongBlocks(readFileSync(file, "utf8"), file, options.max),
      );
    } catch (error) {
      console.error(
        `::error file=${file}::Could not read or parse the file: ${String(error)}`,
      );
      return 2;
    }
  }
  for (const f of findings) {
    console.log(annotation(f, options.max, options.strict));
  }
  const checked = `${files.length} file(s) checked`;
  if (findings.length === 0) {
    console.log(
      `No run block exceeds ${options.max} lines of code (${checked}).`,
    );
    return 0;
  }
  const mode = options.strict ? "failing" : "warn-only, not failing";
  console.log(
    `${findings.length} run block(s) exceed ${options.max} lines of code (${checked}; ${mode}).`,
  );
  return options.strict ? 1 : 0;
}

if (import.meta.main) {
  process.exit(await main());
}
