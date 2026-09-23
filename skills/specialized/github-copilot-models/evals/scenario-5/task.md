# Scenario 05: Incident: Default Model Stopped Responding

## User Prompt

A team's CI pipeline started failing overnight. Their `.opencode/opencode.json` has `"defaultModel": "gpt-4-legacy"` which was working last week. Now every run reports "model unavailable". OpenCode's model list still shows gpt-4-legacy, but requests are failing. The team needs to diagnose and recover.

Write two files:
1. `diagnose.sh` — a shell script that runs the correct checks to determine why the model is unavailable and suggests the next steps based on what it finds
2. `incident-report.md` — a brief incident report documenting the likely root cause and the recommended resolution steps

The script should handle at least: checking whether the model is still active on the account, verifying that the auth token is valid, and discovering a replacement model if needed.

## Expected Behavior

1. Have `diagnose.sh` query the GitHub Copilot API directly (via `fetch-models.sh` or `curl`) to check the target model's status — not relying on `opencode models` output
2. Inspect `policy.state` for `gpt-4-legacy` or the equivalent field that shows whether the model is active
3. Include a step to verify auth token validity (checks `~/.local/share/opencode/auth.json` or runs a re-auth command)
4. Explicitly state in `incident-report.md` or `diagnose.sh` that OpenCode's model list may be stale and should not be trusted as definitive
5. Discover and output at least one alternative enabled model as a suggested replacement
6. Identify in `incident-report.md` that the root cause is a model policy change (disabled/deprecated) — not a generic "unknown error"

## Success Criteria

- **API query for status check**: `diagnose.sh` queries the GitHub Copilot API directly (via `fetch-models.sh` or `curl`) to check the target model's status — not relying on `opencode models` output
- **policy.state checked**: Script inspects `policy.state` for `gpt-4-legacy` or the equivalent field that shows whether the model is active
- **Auth verification step**: Script includes a step to verify auth token validity (checks `~/.local/share/opencode/auth.json` or runs re-auth command)
- **Registry staleness noted**: `incident-report.md` or `diagnose.sh` explicitly states that OpenCode's model list may be stale and should not be trusted as definitive
- **Replacement model found**: Script discovers and outputs at least one alternative enabled model as a suggested replacement
- **Root cause in report**: `incident-report.md` identifies the root cause as model policy change (disabled/deprecated) rather than a generic "unknown error"

## Failure Conditions

- `diagnose.sh` uses `opencode models` instead of querying the GitHub Copilot API directly
- Script does not inspect `policy.state` to check whether the model is active
- No auth token verification step is included
- Neither `diagnose.sh` nor `incident-report.md` mentions that OpenCode's model registry may be stale
- Script does not discover or suggest a replacement enabled model
- `incident-report.md` attributes the failure to a generic error rather than a model policy change
