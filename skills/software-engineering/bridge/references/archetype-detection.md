# Archetype Reference and Auto-Detection

The 10 bridge archetypes, their meaning, and the heuristics used to pick one when the user gives no explicit `[type]`.

## The 10 Archetypes

| Code | Emoji | Archetype | One-liner |
|---|---|---|---|
| `flywheel` | 🔄 | Flywheel loop | Output of A feeds B feeds C, then back to A |
| `knowledge` | 🧠 | Knowledge cascade | Framework or learning from one domain reusable in another |
| `people` | 👤 | People-bridge | Same person carries context across projects |
| `terrain` | 🌱 | Terrain d'essai | One project is the live lab for methods used in another |
| `narrative` | 📖 | Narrative amplifier | One project generates stories that make another credible |
| `identity` | 🎭 | Identity coherence | Projects collectively tell a story about who you are |
| `complexity` | 🔬 | Complexity lab | Managing complexity in one domain trains patterns for another |
| `local` | 🤝 | Local network overlay | Geographic proximity creates compound serendipity |
| `option` | ⚡ | Option value | One project creates future optionality for another |
| `mirror` | 🪞 | Mirror project | Introspective insights reshape how other projects are framed |

## Auto-Detection Signals

Apply the signals in order. Collect every archetype that matches before choosing.

### Signal 1: keywords in the description

- "pattern", "framework", "method", "learned", "reusable" point to `knowledge`.
- A person's name (check config stakeholders), "carries context" or "cross-pollinates" point to `people`.
- "story", "credibility", "proof", "case study" point to `narrative`.
- "test ground", "lab", "experiment", "tried in" point to `terrain`.
- "loop", "feeds back", "cycle" point to `flywheel`.
- "brand", "who I am", "positioning", "identity" point to `identity`.
- "complexity", "admin", "bureaucra" (matches bureaucracy and bureaucratic), "same skill", "transfers" point to `complexity`.
- "local", "geographic" point to `local`.
- "future", "optionality", "if it works", "unlocks" point to `option`.
- "introspect", "philosopher", "reframe", "reshape" point to `mirror`.

### Signal 2: config context

- Source or target with `flywheel_role: terrain` leans towards `terrain`.
- Source or target with `flywheel_role: mirror` leans towards `mirror`.
- A stakeholder named in the description whose entry has `also_in` settles on `people`.

## Choosing Between Matches

1. One archetype matched: use it, unless it is `local`, `mirror` or `complexity` on a single keyword. Those three need two distinct signals.
2. Several matched: prefer the specific archetype over `knowledge`. "Pattern" and "framework" are generic words, so they should not outvote a specific signal such as "optionality".
3. Several specific archetypes matched: pick the one with the most distinct signals. On a tie, use `knowledge`.
4. Nothing matched, or a rare archetype has only one signal: use `knowledge` and say so in the response line.

## Why Archetype Accuracy Matters

`/bridge stats` and `/bridge map` aggregate by archetype. A bridge tagged `narrative` when it was really `knowledge` mislabels more than one row: stats groups by archetype first and by project pair second, so the weekly nudge (three bridges on the same pair) can be suppressed for the right pair or fired for the wrong one. A visible `knowledge` default is cheap for the user to correct. A confident wrong rare archetype is not.
