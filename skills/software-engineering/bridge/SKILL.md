---
name: bridge
description: "Capture a cross-project connection the moment it surfaces, such as a shared pattern, a person carrying context between projects, a story that lends another project credibility, a feedback loop. Use when one project feeds, reuses, reinforces another: 'connects to', 'feeds into', 'same pattern as'. Writes one YAML file per bridge to a bridges directory, with list, map and stats subcommands."
allowed-tools: Bash, Read, Write, Edit, Glob
model: haiku
argument-hint: "<source> → <target>: <description>"
user-invocable: true
---

# Bridge: Cross-Project Connection Capture

Capture a bridge while you are still in context. Keep it in the bridges directory, never in project artifacts.

## Principles

- Capture fast, then step aside: answer in one line and resume the conversation that produced the link.
- Store the user's words, not yours, because a paraphrase erases the mental model that triggered the link.
- Choose an honest default over a confident guess; a wrong rare archetype hides in the stats, while a visible `knowledge` default gets corrected.
- Write to the bridges directory only.

## When to Use

- Use it when the user says "this connects to" or "same pattern as" another project.
- Use it when a person who works across two projects carries context between them.
- Use it when a session produces a story or proof point that strengthens another project.
- Capture a stated link even without `/bridge`, provided both projects resolve from config.

## When Not to Use

- Avoid capturing administrative links (shared calendar, same channel) that move no knowledge.
- Avoid capturing when both names point at one project; fix the alias in config instead.
- Stop and wait when the user is mid-decision, until a natural pause or an explicit `/bridge`.
- Check for an existing bridge first, and never recapture a link that is already stored.
- Avoid capturing during a weekly review, because bridges are for in-the-moment capture.

## Commands

Run `/bridge <src> → <tgt>: <desc>` to capture, adding `[type]` after the target to force an archetype. Read [list, map and stats output](references/list-map-stats.md), then run `/bridge list [project]`, `/bridge map` or `/bridge stats` to read bridges back.

## Capture Procedure

1. Read the project config at `$PRAXIS_DIR`. It holds aliases, names, tiers, flywheel roles and stakeholders with `also_in`. If it is missing, stop with `⚠️ No project config found. Create it with your project definitions.`
2. Split the input on `→` or `↔` for the source, then on `:` for the target, any `[type]` and the description.
3. Resolve both aliases against config keys, case-insensitively, then fuzzy-match against `name`. If either fails, write nothing. Warn and suggest adding the project to config.
4. Pick the archetype. Use `[type]` when given, otherwise apply [archetype detection](references/archetype-detection.md). When the signal is weak, use `knowledge` and say so.
5. Set the direction. Use `bidirectional` for `↔`, "bidirectional" or "both ways", otherwise `one-way`.
6. Set the strength. Check hedges first ("maybe", "in theory" give `theoretical`), then present tense ("already", "carries" give `active`), then "not yet", "should", "if it works" for `potential`. Default to `potential`.
7. Check for a duplicate, then write the file following [storage and schema](references/storage-and-schema.md).
8. Answer with one line, then resume the prior conversation. Use `↔` for bidirectional bridges and add `, defaulted` in the brackets after a defaulted archetype:

```text
🔗 Bridge #N: {emoji} {source} → {target} ({archetype}) — {short desc}
```

When three or more bridges this week share one project pair, append `📊 {N} bridges this week involving {pair}. Update strategic bridges during review.`

Take the highest number used today, add one, then confirm the file parses:

```bash
last="$(ls "$dir" | sed -n "s/^$(date +%F)-\([0-9]*\)-.*/\1/p" | sort -n | tail -1)"; seq=$(( ${last:-0} + 1 ))
yq '.' ./thinking/bridges/"$file" > /dev/null
```

## Anti-Patterns

- NEVER reformulate the user's description. WHY: a paraphrase adds your interpretation and erases the original mental model.
- NEVER comment on the bridge content. Keep the response to one line, because commentary derails the conversation.
- NEVER write to GTD files or project artifacts; mixing captures into task lists pollutes both.
- NEVER invent an alias that is not in config; invented aliases break list, map and stats.
- NEVER count files to get the sequence number, because after a deletion the count collides and overwrites a capture. Always use the highest number plus one.
- NEVER guess a rare archetype (`local`, `mirror`, `complexity`) on one keyword; the pitfall is a mislabel that skews the stats unseen.
- NEVER hardcode `one-way` and `potential`; run steps 5 and 6 so the stats do not repeat a wrong relationship.

```yaml
# BAD: "Matthieu carries context both ways" stored with the defaults
direction: one-way
strength: potential
# GOOD: "both ways" is bidirectional, "carries" is present tense
direction: bidirectional
strength: active
```

## References

- [Archetype Detection](references/archetype-detection.md): the 10 archetypes and the signals for picking one
- [Storage and Schema](references/storage-and-schema.md): filename, sequence number, YAML schema, write guards, duplicate check
- [List, Map and Stats](references/list-map-stats.md): output formats for the read-only subcommands
- [Worked Examples](references/worked-examples.md): three captures showing how the signals combine
