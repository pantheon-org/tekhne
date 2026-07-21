# Typology catalog

The typology is the organizing principle for context files. It maps directly to
the subfolder name under the context root and is drawn from a curated set that
stays mostly static but may evolve deliberately.

## Curated typologies

| Typology | What it holds | Typical lifecycle |
| --- | --- | --- |
| `findings` | Investigation results, analysis, research output | Keep for reference |
| `plans` | Implementation plans, task breakdowns | Retire when the work lands |
| `guides` | Reusable how-tos and reference material | Keep and maintain |
| `follow-ups` | Deferred work, open threads, TODO capture | Close when actioned |
| `merge-requests` | MR/PR notes, descriptions, review context | Retire after merge |
| `tickets` | Issue write-ups and refinement notes | Track with the issue |
| `decisions` | Decision rationale and trade-offs | Keep indefinitely |
| `notes` | General working notes | Prune when stale |
| `research` | Longer-running research logs | Keep while active |

## Selection rule

Pick the typology by **what the artifact is**, not by how long it will live. An
investigation write-up is `findings` whether or not you keep it; a rollout
checklist is `plans` whether it lasts a day or a month.

When none of the curated typologies fits, prefer the closest match over
inventing a new folder. Introduce a new typology only when it will recur.

## Extending the set

The generator validates the requested typology against `KNOWN_TYPES` in
`scripts/create-context-file.sh`. To add a permanent typology, add it to that
list. For a genuine one-off, pass `--allow-new-type` instead of editing the
list, which keeps the curated set intentional.
