# Document an API Migration Commit

## Problem Description

Your team has just completed a major restructuring of a REST API. The old `/v1/*` endpoint hierarchy is being retired and replaced with a new `/api/v2/*` structure. All existing clients integrating with the service will need to update their base URLs.

Specifically:
- `/v1/users` → `/api/v2/users`
- `/v1/auth/login` → `/api/v2/auth/login`
- `/v1/auth/logout` → `/api/v2/auth/logout`
- `/v1/products` → `/api/v2/products`

The change also removes support for the deprecated `X-API-Token` header in favor of Bearer token authentication, which was previously optional but is now required.

A full migration guide has been published at `docs/migration-v2.md`.

Write a commit message documenting this change that will be useful to consumers of the API and to automated release tooling that generates changelogs and determines version bumps.

## Output Specification

Create a file called `commit-message.txt` containing the complete commit message (header, body if appropriate, and any footers).
