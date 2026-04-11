# Scenario 02: Document an API Migration Commit

## User Prompt

Your team has just completed a major restructuring of a REST API. The old `/v1/*` endpoint hierarchy is being retired and replaced with a new `/api/v2/*` structure. All existing clients integrating with the service will need to update their base URLs.

Specifically:
- `/v1/users` → `/api/v2/users`
- `/v1/auth/login` → `/api/v2/auth/login`
- `/v1/auth/logout` → `/api/v2/auth/logout`
- `/v1/products` → `/api/v2/products`

The change also removes support for the deprecated `X-API-Token` header in favor of Bearer token authentication, which was previously optional but is now required.

A full migration guide has been published at `docs/migration-v2.md`.

Write a commit message documenting this change that will be useful to consumers of the API and to automated release tooling that generates changelogs and determines version bumps.

Create a file called `commit-message.txt` containing the complete commit message (header, body if appropriate, and any footers).

## Expected Behavior

1. Use `!` after the type (e.g., `refactor!:` or `feat!:`) to signal a breaking change in the header
2. Include a `BREAKING CHANGE:` footer describing what changed (old paths, new paths, or removed header)
3. Write the footer in `Token: value` format (colon-space separated)
4. Separate the body or header from the footer section with exactly one blank line
5. Select a semantically appropriate type such as `refactor` or `feat`, not `chore`
6. Use imperative mood in the header description (e.g., `migrate`, `consolidate`, `move`)
7. Keep the header at 72 characters or fewer
8. If including a body, explain the motivation or context rather than listing files changed

## Success Criteria

- **Breaking change `!` suffix**: The commit header uses `!` after the type (or type/scope) to signal a breaking change, e.g., `refactor!:` or `feat!:`
- **BREAKING CHANGE footer present**: The commit message includes a `BREAKING CHANGE:` footer (not just the `!` suffix)
- **Footer describes migration**: The `BREAKING CHANGE:` footer value describes what changed (old paths, new paths, or removed header) rather than being empty or generic
- **Footer token:value format**: Footers are written as `Token: value` (colon-space separated), e.g., `BREAKING CHANGE: ...`
- **Blank line before footer**: There is exactly one blank line separating the body (or header) from the footer section
- **Correct type selection**: The type is `refactor`, `feat`, or another semantically appropriate type — NOT `chore`
- **Imperative header**: The header description uses imperative mood (e.g., `migrate`, `consolidate`, `move`) not past tense
- **Header <= 72 chars**: The header line is 72 characters or fewer
- **Body explains why**: If a body is included, it explains the motivation or context for the change rather than listing the files changed

## Failure Conditions

- Omits the `!` suffix from the type to signal a breaking change
- Omits the `BREAKING CHANGE:` footer entirely or relies solely on the `!` suffix
- `BREAKING CHANGE:` footer is empty, generic, or fails to mention the changed paths or header
- Footer does not follow `Token: value` (colon-space) format
- No blank line between the body (or header) and the footer section
- Uses `chore` type for a breaking API change
- Uses past tense in the header description (e.g., `migrated`, `moved`)
- Header exceeds 72 characters
- Body lists which files changed rather than explaining why the change was made
