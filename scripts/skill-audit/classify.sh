#!/usr/bin/env bash
#
# classify.sh
#
# Decide, for each skill in a pull request's audit, whether it blocks the pull
# request. Only a skill graded below B can block, and only when the pull
# request made it worse:
#
#   pass   A or A+
#   warn   B or B+ (non-blocking)
#   debt   below B, but scores no lower than it did at the base commit; the
#          pull request did not cause it, so it passes and is tracked by an issue
#   fail   below B and either new or scoring lower than at the base commit
#
# Both audits must come from the same auditor build so the scores compare. If
# the base audit cannot be read, nothing has a baseline and every skill below B
# fails, which is the safe answer.
#
# Usage: classify.sh <head-batch.json> <base-batch.json>
# Prints a JSON array to stdout: skill, grade, total, maxTotal, dimensions,
# baseTotal (null for a new skill), tier, reason.
set -euo pipefail

HEAD_JSON="${1:?usage: classify.sh <head-batch.json> <base-batch.json>}"
BASE_JSON="${2:?usage: classify.sh <head-batch.json> <base-batch.json>}"

# A missing or unreadable base audit means "no baseline", not an error.
BASE="$(jq -c '
    [ .[] | { key: (.skill | sub(".*/skills/"; "") | sub("/SKILL.md$"; "")), value: .total } ]
    | from_entries
' "$BASE_JSON" 2>/dev/null || echo '{}')"

jq --argjson base "$BASE" '
    [ .[]
      | (.skill | sub(".*/skills/"; "") | sub("/SKILL.md$"; "")) as $rel
      | ($base[$rel]) as $baseTotal
      | (.grade == "A" or .grade == "A+") as $isA
      | (.grade == "B" or .grade == "B+") as $isB
      | {
          skill: $rel, grade, total, maxTotal, dimensions,
          baseTotal: $baseTotal,
          tier: (if $isA then "pass"
                 elif $isB then "warn"
                 elif $baseTotal != null and .total >= $baseTotal then "debt"
                 else "fail" end),
          reason: (if $isA or $isB then null
                   elif $baseTotal == null then "new skill"
                   elif .total >= $baseTotal then "not lower than base"
                   else "lower than base" end)
        } ]
' "$HEAD_JSON"
