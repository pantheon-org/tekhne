# Scenario 02: Direct Input Standards — Full Scan and Report

## User Prompt

"Check my codebase against these standards:
1. No `console.log` in production files
2. All filenames must be kebab-case
3. Use `const` not `let` for variables that are never reassigned
4. No `var` keyword anywhere"

## Context

The codebase is a Vue 3 + JS project with ~30 source files in `web/src/`. The user has provided concrete, checkable standards.

## Expected Behavior

1. Acknowledge the standards and classify each by checkability (all 4 are fully auto-checkable).
2. Run scans for each standard (grep for `console.log`, check filenames, grep for `let` in non-reassigned contexts, grep for `var`).
3. Present a structured report with file paths and line numbers for each violation.
4. Group findings by standard, showing clear pass/fail status.
5. Offer to fix auto-fixable violations — but do NOT fix without asking.

## Success Criteria

- Every standard appears in the report with either ✅ or ⚠️ status.
- Every violation includes an exact file path and line number.
- Findings are grouped by standard, not mixed together.
- Auto-fix offer is made but not applied without confirmation.
- Standards that pass (zero violations) are shown as ✅.

## Failure Conditions

- The agent silently fixes violations without asking.
- Violations are reported without file paths or line numbers.
- A standard is missing from the report entirely.
- The agent skips the scan and generates a report based on assumptions.
