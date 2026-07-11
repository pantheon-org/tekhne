# Scenario 06: Incremental Standards Added Mid-Review

## Phase 1 — User Prompt

"Check my codebase against this: all filenames must be kebab-case."

## Context

The agent scans and reports: 1 violation (`starChart.vue` should be `star-chart.vue`).

## Phase 2 — User Adds Standards

After seeing the report, the user says:

"Oh, also — no `console.log` anywhere, and use `const` not `let`."

## Expected Behavior

1. The agent acknowledges the new standards explicitly.
2. The agent does NOT re-run the kebab-case scan (already done).
3. The agent scans for `console.log` and `let` in non-reassigned variables.
4. The agent produces an *updated* report that merges previous findings with new ones.
5. The final report includes all 3 standards and their violations together.

## Success Criteria

- New standards are acknowledged before acting on them.
- Previously completed scans are not re-run unnecessarily.
- The updated report includes previous findings alongside new ones.
- Violations from all standards include file paths and line numbers.

## Failure Conditions

- The agent silently re-runs the kebab-case scan (wasteful).
- The agent only reports the new violations without merging with previous findings.
- The agent does not acknowledge the addition of new standards.
