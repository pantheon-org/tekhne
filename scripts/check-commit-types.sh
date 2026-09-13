#!/usr/bin/env sh
# Fail if the PR-title gate's type list has drifted from the canonical one.
#
# .github/commit-types.txt is the source of truth. The regex in
# .github/workflows/conventional-commits.yml duplicates it deliberately (see
# that file for why it is not read at runtime). This check is what makes the
# duplication safe: divergence fails loudly instead of silently allowing a
# type release-please ignores, or rejecting one it honours.
set -eu

types_file=".github/commit-types.txt"
workflow=".github/workflows/conventional-commits.yml"

for f in "$types_file" "$workflow"; do
    [ -f "$f" ] || { echo "check-commit-types: missing $f" >&2; exit 1; }
done

# Canonical list, sorted, pipe-separated.
canonical=$(grep -vE '^#|^[[:space:]]*$' "$types_file" | tr -d ' ' | sort -u | paste -sd'|' -)

# The alternation group at the start of the regex in the workflow.
actual=$(grep -o "^ *pattern='\^([a-z|]*)" "$workflow" \
    | head -1 \
    | sed -e "s/.*\^(//" -e "s/)$//" \
    | tr '|' '\n' | sort -u | paste -sd'|' -)

if [ -z "$actual" ]; then
    echo "check-commit-types: could not extract the type list from $workflow" >&2
    echo "  the regex shape changed; update this script alongside it" >&2
    exit 1
fi

if [ "$canonical" != "$actual" ]; then
    echo "check-commit-types: the PR title gate has drifted from $types_file" >&2
    echo "  canonical: $canonical" >&2
    echo "  workflow:  $actual" >&2
    echo "  Update whichever is wrong, then re-run. Do not silence this." >&2
    exit 1
fi

echo "check-commit-types OK: $(echo "$canonical" | tr '|' ' ')"
