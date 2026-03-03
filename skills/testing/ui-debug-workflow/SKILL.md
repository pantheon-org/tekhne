---
name: ui-debug-workflow
description: >
  Debug UI changes with a repeatable evidence-first workflow. Use when validating visual regressions,
  reproducing frontend bugs, comparing baseline vs changed behavior, collecting screenshots/DOM/logs,
  or producing stakeholder-ready UI debug reports. Keywords: ui bug, visual regression, browser devtools,
  playwright, screenshot evidence, dom snapshot, frontend debugging.
allowed-tools: Read, Write, Edit, Bash
---

# UI Debug Workflow

## When to Use

Use this skill when frontend behavior must be verified with reproducible evidence.

## When Not to Use

Do not use this workflow for backend-only issues with no UI symptom.

## Core Principles

1. Reproduce first, then modify code.
2. Capture baseline and changed evidence under the same conditions.
3. Compare artifacts before declaring a fix.
4. Document commands, environment, and outcomes.

## Deterministic Workflow

1. Capture baseline evidence from a known branch.
2. Apply fix and capture changed evidence with identical steps.
3. Compare screenshots, DOM, and logs.
   - If comparison shows unexpected differences: 1) verify identical conditions (URL, viewport, seed data, build mode) 2) check for flaky or animated elements 3) re-capture both runs if needed before concluding.
4. Record pass/fail outcomes and unresolved risks.
5. Publish a concise report with links to artifacts.

## Quick Commands

### Install browser runtime for automation

```bash
npx playwright install chromium
```

Expected result: Chromium runtime is available for automated UI checks.

### Capture baseline evidence

```bash
./skills/ui-debug-workflow/scripts/capture-evidence.sh baseline http://localhost:3000 ./baseline
```

Expected result: screenshots, DOM snapshots, and logs saved in `./baseline`.

### Capture changed evidence

```bash
./skills/ui-debug-workflow/scripts/capture-evidence.sh changed http://localhost:3000 ./changed
```

Expected result: comparable artifacts saved in `./changed`.

### Compare baseline vs changed artifacts

```bash
./skills/ui-debug-workflow/scripts/compare-evidence.sh ./baseline ./changed ./comparison.md
```

Expected result: `comparison.md` with summarized differences.

### Run end-to-end debug session

```bash
./skills/ui-debug-workflow/scripts/full-debug-session.sh main fix/my-branch http://localhost:3000 "npm run build" "npm start"
```

Expected result: complete session output with report-ready evidence.

### Evaluate this skill quality

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh ui-debug-workflow --json
```

Expected result: updated score and grade.

## Troubleshooting

**`capture-evidence.sh` fails to run:**
- Ensure the dev server is running and reachable at the target URL before executing the script.
- Check that the expected port is not occupied by another process (`lsof -i :<port>`).
- Confirm Chromium is installed (`npx playwright install chromium`).

**`compare-evidence.sh` produces no output or errors:**
- Verify both `./baseline` and `./changed` directories exist and contain artifacts.
- Re-run the capture steps if either directory is empty or incomplete.

**Comparison reveals unexpected differences:**
1. Confirm identical conditions: same URL, viewport size, seed data, and build mode.
2. Check for flaky elements (animations, timestamps, dynamic content) that may differ between runs.
3. Re-capture both baseline and changed runs under verified identical conditions before drawing conclusions.

## Anti-Patterns

### NEVER debug without deterministic reproduction steps

**WHY:** Non-reproducible issues cannot be verified or closed with confidence.

**BAD:** "I clicked around and it looked broken once."
**GOOD:** "1) Open /users 2) Click Sort 3) Observe console error."

**Consequence:** Fixes become guesswork and regressions reappear.

### NEVER skip console and network error review

**WHY:** Browser errors often reveal root cause faster than code browsing.

**BAD:** Edit components before checking runtime errors.
**GOOD:** Inspect console/network first, then form hypothesis.

**Consequence:** Time is wasted on unrelated code paths.

### NEVER compare baseline and changed runs under different conditions

**WHY:** Different data, viewport, or build mode invalidates comparisons.

**BAD:** Baseline on desktop, changed on mobile viewport.
**GOOD:** Use identical URL, viewport, seed data, and commands.

**Consequence:** False positives and false negatives in debug conclusions.

### NEVER close a UI fix without evidence artifacts

**WHY:** Unlogged fixes are hard to review and hard to trust.

**BAD:** Merge after manual spot-check with no screenshots/logs.
**GOOD:** Attach baseline, changed, and comparison artifacts.

**Consequence:** Stakeholders cannot verify what changed.

## References

- `references/debugging-checklist.md`
- `references/browser-devtools-guide.md`
- `references/evidence-templates.md`
