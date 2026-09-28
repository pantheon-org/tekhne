# Context Radar dataset: field reference

The dataset fetched by `scripts/fetch-comparison-data.js` is a single JSON object shaped
`{meta, layers, tools}`. This reference documents every field the audit workflow reads. SKILL.md
only needs the summary; come here when you need to know exactly what a field means or how it is
structured.

## `meta`

```json
{
  "last_updated": "2026-08-01",
  "stars_verified": "2026-08-01",
  "tool_count": 94
}
```

Always report `last_updated` and `stars_verified` alongside any finding, since tool verdicts and
star counts drift and a finding without a dated snapshot cannot be checked later against a newer
one.

## `layers[]`

```json
{
  "name": "Code Navigation",
  "order": 5,
  "cardinality": "pick-one",
  "curatedPick": "graphnav",
  "note": "Two active graph servers double-index the same codebase and produce conflicting symbol results."
}
```

- `name` / `order` - identity and the position the layer occupies in Context Radar's own
  20-layer ordering, used only for presenting findings in a stable sequence.
- `cardinality` is the load-bearing field:
  - `pick-one` / `either-or` - install exactly one tool for this layer; running two causes
    duplicate work or ambiguous tool routing.
  - `stackable` - multiple tools may coexist.
  - `install-both` - currently just Tool Search; zero-config, always-on.
  - `reference` - a curated list, not itself installable.
- `curatedPick` - the tool id Context Radar recommends for this layer when nothing is active yet.
  A layer with a `curatedPick` and zero active coverage is an optional-add note, not a finding.
- `note` - the stated reason for the layer's cardinality. Cite this verbatim in a redundancy
  finding rather than restating the reasoning in your own words.

## `tools[]`

```json
{
  "id": "graphnav",
  "tool": "GraphNav",
  "layer": "Code Navigation",
  "whatItDoes": "Builds a call graph and answers structural queries over it.",
  "conflict": { "severity": "high", "projects": ["codelantern"], "note": "Both index the same graph." },
  "activityStatus": { "band": "active" },
  "verdict": { "decision": "best" },
  "decisionRule": "Prefer this over codelantern for new projects; codelantern is unmaintained.",
  "requiresExternal": false,
  "stars": 4200
}
```

- `id` is the stable key you map an active tool onto; it rarely matches an MCP server name or a
  hook binary's basename exactly, so mapping needs judgement, not string matching.
- `layer` links back to a `layers[]` entry by `name`.
- `conflict` names specific other tool ids this one is known to clash with, independent of
  layer cardinality; a `severity` of `high` is worth surfacing even inside a `stackable` layer.
- `activityStatus.band` is one of `active`, `watch`, or `dormant`. A `watch`-decision tool with a
  `dormant` band is a staleness finding: note it, do not recommend removal from that signal alone.
- `verdict.decision` is one of `best | add | add-if | either-or | watch | reference | drop`.
  `drop` is a removal finding on its own.
- `decisionRule` is a one-line summary of when to prefer this tool over its conflicts; treat it as
  a pointer to read the tool's own docs before adopting it, not as installation instructions.
- `requiresExternal` flags a tool that needs an external service or account beyond a local
  install; surface this when recommending an `curatedPick` for an uncovered layer.
- `stars` is a raw GitHub star count as of `meta.stars_verified`; report it alongside a verdict
  rather than treating it as an independent recommendation signal.
