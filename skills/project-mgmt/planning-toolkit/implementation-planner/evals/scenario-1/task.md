# Scenario 01: Create an Implementation Plan for a URL Shortener Service

## User Prompt

Create a complete implementation plan for the following project.

## Requirements

Build a URL Shortener web service with the following capabilities:

- Accept a long URL via a REST API and return a short code
- Redirect short codes to their original URLs via HTTP 301/302
- Track click counts per short code
- Persist data in SQLite
- Expose a health check endpoint
- Provide a minimal HTML form for manual testing

## Constraints

- Runtime: Node.js with TypeScript
- No external authentication required
- Short codes must be URL-safe (alphanumeric only)
- The service must start with `npm start`
- All tests must run with `npm test`

Create a complete implementation plan under `.context/plans/` for this project. The plan should cover all work from bootstrapping the project through to a production-ready release.
