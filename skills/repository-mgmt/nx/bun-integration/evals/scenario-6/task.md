# Scenario 6: Plan the Bun rollout and CI run for an Nx workspace

## User Prompt

An Nx workspace has no Bun tooling yet. Write the ordered rollout plan for adopting Bun, from adding the plugin to the CI job that gates merges. Give the exact commands for each stage and say what you expect each command to produce. Cover how the plan changes if the pilot project fails validation.

## Repo state

- Nx workspace with twelve Node-based projects, all on `main`.
- CI runs on every merge request against `main`.
- Cache is enabled for build and test targets.
