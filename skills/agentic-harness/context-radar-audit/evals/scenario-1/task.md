# Scenario 1: A Pick-One Layer Redundancy Is Cited Against Its Cardinality

## User Prompt

"Audit our Claude Code tool stack against Context Radar and tell me if anything's redundant."

## Repo State

Project: `example-app`. Its `.mcp.json` registers two MCP servers: `graphnav` and `codelantern`.

The fetched Context Radar dataset (already run this turn) includes:

```json
{
  "meta": { "last_updated": "2026-08-01", "stars_verified": "2026-08-01", "tool_count": 94 },
  "layers": [
    {
      "name": "Code Navigation",
      "order": 5,
      "cardinality": "pick-one",
      "curatedPick": "graphnav",
      "note": "Two active graph servers double-index the same codebase and produce conflicting symbol results."
    }
  ],
  "tools": [
    { "id": "graphnav", "tool": "GraphNav", "layer": "Code Navigation", "verdict": { "decision": "best" }, "activityStatus": { "band": "active" } },
    { "id": "codelantern", "tool": "CodeLantern", "layer": "Code Navigation", "verdict": { "decision": "drop" }, "activityStatus": { "band": "active" }, "decisionRule": "Superseded by graphnav; unmaintained since 2025." }
  ]
}
```

Both `graphnav` and `codelantern` map to the `Code Navigation` layer, whose `cardinality` is
`pick-one`.

## Expected Behavior

1. Map both active servers to the `Code Navigation` layer and count them: two tools mapped to a
   `pick-one` layer.
2. Report a redundancy finding, citing the layer's `cardinality` (`pick-one`) and its `note` field
   as the reason, not a general impression that the two "seem duplicative".
3. Separately note that `codelantern`'s `verdict.decision` is `drop`, which is a removal finding
   on its own, independent of the redundancy finding.
4. Report `meta.last_updated` alongside the findings so they are dated against a known snapshot.
5. Present this as a finding for the user to act on. Do not silently remove either server from
   `.mcp.json`.

## Success Criteria

- Both `graphnav` and `codelantern` are identified as mapping to the same layer.
- The redundancy finding names the layer's `cardinality` value (`pick-one`) explicitly.
- The redundancy finding quotes or closely paraphrases the layer's `note` field as the reasoning.
- The response does not justify the finding with a vague phrase like "seems duplicative" instead
  of the cardinality/note reasoning.
- `codelantern`'s `drop` verdict is surfaced as a separate, additional reason to remove it.
- `meta.last_updated` (or an equivalent freshness statement) appears in the response.
- Neither `.mcp.json` entry is edited or removed by the agent in this turn.

## Failure Conditions

- Reports only one of the two conflicting tools, or misses the redundancy entirely.
- Justifies the finding with personal preference ("two code-nav tools is probably overkill")
  instead of citing `cardinality` and `note`.
- Silently edits `.mcp.json` to remove `codelantern`.
- Omits the dataset's freshness date entirely.
- Treats the `drop` verdict and the `pick-one` redundancy as the same finding without
  distinguishing the two reasons.
