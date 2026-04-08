#!/usr/bin/env bash
# shell: bash
# Validate an ANALYSIS-arxiv-<id>-<slug>.md file produced by the triage-paper skill.
# Usage: ./validate-analysis-paper.sh <file.md>
set -euo pipefail

FILE="${1:-}"

if [ -z "$FILE" ]; then
    echo "Usage: $0 <ANALYSIS-arxiv-<id>-<slug>.md>"
    exit 1
fi

if [ ! -f "$FILE" ]; then
    echo "Error: File not found: $FILE"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/analysis-paper.schema.json"

for cmd in yq ajv; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "Error: $cmd is required but not installed"
        echo "  yq:  brew install yq"
        echo "  ajv: npm install -g ajv-cli ajv-formats"
        exit 1
    fi
done

errors=0
err() { echo "  FAIL: $1"; errors=$((errors + 1)); }
ok()  { echo "  OK:   $1"; }

echo "Validating: $FILE"
echo ""

# 1. Extract YAML frontmatter
TMP=$(mktemp /tmp/analysis-paper-XXXXXX.json)
trap 'rm -f "$TMP"' EXIT

FRONTMATTER=$(awk '/^---$/{f++; if(f==2) exit; next} f==1{print}' "$FILE")

if [ -z "$FRONTMATTER" ]; then
    err "No YAML frontmatter found"
    echo ""
    echo "❌ Validation FAILED (1 error)"
    exit 1
fi

# 2. Validate frontmatter against JSON Schema
echo "$FRONTMATTER" | yq eval -o=json '.' - > "$TMP"
if ajv validate -s "$SCHEMA" -d "$TMP" >/dev/null 2>&1; then
    ok "Frontmatter passes schema (title, date, type, paper, source)"
else
    err "Frontmatter schema validation failed:"
    ajv validate -s "$SCHEMA" -d "$TMP" 2>&1 || true
fi

# 3. Check required stage sections
check_section() {
    heading="$1"
    if grep -q "^## ${heading}" "$FILE"; then
        ok "Section present: ## ${heading}"
    else
        err "Missing section: ## ${heading}"
    fi
}

check_section "Stage 1 — Descriptive"
check_section "Stage 2 — Evaluative"
check_section "Stage 3 — Synthesis hooks"

# 4. No unfilled angle-bracket placeholders
if grep -qE '<[A-Za-z][A-Za-z0-9 _-]*>' "$FILE"; then
    err "Unfilled placeholder(s) found — search for < ... > and complete them"
else
    ok "No unfilled placeholders"
fi

echo ""
if [ "$errors" -eq 0 ]; then
    echo "✅ Validation PASSED"
    exit 0
else
    echo "❌ Validation FAILED ($errors error(s))"
    exit 1
fi
