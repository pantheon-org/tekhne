#!/usr/bin/env bun
/**
 * Validates TypeScript conventions for cli/ source files.
 *
 * Rules enforced:
 *   1. Arrow functions only — no named function declarations
 *   2. One function max per module — non-barrel files export exactly one arrow function
 *   3. No internal functions — no arrow functions nested inside another function body
 *   4. Function body ≤ 150 lines
 *
 * Usage: bun cli/scripts/validate-ts-conventions.ts [file ...]
 * Exit 1 if any violation is found.
 */

type Violation = { file: string; line: number; message: string };
type LineState = {
  braceDepth: number;
  fnStartLine: number;
  fnBodyOpenDepth: number;
  fnLineCount: number;
  exportedFnCount: number;
};

// Top-level (no leading whitespace) exported arrow function
const TOP_LEVEL_EXPORT_FN = /^export const \w+ = (async )?\(/;
// Indented arrow function definition (internal function indicator).
// Requires `=>` on the same line to avoid false-positives on expressions like `const x = (a + b)`.
const INDENTED_ARROW_FN =
  /^\s+const \w+ = (async )?(\([^)]*\)\s*(:\s*\S+\s*)?=>|[a-zA-Z_$][\w$]*\s*=>)/;
// Named function declaration (forbidden)
const NAMED_FN_DECL = /^(export )?(export default )?(async )?function\s+\w+/;

const countChar = (s: string, ch: string): number =>
  (s.match(new RegExp(`\\${ch}`, "g")) ?? []).length;

const checkNamedFn = (
  line: string,
  lineNum: number,
  file: string,
): Violation | null =>
  NAMED_FN_DECL.test(line.trimStart())
    ? {
        file,
        line: lineNum,
        message:
          "Named function declaration. Use `const name = [async] () => {...}` instead.",
      }
    : null;

const checkInternalFn = (
  line: string,
  lineNum: number,
  braceDepth: number,
  file: string,
): Violation | null =>
  INDENTED_ARROW_FN.test(line) && braceDepth > 1
    ? {
        file,
        line: lineNum,
        message:
          "Internal function definition. Extract to a separate module-level file.",
      }
    : null;

const checkExtraExport = (
  line: string,
  lineNum: number,
  count: number,
  file: string,
): Violation | null =>
  TOP_LEVEL_EXPORT_FN.test(line) && count > 1
    ? {
        file,
        line: lineNum,
        message:
          "Multiple exported functions in one module. Each module must export exactly one function.",
      }
    : null;

const updateFnTracking = (
  state: LineState,
  line: string,
  lineNum: number,
): void => {
  if (TOP_LEVEL_EXPORT_FN.test(line)) {
    state.exportedFnCount++;
    state.fnStartLine = lineNum;
    state.fnBodyOpenDepth = state.braceDepth + 1;
    state.fnLineCount = 0;
  }
};

const checkFnLength = (state: LineState, file: string): Violation | null => {
  if (state.fnStartLine === -1 || state.braceDepth < state.fnBodyOpenDepth)
    return null;
  state.fnLineCount++;
  if (state.fnLineCount > 150) {
    const v: Violation = {
      file,
      line: state.fnStartLine,
      message: `Function body exceeds 150 lines (${state.fnLineCount}+ lines). Split into smaller modules.`,
    };
    state.fnStartLine = -1; // report once
    return v;
  }
  return null;
};

const checkLine = (
  line: string,
  lineNum: number,
  file: string,
  state: LineState,
): Violation[] => {
  const vs: Violation[] = [];
  const namedFn = checkNamedFn(line, lineNum, file);
  if (namedFn) vs.push(namedFn);
  const internalFn = checkInternalFn(line, lineNum, state.braceDepth, file);
  if (internalFn) vs.push(internalFn);
  updateFnTracking(state, line, lineNum);
  const extraExport = checkExtraExport(
    line,
    lineNum,
    state.exportedFnCount,
    file,
  );
  if (extraExport) vs.push(extraExport);
  const lengthViolation = checkFnLength(state, file);
  if (lengthViolation) vs.push(lengthViolation);
  state.braceDepth += countChar(line, "{") - countChar(line, "}");
  if (state.braceDepth < 0) state.braceDepth = 0;
  if (state.fnStartLine !== -1 && state.braceDepth < state.fnBodyOpenDepth)
    state.fnStartLine = -1;
  return vs;
};

const checkFile = (file: string, content: string): Violation[] => {
  if (file.endsWith("/index.ts") || file.endsWith(".test.ts")) return [];

  const lines = content.split("\n");
  const state: LineState = {
    braceDepth: 0,
    fnStartLine: -1,
    fnBodyOpenDepth: -1,
    fnLineCount: 0,
    exportedFnCount: 0,
  };

  return lines.flatMap((line, i) => checkLine(line, i + 1, file, state));
};

const main = async () => {
  const args = process.argv.slice(2);
  const files = args.filter(
    (f) => f.endsWith(".ts") && f.includes("cli/") && !f.endsWith(".test.ts"),
  );

  if (files.length === 0) {
    console.log("No CLI TypeScript files to check.");
    process.exit(0);
  }

  const allViolations: Violation[] = [];

  for (const file of files) {
    const bunFile = Bun.file(file);
    const exists = await bunFile.exists();
    if (!exists) continue;
    const content = await bunFile.text();
    allViolations.push(...checkFile(file, content));
  }

  if (allViolations.length === 0) {
    console.log(
      `✓ TypeScript conventions OK (${files.length} file(s) checked)`,
    );
    process.exit(0);
  }

  const seen = new Set<string>();
  for (const v of allViolations) {
    const key = `${v.file}:${v.line}:${v.message}`;
    if (seen.has(key)) continue;
    seen.add(key);
    console.error(`${v.file}:${v.line}: ✗ ${v.message}`);
  }

  console.error(
    `\n${seen.size} convention violation(s) found. See cli/CONVENTIONS.md for rules.`,
  );
  process.exit(1);
};

await main();
