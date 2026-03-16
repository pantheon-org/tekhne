# Scenario 1: Create a mise.toml for a New Project

## Context

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

## Task

Produce a `mise.toml` file for this project that:

1. Pins all three tools to the exact versions listed above.
2. Defines the three tasks with self-contained commands (no implicit shell state).
3. Sets `APP_ENV=development` in the `[env]` block.
4. Does not include `DATABASE_URL` or any other secrets.
5. Includes a comment in the file directing developers to load secrets from a
   `.env.local` file using `mise.env` or an equivalent pattern.

## Output Specification

Produce a single file `mise.toml`.
