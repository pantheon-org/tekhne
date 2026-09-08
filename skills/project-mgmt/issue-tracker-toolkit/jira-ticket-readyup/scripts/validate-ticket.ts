#!/usr/bin/env bun
/**
 * Validates a ready-for-refinement YAML data file against the JSON schema,
 * then optionally checks that the markdown output has the required sections.
 *
 * Also runs advisory gap-detection heuristics beyond presence/count (AC
 * testability, CoS/AC aspiration-and-vagueness, MUST-to-AC coverage) and
 * prints them as warnings. Pass --strict-gaps to fail the run on any warning.
 *
 * Usage:
 *   bun run scripts/validate-ticket.ts <ticket-data.yaml>
 *   bun run scripts/validate-ticket.ts <ticket-data.yaml> --markdown <ticket-output.md>
 *   bun run scripts/validate-ticket.ts <ticket-data.yaml> --strict-gaps
 */

import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { load } from "js-yaml";

// ── CLI args ──────────────────────────────────────────────────────────────────

const args = process.argv.slice(2);
let yamlFile = "";
let mdFile = "";
let strictGaps = false;

for (let i = 0; i < args.length; i++) {
  if ((args[i] === "--markdown" || args[i] === "-m") && args[i + 1]) {
    mdFile = args[++i];
  } else if (args[i] === "--strict-gaps") {
    strictGaps = true;
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
const ac = (data.acceptance_criteria ?? []) as string[];
const isNewTicket = ticket.new_ticket === true;

const fieldErrors: string[] = [];
if (!isNewTicket && !String(ticket.key ?? "").trim())
  fieldErrors.push(
    "ticket.key is empty (set ticket.new_ticket: true for a brand-new, not-yet-keyed ticket)",
  );
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

// ── 2b. Gap-detection heuristics (advisory, beyond presence/count) ───────────
//
// These catch what presence/count checks above cannot: an item that exists,
// meets the length minimum, and still is not testable, is vague, or has no
// acceptance criterion verifying it. See docs/knowledge-base/jira/
// cos-ac-interview-and-gap-detection-process.md in the journal repo for the
// design this implements (heuristics 1, 2, and 5 of 5 — the other two, MUST/
// SHOULD conflict and self-containment, need semantic judgment and are run
// as an explicit review prompt in SKILL.md instead of code).

console.log("Running gap-detection heuristics (advisory)...");

const VAGUE_PATTERNS = [
  /\bmore robust\b/i,
  /\bmore reliable\b/i,
  /\bimprove(?:d|s)? reliability\b/i,
  /\bbetter (?:logging|error handling|performance|reliability)\b/i,
  /\bproperly handle[sd]?\b/i,
  /\bhandle[sd]? (?:it |this )?properly\b/i,
  /\bas needed\b/i,
  /\bas appropriate\b/i,
  /\bwhere appropriate\b/i,
  /\bmore efficient\b/i,
  /\bmore performant\b/i,
  /\bmore maintainable\b/i,
  /\bcleaner code\b/i,
  /\benhance(?:s|d)? (?:the )?(?:service|system|performance|reliability)\b/i,
  /\bimprove(?:s|d)? (?:the )?(?:service|system|performance|reliability)\b/i,
];

const OBSERVABLE_VERB =
  /\b(returns?|responds?|produces?|emits?|contains?|displays?|logs?|logged|rejects?|accepts?|matches?|equals?|shows?|lists?|counts?|triggers?|raises?|includes?|excludes?|redirects?|throws?|fails?|passes?|persists?|writes?|reads?|calls?|invokes?|renders?|sends?|receives?|blocks?|allows?|denies?|process(?:es|ed)?|forward(?:s|ed)?|continue[ds]?)\b/i;
const HAS_MEASURABLE_TOKEN =
  /\d|https?:\/\/|`[^`]+`|"[^"]+"|'[^']+'|\b(HTTP|error code|status code)\b/i;

const gapWarnings: string[] = [];

function checkVagueness(label: string, items: string[]): void {
  for (const item of items) {
    if (VAGUE_PATTERNS.some((re) => re.test(item))) {
      gapWarnings.push(
        `${label} reads as an aspiration, not a testable requirement: "${item}"`,
      );
    }
  }
}

function checkTestability(items: string[]): void {
  for (const item of items) {
    if (!OBSERVABLE_VERB.test(item) && !HAS_MEASURABLE_TOKEN.test(item)) {
      gapWarnings.push(
        `Acceptance criterion has no observable outcome (no checkable verb or measurable token): "${item}"`,
      );
    }
  }
}

function significantWords(text: string): Set<string> {
  const stopwords = new Set([
    "must",
    "should",
    "could",
    "the",
    "a",
    "an",
    "and",
    "or",
    "of",
    "to",
    "for",
    "with",
    "when",
    "that",
    "this",
    "is",
    "are",
    "be",
    "it",
    "on",
    "in",
    "at",
    "no",
    "not",
  ]);
  return new Set(
    text
      .toLowerCase()
      .replace(/[^a-z0-9\s]/g, " ")
      .split(/\s+/)
      .filter((w) => w.length >= 4 && !stopwords.has(w)),
  );
}

function checkMustToAcCoverage(musts: string[], criteria: string[]): void {
  const acWordSets = criteria.map(significantWords);
  for (const must of musts) {
    const mustWords = significantWords(must);
    const covered = acWordSets.some((acWords) => {
      let overlap = 0;
      for (const w of mustWords) if (acWords.has(w)) overlap++;
      return overlap >= 2;
    });
    if (!covered) {
      gapWarnings.push(
        `MUST item has no acceptance criterion that appears to verify it (heuristic keyword-overlap check — verify manually): "${must}"`,
      );
    }
  }
}

const musts = Array.isArray(cos.must) ? (cos.must as string[]) : [];
const shoulds = Array.isArray(cos.should) ? (cos.should as string[]) : [];

checkVagueness("CoS MUST item", musts);
checkVagueness("CoS SHOULD item", shoulds);
checkVagueness("Acceptance criterion", ac);
checkTestability(ac);
checkMustToAcCoverage(musts, ac);

if (gapWarnings.length > 0) {
  console.warn(`  ${gapWarnings.length} gap-detection warning(s) (advisory):`);
  for (const w of gapWarnings) console.warn(`  WARN: ${w}`);
  console.warn(
    "  These are heuristic and can false-positive — review each one; they do not block unless --strict-gaps is set.",
  );
  if (strictGaps) {
    console.error(
      `\n--strict-gaps set: failing on ${gapWarnings.length} gap-detection warning(s).`,
    );
    process.exit(1);
  }
} else {
  console.log("  OK: No gap-detection warnings.");
}

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
