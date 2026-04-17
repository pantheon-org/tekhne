#!/usr/bin/env bash
# shell: bash
# validate-ticket.sh
# Validates a ready-for-refinement YAML data file against the JSON schema,
# then checks that the corresponding markdown output has the required sections.
#
# Usage:
#   ./validate-ticket.sh <ticket-data.yaml>
#   ./validate-ticket.sh <ticket-data.yaml> --markdown <ticket-output.md>
#
# Requirements: uv (https://docs.astral.sh/uv/) — no system package install needed

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILL_DIR="$(dirname "$SCRIPT_DIR")"
SCHEMA="$SKILL_DIR/assets/schemas/ready-for-refinement.schema.json"

YAML_FILE=""
MD_FILE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --markdown|-m)
      shift
      MD_FILE="$1"
      shift
      ;;
    -*)
      echo "Unknown flag: $1" >&2
      exit 2
      ;;
    *)
      YAML_FILE="$1"
      shift
      ;;
  esac
done

if [[ -z "$YAML_FILE" ]]; then
  echo "Usage: $0 <ticket-data.yaml> [--markdown <ticket-output.md>]" >&2
  exit 2
fi

if [[ ! -f "$YAML_FILE" ]]; then
  echo "ERROR: YAML file not found: $YAML_FILE" >&2
  exit 3
fi

if [[ ! -f "$SCHEMA" ]]; then
  echo "ERROR: Schema not found: $SCHEMA" >&2
  exit 3
fi

if ! command -v uv &>/dev/null; then
  echo "ERROR: 'uv' is required but not found. Install from https://docs.astral.sh/uv/" >&2
  exit 4
fi

# ── 1. YAML → JSON schema validation ─────────────────────────────────────────
echo "Validating $YAML_FILE against schema..."

uv run --quiet - "$YAML_FILE" "$SCHEMA" <<'PYEOF'
# /// script
# requires-python = ">=3.11"
# dependencies = ["pyyaml", "jsonschema"]
# ///
import sys, json, yaml, jsonschema

yaml_path = sys.argv[1]
schema_path = sys.argv[2]

with open(yaml_path) as f:
    data = yaml.safe_load(f)

with open(schema_path) as f:
    schema = json.load(f)

validator = jsonschema.Draft7Validator(schema)
errors = sorted(validator.iter_errors(data), key=lambda e: list(e.path))

if errors:
    for e in errors:
        path = " > ".join(str(p) for p in e.path) if e.path else "(root)"
        print(f"  FAIL [{path}]: {e.message}", file=sys.stderr)
    print(f"\n{len(errors)} validation error(s) found.", file=sys.stderr)
    sys.exit(1)

print("  OK: YAML data is valid against schema.")
PYEOF

# ── 2. Required fields are non-empty ─────────────────────────────────────────
echo "Checking required fields are populated..."

uv run --quiet - "$YAML_FILE" <<'PYEOF'
# /// script
# requires-python = ">=3.11"
# dependencies = ["pyyaml"]
# ///
import sys, yaml

with open(sys.argv[1]) as f:
    data = yaml.safe_load(f)

errors = []

def nonempty(val, path):
    if not val:
        errors.append(f"Empty required field: {path}")

ticket = data.get("ticket", {})
nonempty(ticket.get("key"), "ticket.key")
nonempty(ticket.get("summary"), "ticket.summary")
nonempty(ticket.get("type"), "ticket.type")

ctx = data.get("context", {})
nonempty(ctx.get("background"), "context.background")
nonempty(ctx.get("implications"), "context.implications")

cos = data.get("conditions_of_satisfaction", {})
nonempty(cos.get("must"), "conditions_of_satisfaction.must")

nonempty(data.get("acceptance_criteria"), "acceptance_criteria")

if errors:
    for e in errors:
        print(f"  FAIL: {e}", file=sys.stderr)
    sys.exit(1)

print("  OK: All required fields are populated.")
PYEOF

# ── 3. Markdown structure validation (optional) ───────────────────────────────
if [[ -n "$MD_FILE" ]]; then
  if [[ ! -f "$MD_FILE" ]]; then
    echo "ERROR: Markdown file not found: $MD_FILE" >&2
    exit 3
  fi

  echo "Validating markdown structure in $MD_FILE..."

  REQUIRED_SECTIONS=(
    "## Context"
    "## Conditions of Satisfaction"
    "## Acceptance Criteria"
  )

  md_errors=0
  for section in "${REQUIRED_SECTIONS[@]}"; do
    if ! grep -qF "$section" "$MD_FILE"; then
      echo "  FAIL: Missing required section '$section'" >&2
      (( md_errors++ )) || true
    fi
  done

  h1_count=$(grep -cE '^# ' "$MD_FILE" || true)
  if [[ "$h1_count" -ne 1 ]]; then
    echo "  FAIL: Expected exactly 1 H1 heading, found $h1_count" >&2
    (( md_errors++ )) || true
  fi

  if [[ $md_errors -gt 0 ]]; then
    echo "$md_errors markdown error(s) found." >&2
    exit 1
  fi

  echo "  OK: Markdown structure is valid."
fi

echo ""
echo "All checks passed for: $(basename "$YAML_FILE")"
