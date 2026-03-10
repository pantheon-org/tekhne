#!/usr/bin/env sh
# Usage: validate-plan.sh <plan-slug> [plans-root]
# Validates all README.md and task files in a plan tree against JSON Schema.
# Requires: node (>=18) on PATH
# Exit codes: 0 = all valid, 1 = one or more files failed validation
set -eu

PLAN_SLUG="${1:?Usage: validate-plan.sh <plan-slug> [plans-root]}"
PLANS_ROOT="${2:-.context/plans}"
PLAN_DIR="${PLANS_ROOT}/plan-${PLAN_SLUG}"
SKILL_DIR="$(cd "$(dirname "$0")/.." && pwd)"
SCHEMA_DIR="${SKILL_DIR}/schemas"

if [ ! -d "${PLAN_DIR}" ]; then
  echo "Error: plan directory not found: ${PLAN_DIR}" >&2
  exit 1
fi

ERRORS=0

# --------------------------------------------------------------------------- #
# validate_file <file> <schema> <label>                                        #
# Runs the JSON Schema regex checks in Node without external dependencies.     #
# --------------------------------------------------------------------------- #
validate_file() {
  FILE="$1"
  SCHEMA="$2"
  LABEL="$3"

  if [ ! -f "${FILE}" ]; then
    echo "  MISSING  ${LABEL}: ${FILE}"
    ERRORS=$((ERRORS + 1))
    return
  fi

  RESULT=$(node - "${FILE}" "${SCHEMA}" <<'NODE'
const fs   = require('fs');
const path = require('path');

const filePath   = process.argv[2];
const schemaPath = process.argv[3];

const content  = fs.readFileSync(filePath, 'utf8');
const schema   = JSON.parse(fs.readFileSync(schemaPath, 'utf8'));
const filename = path.basename(filePath);

const failures = [];

// Validate content patterns
const contentAllOf = schema.properties.content.allOf || [];
for (const rule of contentAllOf) {
  const re = new RegExp(rule.pattern);
  if (!re.test(content)) {
    failures.push(rule.description || `pattern not matched: ${rule.pattern}`);
  }
}

// Validate filename pattern (task schema only)
if (schema.properties.filename && schema.properties.filename.pattern) {
  const re = new RegExp(schema.properties.filename.pattern);
  if (!re.test(filename)) {
    failures.push(`filename does not match pattern ${schema.properties.filename.pattern}`);
  }
}

if (failures.length === 0) {
  process.stdout.write('OK');
  process.exit(0);
} else {
  process.stdout.write(failures.join('\n'));
  process.exit(1);
}
NODE
  )

  if [ "${RESULT}" = "OK" ]; then
    echo "  OK       ${LABEL}"
  else
    echo "  FAIL     ${LABEL}"
    echo "${RESULT}" | while IFS= read -r line; do
      echo "           - ${line}"
    done
    ERRORS=$((ERRORS + 1))
  fi
}

# --------------------------------------------------------------------------- #
# 1. Validate plan README                                                      #
# --------------------------------------------------------------------------- #
echo "Plan: ${PLAN_SLUG}"
echo ""
echo "plan README"
validate_file \
  "${PLAN_DIR}/README.md" \
  "${SCHEMA_DIR}/plan.schema.json" \
  "README.md"

# --------------------------------------------------------------------------- #
# 2. Validate each phase README                                                #
# --------------------------------------------------------------------------- #
echo ""
echo "phases"
for PHASE_DIR in "${PLAN_DIR}/phases"/phase-*/; do
  [ -d "${PHASE_DIR}" ] || continue
  PHASE_NAME="$(basename "${PHASE_DIR}")"
  validate_file \
    "${PHASE_DIR}README.md" \
    "${SCHEMA_DIR}/phase.schema.json" \
    "${PHASE_NAME}/README.md"

  # ------------------------------------------------------------------------ #
  # 3. Validate each task file                                                 #
  # ------------------------------------------------------------------------ #
  for TASK_FILE in "${PHASE_DIR}tasks"/task-*.md; do
    [ -f "${TASK_FILE}" ] || continue
    TASK_NAME="$(basename "${TASK_FILE}")"
    validate_file \
      "${TASK_FILE}" \
      "${SCHEMA_DIR}/task.schema.json" \
      "${PHASE_NAME}/tasks/${TASK_NAME}"
  done
done

# --------------------------------------------------------------------------- #
# Summary                                                                      #
# --------------------------------------------------------------------------- #
echo ""
if [ "${ERRORS}" -eq 0 ]; then
  echo "Validation passed — all files conform to schema."
  exit 0
else
  echo "Validation failed — ${ERRORS} file(s) have errors."
  exit 1
fi
