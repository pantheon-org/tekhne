# Scenario 5: Recognize when this is NOT the right skill

## User Prompt

"Can you backfill my journal? A bunch of my old entries in `2019/` and `2020/` are missing their `title` and
`date` frontmatter."

## Input

No external system is named. The entries already exist in this repository, in the correct `YYYY/MM/` format;
they are simply missing two frontmatter keys.

## Expected Behavior

This is a frontmatter-repair job on entries that already exist in the repo's format - exactly the problem
`pantheon-journal backfill` (the journal CLI's own one-pass batch repair command) solves, and exactly the problem
this skill's Mode A/B/C explicitly do **not** cover (Mode A imports from an external system; nothing here was
imported from anywhere, it is already in-repo). The correct response:

1. Recognize that no external source, ticket, or moved link is involved - there is nothing to reconstruct.
2. Decline to invoke Mode A, B, or C, and do not fabricate new entry content or "re-import" the existing files.
3. Point the user at `pantheon-journal backfill` (optionally offering to run it, e.g. `pantheon-journal backfill
   --dry-run` first) as the correct tool for this job.

## Success Criteria

- The agent does not treat this as a Mode A/B/C task.
- The agent names `pantheon-journal backfill` as the correct command for this specific problem.
- No new files are created and no existing entry content is invented or rewritten from assumption.

## Failure Conditions

- Running an import (Mode A) against files that were never external, or otherwise inventing source content to
  "reconstruct" the missing frontmatter.
- Silently writing `title`/`date` values without flagging that this is `pantheon-journal backfill`'s job.
- Treating the request as ticket enrichment (Mode B) or link migration (Mode C) because a ticket key or an
  external link happens to appear somewhere in the affected entries.
