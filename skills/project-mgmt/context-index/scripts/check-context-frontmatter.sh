#!/usr/bin/env bash
# shell: bash
# check-context-frontmatter.sh - fail fast on .context/*.md files that don't
# even start with a YAML frontmatter delimiter. This is a cheap first pass;
# validate-context-frontmatter.sh does the full schema check.
#
# Usage: check-context-frontmatter.sh <file> [<file> ...]
set -euo pipefail

failed=()
for f in "$@"; do
    [[ -f "$f" ]] || continue
    first_line=$(head -1 "$f")
    if [[ "$first_line" != "---" ]]; then
        failed+=("$f")
    fi
done

if [[ ${#failed[@]} -gt 0 ]]; then
    echo "ERROR: Missing YAML frontmatter in the following .context files:"
    for f in "${failed[@]}"; do
        echo "  $f"
    done
    printf '\nAdd a frontmatter block (title, type, status, date, tags) and re-run.\n'
    printf 'Use the create-context-file skill for the schema and templates.\n'
    exit 1
fi

echo "OK: $# file(s) start with a frontmatter block"
