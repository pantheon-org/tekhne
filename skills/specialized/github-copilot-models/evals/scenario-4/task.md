# Incident: Default Model Stopped Responding

## Problem Description

A team's CI pipeline started failing overnight. Their `.opencode/opencode.json` has `"defaultModel": "gpt-4-legacy"` which was working last week. Now every run reports "model unavailable". OpenCode's model list still shows gpt-4-legacy, but requests are failing. The team needs to diagnose and recover.

Write two files:
1. `diagnose.sh` — a shell script that runs the correct checks to determine why the model is unavailable and suggests the next steps based on what it finds
2. `incident-report.md` — a brief incident report documenting the likely root cause and the recommended resolution steps

The script should handle at least: checking whether the model is still active on the account, verifying that the auth token is valid, and discovering a replacement model if needed.
