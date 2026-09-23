# Archetype Reference and Auto-Detection

The 10 bridge archetypes, their meaning, and the keyword/context heuristics used to auto-detect them when the user does not supply an explicit `[type]`.

## The 10 Archetypes

| Code | Archetype | One-liner |
|---|---|---|
| `flywheel` | 🔄 Flywheel loop | Output of A feeds B feeds C → back to A |
| `knowledge` | 🧠 Knowledge cascade | Framework/learning from one domain reusable in another |
| `people` | 👤 People-bridge | Same person carries context across projects |
| `terrain` | 🌱 Terrain d'essai | One project is live lab for methods used in another |
| `narrative` | 📖 Narrative amplifier | One project generates stories that make another credible |
| `identity` | 🎭 Identity coherence | Projects collectively tell a story about who you are |
| `complexity` | 🔬 Complexity lab | Managing complexity in one domain trains patterns for another |
| `local` | 🤝 Local network overlay | Geographic proximity creates compound serendipity |
| `option` | ⚡ Option value | One project creates future optionality for another |
| `mirror` | 🪞 Mirror project | Introspective insights reshape how other projects are framed |

## Auto-Detection Signals

Applied in order when no explicit `[type]` is given in the capture command.

**Signal 1 — Keywords in description:**

- "pattern", "framework", "method", "learned", "reusable" → `knowledge`
- Person name (check config stakeholders) or "carries context", "cross-pollinates" → `people`
- "story", "credibility", "proof", "case study" → `narrative`
- "test ground", "lab", "experiment", "tried in" → `terrain`
- "loop", "feeds back", "cycle" → `flywheel`
- "brand", "who I am", "positioning", "identity" → `identity`
- "admin", "bureaucracy", "same skill", "transfers" → `complexity`
- "local", "geographic" → `local`
- "future", "optionality", "if it works", "unlocks" → `option`
- "introspect", "philosopher", "reframe", "reshape" → `mirror`

**Signal 2 — Config context:**

- If source or target has `flywheel_role: terrain` → lean toward `terrain`
- If source or target has `flywheel_role: mirror` → lean toward `mirror`
- If a stakeholder name appears in description and has `also_in` → `people`

If ambiguous after both signals, default to `knowledge` and say so in the response — NEVER silently guess a rarer archetype (`local`, `mirror`, `complexity`) on weak signal; a wrong rare-archetype tag is harder for the user to spot in `/bridge stats` than an honest `knowledge` default they can correct.

## Why Archetype Accuracy Matters

`/bridge stats` and `/bridge map` aggregate by archetype. A bridge tagged `narrative` when it was really `knowledge` doesn't just mislabel one row — it silently shifts the weekly-nudge threshold (≥3 same-pair bridges) because the pattern-matching logic in `/bridge stats` groups by archetype first, then by project pair. A misclassified bridge can suppress a nudge that should have fired, or trigger one for the wrong pair.
