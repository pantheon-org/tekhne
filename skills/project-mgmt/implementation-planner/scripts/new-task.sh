#!/usr/bin/env sh
# Usage: new-task.sh <plan-slug> <phase-number> <task-number> <task-slug>
# Creates: .context/plans/plan-<plan-slug>/phases/phase-<NN>-*/tasks/task-P<NN>T<NN>-<slug>.md
set -eu

PLAN_SLUG="${1:?Usage: new-task.sh <plan-slug> <phase-number> <task-number> <task-slug>}"
PHASE_NUM="${2:?Usage: new-task.sh <plan-slug> <phase-number> <task-number> <task-slug>}"
TASK_NUM="${3:?Usage: new-task.sh <plan-slug> <phase-number> <task-number> <task-slug>}"
TASK_SLUG="${4:?Usage: new-task.sh <plan-slug> <phase-number> <task-number> <task-slug>}"

PN="$(printf '%02d' "${PHASE_NUM}")"
TN="$(printf '%02d' "${TASK_NUM}")"
ID="P${PN}T${TN}"

PHASE_DIR=".context/plans/plan-${PLAN_SLUG}/phases/phase-${PN}-"

# Resolve the phase directory — must already exist
RESOLVED="$(find ".context/plans/plan-${PLAN_SLUG}/phases" -maxdepth 1 -type d -name "phase-${PN}-*" | head -n 1)"
if [ -z "${RESOLVED}" ]; then
  echo "Error: no phase directory matching phase-${PN}-* found under .context/plans/plan-${PLAN_SLUG}/phases/" >&2
  exit 1
fi

TASK_FILE="${RESOLVED}/tasks/task-${ID}-${TASK_SLUG}.md"

cat > "${TASK_FILE}" <<EOF
# ${ID} — ${TASK_SLUG}

## Phase

<!-- Phase $(printf '%02d' "${PHASE_NUM}") — <phase title> -->

## Goal

<!-- One sentence: what completing this task produces. -->

## File to create / modify

\`\`\`
<!-- relative/path/to/file -->
\`\`\`

## Implementation

\`\`\`
<!-- Exact file content or detailed pseudocode. -->
\`\`\`

## Notes

<!-- Bullet list of rationale, cross-references, or constraints. Omit if not needed. -->

## Verification

\`\`\`sh
# Commands that prove this task is complete. Every command must exit 0.
\`\`\`
EOF

echo "Created ${TASK_FILE}"
