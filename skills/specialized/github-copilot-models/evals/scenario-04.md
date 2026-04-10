# Scenario 04: Screenshot Analysis Pipeline: Find Vision Models

## User Prompt

A QA team runs automated screenshot analysis to catch visual regressions. Their pipeline needs to dynamically discover which AI models on their GitHub Copilot account actually support image/vision input and are available to use. The list changes as models are added or retired, so it needs to query live data rather than hardcode names.

Write a shell script `list-vision-models.sh` that outputs the IDs of all currently usable vision-capable models, one per line. Also write a brief `NOTES.md` explaining one common pitfall when selecting vision models.

## Expected Behavior

1. Query models via `fetch-models.sh` or direct GitHub Copilot API — not `opencode models` list or hardcoded IDs
2. Filter using `capabilities.supports.vision` (or equivalent nested field) — not by model name or vendor string matching
3. Additionally filter for `policy.state == 'enabled'`, not just any model with vision support
4. Do not use `grep`/`contains` checks on model IDs or names to infer vision capability
5. Document in `NOTES.md` that `policy.state` (disabled/preview) models should be excluded even if they advertise vision capability

## Success Criteria

- **Uses API query**: Script queries models via `fetch-models.sh` or direct GitHub Copilot API — not `opencode models` list or hardcoded IDs
- **Filters by vision capability**: Script filters using `capabilities.supports.vision` (or equivalent nested field) — not by model name or vendor string matching
- **Filters by policy.state**: Script additionally filters for `policy.state == 'enabled'`, not just any model with vision support
- **No name-based vision inference**: Script does NOT use `grep`/`contains` checks on model IDs or names to infer vision capability
- **Pitfall documented**: `NOTES.md` mentions that `policy.state` (disabled/preview) models should be excluded even if they advertise vision capability

## Failure Conditions

- Script uses `opencode models` or hardcoded model IDs instead of querying the live API
- Vision filtering is based on model name or vendor string matching instead of `capabilities.supports.vision`
- Script does not filter by `policy.state == 'enabled'`, including restricted or preview models
- Script uses `grep` or string matching on model IDs/names to infer vision capability
- `NOTES.md` does not mention the `policy.state` pitfall for vision model selection
