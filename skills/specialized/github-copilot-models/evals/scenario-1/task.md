# Model Recommendation Report

## Problem Description

A developer needs to pick GitHub Copilot models for three different automated tasks in their CI pipeline: (1) reviewing large pull requests that may span 300,000 tokens of context, (2) generating quick one-line docstrings for functions, and (3) analyzing screenshots of UI bugs. They ran the models API and got the response below. Write a markdown report called `model-recommendations.md` that recommends the best model for each task and explains why.

## Input Files

The following file is provided. Extract it before beginning.

=============== FILE: inputs/models-api-response.json ===============
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
