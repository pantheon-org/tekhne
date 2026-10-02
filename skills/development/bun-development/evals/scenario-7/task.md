# Validate and Fix SQL Usage in an Existing Bun Service

## User Prompt

Our Bun service in `src/` uses `bun:sqlite`. Please check it for unsafe query patterns using the project's standard validation step, fix anything you find, and verify with the usual commands. Repo state: `src/users.ts` contains `db.query("SELECT * FROM users WHERE name = '" + name + "'")`; `src/config.ts` uses a fixed query with no inputs.
