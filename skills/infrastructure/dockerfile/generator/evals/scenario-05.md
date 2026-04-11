# Scenario 05: Document a Containerization Decision for a TypeScript Next.js App

## User Prompt

A startup is preparing to ship their first container image to production. The engineering team has a Next.js 14 application (`storefront`) that needs to be containerized. The CTO wants a formal containerization decision document alongside the Dockerfile so the team understands the tradeoffs made: expected image sizes, which layers will be cached on code-only changes, and what security properties the image has.

The team has had bad experiences with "mystery builds" in the past — they need the Dockerfile to include a syntax directive at the top so they can take advantage of BuildKit features, and they want the decision document to explicitly list the next concrete steps before the image goes to production (CI pipeline wiring, local test commands, vulnerability scanning setup).

The Next.js app is built with `npm run build` and served with `npm start`. It listens on port 3000. Node.js 20 should be used.

Produce two files:
1. `Dockerfile` — a production Dockerfile for the Next.js storefront application
2. `containerization-decisions.md` — covering image size estimate vs full Node.js image, cache layer explanation, security properties, and next steps checklist

Also produce a `.dockerignore` for a Next.js project.

## Expected Behavior

1. Include `# syntax=docker/dockerfile:1` (or a versioned variant) at the top of the Dockerfile
2. Include an estimated image size in MB and compare it to a full Node.js image size in the decision document
3. Describe which Dockerfile layers are cache hits on code-only changes vs dependency changes
4. Include a bulleted or checkbox list of next steps before production (local build test, CI pipeline, vulnerability scanning)
5. Include at least two security properties: non-root user, minimal base image, no hardcoded secrets
6. Use multiple `FROM` stages (deps/builder/runner or similar) for the Next.js build
7. Pin all base image tags — no `:latest`
8. Create a non-root user and include a `USER` instruction before `CMD`
9. Include `.next/` or `node_modules/` in `.dockerignore`
10. Copy `package.json`/`package-lock.json` before `COPY . .` in the build stage

## Success Criteria

- **Syntax directive present**: Dockerfile starts with or contains '# syntax=docker/dockerfile:1' (or a versioned variant)
- **Image size estimate provided**: The decision document (or Dockerfile comments) includes an estimated image size in MB and compares it to a full Node.js image size
- **Cache layer explanation**: The decision document describes which layers are cache hits on code-only changes vs. dependency changes
- **Next steps checklist**: The decision document includes a bulleted or checkbox list of next steps before production (e.g., local build test, CI pipeline, vulnerability scanning)
- **Security summary**: The decision document includes at least two of: non-root user, minimal base image, no hardcoded secrets — as security properties
- **Multi-stage build**: Dockerfile uses multiple FROM stages (deps/builder/runner or similar) for the Next.js build
- **Pinned base image tags**: All FROM instructions use specific version tags and NOT :latest
- **Non-root user in Dockerfile**: Dockerfile creates a non-root user and includes a USER instruction before CMD
- **.dockerignore excludes Next.js build output**: .dockerignore is present and contains at least .next/ or node_modules/
- **Dependency layer caching order**: package.json/package-lock.json is COPYed before COPY . . in the build stage

## Failure Conditions

- Dockerfile is missing the `# syntax=docker/dockerfile:1` directive
- Decision document has no image size estimate or comparison to full Node.js image
- Decision document does not explain layer caching behavior
- Decision document has no next steps checklist
- Decision document includes fewer than 2 security properties
- Only one `FROM` stage (no multi-stage build)
- Any `FROM` uses `:latest` or no tag
- No non-root user, or `USER` instruction is absent or placed after `CMD`
- `.dockerignore` is missing `.next/` or `node_modules/`
- Source code is copied before `package.json`/`package-lock.json` in the build stage
