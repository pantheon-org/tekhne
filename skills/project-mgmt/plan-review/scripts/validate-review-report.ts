#!/usr/bin/env bun
/**
 * Validates plan-review report YAML files against the review-report schema.
 *
 * Usage: validate-review-report.ts <file> [<file> ...]
 *
 * Uses bun built-ins and node: modules only, so an installed copy of the skill (which has no node_modules) runs as it is.
 * Run it through validate-review-report.sh, which checks the bun version.
 *
 * It is a port of the python validator, so the lines it prints are the python lines. It reads the schema for the
 * required lists and the allowed lens names, and checks `plan`, `lenses_run` and `plan_hash` by hand. The differences
 * from the python version (a missing file fails, YAML is read as 1.2, parser text, invalid UTF-8) are listed in
 * scripts/plan-review/fixtures/README.md of the tekhne repository.
 */

import { existsSync, readdirSync, readFileSync, realpathSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const SCHEMA_NAME = "review-report.schema.json";
const HASH = /^[0-9a-f]{64}$/;

type Schema = {
  properties?: {
    report?: {
      required?: string[];
      properties?: {
        lenses_run?: { items?: { enum?: string[] } };
        sections?: { required?: string[] };
      };
    };
  };
};

// A value the way python prints it: 'x' for a string, True, False, None, and [a, b] for a list.
export const formatPyValue = (value: unknown): string => {
  if (value === null || value === undefined) return "None";
  if (typeof value === "boolean") return value ? "True" : "False";
  if (typeof value === "string") return `'${value}'`;
  if (Array.isArray(value)) return `[${value.map(formatPyValue).join(", ")}]`;
  return String(value);
};

// The python name of a parsed value's type, for the "root must be a mapping" line.
const pyTypeName = (value: unknown): string => {
  if (value === null || value === undefined) return "NoneType";
  if (Array.isArray(value)) return "list";
  if (typeof value === "string") return "str";
  if (typeof value === "boolean") return "bool";
  if (typeof value === "number")
    return Number.isInteger(value) ? "int" : "float";
  return "dict";
};

const isMapping = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);

// Python truthiness, for "if not report".
const isEmpty = (value: unknown): boolean =>
  value === null ||
  value === undefined ||
  value === false ||
  value === 0 ||
  value === "" ||
  (Array.isArray(value) && value.length === 0) ||
  (isMapping(value) && Object.keys(value).length === 0);

// The one schema file under the skill that holds the script: walk up from the real path of `startDir` to SKILL.md, then
// look under that folder for the exact file name. A skill with none or several is an error that names what was found.
export const findSchemaPath = (startDir: string): string => {
  let dir = realpathSync(startDir);
  while (!existsSync(join(dir, "SKILL.md"))) {
    const parent = dirname(dir);
    if (parent === dir) throw new Error(`no SKILL.md found above ${startDir}`);
    dir = parent;
  }
  const found = (readdirSync(dir, { recursive: true }) as string[])
    .filter((path) => basename(path) === SCHEMA_NAME)
    .map((path) => join(dir, path));
  if (found.length === 1) return found[0];
  throw new Error(
    found.length === 0
      ? `${SCHEMA_NAME} not found under ${dir}`
      : `${found.length} files named ${SCHEMA_NAME} under ${dir}: ${found.join(", ")}`,
  );
};

type ReportSchema = NonNullable<Schema["properties"]>["report"];
type Report = Record<string, unknown>;

const checkRequired = (
  file: string,
  report: Report,
  schema: ReportSchema,
): string[] =>
  (schema?.required ?? [])
    .filter((field) => !(field in report))
    .map((field) => `${file}: report missing required field '${field}'`);

const checkPlan = (file: string, report: Report): string[] => {
  if (!("plan" in report)) return [];
  const plan = report.plan;
  return typeof plan === "string" && plan.trim() !== ""
    ? []
    : [`${file}: report 'plan' must be a non-empty string`];
};

const checkLenses = (
  file: string,
  report: Report,
  allowed: string[],
): string[] => {
  if (!("lenses_run" in report)) return [];
  const lenses = report.lenses_run;
  if (!Array.isArray(lenses) || lenses.length === 0)
    return [`${file}: report 'lenses_run' must be a non-empty list`];
  const errors: string[] = [];
  const bad = lenses.filter((lens) => !allowed.includes(lens as string));
  if (bad.length > 0)
    errors.push(
      `${file}: report 'lenses_run' has ${formatPyValue(bad)}, allowed ${formatPyValue(allowed)}`,
    );
  if (new Set(lenses.map(String)).size !== lenses.length)
    errors.push(`${file}: report 'lenses_run' lists a lens more than once`);
  if (lenses.length > allowed.length)
    errors.push(
      `${file}: report 'lenses_run' has more than ${allowed.length} entries`,
    );
  return errors;
};

const checkHash = (file: string, report: Report): string[] => {
  if (!("plan_hash" in report)) return [];
  const hash = report.plan_hash;
  return typeof hash === "string" && HASH.test(hash)
    ? []
    : [
        `${file}: report 'plan_hash' must be 64 lowercase hex characters (shasum -a 256)`,
      ];
};

const checkSections = (
  file: string,
  report: Report,
  required: string[],
): string[] => {
  const sections = report.sections ?? {};
  const errors = isEmpty(sections)
    ? [`${file}: report missing 'sections'`]
    : [];
  for (const field of required) {
    if (!isMapping(sections) || !(field in sections))
      errors.push(`${file}: sections missing required field '${field}'`);
  }
  return errors;
};

// The problems of one parsed report, as the python validator words them. No problem is an empty list.
export const checkReport = (
  file: string,
  data: unknown,
  schema: Schema,
): string[] => {
  if (!isMapping(data))
    return [`${file}: root must be a mapping (got ${pyTypeName(data)})`];
  const report = data.report;
  if (isEmpty(report))
    return [`${file}: missing required top-level key 'report'`];
  if (!isMapping(report))
    return [`${file}: report must be a mapping (got ${pyTypeName(report)})`];
  const reportSchema = schema.properties?.report;
  const properties = reportSchema?.properties;
  return [
    ...checkRequired(file, report, reportSchema),
    ...checkPlan(file, report),
    ...checkLenses(file, report, properties?.lenses_run?.items?.enum ?? []),
    ...checkHash(file, report),
    ...checkSections(file, report, properties?.sections?.required ?? []),
  ];
};

// Reads and checks one file: a missing file, bytes that are not UTF-8 and invalid YAML are problems of the file.
const checkFile = (file: string, schema: Schema): string[] => {
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
    return [`${file}: invalid YAML — ${detail}`];
  }
  return checkReport(file, data, schema);
};

const main = (): number => {
  const files = process.argv.slice(2);
  if (files.length === 0) return 0;
  const schema = JSON.parse(
    readFileSync(findSchemaPath(import.meta.dir), "utf8"),
  ) as Schema;
  const errors = files.flatMap((file) => checkFile(file, schema));
  if (errors.length > 0) {
    process.stdout.write(
      `Review report validation errors:\n${errors.map((e) => `  ${e}`).join("\n")}\n`,
    );
    return 1;
  }
  process.stdout.write(
    `OK: ${files.length} file(s) validated against review-report schema\n`,
  );
  return 0;
};

if (import.meta.main) process.exitCode = main();
