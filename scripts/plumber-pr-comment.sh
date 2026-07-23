#!/usr/bin/env bash
#
# plumber-pr-comment.sh
#
# Posts a PR comment with the Critical gate result and a per-severity breakdown
# of every finding on this branch (code, location, source link) — the same
# detail plumber-file-issues.sh writes into the rollup issue, so a PR author can
# see exactly what to fix without opening the Plumber check's log.
#
# Finds the existing comment by its <!-- plumber-pr-comment --> marker and edits
# it in place, rather than posting a fresh comment every run. Deliberately NOT
# `gh pr comment --edit-last`: that edits the last comment by the authenticated
# identity, not by this script — and every automated comment in this repo posts
# as the same github-actions[bot] identity, so --edit-last would silently
# overwrite whichever tool commented most recently. Marker-based lookup targets
# only this script's own comment.
#
# Finding extraction mirrors plumber-file-issues.sh: Plumber has no unified
# findings[] array with a per-finding severity, so this walks every top-level
# `*Result` key's issues[] generically and joins each one back to its severity
# via plumberScore.codeLosses.
set -euo pipefail

RESULTS_PATH="${1:?usage: plumber-pr-comment.sh <results.json>}"
REPO="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY must be set}"
PR_NUMBER="${PR_NUMBER:?PR_NUMBER must be set}"
ROLLUP_MARKER="<!-- plumber-compliance-backlog -->"
COMMENT_MARKER="<!-- plumber-pr-comment -->"
TMP_BODY="${RUNNER_TEMP:-/tmp}/plumber-pr-comment.md"
MAX_ROWS_PER_SECTION=25

if [ ! -f "$RESULTS_PATH" ]; then
    echo "No ${RESULTS_PATH} found; skipping PR comment."
    exit 0
fi

FINDINGS_JSON="$(jq -c '
    (.plumberScore.codeLosses // []) as $losses
    | ($losses | map({(.code): .severity}) | add // {}) as $sevmap
    | [
        to_entries[]
        | select(.key | endswith("Result"))
        | .key as $rk
        | (.value.issues // [])[]
        | {resultKey: $rk, code: (.code // "UNKNOWN"), severity: ($sevmap[.code] // "unknown"), issue: .}
    ]
' "$RESULTS_PATH")"

CRITICAL_COUNT="$(jq '.plumberScore.counts.critical // 0' "$RESULTS_PATH")"
HIGH_COUNT="$(jq '.plumberScore.counts.high // 0' "$RESULTS_PATH")"
MEDIUM_COUNT="$(jq '.plumberScore.counts.medium // 0' "$RESULTS_PATH")"
LOW_COUNT="$(jq '.plumberScore.counts.low // 0' "$RESULTS_PATH")"
TOTAL_COUNT="$((CRITICAL_COUNT + HIGH_COUNT + MEDIUM_COUNT + LOW_COUNT))"

ROLLUP_ISSUE="$(gh issue list --repo "$REPO" --search "\"${ROLLUP_MARKER}\" in:body" --state open --json number --jq '.[0].number // empty' 2>/dev/null || true)"

write_section() {
    local sev="$1" label="$2"
    local sev_findings sev_count
    sev_findings="$(jq -c --arg sev "$sev" '[.[] | select(.severity == $sev)]' <<<"$FINDINGS_JSON")"
    sev_count="$(jq 'length' <<<"$sev_findings")"
    [ "$sev_count" -eq 0 ] && return 0

    echo "### ${label} (${sev_count})"
    echo ""
    echo "| Code | Location | Source |"
    echo "| --- | --- | --- |"
    jq -r --argjson max "$MAX_ROWS_PER_SECTION" '
        .[:$max][]
        | "| \(.code) | \(.issue.jobName // .issue.job // .issue.branchName // "unknown") | \(.issue.url // "-") |"
    ' <<<"$sev_findings"
    if [ "$sev_count" -gt "$MAX_ROWS_PER_SECTION" ]; then
        echo ""
        echo "_...and $((sev_count - MAX_ROWS_PER_SECTION)) more. See \`plumber explain <code>\` or the full \`plumber-compliance\` artifact on this run for the rest._"
    fi
    echo ""
}

{
    echo "## 🛡️ Plumber — CI/CD Security Compliance"
    echo ""

    if [ "$CRITICAL_COUNT" -gt 0 ]; then
        echo "### 🔴 ${CRITICAL_COUNT} Critical-severity finding(s) — blocking merge"
        echo ""
    else
        echo "### ✅ No Critical-severity findings"
        echo ""
    fi

    write_section critical "🔴 Critical"
    write_section high "🟠 High"
    write_section medium "🟡 Medium"
    write_section low "🔵 Low"

    if [ "$TOTAL_COUNT" -eq 0 ]; then
        echo "No findings on this branch."
        echo ""
    elif [ -n "$ROLLUP_ISSUE" ]; then
        echo "High/Medium/Low findings are tracked in #${ROLLUP_ISSUE} once this merges — that issue reflects \`main\`, not this PR's branch, and only updates on push to \`main\`, so it may not match the tables above until then."
        echo ""
    fi

    echo "_[View this run](${GITHUB_SERVER_URL:-https://github.com}/${REPO}/actions/runs/${GITHUB_RUN_ID:-})_"
    echo ""
    echo "$COMMENT_MARKER"
} >"$TMP_BODY"

EXISTING_COMMENT_ID="$(gh api "repos/${REPO}/issues/${PR_NUMBER}/comments" --paginate 2>/dev/null \
    | jq -r --arg marker "$COMMENT_MARKER" '[.[] | select((.body // "") | contains($marker))] | last.id // empty' \
    || true)"

if [ -n "$EXISTING_COMMENT_ID" ]; then
    # -F (not -f): the "@file reads the value from a file" convention is
    # documented only for -F/--field. -f/--raw-field takes its value completely
    # literally, so -f body="@$TMP_BODY" would post the literal string
    # "@/path/to/file" as the comment body instead of its contents.
    gh api "repos/${REPO}/issues/comments/${EXISTING_COMMENT_ID}" -X PATCH -F body="@${TMP_BODY}" >/dev/null
else
    gh pr comment "$PR_NUMBER" --repo "$REPO" --body-file "$TMP_BODY"
fi
