#!/usr/bin/env sh
# Validate a remediation plan against the JSON Schema
# Usage: ./validate-remediation-plan.sh <plan-file>

set -e

PLAN_FILE="${1:-}"

if [ -z "$PLAN_FILE" ]; then
    echo "Usage: $0 <plan-file>"
    echo "Example: $0 .context/plans/typescript-advanced-remediation-plan.md"
    exit 1
fi

if [ ! -f "$PLAN_FILE" ]; then
    echo "Error: File not found: $PLAN_FILE"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SCHEMA_FILE="$SCRIPT_DIR/../schemas/remediation-plan.schema.json"

if [ ! -f "$SCHEMA_FILE" ]; then
    echo "Error: Schema not found: $SCHEMA_FILE"
    exit 1
fi

echo "Validating: $PLAN_FILE"
echo "Using schema: $SCHEMA_FILE"

TEMP_JSON=$(mktemp)
trap 'rm -f "$TEMP_JSON"' EXIT

if ! command -v yq >/dev/null 2>&1; then
    echo "Error: yq is required but not installed"
    echo "Install with: brew install yq"
    exit 1
fi

if ! command -v ajv >/dev/null 2>&1; then
    echo "Error: ajv is required but not installed"
    echo "Install with: npm install -g ajv ajv-formats"
    exit 1
fi

yq eval -o=json "$PLAN_FILE" > "$TEMP_JSON"

if ajv validate -s "$SCHEMA_FILE" -d "$TEMP_JSON" 2>/dev/null; then
    echo "✅ Validation PASSED"
    exit 0
else
    echo "❌ Validation FAILED"
    ajv validate -s "$SCHEMA_FILE" -d "$TEMP_JSON" 2>&1 || true
    exit 1
fi
