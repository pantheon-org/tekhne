# Scenario 4: Write a Stakeholder-Ready UI Debug Report

## Context

A debug session for a login button rendering bug has been completed. The following
artifacts exist:

- Baseline screenshots saved to `./baseline/` (button rendered correctly, no console errors)
- Changed screenshots saved to `./changed/` (button is 8px too low, one console warning:
  `CSS custom property --btn-offset not defined`)
- Comparison output at `./comparison.md` (shows pixel diff on button position)

## Your Task

Using the UI Debug Workflow skill, produce a stakeholder-ready debug report saved to
`debug-report.md`.

## Requirements

The report must include:

1. A summary section: what was investigated, what was found.
2. Reproduction steps used to capture evidence.
3. A pass/fail outcome table listing what was compared and what the finding was.
4. Links (relative paths) to the baseline, changed, and comparison artifact directories/files.
5. Root cause hypothesis based on the console warning.
6. Recommended next action.
7. Any unresolved risks.

## Output Spec

File: `debug-report.md`
