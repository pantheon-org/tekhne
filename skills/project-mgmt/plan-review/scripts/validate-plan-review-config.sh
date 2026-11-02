#!/usr/bin/env bash
# shell: bash
# Validates a plan-review config (.claude/plan-review.json) against the plan-review-config schema.
# Usage: validate-plan-review-config.sh <file> [<file> ...]
# Warns (exit 0) when both reviewers use the same model, since that loses model diversity.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/plan-review-config.schema.json"

if [ "$#" -eq 0 ]; then
  exit 0
fi

command -v jq >/dev/null || { echo "jq is required" >&2; exit 2; }

ALLOWED_JSON=$(jq -c '.properties.technicalStrategic.enum' "$SCHEMA")
errors=0

for f in "$@"; do
  [ -f "$f" ] || continue
  if ! jq -e . "$f" >/dev/null 2>&1; then
    echo "  $f: invalid JSON"
    errors=$((errors + 1))
    continue
  fi
  problems=$(jq -r --argjson allowed "$ALLOWED_JSON" '
    (if type != "object" then "top level must be an object" else empty end),
    (keys - ["technicalStrategic", "risk"] | map("unknown key " + .) | .[]),
    ("technicalStrategic", "risk") as $k
      | (if has($k) | not then "missing required key " + $k
         elif ([.[$k]] | inside($allowed)) | not then $k + " must be one of " + ($allowed | join(", "))
         else empty end)
  ' "$f")
  if [ -n "$problems" ]; then
    while IFS= read -r line; do echo "  $f: $line"; done <<<"$problems"
    errors=$((errors + 1))
    continue
  fi
  if [ "$(jq -r .technicalStrategic "$f")" = "$(jq -r .risk "$f")" ]; then
    echo "  $f: warning: both reviewers use the same model, model diversity is lost"
  fi
done

if [ "$errors" -gt 0 ]; then
  echo "Plan review config validation errors"
  exit 1
fi
echo "OK: $# file(s) validated against plan-review-config schema"
