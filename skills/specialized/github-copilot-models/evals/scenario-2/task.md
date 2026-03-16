# Setting a Project Default Model

## Problem Description

A developer is setting up a new project and wants to configure `claude-opus-4-6` as the default model in their OpenCode project config. They heard this model has a 200K context window, which is what they need for their use case. Before locking it in as the default, they want a safe onboarding process that validates the model is actually available on their account and behaves correctly.

Produce two files:
1. `validate-model.sh` — a shell script that verifies the model is accessible and active before configuring it as the default
2. `opencode.json` — the project config file with the model set as the default

The validation script should handle the case where the model is not available or is in a restricted state.
