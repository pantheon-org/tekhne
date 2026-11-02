#!/usr/bin/env bash
# shell: bash
# Validates a plan-review config (.claude/plan-review.json) against the plan-review-config schema.
# Usage: validate-plan-review-config.sh <file> [<file> ...]
# Exit: 0 valid (a same-model pair only warns), 1 invalid or missing file, 2 usage or missing jq.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/plan-review-config.schema.json"

if [ "$#" -eq 0 ]; then
  echo "Usage: validate-plan-review-config.sh <file> [<file> ...]" >&2
  exit 2
fi
command -v jq >/dev/null || { echo "jq is required" >&2; exit 2; }

errors=0

for f in "$@"; do
  if [ ! -f "$f" ]; then
    echo "  $f: file not found"
    errors=$((errors + 1))
    continue
  fi
  if ! jq -se 'length == 1' "$f" >/dev/null 2>&1; then
    echo "  $f: invalid JSON"
    errors=$((errors + 1))
    continue
  fi
  problems=$(jq -r --slurpfile schema "$SCHEMA" '
    $schema[0].properties as $props
    | . as $c
    | if ($c | type) != "object" then "top level must be an object"
      else
        (($c | keys) - ($props | keys) | map("unknown key " + .) | .[]),
        ($props | keys[]) as $k
        | if ($c | has($k)) | not then "missing required key " + $k
          elif ($c[$k] | type) != "string" or (($props[$k].enum | index($c[$k])) == null)
            then $k + " must be one of " + ($props[$k].enum | join(", "))
          else empty end
      end
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
