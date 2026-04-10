# Scenario 02: Model Recommendation Report

## User Prompt

A developer needs to pick GitHub Copilot models for three different automated tasks in their CI pipeline: (1) reviewing large pull requests that may span 300,000 tokens of context, (2) generating quick one-line docstrings for functions, and (3) analyzing screenshots of UI bugs. They ran the models API and got the response below. Write a markdown report called `model-recommendations.md` that recommends the best model for each task and explains why.

## Input Files

The following file is provided. Extract it before beginning.

=============== FILE: inputs/models-api-response.json ===============
```json
{
  "data": [
    {
      "id": "claude-sonnet-4-5",
      "name": "Claude Sonnet 4.5",
      "vendor": "Anthropic",
      "policy": { "state": "enabled" },
      "capabilities": {
        "limits": { "max_context_window_tokens": 200000 },
        "supports": { "vision": true }
      },
      "tags": ["powerful", "versatile"]
    },
    {
      "id": "gpt-5-turbo",
      "name": "GPT-5 Turbo",
      "vendor": "OpenAI",
      "policy": { "state": "enabled" },
      "capabilities": {
        "limits": { "max_context_window_tokens": 128000 },
        "supports": { "vision": false }
      },
      "tags": ["fast", "versatile"]
    },
    {
      "id": "o3-ultra",
      "name": "O3 Ultra",
      "vendor": "OpenAI",
      "policy": { "state": "preview" },
      "capabilities": {
        "limits": { "max_context_window_tokens": 500000 },
        "supports": { "vision": false }
      },
      "tags": ["powerful"]
    },
    {
      "id": "claude-haiku-4-5",
      "name": "Claude Haiku 4.5",
      "vendor": "Anthropic",
      "policy": { "state": "enabled" },
      "capabilities": {
        "limits": { "max_context_window_tokens": 200000 },
        "supports": { "vision": false }
      },
      "tags": ["fast"]
    },
    {
      "id": "gpt-4-legacy",
      "name": "GPT-4 Legacy",
      "vendor": "OpenAI",
      "policy": { "state": "disabled" },
      "capabilities": {
        "limits": { "max_context_window_tokens": 128000 },
        "supports": { "vision": true }
      },
      "tags": ["versatile"]
    }
  ]
}
```

## Expected Behavior

1. Do not recommend `gpt-4-legacy` for the screenshot task — its `policy.state` is `"disabled"`
2. Note that `o3-ultra` has restricted availability or should be verified before use — its `policy.state` is `"preview"`
3. Cite context window sizes that match the `capabilities.limits.max_context_window_tokens` values from the JSON — not guessed from model names
4. Base the screenshot task recommendation on `capabilities.supports.vision === true` — not assumed from the model name
5. Reference a model tagged `"fast"` (`claude-haiku-4-5` or `gpt-5-turbo`) for the quick docstring task
6. Recommend only models with `policy.state === "enabled"` for production pipeline use

## Success Criteria

- **Excludes disabled model**: Report does NOT recommend `gpt-4-legacy` for the screenshot task (`policy.state` is `"disabled"`)
- **Flags preview limitation**: Report notes that `o3-ultra` has restricted availability or should be verified before use (`policy.state` is `"preview"`)
- **Context from capabilities field**: Context window sizes cited in the report match the `capabilities.limits.max_context_window_tokens` values from the JSON — not guessed from model name
- **Vision capability checked**: Screenshot task recommendation is based on `capabilities.supports.vision === true` (not assumed from model name)
- **Fast tag for docstrings**: Quick docstring task recommendation references a model tagged `"fast"` (`claude-haiku-4-5` or `gpt-5-turbo`)
- **Enabled-only for production**: All models recommended for production pipeline use have `policy.state === "enabled"`

## Failure Conditions

- Report recommends `gpt-4-legacy` for any task despite its disabled policy state
- Report recommends `o3-ultra` for production use without noting its preview/restricted status
- Context window sizes in the report do not match the `capabilities.limits.max_context_window_tokens` values from the input JSON
- Screenshot task recommendation is based on model name or vendor rather than the `capabilities.supports.vision` field
- Quick docstring task recommendation uses a model not tagged `"fast"` when faster alternatives exist
- Any recommended production model has `policy.state` other than `"enabled"`
