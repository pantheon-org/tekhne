# Scenario 08: Document a Containerization Decision for a TypeScript Next.js App

## User Prompt

A startup is preparing to ship their first container image to production. The engineering team has a Next.js 14 application (`storefront`) that needs to be containerized. The CTO wants a formal containerization decision document alongside the Dockerfile so the team understands the tradeoffs made: expected image sizes, which layers will be cached on code-only changes, and what security properties the image has.

The team has had bad experiences with "mystery builds" in the past — they need the Dockerfile to include a syntax directive at the top so they can take advantage of BuildKit features, and they want the decision document to explicitly list the next concrete steps before the image goes to production (CI pipeline wiring, local test commands, vulnerability scanning setup).

The Next.js app is built with `npm run build` and served with `npm start`. It listens on port 3000. Node.js 20 should be used.

Produce two files:
1. `Dockerfile` — a production Dockerfile for the Next.js storefront application
2. `containerization-decisions.md` — covering image size estimate vs full Node.js image, cache layer explanation, security properties, and next steps checklist

Also produce a `.dockerignore` for a Next.js project.
