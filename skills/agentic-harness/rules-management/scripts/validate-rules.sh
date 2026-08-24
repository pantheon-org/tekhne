#!/usr/bin/env bash
# shell: bash
# validate-rules.sh — Validate .claude/RULES.md against JSON Schema, or generate a rule entry
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ASSETS_DIR="$(cd "$SCRIPT_DIR/../assets" && pwd)"
SCHEMA_FILE="$ASSETS_DIR/schemas/rule-entry.schema.json"
TEMPLATE_FILE="$ASSETS_DIR/templates/rule-entry.yaml"

usage() {
  cat <<EOF
Usage:
  $0 validate <path-to-rules-file>   — Validate rules file against schema
  $0 generate <title> <directive> <rationale> — Generate a rule entry from template
EOF
  exit 1
}

MODE="${1:-}"
shift || usage

case "$MODE" in
  validate)
    RULES_FILE="${1:-}"
    [ -z "$RULES_FILE" ] && usage
    [ ! -f "$RULES_FILE" ] && echo "FAIL: $RULES_FILE not found" && exit 1
    [ ! -f "$SCHEMA_FILE" ] && echo "FAIL: schema not found at $SCHEMA_FILE" && exit 1

    structural_check() {
      local file="$1"
      local errors=0
      local current_rule="" has_directive=false has_rationale=false dir_value="" rat_value=""
      while IFS= read -r line; do
        if [[ "$line" =~ ^###\ Rule:\  ]]; then
          if [ -n "$current_rule" ]; then
            if [ "$has_directive" = false ] || [ "$has_rationale" = false ]; then
              echo "FAIL: Rule '$current_rule' missing sections (directive=$has_directive, rationale=$has_rationale)"
              ((errors++))
            elif [ -z "${dir_value// }" ] || [ -z "${rat_value// }" ]; then
              echo "FAIL: Rule '$current_rule' has empty directive or rationale"
              ((errors++))
            fi
          fi
          current_rule="${line#\#\#\# Rule: }"
          has_directive=false; has_rationale=false; dir_value=""; rat_value=""
        elif [[ "$line" == "**Directive:**"* ]]; then
          has_directive=true
          dir_value="${line#\*\*Directive:\*\* }"
        elif [[ "$line" == "**Rationale:**"* ]]; then
          has_rationale=true
          rat_value="${line#\*\*Rationale:\*\* }"
        fi
      done < "$file"
      if [ -n "$current_rule" ]; then
        if [ "$has_directive" = false ] || [ "$has_rationale" = false ]; then
          echo "FAIL: Rule '$current_rule' missing sections (directive=$has_directive, rationale=$has_rationale)"
          ((errors++))
        elif [ -z "${dir_value// }" ] || [ -z "${rat_value// }" ]; then
          echo "FAIL: Rule '$current_rule' has empty directive or rationale"
          ((errors++))
        fi
      fi
      return "$errors"
    }

    schema_check() {
      local file="$1"
      local errors=0
      local entries_json="[" title="" directive="" rationale="" first=true

      while IFS= read -r line; do
        if [[ "$line" =~ ^###\ Rule:\  ]]; then
          if [ -n "$title" ]; then
            $first && first=false || entries_json+=","
            entries_json+="{\"title\":$(echo "$title" | jq -R -s .),\"directive\":$(echo "$directive" | jq -R -s .),\"rationale\":$(echo "$rationale" | jq -R -s .)}"
          fi
          title="${line#\#\#\# Rule: }"; directive=""; rationale=""
        elif [[ "$line" == "**Directive:**"* ]]; then
          directive="${line#\*\*Directive:\*\* }"
        elif [[ "$line" == "**Rationale:**"* ]]; then
          rationale="${line#\*\*Rationale:\*\* }"
        fi
      done < "$file"

      if [ -n "$title" ]; then
        $first && first=false || entries_json+=","
        entries_json+="{\"title\":$(echo "$title" | jq -R -s .),\"directive\":$(echo "$directive" | jq -R -s .),\"rationale\":$(echo "$rationale" | jq -R -s .)}"
      fi
      entries_json+="]"

      echo "$entries_json" | jq -c '.[]' | while read -r entry; do
        if ! echo "$entry" | jq -e --slurpfile s "$SCHEMA_FILE" '
          . as $e | .title as $t | .directive as $d | .rationale as $r
          | if ($s[0].required | map(. as $f | $e | has($f)) | all | not) then error("missing required field") else . end
          | if ($t | type) != "string" or ($t | length) == 0 then error("invalid title") else . end
          | if ($d | type) != "string" or ($d | length) == 0 then error("invalid directive") else . end
          | if ($r | type) != "string" or ($r | length) == 0 then error("invalid rationale") else . end
        ' > /dev/null 2>&1; then
          echo "FAIL: schema violation for rule: $(echo "$entry" | jq -r '.title')"
          ((errors++))
        fi
      done
      return "$errors"
    }

    echo "--- Structural check ---"
    structural_check "$RULES_FILE"; struct_err=$?
    echo "--- Schema validation ---"
    schema_check "$RULES_FILE"; schema_err=$?
    total=$((struct_err + schema_err))
    [ "$total" -gt 0 ] && echo "FAIL: $total violation(s)" && exit 1
    echo "PASS: All rules validate against $SCHEMA_FILE"
    ;;

  generate)
    command -v yq >/dev/null 2>&1 || { echo "FAIL: yq not found on PATH (required for generate)" >&2; exit 1; }
    [ -f "$TEMPLATE_FILE" ] || { echo "FAIL: template not found at $TEMPLATE_FILE" >&2; exit 1; }

    TITLE="${1:-}"; shift || usage
    DIRECTIVE="${1:-}"; shift || usage
    RATIONALE="${1:-}"; shift || usage

    # Populate the template via yq's strenv() rather than string-interpolating
    # $TITLE/$DIRECTIVE/$RATIONALE directly into the filter expression, so a value
    # containing a quote or other yq-significant character can't break the query.
    entry_yaml="$(TITLE="$TITLE" DIRECTIVE="$DIRECTIVE" RATIONALE="$RATIONALE" yq eval '
      .title = strenv(TITLE) | .directive = strenv(DIRECTIVE) | .rationale = strenv(RATIONALE)
    ' "$TEMPLATE_FILE")"

    echo "---
### Rule: $(yq eval '.title' - <<<"$entry_yaml")

**Directive:** $(yq eval '.directive' - <<<"$entry_yaml")

**Rationale:** $(yq eval '.rationale' - <<<"$entry_yaml")
---"

    # Validate generated entry against schema
    entry_json="$(yq eval -o=json '.' - <<<"$entry_yaml")"
    if echo "$entry_json" | jq -e --slurpfile s "$SCHEMA_FILE" '
      . as $e | .title as $t | .directive as $d | .rationale as $r
      | if ($t | type) != "string" or ($t | length) == 0 then error("invalid title") else . end
      | if ($d | type) != "string" or ($d | length) == 0 then error("invalid directive") else . end
      | if ($r | type) != "string" or ($r | length) == 0 then error("invalid rationale") else . end
    ' > /dev/null 2>&1; then
      echo "PASS: generated entry validates against $SCHEMA_FILE"
    else
      echo "FAIL: generated entry does not validate against $SCHEMA_FILE" >&2
      exit 1
    fi
    ;;

  *)
    usage
    ;;
esac
