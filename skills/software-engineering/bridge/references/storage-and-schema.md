# Storage, Schema and Write Guards

Detail for step 7 of the capture procedure. Read this before writing a bridge file.

## Location and filename

Bridges directory: `thinking/bridges/` under the configured workspace root (`$PRAXIS_DIR` or equivalent). Create it if it does not exist.

Filename: `{date}-{seq}-{source}-to-{target}.yaml`, with the aliases lower-cased. Example: `2026-04-08-1-hp-to-brand.yaml`. A bidirectional bridge is still one file, named source first.

## Choosing the sequence number

Take the highest sequence already used today and add one. Do not count the files.

Counting breaks as soon as a file has been deleted. With files 1, 2 and 3 and file 2 removed, the count is 2, so the next bridge becomes number 3 and overwrites an existing capture without any warning.

```bash
dir="${PRAXIS_DIR}/thinking/bridges"
today="$(date +%F)"
last="$(ls "$dir" 2>/dev/null | sed -n "s/^${today}-\([0-9]*\)-.*/\1/p" | sort -n | tail -1)"
seq=$(( ${last:-0} + 1 ))
```

Take the date from the system clock, not from the conversation. A date mentioned in the discussion is usually the date of the event, not of the capture.

## Schema

```yaml
source: HP
target: BR
archetype: narrative
emoji: 📖
description: "Philosopher council on construct AI → reframes human-centric positioning from marketing to philosophy"
direction: one-way  # one-way | bidirectional
strength: potential  # active | potential | theoretical
context: "During introspect session, realized philosopher encounters are unique differentiator"
date: 2026-04-08
```

| Field | Rule |
|---|---|
| `source`, `target` | Project alias, upper case, exactly as the config key |
| `archetype` | One of the 10 codes |
| `emoji` | The archetype's emoji from the table in the archetype reference |
| `description` | The user's words, unchanged. It is what flows from source to target |
| `direction` | `one-way` (A to B only) or `bidirectional` (A and B, one file) |
| `strength` | `active` (happening now), `potential` (could happen, not yet activated), `theoretical` (speculative) |
| `context` | Optional. What triggered the discovery: a session, a meeting, a realisation |
| `date` | ISO date of capture |

## Write guards

1. Confirm the workspace root is set. If not, stop with `⚠️ Workspace root not set. Configure via environment variable (e.g. export PRAXIS_DIR="$HOME/dev/praxis")`.
2. Confirm the target filename does not exist before writing. Never overwrite.
3. Quote `description` and `context` in double quotes. A description containing a colon, a hash or an arrow breaks an unquoted YAML scalar. Escape any double quote inside it.
4. Use a block scalar (`|`) when the description spans several lines. The characters are still kept verbatim.
5. Read the file back after writing and confirm it parses. A bridge that fails to parse is invisible to `/bridge list` and `/bridge map`.

## Duplicate check

Before writing, glob the directory and compare source, target and archetype with existing bridges. If a bridge on the same pair already carries the same idea, do not write a second file. Answer in one line: `🔗 Already captured: {filename}`. A weaker or stronger version of the same idea is a different bridge, so write it.
