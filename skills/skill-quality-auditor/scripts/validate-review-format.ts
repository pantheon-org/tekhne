#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

type Requirements = {
  required_title_prefix: string;
  required_metadata_labels: string[];
  recommended_metadata_labels?: string[];
  required_h2_groups: string[][];
  required_h2_order: string[][];
  recommended_h2_groups?: string[][];
  required_dimension_labels: string[];
  required_commands: string[];
  recommended_commands?: string[];
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
const DEFAULT_REPORT = ".context/audits/skill-quality-auditor-review.md";
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

function extractH1Heading(markdown: string): string | null {
  const match = markdown.match(/^#\s+(.+)$/m);
  return match ? match[1].trim() : null;
}

function findFirstHeadingIndex(
  headings: string[],
  alternatives: string[],
): number {
  let first = -1;
  for (const heading of alternatives) {
    const index = headings.indexOf(heading);
    if (index >= 0 && (first < 0 || index < first)) {
      first = index;
    }
  }
  return first;
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
  strictRecommended: boolean,
): { errors: string[]; warnings: string[] } {
  const errors: string[] = [];
  const warnings: string[] = [];
  const reportH2 = new Set(extractH2Headings(report));
  const reportH2List = extractH2Headings(report);
  const templateTitle = extractH1Heading(template);
  const reportTitle = extractH1Heading(report);

  if (!reportTitle) {
    errors.push("missing H1 title");
  } else if (!reportTitle.startsWith(requirements.required_title_prefix)) {
    errors.push(
      `title must start with '${requirements.required_title_prefix}'`,
    );
  }

  if (!templateTitle?.startsWith(requirements.required_title_prefix)) {
    errors.push("template title is out of sync with required title prefix");
  }

  for (const label of requirements.required_metadata_labels) {
    if (!report.includes(`**${label}**:`)) {
      errors.push(`missing metadata label: ${label}`);
    }
  }

  for (const label of requirements.recommended_metadata_labels ?? []) {
    if (!report.includes(`**${label}**:`)) {
      const message = `missing recommended metadata label: ${label}`;
      if (strictRecommended) {
        errors.push(message);
      } else {
        warnings.push(message);
      }
    }
  }

  for (const alternatives of requirements.required_h2_groups) {
    const hasOne = alternatives.some((heading) => reportH2.has(heading));
    if (!hasOne) {
      errors.push(
        `missing required H2 heading group (one of): ${alternatives.join(", ")}`,
      );
    }
  }

  let previous = -1;
  for (const alternatives of requirements.required_h2_order) {
    const index = findFirstHeadingIndex(reportH2List, alternatives);
    if (index < 0) {
      errors.push(
        `missing ordered H2 heading group (one of): ${alternatives.join(", ")}`,
      );
      continue;
    }

    if (index < previous) {
      errors.push(
        `H2 order violation near group: ${alternatives.join(", ")}; expected after prior required section`,
      );
    }
    previous = index;
  }

  for (const alternatives of requirements.recommended_h2_groups ?? []) {
    const hasOne = alternatives.some((heading) => reportH2.has(heading));
    if (!hasOne) {
      const message = `missing recommended H2 heading group (one of): ${alternatives.join(", ")}`;
      if (strictRecommended) {
        errors.push(message);
      } else {
        warnings.push(message);
      }
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

  for (const command of requirements.recommended_commands ?? []) {
    if (!report.includes(command)) {
      const message = `missing recommended command: ${command}`;
      if (strictRecommended) {
        errors.push(message);
      } else {
        warnings.push(message);
      }
    }
  }

  return { errors, warnings };
}

function main(): void {
  const args = process.argv.slice(2);
  const strictRecommended = args.includes("--strict-recommended");
  const reportArg = args.find((arg) => !arg.startsWith("--"));

  const reportPath = resolve(ROOT, reportArg ?? DEFAULT_REPORT);
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
  const { errors: reportErrors, warnings } = validateReport(
    report,
    template,
    requirements,
    strictRecommended,
  );

  if (warnings.length > 0) {
    console.warn(`Review format warnings for: ${reportPath}`);
    for (const warning of warnings) {
      console.warn(`- ${warning}`);
    }
  }

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
