#!/usr/bin/env bash
#
# plumber-file-issues.sh
#
# Maintains a single persistent GitHub issue listing every High/Medium/Low
# severity Plumber finding, located by a stable marker in the issue body
# (not by title, which someone might edit). Every run regenerates the full
# body from the current report: creates the issue if none exists, edits it in
# place if open, reopens + edits it if a prior run closed it. If a run has zero
# High/Medium/Low findings and the issue is open, it is closed as resolved.
# This never creates more than one open issue.
#
# Plumber has no native "findings" array with a per-finding severity field —
# each control writes its own `<control>Result.issues[]` with a different
# shape (jobName vs job vs branchName, etc). Severity per issue code is only
# available in `plumberScore.codeLosses[]`, so this walks every top-level
# `*Result` key generically and joins each issue back to its severity by
# code. Critical findings are not listed here — plumber-gate.sh blocks on
# those instead.
set -euo pipefail

RESULTS_PATH="${1:?usage: plumber-file-issues.sh <results.json>}"
REPO="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY must be set}"
TMP_BODY="${RUNNER_TEMP:-/tmp}/plumber-issue-body.md"
MARKER="<!-- plumber-compliance-backlog -->"
TITLE="[Plumber] Compliance backlog (High/Medium/Low)"

if [ ! -f "$RESULTS_PATH" ]; then
    echo "No ${RESULTS_PATH} found; skipping issue filing."
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
        | select(.severity == "high" or .severity == "medium" or .severity == "low")
    ]
' "$RESULTS_PATH")"

COUNT="$(jq 'length' <<<"$FINDINGS_JSON")"

EXISTING_JSON="$(gh issue list --repo "$REPO" --search "\"${MARKER}\" in:body" --state all --json number,state --jq '.[0] // empty' 2>/dev/null || true)"
EXISTING_NUM=""
EXISTING_STATE=""
if [ -n "$EXISTING_JSON" ]; then
    EXISTING_NUM="$(jq -r '.number' <<<"$EXISTING_JSON")"
    EXISTING_STATE="$(jq -r '.state' <<<"$EXISTING_JSON")"
fi

if [ "$COUNT" -eq 0 ]; then
    echo "No High/Medium/Low findings this run."
    if [ -n "$EXISTING_NUM" ] && [ "$EXISTING_STATE" = "OPEN" ]; then
        gh issue comment "$EXISTING_NUM" --repo "$REPO" --body "All previously tracked findings are resolved as of this run. Closing."
        gh issue close "$EXISTING_NUM" --repo "$REPO" --reason completed
        echo "Closed #${EXISTING_NUM} (all findings resolved)."
    fi
    exit 0
fi

{
    echo "Plumber found ${COUNT} High/Medium/Low-severity finding(s) as of the latest run on \`${GITHUB_SHA:-unknown}\`."
    echo ""
    echo "This issue is regenerated on every run — its body reflects the current findings, not a history. Do not edit it by hand."
    echo ""
    for sev in high medium low; do
        sev_findings="$(jq -c --arg sev "$sev" '[.[] | select(.severity == $sev)]' <<<"$FINDINGS_JSON")"
        sev_count="$(jq 'length' <<<"$sev_findings")"
        [ "$sev_count" -eq 0 ] && continue
        title_case="$(tr '[:lower:]' '[:upper:]' <<<"${sev:0:1}")${sev:1}"
        echo "## ${title_case} (${sev_count})"
        echo ""
        echo "| Code | Location | Source |"
        echo "| --- | --- | --- |"
        jq -r '.[] | "| \(.code) | \(.issue.jobName // .issue.job // .issue.branchName // "unknown") | \(.issue.url // "-") |"' <<<"$sev_findings"
        echo ""
    done
    echo "$MARKER"
} >"$TMP_BODY"

if [ -n "$EXISTING_NUM" ]; then
    gh issue edit "$EXISTING_NUM" --repo "$REPO" --title "$TITLE" --body-file "$TMP_BODY"
    if [ "$EXISTING_STATE" = "CLOSED" ]; then
        gh issue reopen "$EXISTING_NUM" --repo "$REPO"
        echo "Reopened and updated #${EXISTING_NUM}."
    else
        echo "Updated #${EXISTING_NUM}."
    fi
else
    ISSUE_URL="$(gh issue create --repo "$REPO" --title "$TITLE" --body-file "$TMP_BODY")"
    echo "Filed ${ISSUE_URL}."
fi
