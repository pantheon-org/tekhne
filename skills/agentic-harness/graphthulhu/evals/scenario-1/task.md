# Save Investigation Findings to the Knowledge Graph

## Problem/Feature Description

You have just finished analysing the authentication flow of a TypeScript API. Your investigation revealed the following findings:

- The token refresh logic in `src/auth/refresh.ts` has a race condition when multiple requests arrive simultaneously.
- The service uses HS256 JWT signing with a hardcoded 24h expiry — rotation is not implemented.
- Middleware ordering in `src/app.ts` means the rate-limiter runs after auth, so unauthenticated requests are not rate-limited.

These findings are non-obvious and will be relevant in future sessions.

## Task

Persist these investigation findings to the knowledge graph so they can be retrieved in future sessions. Demonstrate the full write workflow including namespace selection, deduplication check, and linking.

## Expected Behaviour

- Search for any existing page about auth or authentication before creating a new one.
- Create a page using a correct namespaced name (e.g. `investigations/auth-flow-analysis` or `analysis/auth-vulnerabilities`).
- Store the findings as blocks on the page.
- Link the page to any related pages that exist.
- Do NOT use a flat page name like `create_page("auth")`.
- Do NOT save speculative content — only the verified findings listed above.
