# Scenario 04: Write a Migration Checklist for Switching from npm to Bun in an Nx Workspace

## User Prompt

Your team is migrating an Nx monorepo from npm to Bun. You have been asked to produce a written migration checklist that covers the lockfile cleanup and toolchain transition steps required to avoid the anti-patterns documented in the nx-bun-integration skill.

## Requirements

Produce a Markdown file `migration-checklist.md` that:

1. Lists the lockfile files that must be removed when switching from npm to Bun (e.g. `package-lock.json`).
2. States that only Bun should be used for package installs after migration (no `npm install` or `yarn` invocations in the migrated scope).
3. Covers the step of running `nx reset` after updating executor configurations to clear stale Nx daemon cache.
4. Mentions that validation of `nx build`, `nx test`, and `nx serve` on one project must pass before rolling out the migration to all projects.
5. Notes that hot/watch mode alone is not sufficient for production validation and that a non-hot build run must be verified.

The checklist should be organized with clear headings and bullet points. Each item should be actionable.

## Output

Produce the file `migration-checklist.md`.

## Expected Behavior

1. Explicitly name `package-lock.json` (and/or `yarn.lock`) as files to remove when switching to Bun
2. State clearly that `npm install` or `yarn` must not be used after migrating — only Bun for installs
3. Include a step to run `nx reset` after executor/plugin changes to clear stale Nx cache
4. Require validating `nx build`, `nx test`, and `nx serve` on a single pilot project before rolling out to all projects
5. Note that hot or watch mode alone is insufficient for production validation — a non-hot production-oriented build run must also be verified

## Success Criteria

- **Lockfile removal steps covered**: The checklist explicitly names `package-lock.json` (and/or `yarn.lock`) as files to remove when switching to Bun.
- **Single package manager discipline stated**: The checklist states that `npm install` or `yarn` must not be used after migrating to Bun; only Bun should be used for installs.
- **nx reset step included**: The checklist includes running `nx reset` after executor/plugin changes to clear stale Nx cache.
- **Phased validation on one project before rollout**: The checklist requires validating build, test, and serve on a single pilot project before rolling out to all projects.
- **Non-hot production build verification mentioned**: The checklist notes that hot or watch mode alone is insufficient and that a non-hot production-oriented run must be verified.

## Failure Conditions

- The checklist does not mention removing `package-lock.json` or equivalent lockfiles
- The checklist does not state that only Bun should be used for installs after migration
- The checklist omits the `nx reset` step, risking stale Nx daemon cache issues after the migration
- The checklist does not require a pilot project validation phase before rolling out to all projects
- The checklist does not mention verifying a non-hot production build, leaving the risk that only watch mode was tested
