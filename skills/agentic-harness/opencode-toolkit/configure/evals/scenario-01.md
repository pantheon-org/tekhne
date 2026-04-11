# Scenario 01: Configure Anthropic Provider with Environment Variable API Key

## User Prompt

Set up OpenCode to use Anthropic Claude claude-sonnet-4-5 as the default model. The user's API key is stored in the `ANTHROPIC_API_KEY` environment variable. Show both the `opencode.json` configuration and how to verify it works.

## Expected Behavior

1. Use `baseEnv: 'ANTHROPIC_API_KEY'` in the provider configuration — never hardcode the actual key value
2. Place the provider config in `opencode.json`, not in `AGENTS.md` or any other file
3. Mark the claude-sonnet-4-5 model as `default: true`
4. Include a validation step (e.g., `jq . opencode.json` or `opencode run 'test'`) to verify the configuration is correct
5. Produce valid JSON with a `providers` array containing `id`, `baseEnv`, and `models` fields

## Success Criteria

- **Uses baseEnv not hardcoded key**: Uses `baseEnv: 'ANTHROPIC_API_KEY'` — NOT the actual key value in the config
- **Config placed in opencode.json**: Provider config goes in `opencode.json`, not in `AGENTS.md`
- **Model marked as default**: The claude-sonnet-4-5 model has `default: true`
- **Includes validation step**: Shows `jq . opencode.json` or `opencode run 'test'` to verify configuration
- **Correct JSON structure**: opencode.json has valid JSON with `providers` array containing id, baseEnv, models

## Failure Conditions

- Embeds the raw API key value directly in `opencode.json` instead of referencing the environment variable name
- Places provider configuration in `AGENTS.md` or a non-JSON config file
- Omits `default: true` from the model configuration
- Provides no way to verify the configuration after writing it
- Produces malformed JSON or uses a wrong structure (e.g., `provider` singular instead of `providers` array)
