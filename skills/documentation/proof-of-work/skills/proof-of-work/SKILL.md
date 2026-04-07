---
name: proof-of-work
description: Captures and documents agent findings as verifiable artifacts — screenshots via agent-browser or playwright-mcp, captured logs, and script output. Use when completing investigations, audits, UI changes, infrastructure tasks, or any task where evidence of actions taken must be preserved.
metadata:
  version: 1.0.0
---

# Proof-of-Work Documentation

## When to Use

- "Capture a screenshot after deploying the change."
- "Document what you found in the logs."
- "Show me proof this worked."
- "Save the output of that command."
- After any investigation, audit, or multi-step task where a stakeholder needs to verify the outcome.

## When Not to Use

- Purely generative tasks (writing code, drafting text) where no external state was observed.
- Tasks where the user has explicitly said output or artifacts are not needed.

## Artifact Types

| Type | Tool | Use For |
|---|---|---|
| Screenshot | `agent-browser` or `playwright-mcp` | UI state, browser-rendered output, visual regressions |
| Log capture | shell / `ctx_execute` | Terminal output, CI logs, service logs |
| Script output | shell / `ctx_execute` | Command results, API responses, test runs |

## Workflow

1. **Identify evidence scope** — determine which artifact type(s) are appropriate for the task.
   Output: evidence plan (screenshot / log / script output, destination path).

2. **Capture the evidence** — use the appropriate tool (see below).
   Output: raw artifact saved to disk or embedded in response.

3. **Store artifacts** — write to the evidence directory using the naming convention.
   Output: file path(s) confirmed.

4. **Reference in response** — include the file path and a one-line description of what it shows.
   Output: summary block appended to the task response.

## Capturing Screenshots

### With `playwright-mcp`

```javascript
// Navigate and capture
await page.goto('https://example.com/dashboard');
await page.screenshot({ path: '.context/evidence/2026-04-07-dashboard.png', fullPage: true });
```

### With `agent-browser` (computer use)

Use the `screenshot` action after performing browser interactions:

```
action: screenshot
save_to: .context/evidence/2026-04-07-<slug>.png
```

## Capturing Logs

```bash
# Tail and save service logs
kubectl logs deploy/my-service --tail=100 | tee .context/evidence/2026-04-07-service-logs.txt

# Save CI step output
cat build.log > .context/evidence/2026-04-07-build-output.txt
```

## Capturing Script Output

```bash
# Run and persist output
bun run test 2>&1 | tee .context/evidence/2026-04-07-test-run.txt

# AWS CLI query result
aws cloudwatch get-metric-data ... > .context/evidence/2026-04-07-metrics.json
```

## Artifact Naming Convention

```
.context/evidence/YYYY-MM-DD-<slug>.<ext>
```

- `YYYY-MM-DD` — date the evidence was captured (use today's date)
- `<slug>` — kebab-case description of what it shows (`login-page-after-fix`, `test-pass`, `memory-usage-spike`)
- `<ext>` — `png` for screenshots, `txt` for logs, `json` for structured output

## Response Summary Block

Always append a summary block to your response when artifacts are created:

```markdown
## Evidence

| Artifact | Path | Description |
|---|---|---|
| Screenshot | `.context/evidence/2026-04-07-dashboard.png` | Dashboard after deployment — no errors visible |
| Log | `.context/evidence/2026-04-07-test-run.txt` | All 42 tests passed |
```

## Anti-Patterns

### NEVER describe what you observed without saving an artifact

WHY: prose summaries are not verifiable; artifacts are.
BAD: "The page loaded correctly." GOOD: save a screenshot and reference it.

### NEVER use vague slugs

WHY: unidentifiable filenames make evidence useless in audit trails.
BAD: `screenshot1.png`. GOOD: `2026-04-07-checkout-page-post-deploy.png`.

### NEVER skip evidence capture when the task involves external state

WHY: external state (UI, logs, infrastructure) cannot be re-observed after the fact.
BAD: run a migration and report "done". GOOD: capture and save the migration output.

## Verification

```bash
# Confirm artifacts were written
ls -lh .context/evidence/
```

```bash
# Check screenshots are valid images
file .context/evidence/*.png
```

## References

- [Artifact storage paths, naming convention, evidence table template](references/artifact-storage.md)
