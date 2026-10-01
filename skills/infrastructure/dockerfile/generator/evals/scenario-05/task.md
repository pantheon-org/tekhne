# Scenario 05: Speed Up Docker Builds for a Node.js API

## User Prompt

A backend team maintains a Node.js Express API (`order-service`) that is deployed via Docker. During a recent sprint retrospective, engineers complained that Docker builds take 3-4 minutes on every small code change — even when no dependencies have changed. The tech lead suspects the build cache is not being used effectively, since each rebuild reinstalls all npm packages from scratch.

The team needs the Dockerfile restructured so that `npm ci` (the dependency install step) is only re-executed when `package.json` or `package-lock.json` actually changes. Code-only changes should reuse the cached dependency layer and complete in under 30 seconds.

Produce a new `Dockerfile` for the `order-service` Node.js application. The application listens on port 3000, the start command is `node src/index.js`, and Node.js 20 should be used.

Also produce an appropriate `.dockerignore` for a Node.js project.

Both files should be placed in the current directory.
