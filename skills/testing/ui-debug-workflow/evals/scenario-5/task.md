# Scenario 5: Diagnose a Failed Evidence Capture Run

## Context

A developer ran the following command and received an error:

```bash
./skills/ui-debug-workflow/scripts/capture-evidence.sh baseline http://localhost:3000 ./baseline
```

Error output:

```
Error: browserType.launch: Executable doesn't exist at
/home/runner/.cache/ms-playwright/chromium-1148/chrome-linux/chrome
```

The developer also reports that port 3000 is occupied by another service accidentally.

## Your Task

Using the UI Debug Workflow skill's troubleshooting section, produce a diagnosis and
fix plan saved to `capture-fix-plan.md`.

## Requirements

Your output must:

1. Identify all root causes present in the described failure (Chromium not installed,
   port conflict).
2. Provide the exact command(s) to resolve each root cause.
3. Provide the correct command to verify the dev server is running on the right port
   before re-running the script.
4. Provide the corrected capture command to run after fixing.

## Output Spec

File: `capture-fix-plan.md`
