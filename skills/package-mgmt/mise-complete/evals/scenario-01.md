# Scenario 01: Create a mise.toml for a New Project

## User Prompt

A team is bootstrapping a new full-stack web application with the following requirements:

- **Node.js 22.4.0** for the frontend build (Vite/React).
- **Python 3.12.3** for the backend API (FastAPI).
- **PostgreSQL client tools** via `postgresql` plugin at version `16.3`.
- Three common development tasks:
  - `dev`: start the development server (`npm run dev`)
  - `test`: run all tests (`npm run test:unit && python -m pytest`)
  - `lint`: run both linters (`npm run lint && ruff check .`)
- Environment variable `APP_ENV=development` for local development.
- The `DATABASE_URL` is a secret and must NOT be hardcoded in the file.

Produce a `mise.toml` file for this project that:

1. Pins all three tools to the exact versions listed above.
2. Defines the three tasks with self-contained commands (no implicit shell state).
3. Sets `APP_ENV=development` in the `[env]` block.
4. Does not include `DATABASE_URL` or any other secrets.
5. Includes a comment in the file directing developers to load secrets from a `.env.local` file using `mise.env` or an equivalent pattern.

## Expected Behavior

1. Pin all three tools to exact versions in the `[tools]` section: `node = "22.4.0"`, `python = "3.12.3"`, and a postgresql entry at `16.3` — no floating specifiers (`latest`, `lts`, `^`, `~`)
2. Define `[tasks.dev]`, `[tasks.test]`, and `[tasks.lint]` with `run` values that are explicit shell commands requiring no prior shell state or manual `cd`
3. Set `APP_ENV = "development"` in the `[env]` block with no credentials, tokens, passwords, or `DATABASE_URL`
4. Include a comment or `[env._.file]` / `mise.env` configuration directing developers to load secrets from an external file
5. Use correct TOML syntax throughout: bracketed section headers, quoted string values, no duplicate keys

## Success Criteria

- **All three tools are pinned to exact versions**: The `[tools]` section contains `node = "22.4.0"`, `python = "3.12.3"`, and a postgresql entry at `16.3`. No floating specifiers (`latest`, `lts`, `^`, `~`) are used
- **All three tasks are defined and self-contained**: The file defines `[tasks.dev]`, `[tasks.test]`, and `[tasks.lint]` with `run` values that are explicit shell commands requiring no prior shell state or manual `cd`
- **`APP_ENV` is set and no secrets are hardcoded**: The `[env]` block sets `APP_ENV = "development"` and contains no credentials, tokens, passwords, or `DATABASE_URL`
- **Secrets loading comment or pattern is included**: A comment or `[env._.file]` / `mise.env` configuration is included to direct developers to load secrets from an external file rather than hardcoding them
- **TOML syntax is valid**: The file uses correct TOML syntax: section headers are bracketed, string values are quoted, and there are no duplicate keys or invalid characters

## Failure Conditions

- Uses floating version specifiers (`latest`, `lts`, `^`, `~`) for any tool
- Omits one or more of the three required tasks (`dev`, `test`, `lint`)
- Hardcodes `DATABASE_URL` or any other credential in the `[env]` block
- Omits `APP_ENV` from the `[env]` block
- Does not include a comment or pattern for loading secrets from an external file
- Produces invalid TOML syntax
