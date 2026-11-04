#!/usr/bin/env bun
/**
 * Validates tech-evaluation YAML files against the tech-evaluation schema.
 *
 * Usage: validate-tech-evaluation.ts <file> [<file> ...]
 *
 * Port of the python validator: the same checks and output lines, with the differences listed in
 * scripts/tech-evaluation/fixtures/README.md of the tekhne repository. Run it through validate-tech-evaluation.sh, which
 * checks the bun version. It reads the schema for the required lists, the verdict values and the minimum lengths.
 */

import { readFileSync } from "node:fs";
import {
  findSchemaPath,
  formatPyValue,
  isEmpty,
  isMapping,
  pyTypeName,
  validateFiles,
} from "./validator-skeleton";

type Obj = Record<string, unknown>;
type ItemSchema = { required?: string[] };
type Schema = {
  properties?: {
    evaluation?: {
      required?: string[];
      properties?: {
        questions?: {
          items?: ItemSchema & {
            properties?: { evidence?: { items?: ItemSchema } };
          };
        };
        recommendation?: {
          required?: string[];
          properties?: {
            verdict?: { enum?: string[] };
            rationale?: { minLength?: number };
            single_action?: { minLength?: number };
          };
        };
      };
    };
  };
};

// A value the way python's str() writes it inside the quotes of a message.
const pyStr = (value: unknown): string =>
  typeof value === "string" ? value : formatPyValue(value);

const missing = (
  required: string[] | undefined,
  value: Obj,
  describe: (field: string) => string,
): string[] =>
  (required ?? []).filter((field) => !(field in value)).map(describe);

const checkQuestion = (
  file: string,
  question: unknown,
  index: number,
  schema: NonNullable<
    NonNullable<NonNullable<Schema["properties"]>["evaluation"]>["properties"]
  >["questions"],
): string[] => {
  const where = `questions[${index}]`;
  if (!isMapping(question))
    return [
      `${file}: ${where} must be a mapping (got ${pyTypeName(question)})`,
    ];
  const errors = missing(
    schema?.items?.required,
    question,
    (field) => `${file}: ${where} missing required field '${field}'`,
  );
  const evidence = question.evidence;
  if (isEmpty(evidence)) {
    const id = "id" in question ? pyStr(question.id) : "?";
    errors.push(
      `${file}: ${where} ('${id}') has no evidence -- a verdict without a cited source is a guess, not a finding`,
    );
  } else if (!Array.isArray(evidence)) {
    errors.push(
      `${file}: ${where}.evidence must be a list (got ${pyTypeName(evidence)})`,
    );
  } else {
    evidence.forEach((item, j) => {
      if (!isMapping(item))
        errors.push(
          `${file}: ${where}.evidence[${j}] must be a mapping (got ${pyTypeName(item)})`,
        );
      else
        errors.push(
          ...missing(
            schema?.items?.properties?.evidence?.items?.required,
            item,
            (field) =>
              `${file}: ${where}.evidence[${j}] missing required field '${field}'`,
          ),
        );
    });
  }
  return errors;
};

const checkLength = (
  file: string,
  recommendation: Obj,
  field: "rationale" | "single_action",
  minLength: number,
  tooShort: string,
): string[] => {
  const value = isEmpty(recommendation[field]) ? "" : recommendation[field];
  if (typeof value !== "string")
    return [
      `${file}: recommendation.${field} must be a string (got ${pyTypeName(value)})`,
    ];
  return value.length < minLength ? [`${file}: ${tooShort}`] : [];
};

const checkRecommendation = (
  file: string,
  recommendation: unknown,
  schema: NonNullable<
    NonNullable<Schema["properties"]>["evaluation"]
  >["properties"],
): string[] => {
  const rec = schema?.recommendation;
  if (isEmpty(recommendation)) recommendation = {};
  if (!isMapping(recommendation))
    return [
      `${file}: 'recommendation' must be a mapping (got ${pyTypeName(recommendation)})`,
    ];
  const errors = missing(
    rec?.required,
    recommendation,
    (field) => `${file}: recommendation missing required field '${field}'`,
  );
  const verdict = recommendation.verdict;
  const allowed = rec?.properties?.verdict?.enum ?? [];
  if (
    verdict !== null &&
    verdict !== undefined &&
    !allowed.includes(verdict as string)
  )
    errors.push(
      `${file}: recommendation.verdict '${pyStr(verdict)}' is not one of ${formatPyValue(allowed)}`,
    );
  const rationaleMin = rec?.properties?.rationale?.minLength ?? 0;
  const actionMin = rec?.properties?.single_action?.minLength ?? 0;
  errors.push(
    ...checkLength(
      file,
      recommendation,
      "rationale",
      rationaleMin,
      `recommendation.rationale is shorter than ${rationaleMin} characters -- too thin to be a rationale`,
    ),
    ...checkLength(
      file,
      recommendation,
      "single_action",
      actionMin,
      `recommendation.single_action is shorter than ${actionMin} characters`,
    ),
  );
  return errors;
};

// The problems of one parsed tech evaluation, as the python validator words them. No problem is an empty list.
export const checkTechEvaluation = (
  file: string,
  data: Obj,
  schema: Schema,
): string[] => {
  const evaluation = data.evaluation;
  if (isEmpty(evaluation))
    return [`${file}: missing required top-level key 'evaluation'`];
  if (!isMapping(evaluation))
    return [
      `${file}: 'evaluation' must be a mapping (got ${pyTypeName(evaluation)})`,
    ];
  const evalSchema = schema.properties?.evaluation;
  const errors = missing(
    evalSchema?.required,
    evaluation,
    (field) => `${file}: evaluation missing required field '${field}'`,
  );
  const questions = isEmpty(evaluation.questions) ? [] : evaluation.questions;
  if (!Array.isArray(questions)) {
    errors.push(
      `${file}: 'questions' must be a list (got ${pyTypeName(questions)})`,
    );
  } else {
    if (questions.length < 1)
      errors.push(`${file}: 'questions' must have at least 1 entry`);
    for (const [index, question] of questions.entries())
      errors.push(
        ...checkQuestion(
          file,
          question,
          index,
          evalSchema?.properties?.questions,
        ),
      );
  }
  errors.push(
    ...checkRecommendation(
      file,
      evaluation.recommendation,
      evalSchema?.properties,
    ),
  );
  return errors;
};

if (import.meta.main) {
  const files = process.argv.slice(2);
  let schema: Schema = {};
  if (files.length > 0) {
    try {
      schema = JSON.parse(
        readFileSync(
          findSchemaPath(import.meta.dir, "tech-evaluation.schema.json"),
          "utf8",
        ),
      ) as Schema;
    } catch (error) {
      process.stderr.write(`${(error as Error).message}\n`);
      process.exit(1);
    }
  }
  const result = validateFiles(files, {
    header: "Tech-evaluation validation errors:",
    okLabel: "tech-evaluation",
    yamlSeparator: "--",
    check: (file, data) => checkTechEvaluation(file, data, schema),
  });
  process.stdout.write(result.stdout);
  process.exitCode = result.exitCode;
}
