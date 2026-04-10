# Scenario 04: Write a Stakeholder-Ready UI Debug Report

## User Prompt

A debug session for a login button rendering bug has been completed. The following artifacts exist:

- Baseline screenshots saved to `./baseline/` (button rendered correctly, no console errors)
- Changed screenshots saved to `./changed/` (button is 8px too low, one console warning: `CSS custom property --btn-offset not defined`)
- Comparison output at `./comparison.md` (shows pixel diff on button position)

Using the UI Debug Workflow skill, produce a stakeholder-ready debug report saved to `debug-report.md`.

The report must include:

1. A summary section: what was investigated, what was found.
2. Reproduction steps used to capture evidence.
3. A pass/fail outcome table listing what was compared and what the finding was.
4. Links (relative paths) to the baseline, changed, and comparison artifact directories/files.
5. Root cause hypothesis based on the console warning.
6. Recommended next action.
7. Any unresolved risks.

## Expected Behavior

1. Include a clearly labeled summary section stating what was investigated and the key finding (button 8px too low)
2. Include a table or list with explicit pass/fail outcomes for each compared artifact (screenshots, console errors)
3. Include relative path references to `./baseline/`, `./changed/`, and `./comparison.md`
4. Reference the CSS custom property warning (`--btn-offset not defined`) as the root cause hypothesis
5. Include both a recommended next action and an explicit section for unresolved risks

## Success Criteria

- **Summary section present**: `debug-report.md` contains a clearly labeled summary section stating what was investigated and the key finding (button 8px too low)
- **Pass/fail outcome table**: `debug-report.md` contains a table or list with explicit pass/fail outcomes for each compared artifact (screenshots, console errors)
- **Artifact links included**: `debug-report.md` contains relative path references to `./baseline/`, `./changed/`, and `./comparison.md`
- **Root cause hypothesis**: `debug-report.md` references the CSS custom property warning (`--btn-offset not defined`) as the root cause hypothesis
- **Next action and unresolved risks**: `debug-report.md` includes both a recommended next action and an explicit section for unresolved risks

## Failure Conditions

- `debug-report.md` has no summary section or does not mention the 8px button displacement finding
- The file has no pass/fail table or list covering the compared artifacts
- Relative paths to `./baseline/`, `./changed/`, and `./comparison.md` are missing
- The CSS custom property warning (`--btn-offset not defined`) is not mentioned as the root cause hypothesis
- The file is missing either the recommended next action or the unresolved risks section
