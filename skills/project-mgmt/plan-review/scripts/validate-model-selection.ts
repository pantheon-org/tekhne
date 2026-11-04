#!/usr/bin/env bun
/**
 * Validates model-selection YAML files.
 *
 * Usage: validate-model-selection.ts <file> [<file> ...]
 *
 * Port of the python validator: the same checks and output lines, with the differences listed in
 * scripts/plan-review/fixtures-model-selection/README.md of the tekhne repository. Run it through
 * validate-model-selection.sh, which checks the bun version. The python version loaded the schema file and never used
 * it, so this one does not read it.
 */

import {
  isEmpty,
  isMapping,
  pyTypeName,
  validateFiles,
} from "./validator-skeleton";

const REQUIRED = ["question_text", "options"];

// The problems of one parsed model selection, as the python validator words them. No problem is an empty list.
export const checkModelSelection = (
  file: string,
  data: Record<string, unknown>,
): string[] => {
  const selection = data.model_selection;
  if (isEmpty(selection))
    return [`${file}: missing required top-level key 'model_selection'`];
  if (!isMapping(selection))
    return [
      `${file}: 'model_selection' must be a mapping (got ${pyTypeName(selection)})`,
    ];
  const errors = REQUIRED.filter((field) => !(field in selection)).map(
    (field) => `${file}: model_selection missing required field '${field}'`,
  );
  const options = "options" in selection ? selection.options : [];
  if (!Array.isArray(options)) errors.push(`${file}: options must be an array`);
  else if (options.length < 1)
    errors.push(`${file}: options must have at least one entry`);
  return errors;
};

if (import.meta.main) {
  const result = validateFiles(process.argv.slice(2), {
    header: "Model selection validation errors:",
    okLabel: "model-selection",
    yamlSeparator: "—",
    check: checkModelSelection,
  });
  process.stdout.write(result.stdout);
  process.exitCode = result.exitCode;
}
