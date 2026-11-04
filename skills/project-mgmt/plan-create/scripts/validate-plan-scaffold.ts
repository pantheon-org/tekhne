#!/usr/bin/env bun
/**
 * Validates plan-scaffold YAML files.
 *
 * Usage: validate-plan-scaffold.ts <file> [<file> ...]
 *
 * Port of the python validator: the same checks and output lines, with the differences listed in
 * scripts/plan-create/fixtures/README.md of the tekhne repository. Run it through validate-plan-scaffold.sh, which checks
 * the bun version. The python version loaded the schema file and never used it, so this one does not read it.
 */

import {
  formatPyValue,
  isEmpty,
  isMapping,
  pyTypeName,
  validateFiles,
} from "./validator-skeleton";

const REQUIRED = [
  "filename",
  "frontmatter",
  "goal",
  "phases",
  "open_questions",
];
const FRONTMATTER = ["title", "type", "status", "date"];
const MAX_TASKS = 8;

// A value the way python's str() writes it inside the quotes of a message.
const pyStr = (value: unknown): string =>
  typeof value === "string" ? value : formatPyValue(value);

const checkFrontmatter = (
  file: string,
  plan: Record<string, unknown>,
): string[] => {
  const frontmatter = "frontmatter" in plan ? plan.frontmatter : {};
  if (!isMapping(frontmatter))
    return [
      `${file}: 'frontmatter' must be a mapping (got ${pyTypeName(frontmatter)})`,
    ];
  return FRONTMATTER.filter((field) => !(field in frontmatter)).map(
    (field) => `${file}: frontmatter missing '${field}'`,
  );
};

const checkPhase = (file: string, phase: unknown, index: number): string[] => {
  if (!isMapping(phase))
    return [
      `${file}: phase ${index + 1} must be a mapping (got ${pyTypeName(phase)})`,
    ];
  const errors: string[] = [];
  if (isEmpty(phase.name))
    errors.push(`${file}: phase ${index + 1} missing 'name'`);
  const label = "name" in phase ? pyStr(phase.name) : String(index + 1);
  const tasks = "tasks" in phase ? phase.tasks : [];
  if (isEmpty(tasks)) errors.push(`${file}: phase '${label}' has no tasks`);
  else if (!Array.isArray(tasks))
    errors.push(
      `${file}: phase '${label}' tasks must be a list (got ${pyTypeName(tasks)})`,
    );
  else if (tasks.length > MAX_TASKS)
    errors.push(
      `${file}: phase '${label}' has ${tasks.length} tasks (max ${MAX_TASKS})`,
    );
  return errors;
};

const checkPhases = (file: string, plan: Record<string, unknown>): string[] => {
  const phases = "phases" in plan ? plan.phases : [];
  if (isEmpty(phases)) return [`${file}: must have at least 1 phase`];
  if (!Array.isArray(phases))
    return [`${file}: 'phases' must be a list (got ${pyTypeName(phases)})`];
  return phases.flatMap((phase, index) => checkPhase(file, phase, index));
};

// The problems of one parsed plan scaffold, as the python validator words them. No problem is an empty list.
export const checkPlanScaffold = (
  file: string,
  data: Record<string, unknown>,
): string[] => {
  const plan = data.plan;
  if (isEmpty(plan)) return [`${file}: missing required top-level key 'plan'`];
  if (!isMapping(plan))
    return [`${file}: 'plan' must be a mapping (got ${pyTypeName(plan)})`];
  const errors = REQUIRED.filter((field) => !(field in plan)).map(
    (field) => `${file}: plan missing required field '${field}'`,
  );
  errors.push(...checkFrontmatter(file, plan));
  errors.push(...checkPhases(file, plan));
  const questions = "open_questions" in plan ? plan.open_questions : [];
  if (isEmpty(questions))
    errors.push(`${file}: open_questions is empty — add at least 1`);
  return errors;
};

if (import.meta.main) {
  const result = validateFiles(process.argv.slice(2), {
    header: "Plan scaffold validation errors:",
    okLabel: "plan-scaffold",
    yamlSeparator: "—",
    check: checkPlanScaffold,
  });
  process.stdout.write(result.stdout);
  process.exitCode = result.exitCode;
}
