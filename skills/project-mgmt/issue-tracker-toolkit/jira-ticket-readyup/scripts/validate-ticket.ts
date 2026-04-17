#!/usr/bin/env bun
/**
 * Validates a ready-for-refinement YAML data file against the JSON schema,
 * then optionally checks that the markdown output has the required sections.
 *
 * Usage:
 *   bun run scripts/validate-ticket.ts <ticket-data.yaml>
 *   bun run scripts/validate-ticket.ts <ticket-data.yaml> --markdown <ticket-output.md>
 */

import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { load } from "js-yaml";

// ── CLI args ──────────────────────────────────────────────────────────────────

const args = process.argv.slice(2);
let yamlFile = "";
let mdFile = "";

for (let i = 0; i < args.length; i++) {
  if ((args[i] === "--markdown" || args[i] === "-m") && args[i + 1]) {
    mdFile = args[++i];
  } else if (!args[i].startsWith("-")) {
    yamlFile = args[i];
  } else {
    console.error(`Unknown flag: ${args[i]}`);
    process.exit(2);
  }
}

if (!yamlFile) {
  console.error(
    "Usage: bun run scripts/validate-ticket.ts <ticket-data.yaml> [--markdown <ticket-output.md>]",
  );
  process.exit(2);
}

// ── Resolve schema path relative to this script ───────────────────────────────

const skillDir = join(import.meta.dir, "..");
const schemaPath = join(
  skillDir,
  "assets/schemas/ready-for-refinement.schema.json",
);

// ── 1. YAML → JSON schema validation ─────────────────────────────────────────

console.log(`Validating ${yamlFile} against schema...`);

let rawData: unknown;
try {
  rawData = load(readFileSync(resolve(yamlFile), "utf8"));
} catch (err) {
  console.error(`  FAIL: Could not read/parse YAML: ${(err as Error).message}`);
  process.exit(3);
}

let schema: unknown;
try {
  schema = JSON.parse(readFileSync(schemaPath, "utf8"));
} catch (err) {
  console.error(`  FAIL: Could not read schema: ${(err as Error).message}`);
  process.exit(3);
}

const ajv = new Ajv({ allErrors: true });
const valid = ajv.validate(schema as object, rawData);

if (!valid && ajv.errors) {
  for (const e of ajv.errors) {
    const path = e.instancePath || "(root)";
    console.error(`  FAIL [${path}]: ${e.message}`);
  }
  console.error(`\n${ajv.errors.length} validation error(s) found.`);
  process.exit(1);
}

console.log("  OK: YAML data is valid against schema.");

// ── 2. Required fields non-empty ──────────────────────────────────────────────

console.log("Checking required fields are populated...");

const data = rawData as Record<string, unknown>;
const ticket = (data.ticket ?? {}) as Record<string, unknown>;
const context = (data.context ?? {}) as Record<string, unknown>;
const cos = (data.conditions_of_satisfaction ?? {}) as Record<string, unknown>;
const ac = data.acceptance_criteria as unknown[];

const fieldErrors: string[] = [];
if (!String(ticket.key ?? "").trim()) fieldErrors.push("ticket.key is empty");
if (!String(ticket.summary ?? "").trim())
  fieldErrors.push("ticket.summary is empty");
if (!String(context.background ?? "").trim())
  fieldErrors.push("context.background is empty");
if (!Array.isArray(cos.must) || cos.must.length === 0)
  fieldErrors.push("conditions_of_satisfaction.must has no items");
if (!Array.isArray(ac) || ac.length < 2)
  fieldErrors.push("acceptance_criteria must have at least 2 items");

if (fieldErrors.length > 0) {
  for (const e of fieldErrors) console.error(`  FAIL: ${e}`);
  process.exit(1);
}

console.log("  OK: All required fields are populated.");

// ── 3. Markdown structure validation (optional) ───────────────────────────────

if (mdFile) {
  console.log(`Validating markdown structure in ${mdFile}...`);

  let md: string;
  try {
    md = readFileSync(resolve(mdFile), "utf8");
  } catch {
    console.error(`  FAIL: Markdown file not found: ${mdFile}`);
    process.exit(3);
  }

  const required = [
    "## Context",
    "## Conditions of Satisfaction",
    "## Acceptance Criteria",
  ];

  let mdErrors = 0;
  for (const section of required) {
    if (!md.includes(section)) {
      console.error(`  FAIL: Missing required section '${section}'`);
      mdErrors++;
    }
  }

  const h1Count = (md.match(/^# /gm) ?? []).length;
  if (h1Count !== 1) {
    console.error(`  FAIL: Expected exactly 1 H1 heading, found ${h1Count}`);
    mdErrors++;
  }

  if (mdErrors > 0) {
    console.error(`${mdErrors} markdown error(s) found.`);
    process.exit(1);
  }

  console.log("  OK: Markdown structure is valid.");
}

console.log(`\nAll checks passed for: ${yamlFile.split("/").at(-1)}`);
