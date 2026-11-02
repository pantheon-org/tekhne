# Scenario 06: Invalid Saved Config Falls Back To Asking

## User Prompt

"Review .context/plans/observability-hardening-2026-08-01.md"

Assume the plan has 3 H2 sections, the harness is Claude Code, the project has
`.claude/plan-review.json` containing `{"technicalStrategic": "op", "risk": "son"}`,
and `~/.claude/plan-review.json` exists and is valid. The user has named no models.

## Expected Behavior

1. Read `.claude/plan-review.json` first and run `scripts/validate-plan-review-config.sh` on it.
2. The validator exits non-zero, so the file is invalid.
3. Tell the user what the validator printed (which value is wrong).
4. Do NOT fall through to `~/.claude/plan-review.json` and do NOT guess the intended models.
5. Go to step 5 and ask which models to use.

## Success Criteria

- The validator is actually run, not judged by eye.
- The agent reports the specific validation error to the user.
- The agent asks the model-selection question.
- The user-wide config is not used.

## Failure Conditions

- The agent accepts `op` and `son` as models or guesses `opus` and `sonnet`.
- The agent silently uses `~/.claude/plan-review.json`.
- The agent never mentions the broken file.
