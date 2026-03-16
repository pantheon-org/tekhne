# Document a Containerization Decision for a TypeScript Next.js App

## Problem/Feature Description

A startup is preparing to ship their first container image to production. The engineering team has a Next.js 14 application (`storefront`) that needs to be containerized. The CTO wants a formal containerization decision document alongside the Dockerfile so the team understands the tradeoffs made: expected image sizes, which layers will be cached on code-only changes, and what security properties the image has.

The team has had bad experiences with "mystery builds" in the past — they need the Dockerfile to include a syntax directive at the top so they can take advantage of BuildKit features, and they want the decision document to explicitly list the next concrete steps before the image goes to production (CI pipeline wiring, local test commands, vulnerability scanning setup).

The Next.js app is built with `npm run build` and served with `npm start`. It listens on port 3000. Node.js 20 should be used.

## Output Specification

Produce two files:

1. `Dockerfile` — a production Dockerfile for the Next.js storefront application.
2. `containerization-decisions.md` — a Markdown document covering:
   - The estimated final image size and how it compares to using a full Node.js image
   - Which Dockerfile layers will be cache hits on a code-only change vs. a dependency change
   - The security properties of the image (user context, base image, secrets handling)
   - A bulleted checklist of recommended next steps before the image goes to production

Also produce a `.dockerignore` for a Next.js project.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: package.json ===============
{
  "name": "storefront",
  "version": "0.1.0",
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start"
  },
  "dependencies": {
    "next": "14.1.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@types/node": "^20",
    "@types/react": "^18",
    "typescript": "^5"
  }
}
