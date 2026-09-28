# Scenario 2: A Hook-Invoked Tool Is Not Treated As Inactive

## User Prompt

"Is lintguard actually in use anywhere? I don't see it in our .mcp.json."

## Repo State

Project: `example-app`. Neither the project's nor the global `.mcp.json` registers any server
named `lintguard`.

Output of `collect-active-tools.sh example-app` (already run this turn) includes:

```text
=== Global hook commands (~/.claude/settings.json) ===
PostToolUse | Edit|Write -> /home/dev/.local/share/lintguard/bin/lintguard-cli check --staged
```

The fetched Context Radar dataset includes:

```json
{
  "layers": [
    { "name": "Diagnostics Feed", "order": 11, "cardinality": "stackable", "curatedPick": null, "note": "Multiple diagnostics feeds can run without conflict." }
  ],
  "tools": [
    {
      "id": "lintguard",
      "tool": "LintGuard",
      "layer": "Diagnostics Feed",
      "verdict": { "decision": "watch" },
      "activityStatus": { "band": "dormant" },
      "decisionRule": "Maintained but low recent activity; keep running, don't newly adopt."
    }
  ]
}
```

## Expected Behavior

1. Recognize the hook command as evidence that `lintguard` is active, even though it is absent
   from both `.mcp.json` files.
2. Map the hook binary path (`lintguard-cli`, under a `lintguard` install directory) to the
   dataset id `lintguard`, using judgement rather than requiring an exact string match.
3. Do not conclude "not installed" or "inactive" from the `.mcp.json` absence alone.
4. Having correctly classified it as active, check its `verdict`/`activityStatus`: `watch` with
   `dormant` band is a staleness finding to note, not a recommendation to remove it on that
   signal alone.
5. Answer the user's literal question (yes, it is in use) before adding the staleness note.

## Success Criteria

- The response states that `lintguard` is active, based on the hook command.
- The response explicitly notes that MCP-server registration and hook wiring are checked
  independently, or otherwise makes clear that `.mcp.json` absence was not treated as proof of
  inactivity.
- The mapping from the hook's binary path to the dataset id `lintguard` is stated or evident.
- The `watch`/`dormant` combination is reported as a staleness note, not a removal
  recommendation.
- The response does not recommend removing the hook based on the staleness signal alone.

## Failure Conditions

- Concludes `lintguard` is "not installed" or "not in use" because it is absent from
  `.mcp.json`.
- Fails to connect the hook command to the dataset's `lintguard` entry at all.
- Treats the `watch`/`dormant` combination as a `drop`-equivalent removal finding.
- Answers only with the staleness note and never actually answers whether the tool is in use.
