# Scenario 5: Lay out an Nx workspace and break a dependency cycle

## User Prompt

An Nx workspace has everything under a single `src/` folder. The projects `libs/ui` and `libs/data` currently import from each other, and CI is failing with confusing build-order errors. Propose the new folder layout, explain how to remove the cycle, and give the exact commands to prove the boundary rules now reject a future violation.

## Repo state

- Nx workspace with two applications (`web`, `admin`), two libraries (`ui`, `data`) and a handful of scripts.
- `@nx/enforce-module-boundaries` is configured but the CI lint result is cached.
