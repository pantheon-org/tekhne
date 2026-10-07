#!/usr/bin/env bash
#
# introduced-errors.sh
#
# Decides whether a pull request introduces agnix errors: an error in the head
# output whose rule and file are not an error in the base output. An error the
# base branch already has is tracked by its GitHub issue and does not fail the
# pull request, so a branch is never held up by a finding it did not cause.
#
#   - Only errors count; warnings are tracked by file-issues.sh and never fail.
#   - Errors are compared by rule and file, not line, so an edit that moves an
#     existing error does not make it new. A second instance of an error the
#     base already has in the same file is not counted either.
#   - If either output is unusable, or the base checked no files, the gate does
#     not fail: it says so with a ::warning:: rather than blocking on a broken
#     comparison.
#
# The introduced errors are written to $INTRODUCED_FILE as "RULE<TAB>FILE" lines
# and the script exits 1 if there are any.
#
# Usage: introduced-errors.sh <head agnix.json> <base agnix.json>
# Env:   INTRODUCED_FILE  output list (default: introduced-errors.txt)

set -uo pipefail

HEAD_JSON="${1:-}"
BASE_JSON="${2:-}"
INTRODUCED_FILE="${INTRODUCED_FILE:-introduced-errors.txt}"

if [ -z "$HEAD_JSON" ] || [ -z "$BASE_JSON" ]; then
    echo "usage: introduced-errors.sh <head agnix.json> <base agnix.json>" >&2
    exit 2
fi

: >"$INTRODUCED_FILE"

usable() {
    jq -e '(.diagnostics | type == "array") and (.files_checked | type == "number")
           and (.files_checked > 0)' "$1" >/dev/null 2>&1
}

if ! usable "$HEAD_JSON" || ! usable "$BASE_JSON"; then
    echo "::warning::No usable agnix output to compare with the base branch; not failing on introduced errors."
    exit 0
fi

jq -rn --slurpfile head "$HEAD_JSON" --slurpfile base "$BASE_JSON" '
    def plain: if startswith("/") then sub("^.*?/skills/"; "skills/") else ltrimstr("./") end;
    def keys_of($o): [ $o.diagnostics[] | select(.level == "error")
                       | "\(.rule)\t\(.file | plain)" ] | unique;
    (keys_of($base[0])) as $known
    | keys_of($head[0]) | map(select(. as $k | $known | index($k) | not)) | .[]
' >"$INTRODUCED_FILE"

if [ -s "$INTRODUCED_FILE" ]; then
    while IFS=$'\t' read -r rule file; do
        echo "::error::This pull request introduces agnix error ${rule} in ${file}."
    done <"$INTRODUCED_FILE"
    exit 1
fi
exit 0
