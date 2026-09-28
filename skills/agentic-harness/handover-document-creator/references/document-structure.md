# Required Document Structure

Full detail for each required (and one optional) section of a `.context/handovers/YYYY-MM-DD-<slug>.md`
file. `SKILL.md` lists the sections in order; this reference explains what belongs in each one and why.

1. YAML frontmatter (see `assets/schemas/handover-frontmatter.schema.json`)
2. `# Handover: <short description matching the frontmatter title>`
3. `## Session Summary` - one paragraph: what this session set out to do and why, written for a reader
   with zero prior context.
4. `## Completed` - bulleted, each item load-bearing: what changed AND the evidence it happened (commit
   SHA, file path, test count, PR/MR link). "Ran the tests" is not evidence; "`bun test`: 42 pass, 0
   fail" is. The same rule applies to any proof-of-work record: a claim without evidence is worse than
   no claim, because it reads as done and isn't.
5. `## Outstanding` - what is NOT done: open items, blockers, decisions still needed, each with enough
   context to act on without this session's memory (file paths, what was already tried, why it
   stalled). If an item already has a `.context/follow-ups/` entry, link it instead of duplicating the
   description; if it doesn't and probably should, say so explicitly here rather than letting it exist
   only in this file.
6. `## Current State` - branch name, `git status` cleanliness, last commit SHA, any WIP stashes (tagged
   uniquely, never a bare `git stash`), and whether the branch is rebased/merged against `main`.
7. `## Next Steps` - concrete, numbered actions for whoever picks this up; each step independently
   actionable, not "continue the investigation".
8. `## Gotchas / Context` (optional) - non-obvious findings: decisions made and why, dead ends already
   ruled out, constraints discovered mid-session. Omit only if Completed/Outstanding already cover
   everything worth knowing.
9. `## References` - links to related plans, follow-ups, tickets, PRs/MRs, or journal entries.

## Frontmatter fields in full

Required: `title`, `type` (fixed to `handover`), `date` (ISO `YYYY-MM-DD`, must match the filename's
date segment), `branch`, `status` (`active` | `done` | `superseded`).

Optional: `session_id` (the outgoing session/worktree identifier, matching a `.context/logs/<session_id>/`
directory if one exists), `author`, `tags` (array), `related` (array of repo-relative paths to
plans/follow-ups/tickets/PRs), `supersedes` (path to an earlier handover this one replaces).

Example:

```yaml
---
title: "TICKET-123 example-service dev rollout - mid-migration"
type: handover
date: 2026-09-08
branch: feat/ticket-123-example-service-dev
status: active
---
```

Full contract: `assets/schemas/handover-frontmatter.schema.json`. Structure reference:
`assets/templates/handover.yaml`.
