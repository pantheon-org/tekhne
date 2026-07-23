#!/usr/bin/env bash
#
# plumber-gate.sh
#
# Fails the job if the Plumber JSON report contains any Critical-severity
# finding. Plumber's own exit code reflects overall compliance against its
# score threshold, not per-finding severity, so this gates on
# plumberScore.counts.critical directly instead.
#
# Ported from pantheon-org/skill-quality-auditor, with one tekhne-specific
# change: a MISSING report is treated as a transient scan failure (warn, do not
# block), not a hard failure. plumber.yml runs the Plumber action with
# continue-on-error precisely so transient runtime errors (e.g. a rate-limited
# SLSA attestation check during a PR burst) never block merges; hard-failing
# here on a missing report would defeat that. A report that IS present and shows
# Critical findings still blocks.
set -euo pipefail

RESULTS_PATH="${1:?usage: plumber-gate.sh <results.json>}"

if [ ! -f "$RESULTS_PATH" ]; then
    echo "::warning::${RESULTS_PATH} not found; Plumber did not produce a report (likely a transient scan error). Critical gate skipped this run."
    exit 0
fi

CRITICAL_COUNT="$(jq '.plumberScore.counts.critical // 0' "$RESULTS_PATH")"

if [ "$CRITICAL_COUNT" -gt 0 ]; then
    echo "::error::${CRITICAL_COUNT} Critical-severity Plumber finding(s) present — see the Code Scanning alerts or job summary for detail."
    jq -r '
        (.plumberScore.codeLosses // [])[]
        | select(.severity == "critical")
        | "::error::[\(.code)] \(.count) Critical-severity finding(s)"
    ' "$RESULTS_PATH"
    exit 1
fi

echo "No Critical-severity Plumber findings. Gate passed."
