#!/usr/bin/env sh
# Usage: new-plan.sh <slug>
# Creates: .context/plans/plan-<slug>/README.md
set -eu

SLUG="${1:?Usage: new-plan.sh <slug>}"
BASE=".context/plans/plan-${SLUG}"

mkdir -p "${BASE}/phases"

cat > "${BASE}/README.md" <<EOF
# Implementation Plan — ${SLUG}

## Goal

<!-- One paragraph: what the project builds, value delivered, and target users. -->

## Phases

| # | Phase | Status | Tasks |
|---|---|---|---|

## Constraints

<!-- Bullet list of non-negotiable technical, timeline, or resource constraints. -->

## References

<!-- Links to PRD, architecture docs, or ADRs. -->
EOF

echo "Created ${BASE}/README.md"
