#!/usr/bin/env sh
# Usage: new-phase.sh <plan-slug> <phase-number> <phase-slug>
# Creates: .context/plans/plan-<plan-slug>/phases/phase-<NN>-<phase-slug>/README.md
set -eu

PLAN_SLUG="${1:?Usage: new-phase.sh <plan-slug> <phase-number> <phase-slug>}"
PHASE_NUM="${2:?Usage: new-phase.sh <plan-slug> <phase-number> <phase-slug>}"
PHASE_SLUG="${3:?Usage: new-phase.sh <plan-slug> <phase-number> <phase-slug>}"

PHASE_DIR=".context/plans/plan-${PLAN_SLUG}/phases/phase-$(printf '%02d' "${PHASE_NUM}")-${PHASE_SLUG}"

mkdir -p "${PHASE_DIR}/tasks"

cat > "${PHASE_DIR}/README.md" <<EOF
# Phase $(printf '%02d' "${PHASE_NUM}") — ${PHASE_SLUG}

## Goal

<!-- One or two sentences: what this phase delivers and why it is a natural milestone. -->

## Gate

<!-- Checklist of binary, runnable acceptance criteria. -->

- [ ] <!-- \`command\` exits 0 -->

## Dependencies

<!-- Bullet list of prior phases, files, services, or ADRs this phase depends on. -->

## Tasks

<!-- One H3 per task. Add rows as tasks are created. -->
EOF

echo "Created ${PHASE_DIR}/README.md"
