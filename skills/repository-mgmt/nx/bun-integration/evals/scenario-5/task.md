# Scenario 5: Port a Node database and subprocess module to Bun inside an Nx project

## User Prompt

The Nx application `reports-api` (`apps/reports-api`) has just been converted to Bun executors. One module, `src/db.ts`, still contains Node-era code: it creates a pool of ten database connections at start-up, shares them across requests, and spawns a `pdftotext` child process for each report, reading its output as soon as the call returns.

Write a short migration note for this module that explains how the SQLite access and the subprocess call should be changed for Bun, and what to watch for in each. Include a small code sketch for each change.

## Repo state

- Nx workspace with `@nx-bun/nx` already installed.
- `apps/reports-api/project.json` already uses `@nx-bun/nx:run` and `@nx-bun/nx:build`.
