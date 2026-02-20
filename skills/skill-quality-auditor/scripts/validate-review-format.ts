#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

type Requirements = {
  required_h2_headings: string[];
  required_metadata_labels: string[];
  required_dimension_labels: string[];
  required_commands: string[];
  require_grade_scale_section?: boolean;
  require_score_evolution_section?: boolean;
};

type Schema = {
  required: string[];
  properties: Record<
    string,
    {
      type?: string;
      minItems?: number;
      items?: { type?: string };
    }
  >;
};

const ROOT = process.cwd();
const DEFAULT_REPORT = ".context/reviews/skill-quality-auditor-review.md";
const DEFAULT_TEMPLATE =
  "skills/skill-quality-auditor/templates/review-report-template.md";
const DEFAULT_SCHEMA =
  "skills/skill-quality-auditor/schemas/review-report.schema.json";
const DEFAULT_REQUIREMENTS =
  "skills/skill-quality-auditor/templates/review-report.requirements.json";

function readJson<T>(path: string): T {
  return JSON.parse(readFileSync(path, "utf8")) as T;
}

function extractH2Headings(markdown: string): string[] {
  return Array.from(markdown.matchAll(/^##\s+(.+)$/gm), (m) => m[1].trim());
}

function validateRequirementsShape(
  requirements: unknown,
  schema: Schema,
): string[] {
  const errors: string[] = [];

  if (typeof requirements !== "object" || requirements === null) {
    return ["requirements file must be a JSON object"];
  }

  const reqObj = requirements as Record<string, unknown>;

  for (const key of schema.required ?? []) {
    if (!(key in reqObj)) {
      errors.push(`missing required property in requirements: ${key}`);
    }
  }

  for (const [key, descriptor] of Object.entries(schema.properties ?? {})) {
    const value = reqObj[key];
    if (value === undefined) {
      continue;
    }

    if (descriptor.type === "array") {
      if (!Array.isArray(value)) {
        errors.push(`property ${key} must be an array`);
        continue;
      }

      if (
        typeof descriptor.minItems === "number" &&
        value.length < descriptor.minItems
      ) {
        errors.push(
          `property ${key} must have at least ${descriptor.minItems} items`,
        );
      }

      if (descriptor.items?.type === "string") {
        for (const [index, item] of value.entries()) {
          if (typeof item !== "string") {
            errors.push(`property ${key}[${index}] must be a string`);
          }
        }
      }
    }

    if (descriptor.type === "boolean" && typeof value !== "boolean") {
      errors.push(`property ${key} must be a boolean`);
    }
  }

  return errors;
}

function validateReport(
  report: string,
  template: string,
  requirements: Requirements,
): string[] {
  const errors: string[] = [];
  const reportH2 = new Set(extractH2Headings(report));
  const templateH2 = extractH2Headings(template);

  for (const label of requirements.required_metadata_labels) {
    if (!report.includes(`**${label}**:`)) {
      errors.push(`missing metadata label: ${label}`);
    }
  }

  for (const heading of requirements.required_h2_headings) {
    if (!reportH2.has(heading)) {
      errors.push(`missing required H2 heading: ${heading}`);
    }
  }

  // Template guard: all template H2 headings should exist in the report.
  for (const heading of templateH2) {
    if (!reportH2.has(heading)) {
      errors.push(`missing template H2 heading: ${heading}`);
    }
  }

  for (const label of requirements.required_dimension_labels) {
    if (!report.includes(label)) {
      errors.push(`missing dimension label: ${label}`);
    }
  }

  for (const command of requirements.required_commands) {
    if (!report.includes(command)) {
      errors.push(`missing required command: ${command}`);
    }
  }

  if (
    requirements.require_grade_scale_section &&
    !reportH2.has("Grade Scale Reference")
  ) {
    errors.push("Grade Scale Reference section is required");
  }

  if (
    requirements.require_score_evolution_section &&
    !reportH2.has("Score Evolution")
  ) {
    errors.push("Score Evolution section is required");
  }

  return errors;
}

function main(): void {
  const args = process.argv.slice(2);

  const reportPath = resolve(ROOT, args[0] ?? DEFAULT_REPORT);
  const templatePath = resolve(ROOT, DEFAULT_TEMPLATE);
  const schemaPath = resolve(ROOT, DEFAULT_SCHEMA);
  const requirementsPath = resolve(ROOT, DEFAULT_REQUIREMENTS);

  for (const path of [reportPath, templatePath, schemaPath, requirementsPath]) {
    if (!existsSync(path)) {
      console.error(`Missing required file: ${path}`);
      process.exit(1);
    }
  }

  const report = readFileSync(reportPath, "utf8");
  const template = readFileSync(templatePath, "utf8");
  const schema = readJson<Schema>(schemaPath);
  const requirementsRaw = readJson<unknown>(requirementsPath);

  const schemaErrors = validateRequirementsShape(requirementsRaw, schema);
  if (schemaErrors.length > 0) {
    console.error("Requirements schema validation failed:");
    for (const error of schemaErrors) {
      console.error(`- ${error}`);
    }
    process.exit(1);
  }

  const requirements = requirementsRaw as Requirements;
  const reportErrors = validateReport(report, template, requirements);

  if (reportErrors.length > 0) {
    console.error(`Review format validation failed for: ${reportPath}`);
    for (const error of reportErrors) {
      console.error(`- ${error}`);
    }
    process.exit(1);
  }

  console.log(`Review format validation passed: ${reportPath}`);
}

main();
