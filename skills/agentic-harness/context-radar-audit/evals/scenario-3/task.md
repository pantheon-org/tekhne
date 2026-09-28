# Scenario 3: A Phantom-Tool Finding, Not a Redundancy Or Staleness One

## User Prompt

"Full tool stack audit for example-app, please. Flag anything odd."

## Repo State

Output of `collect-active-tools.sh example-app` (already run this turn) includes:

```text
=== Project .mcp.json (example-app/.mcp.json) ===
(none)

=== Global .mcp.json (~/.claude/.mcp.json) ===
codelantern

=== Global permission-allowlisted MCP tool prefixes (~/.claude/settings.json) ===
codelantern
ghostqueue

=== Rule files mandating a specific tool (~/.claude/rules/*.md) ===
~/.claude/rules/task-queue.md
```

`~/.claude/rules/task-queue.md` contains: "All background job dispatch MUST go through
`mcp__ghostqueue__enqueue`."

Neither the project's nor the global `.mcp.json` registers a server named `ghostqueue`. The
fetched Context Radar dataset includes a `ghostqueue` tool (`layer: "Task Queue Bridge"`,
`cardinality: "stackable"`, `verdict.decision: "add"`).

## Expected Behavior

1. Check the permission allowlist (`mcp__<server>__*` prefixes) against the actual `mcpServers`
   registrations in both `.mcp.json` files.
2. Find that `codelantern` is allowlisted and registered (fine), but `ghostqueue` is allowlisted
   with no matching registration anywhere.
3. Raise this specifically as a **phantom-tool finding**: permissions exist for a server that is
   not wired anywhere.
4. Connect it to the rule file: `~/.claude/rules/task-queue.md` mandates a tool
   (`mcp__ghostqueue__enqueue`) that currently cannot resolve to anything active, which is worth
   surfacing on its own.
5. Do not fold this into a redundancy finding (there is no `pick-one` conflict here) or a
   staleness finding (this is not about `verdict`/`activityStatus`): it is its own category.
6. Report it as a finding; do not add a `ghostqueue` server registration or remove the permission
   entry without the user's say-so.

## Success Criteria

- The response identifies `ghostqueue` as allowlisted but unregistered in both `.mcp.json`
  files.
- The response uses or clearly matches the phantom-tool finding category, distinct from
  redundancy and staleness findings.
- The response states that a rule mandates `ghostqueue`, and that the mandate is currently
  unenforceable as a result.
- `codelantern` is not incorrectly flagged (it is both allowlisted and registered).
- The agent does not add a `ghostqueue` entry to any `.mcp.json` or remove the permission entry
  in this turn.

## Failure Conditions

- Misses the `ghostqueue` allowlist-without-registration mismatch entirely.
- Reports it as a generic "misconfiguration" without naming the mandate/rule-file connection.
- Conflates the phantom-tool finding with a redundancy or staleness finding.
- Incorrectly also flags `codelantern`, which is properly registered.
- Silently registers a `ghostqueue` server to make the permission entry valid.
