#!/usr/bin/env sh
set -eu

REF_NAME="${1:-}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SKILL_DIR="$(dirname "$SCRIPT_DIR")"
REF_DIR="$SKILL_DIR/references"

if [ -z "$REF_NAME" ]; then
  echo "Usage: read-reference.sh <reference-name>"
  echo ""
  echo "Available references:"
  for f in "$REF_DIR"/*.md; do
    name="$(basename "$f" .md)"
    desc="$(head -1 "$f" | sed 's/^# //')"
    printf "  %-30s %s\n" "$name" "$desc"
  done
  exit 1
fi

MATCH=$(find "$REF_DIR" -name "${REF_NAME}.md" 2>/dev/null | head -1)
if [ -z "$MATCH" ]; then
  echo "Reference '${REF_NAME}' not found."
  echo "Available references:"
  for f in "$REF_DIR"/*.md; do
    basename "$f" .md
  done
  exit 1
fi

cat "$MATCH"
