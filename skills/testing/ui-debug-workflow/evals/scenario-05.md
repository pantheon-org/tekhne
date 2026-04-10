# Scenario 05: Diagnose a Failed Evidence Capture Run

## User Prompt

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

Using the UI Debug Workflow skill's troubleshooting section, produce a diagnosis and fix plan saved to `capture-fix-plan.md`.

Your output must:

1. Identify all root causes present in the described failure (Chromium not installed, port conflict).
2. Provide the exact command(s) to resolve each root cause.
3. Provide the correct command to verify the dev server is running on the right port before re-running the script.
4. Provide the corrected capture command to run after fixing.

## Expected Behavior

1. Identify the missing Chromium executable as one root cause in `capture-fix-plan.md`
2. Identify the port 3000 conflict as a second root cause
3. Include `npx playwright install chromium` as the fix for the missing executable
4. Include a command to check port occupancy (e.g. `lsof -i :3000`) before re-running the script
5. Include the full corrected `capture-evidence.sh` command to run after all fixes are applied

## Success Criteria

- **Chromium not installed root cause identified**: `capture-fix-plan.md` identifies the missing Chromium executable as one root cause
- **Port conflict root cause identified**: `capture-fix-plan.md` identifies the port 3000 conflict as a second root cause
- **Chromium install command provided**: `capture-fix-plan.md` includes `npx playwright install chromium` as the fix for the missing executable
- **Port verification command provided**: `capture-fix-plan.md` includes a command to check port occupancy (e.g. `lsof -i :3000`) before re-running the script
- **Corrected capture command provided**: `capture-fix-plan.md` includes the full corrected `capture-evidence.sh` command to run after all fixes are applied

## Failure Conditions

- `capture-fix-plan.md` does not identify the missing Chromium executable as a root cause
- `capture-fix-plan.md` does not identify the port 3000 conflict as a root cause
- `npx playwright install chromium` is not included as the Chromium fix command
- No port verification command (e.g. `lsof -i :3000`) is included
- The corrected `capture-evidence.sh` command is missing from the fix plan
