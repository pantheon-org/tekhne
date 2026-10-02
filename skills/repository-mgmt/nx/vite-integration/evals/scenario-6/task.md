# Scenario 6: Untangle a Vite configuration that mixes concerns

## User Prompt

A teammate's `apps/dashboard/vite.config.ts` has grown organically. It sets the Nx executor options for the build target inside comments, repeats dependency lists, sets a development proxy in the same object as coverage options, and has several ad-hoc overrides with no structure. Explain how you would reorganise the configuration so the two layers each live in the right file and each section is easy to debug, then show the restructured `vite.config.ts` skeleton.

## Repo state

- `apps/dashboard/project.json` exists with `build`, `serve` and `test` targets.
- The configuration is a single `defineConfig` call of about 150 lines.
