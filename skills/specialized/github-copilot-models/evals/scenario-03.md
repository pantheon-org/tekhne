# Scenario 03: Setting a Project Default Model

## User Prompt

A developer is setting up a new project and wants to configure `claude-opus-4-6` as the default model in their OpenCode project config. They heard this model has a 200K context window, which is what they need for their use case. Before locking it in as the default, they want a safe onboarding process that validates the model is actually available on their account and behaves correctly.

Produce two files:
1. `validate-model.sh` — a shell script that verifies the model is accessible and active before configuring it as the default
2. `opencode.json` — the project config file with the model set as the default

The validation script should handle the case where the model is not available or is in a restricted state.

## Expected Behavior

1. Have `validate-model.sh` fetch models from the GitHub Copilot API (via `fetch-models.sh` or direct `curl`) — not from `opencode models` or a hardcoded list
2. Verify the target model has `policy.state == 'enabled'` before proceeding
3. Read the auth token from `~/.local/share/opencode/auth.json`
4. Exit with a non-zero code or error message if the model is not found or not enabled
5. Include a per-command test step (e.g. `opencode run --model ... 'test prompt'`) before setting the project default
6. Include a `"defaultModel"` key set to the target model ID in `opencode.json`

## Success Criteria

- **API query in validation script**: `validate-model.sh` fetches models from the GitHub Copilot API (via `fetch-models.sh` or direct `curl`) — not from `opencode models` or a hardcoded list
- **policy.state check**: Script verifies the target model has `policy.state == 'enabled'` before proceeding
- **Auth from correct path**: Script reads auth token from `~/.local/share/opencode/auth.json`
- **Exits on unavailable model**: Script exits with a non-zero code or error message if model is not found or not enabled
- **Per-command test step**: `validate-model.sh` or instructions include a per-command test (e.g. `opencode run --model ... 'test prompt'`) before setting the project default
- **opencode.json defaultModel**: `opencode.json` contains a `"defaultModel"` key set to the target model ID

## Failure Conditions

- `validate-model.sh` uses `opencode models` or a hardcoded list instead of querying the GitHub Copilot API
- Script does not check `policy.state` to confirm the model is enabled
- Script reads auth from a path other than `~/.local/share/opencode/auth.json`
- Script does not exit with an error when the target model is unavailable or restricted
- No per-command test step is included before configuring the project default
- `opencode.json` does not contain a `"defaultModel"` key
