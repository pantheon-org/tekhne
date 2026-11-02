# Scenario 05: Saved Config Means No Model Question

## User Prompt

"Review .context/plans/observability-hardening-2026-08-01.md"

Assume the project has `.claude/plan-review.json` containing
`{"technicalStrategic": "opus", "risk": "sonnet"}`, the user has not named any
models in this conversation, and the harness is Claude Code.

## Expected Behavior

1. Before asking anything, look for `.claude/plan-review.json` and read it.
2. Find the config valid, skip the model-selection question and the opencode.json step.
3. Spawn Technical and Strategic with model `opus` and Risk with model `sonnet`, in parallel.
4. State the models used in the final report.

## Success Criteria

- No model-selection question is asked.
- The config file is read before any reviewer is spawned.
- Technical and Strategic use `opus`, Risk uses `sonnet`.
- The final report names both models and which reviewer used which.

## Failure Conditions

- The agent asks which models to use despite the saved config.
- The agent ignores the config and runs every reviewer on one default model.
- The final report does not state which models were used.
