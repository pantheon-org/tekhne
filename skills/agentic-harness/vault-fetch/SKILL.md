---
name: vault-fetch
description: >
  Fetch a URL, extract its text content, and persist it as a semantic memory
  tagged with the source URL for future retrieval. Use when the user shares a
  link to documentation, a GitHub issue, a blog post, or any reference material
  that should be available across sessions without re-fetching.
allowed-tools: "Bash"
---

# vault-fetch

Fetch a URL, extract its text content, and persist it as a semantic memory.

## Mindset

Any URL the user pastes that represents reference material — documentation, an
issue, an RFC, a blog post — should be fetched proactively without waiting to be
asked. If the URL will be useful in more than the current message, persist it.

## When to use

- User shares a documentation link to consult during the project
- A GitHub issue or PR contains important context
- External reference material needs to be available across sessions

## How to use

```bash
vault-cli fetch "<url>" [--project <id>]
```

The content is automatically:
- Captured with `tier: semantic` and `forceCapture: true`
- Truncated to 4000 characters
- Tagged with the source URL

No `--tier` flag is needed or accepted — tier is set automatically.

## Error handling

If the fetch fails (non-2xx response, timeout, or auth-gated page), fall back to
capturing the URL and title manually:

```bash
vault-cli capture --text "Reference: <title> — <url>" --tier semantic --tags reference,<topic>
```

## Examples

```bash
vault-cli fetch "https://bun.sh/docs/api/sqlite"
vault-cli fetch "https://github.com/org/repo/issues/42" --project my-app
vault-cli fetch "https://datatracker.ietf.org/doc/html/rfc9457" --project api
```

## Never

- **Never add `--tier`** — tier is set automatically to `semantic`; passing it will cause an error
- **Never fetch auth-gated URLs** (e.g. private GitHub repos, internal wikis behind SSO) — they return 401/403 and capture an error page instead of useful content; use the manual capture fallback instead
- **Never fetch `file://` or `localhost` URLs** — these are local paths unavailable to the fetch command
- **Never wait to be asked** when the user pastes a reference URL — fetch it proactively
