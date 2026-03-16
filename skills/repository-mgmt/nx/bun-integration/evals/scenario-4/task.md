# Task: Write a migration checklist for switching from npm to Bun in an Nx workspace

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
