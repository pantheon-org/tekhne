#!/usr/bin/env bun
/**
 * The part that the schema validators of the project-management skills share: read each file, report a file that does not
 * exist, bytes that are not UTF-8 and invalid YAML as problems of that file, hand the parsed document to a check, and
 * print the python validators' lines. Uses bun built-ins and node: modules only, so an installed skill (which has no
 * node_modules) runs as it is.
 *
 * This file is copied into each skill that has a validator, and the copies must stay byte for byte identical: a test in
 * the tekhne repository (scripts/validator-recording/validator-skeleton.test.ts) fails if they differ.
 */

import { existsSync, readdirSync, readFileSync, realpathSync } from "node:fs";
import { basename, dirname, join } from "node:path";

export type Check = (file: string, data: Record<string, unknown>) => string[];
export type ValidatorSpec = {
  header: string;
  okLabel: string;
  yamlSeparator: string;
  check: Check;
};

// A value the way python prints it: 'x' for a string, True, False, None, and [a, b] for a list.
export const formatPyValue = (value: unknown): string => {
  if (value === null || value === undefined) return "None";
  if (typeof value === "boolean") return value ? "True" : "False";
  if (typeof value === "string") return `'${value}'`;
  if (Array.isArray(value)) return `[${value.map(formatPyValue).join(", ")}]`;
  return String(value);
};

// The python name of a parsed value's type, for the "must be a mapping (got ...)" lines.
export const pyTypeName = (value: unknown): string => {
  if (value === null || value === undefined) return "NoneType";
  if (Array.isArray(value)) return "list";
  if (typeof value === "string") return "str";
  if (typeof value === "boolean") return "bool";
  if (typeof value === "number")
    return Number.isInteger(value) ? "int" : "float";
  return "dict";
};

export const isMapping = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);

// Python truthiness, for "if not x".
export const isEmpty = (value: unknown): boolean =>
  value === null ||
  value === undefined ||
  value === false ||
  value === 0 ||
  value === "" ||
  (Array.isArray(value) && value.length === 0) ||
  (isMapping(value) && Object.keys(value).length === 0);

// The one file with this exact name under the skill that holds the script: walk up from the real path of `startDir` to
// SKILL.md, then look under that folder. None or several is an error that names what was found.
export const findSchemaPath = (
  startDir: string,
  schemaName: string,
): string => {
  let dir = realpathSync(startDir);
  while (!existsSync(join(dir, "SKILL.md"))) {
    const parent = dirname(dir);
    if (parent === dir) throw new Error(`no SKILL.md found above ${startDir}`);
    dir = parent;
  }
  const found = (readdirSync(dir, { recursive: true }) as string[])
    .filter((path) => basename(path) === schemaName)
    .map((path) => join(dir, path));
  if (found.length === 1) return found[0];
  throw new Error(
    found.length === 0
      ? `${schemaName} not found under ${dir}`
      : `${found.length} files named ${schemaName} under ${dir}: ${found.join(", ")}`,
  );
};

// Reads and checks one file. The problems of the file, as lines; none means the file is valid.
const checkFile = (file: string, spec: ValidatorSpec): string[] => {
  if (!existsSync(file)) return [`${file}: file not found`];
  let text: string;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(file));
  } catch {
    return [`${file}: not valid UTF-8`];
  }
  let data: unknown;
  try {
    data = Bun.YAML.parse(text);
  } catch (error) {
    const detail = String((error as Error).message).split("\n")[0];
    return [`${file}: invalid YAML ${spec.yamlSeparator} ${detail}`];
  }
  if (!isMapping(data))
    return [`${file}: root must be a mapping (got ${pyTypeName(data)})`];
  return spec.check(file, data);
};

// Validates every file and returns what the command prints and its exit status. No files is a success that prints nothing.
export const validateFiles = (
  files: string[],
  spec: ValidatorSpec,
): { stdout: string; exitCode: number } => {
  if (files.length === 0) return { stdout: "", exitCode: 0 };
  const errors = files.flatMap((file) => checkFile(file, spec));
  if (errors.length === 0)
    return {
      stdout: `OK: ${files.length} file(s) validated against ${spec.okLabel} schema\n`,
      exitCode: 0,
    };
  return {
    stdout: `${spec.header}\n${errors.map((e) => `  ${e}`).join("\n")}\n`,
    exitCode: 1,
  };
};
