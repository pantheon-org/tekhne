# Scenario 02: Speed Up Docker Builds for a Node.js API

## User Prompt

A backend team maintains a Node.js Express API (`order-service`) that is deployed via Docker. During a recent sprint retrospective, engineers complained that Docker builds take 3-4 minutes on every small code change — even when no dependencies have changed. The tech lead suspects the build cache is not being used effectively, since each rebuild reinstalls all npm packages from scratch.

The team needs the Dockerfile restructured so that `npm ci` (the dependency install step) is only re-executed when `package.json` or `package-lock.json` actually changes. Code-only changes should reuse the cached dependency layer and complete in under 30 seconds.

Produce a new `Dockerfile` for the `order-service` Node.js application. The application listens on port 3000, the start command is `node src/index.js`, and Node.js 20 should be used.

Also produce an appropriate `.dockerignore` for a Node.js project.

Both files should be placed in the current directory.

## Expected Behavior

1. `COPY` of `package.json` and/or `package-lock.json` must appear BEFORE the `npm ci` (or install) `RUN` instruction
2. `COPY . .` (or broad source code copy) must appear AFTER the npm install instruction
3. Use a specific Node.js version tag — not `:latest`
4. Create a non-root user and place the `USER` instruction before `CMD`/`ENTRYPOINT`
5. Use `COPY` (not `ADD`) for all file copy operations
6. Include `node_modules/` in `.dockerignore`
7. Include `.env` and/or `.env.*` in `.dockerignore` to prevent secret leaks
8. Set `WORKDIR` to an absolute path
9. Use exec-form (JSON array) syntax for `CMD`

## Success Criteria

- **Dependency files copied first**: COPY of package.json and/or package-lock.json appears BEFORE the npm install/ci RUN instruction in the Dockerfile
- **App code copied after install**: COPY . . (or equivalent broad copy of source code) appears AFTER the npm install/ci RUN instruction
- **Pinned base image tag**: FROM uses a specific Node.js version tag (e.g., node:20-alpine) and NOT :latest
- **Non-root user**: A non-root user is created and the USER instruction appears before CMD/ENTRYPOINT
- **COPY used instead of ADD**: Dockerfile uses COPY for all file-copy operations; ADD is NOT used for plain file copying
- **.dockerignore excludes node_modules**: .dockerignore file is present and contains a node_modules/ entry
- **.dockerignore excludes secrets**: .dockerignore contains entries for .env and/or .env.* to prevent secret leaks
- **Absolute WORKDIR**: WORKDIR is set to an absolute path
- **Exec-form CMD**: CMD uses JSON array syntax rather than shell string form

## Failure Conditions

- `package.json`/`package-lock.json` are not copied before `npm ci` runs (cache busting on every code change)
- Source code (`COPY . .`) is copied before `npm install` runs
- `FROM` uses `:latest` or no tag
- No non-root user, or `USER` instruction is absent or placed after `CMD`
- `ADD` is used instead of `COPY` for file copying
- `.dockerignore` is missing `node_modules/` entry
- `.dockerignore` is missing `.env` or `.env.*` entries
- `WORKDIR` is relative or absent
- `CMD` uses shell string form instead of JSON array syntax
