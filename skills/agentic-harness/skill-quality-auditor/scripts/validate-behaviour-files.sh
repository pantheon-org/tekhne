#!/usr/bin/env sh
# Checks the shape of one behaviour file in an eval scenario folder:
#   evals/scenario-N/behaviour.yaml
#   evals/scenario-N/traces/old.json
#   evals/scenario-N/traces/new.json
#
# Usage: validate-behaviour-files.sh <file>
#
# Shape only, and it mirrors assets/schemas/behaviour.schema.json and
# trace.schema.json. The auditor itself (`pantheon-skill-auditor evaluate`) is
# the canonical check: it reports the same problems as a report-only D9 warning.
# This script exists so a broken file also fails the artifact validator.
#
# Needs yq (for behaviour.yaml) and jq. Without them the checks that need them
# are skipped, as in validate-skill-artifacts.sh.
set -eu

errors=0

error() {
  printf 'ERROR: %s\n' "$1" >&2
  errors=1
}

if [ "$#" -ne 1 ]; then
  printf 'usage: %s <behaviour.yaml | traces/old.json | traces/new.json>\n' "$0" >&2
  exit 2
fi

file=$1
dir=$(dirname "$file")

# $json holds the file as JSON for the jq checks below.
json=""

jq_ok() {
  printf '%s' "$json" | jq -e "$1" >/dev/null 2>&1
}

# Report every key of the object at path $1 that is not in the allowed list $2.
unknown_keys() {
  path=$1
  allowed=$2
  printf '%s' "$json" | jq -r --argjson allowed "$allowed" \
    "($path // {}) | if type == \"object\" then keys[] else empty end | select(. as \$k | \$allowed | index(\$k) | not)" 2>/dev/null |
    while IFS= read -r key; do
      error "$file has an unknown field '$key' (a misspelt field is an error, not ignored)."
    done
}

check_behaviour() {
  command -v yq >/dev/null 2>&1 || return 0
  command -v jq >/dev/null 2>&1 || return 0

  if ! json=$(yq eval -o=json '.' "$file" 2>/dev/null); then
    error "$file is not valid YAML."
    return
  fi

  jq_ok '.schema_version == 1' || error "$file needs schema_version: 1."
  jq_ok '(.name | type) == "string" and (.name | length) > 0' || error "$file needs a non-empty name."
  jq_ok '(.expect | type) == "object"' || {
    error "$file needs an expect mapping."
    return
  }

  unknown_keys '.' '["schema_version","name","prompt","fixture","skillPaths","expect"]'
  unknown_keys '.expect' '["files_changed","commands_run","tool_calls","must_not","output_contains","output_not_contains"]'
  unknown_keys '.expect.must_not' '["files_changed","commands_run","tool_calls"]'

  jq_ok '[.expect.files_changed, .expect.commands_run, .expect.tool_calls, .expect.output_contains, .expect.output_not_contains, .expect.must_not.files_changed, .expect.must_not.commands_run, .expect.must_not.tool_calls] | map(. // []) | all(type == "array" and all(type == "string"))' ||
    error "$file has an expectation that is not a list of strings."
  jq_ok '[.expect.files_changed, .expect.commands_run, .expect.tool_calls, .expect.output_contains, .expect.output_not_contains, .expect.must_not.files_changed, .expect.must_not.commands_run, .expect.must_not.tool_calls] | map(. // []) | add | length > 0' ||
    error "$file expect lists no assertions."

  for which in old new; do
    [ -f "$dir/traces/$which.json" ] || error "$file needs traces/$which.json beside it (missing $dir/traces/$which.json)."
  done
}

check_trace() {
  command -v jq >/dev/null 2>&1 || return 0

  if ! json=$(jq -c . "$file" 2>/dev/null); then
    error "$file is not valid JSON."
    return
  fi

  jq_ok 'type == "object"' || {
    error "$file must be a JSON object."
    return
  }

  jq_ok '.schema_version == 1' || error "$file needs schema_version: 1."
  unknown_keys '.' '["schema_version","header","toolCalls","filesChanged","commandsRun","output","warnings"]'

  for key in header toolCalls filesChanged commandsRun output; do
    jq_ok "has(\"$key\")" || error "$file is missing '$key'."
  done

  jq_ok '(.output // "") | type == "string"' || error "$file output must be text."
  jq_ok '(.filesChanged // []) | type == "array" and all(type == "string")' || error "$file filesChanged must be a list of strings."
  jq_ok '(.commandsRun // []) | type == "array" and all(type == "string")' || error "$file commandsRun must be a list of strings."
  jq_ok '(.toolCalls // []) | type == "array" and all(type == "object" and (.tool | type) == "string")' || error "$file toolCalls must be a list of objects that each name a tool."

  if jq_ok '(.header | type) == "object"'; then
    unknown_keys '.header' '["harness","date","skill_commit","source"]'
    jq_ok '(.header.harness | type) == "string" and (.header.harness | length) > 0' || error "$file header.harness must not be empty."
    jq_ok '(.header.date | type) == "string" and (.header.date | test("^[0-9]{4}-[0-9]{2}-[0-9]{2}$"))' || error "$file header.date must be YYYY-MM-DD."
    jq_ok '(.header.skill_commit | type) == "string" and (.header.skill_commit | length) > 0' || error "$file header.skill_commit must not be empty."
    jq_ok '.header.source == "recorded" or .header.source == "hand-written"' || error "$file header.source must be 'recorded' or 'hand-written'."
  fi

  scenario_dir=$(dirname "$dir")
  [ -f "$scenario_dir/behaviour.yaml" ] || error "$file has no behaviour.yaml beside its traces/ folder."
}

case "$file" in
  */behaviour.yaml)
    check_behaviour
    ;;
  */traces/old.json | */traces/new.json)
    check_trace
    ;;
  *)
    error "$file is not a behaviour.yaml, traces/old.json or traces/new.json."
    ;;
esac

if [ "$errors" -ne 0 ]; then
  exit 1
fi
