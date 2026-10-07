# Worked Examples

Three captures showing how the signals combine. Each response is a single line.

## Knowledge bridge, defaults

```text
/bridge PROJ-A → PROJ-B: observability patterns from service mesh apply directly to the atelier content structure
```

- "patterns" matches `knowledge`. Nothing more specific matches.
- No `↔` and no hedge or present-tense word, so `one-way` and `potential`.
- Response: `🔗 Bridge #1: 🧠 PROJ-A → PROJ-B (knowledge) — observability patterns from service mesh apply directly to the atelier content structure`

## People bridge, bidirectional and active

```text
/bridge PROJ-A ↔ PROJ-C [people]: Matthieu carries positioning context both ways and cross-pollinates priorities
```

- `↔` and "both ways" give `bidirectional`.
- "carries" is present tense, so `active`.
- Response: `🔗 Bridge #2: 👤 PROJ-A ↔ PROJ-C (people) — Matthieu carries positioning context both ways and cross-pollinates priorities`

## Ambiguous archetype, hedged strength

```text
/bridge PROJ-A → PROJ-D: maybe the way we run planning here could inform how that team works
```

- No archetype keyword matches, so default to `knowledge` and flag it.
- "maybe" is a hedge, so `theoretical`, even though "could" also appears. Hedges are checked first.
- Response: `🔗 Bridge #3: 🧠 PROJ-A → PROJ-D (knowledge, defaulted) — maybe the way we run planning here could inform how that team works`

## Direction and strength signals

| Signal in the text | Result |
|---|---|
| `↔`, "bidirectional", "both ways" | `direction: bidirectional` |
| "maybe", "speculative", "in theory" | `strength: theoretical` |
| "already", "happening", "doing this", "carries" | `strength: active` |
| "could", "should", "would", "not yet", "if it works", "unlocks" | `strength: potential` |
| none of the above | `one-way` and `potential` |

Check in the order theoretical, active, potential. The first match decides.
