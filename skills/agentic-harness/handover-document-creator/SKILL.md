---
name: handover-document-creator
description:
  "Write a session handover document in .context/handovers/ so another agent or session can resume the work without re-deriving context. Use when a session is ending (or pausing) with work still in
  progress, when the user asks to 'write a handover', 'document what's outstanding for the next session', 'hand this off', or 'summarise where we got to', or when a long-running task is about to be
  interrupted (context limit, user stepping away, switching worktrees). Covers the frontmatter contract, required sections (Session Summary, Completed, Outstanding, Current State, Next Steps,
  References), the JSON Schema and shell validator that check a handover's shape, and how a follow-up session should pick one up and close it out."
---

# Handover Document Creator

Write (or pick up) a session handover: a point-in-time snapshot of what happened and what remains outstanding, filed under `.context/handovers/` so a different agent or a future session of the same
agent can resume the work cold, without needing this conversation's context.

## When to use

- The session is ending (context limit approaching, user stepping away, `/clear` about to happen) and meaningful work is still in progress
- The user explicitly asks for a handover, a "what's outstanding" summary for someone else, or to "hand this off"
- Work is moving between worktrees, branches, or agents (e.g. a wave-based parallel implementation, or handing a spike off to a colleague's session)

## When not to use

- The session's work is fully complete and nothing is outstanding: no handover is needed, the git history and any `.context/follow-ups/` entries already filed are sufficient
- A single outstanding item with no broader session context: file it as a `.context/follow-ups/` entry via the `create-context-file` skill instead (see Relationship to other typologies
  below) rather than writing a whole handover for one item
- Journal entries, tool assessments, or plan documents already cover the need: don't duplicate this skill's job into those, or vice versa

## Directory and naming convention

```text
.context/handovers/
  YYYY-MM-DD-<slug>.md
```

One dated file per handover, following the same `YYYY-MM-DD-<slug>.md` convention as `.context/follow-ups/` and `.context/plans/`. A handover is a snapshot, not a living document: a second handover on
the same topic is a **new** dated file with `supersedes` set, never an in-place edit of the first. Why, and how this differs from `tool-review-assessment`'s in-place convention:
`references/typology-and-lifecycle.md`.

## Frontmatter (required)

```yaml
---
title: "TICKET-123 example-service dev rollout - mid-migration"
type: handover
date: 2026-09-08 # ISO date (YYYY-MM-DD), must match the filename's date segment
branch: feat/ticket-123-example-service-dev
status: active # active | done | superseded
---
```

Required fields: `title`, `type` (fixed to `handover`), `date`, `branch`, `status`. Optional fields
(`session_id`, `author`, `tags`, `related`, `supersedes`) and a full worked example are in
`references/document-structure.md`.

## Required document structure

A handover has nine sections in a fixed order: frontmatter, an `# Handover:` title, `## Session Summary`,
`## Completed`, `## Outstanding`, `## Current State`, `## Next Steps`, an optional `## Gotchas / Context`,
and `## References`. The two sections most often written wrong:

- `## Completed` needs evidence per item (commit SHA, file path, test count, PR/MR link), not a narration
  of what was attempted. "Ran the tests" is not evidence; "`bun test`: 42 pass, 0 fail" is.
- `## Outstanding` links to an existing `.context/follow-ups/` entry instead of re-describing it, and
  flags plainly when an item probably needs a follow-up that doesn't exist yet.

Full section-by-section detail (what each one must contain, and why): `references/document-structure.md`.

## Workflow: writing a handover

1. **Gather the facts before writing.** Run `git status`, `git log -1`, `git stash list` (never bare `git stash`/`git stash pop` when several worktrees share one stash stack), and
   list any `.context/follow-ups/` entries filed this session.
2. **Write `.context/handovers/YYYY-MM-DD-<slug>.md`** following the structure above. Prefer a short, specific slug over a generic one (`ticket-123-example-service-dev-migration`, not `handover` or
   `session-end`).
3. **Cross-link, don't duplicate.** If outstanding work already has a `.context/follow-ups/` entry or a `.context/plans/` document, link to it from `## Outstanding` / `## References` rather than
   re-describing it in full.
4. **Run the validator** before considering the handover done:
   `bash <skill-dir>/scripts/validate-handover.sh .context/handovers/YYYY-MM-DD-<slug>.md`. Fix anything it flags.
5. **Commit it** with the outstanding work (or on its own if the handover is the only change left): `docs(handover): <short description>`.

## Workflow: picking up a handover

1. Read the handover file in full before touching any code - don't skim just `## Next Steps`; `## Gotchas / Context` and `## Current State` often carry the details that prevent repeating a dead end.
2. Verify `## Current State` still matches reality (`git status`, `git log -1`) before acting on it - time may have passed since it was written, or someone else may already have progressed the branch.
3. Do the outstanding work.
4. **Flip `status` to `done` in the same change that resolves the outstanding work** - never delete the file, matching the `create-context-file` follow-up lifecycle (`status: active` until actioned,
   then `done`). If only some outstanding items were resolved, leave `status: active` and update `## Outstanding` to reflect what remains, rather than marking the whole handover done prematurely.
5. Re-run the validator to confirm the frontmatter edit didn't break the schema.

## Relationship to other typologies

`.context/handovers/` is a distinct, standalone typology from `.context/follow-ups/`, `.context/plans/`,
and `.context/findings/`, and is not tracked in the auto-generated `.context/index.yaml`. A handover is
broader than a single follow-up item: it is the whole session's end state. Full explanation of the
boundary, and why a handover is a snapshot rather than an in-place-edited document (unlike
`tool-review-assessment`): `references/typology-and-lifecycle.md`.

## Assets

- **`assets/templates/handover.yaml`** - the required section list and frontmatter fields, in the same YAML-template style the `journal-entry-creator` skill uses for its own templates.
- **`assets/schemas/handover-frontmatter.schema.json`** - the JSON Schema every handover's frontmatter must satisfy.
- **`scripts/validate-handover.sh`** - run against a single handover file or the whole `.context/handovers/` directory:
  `bash <skill-dir>/scripts/validate-handover.sh .context/handovers/`. Checks filename shape, that the frontmatter date matches the filename's date segment, required
  frontmatter fields (`title`, `type: handover`, `date`, `branch`, `status`), and that all required sections are present.

## Mindset

- Evidence beats self-report: a "Completed" item without a commit SHA, test count, or file path is a claim, not a record
- Write for a reader who was not in this conversation and never will be: no unexplained shorthand, no "as discussed above"
- Outstanding work is either linked to an existing `.context/follow-ups/` entry or flagged as needing one - never left to exist only inside prose that will be someone's second read, not their first

## Anti-Patterns

**NEVER** write a handover that only lists what was attempted, with no evidence of what actually landed. **WHY:** a reader picking this up needs to know what is safe to build on, not just what was
tried. **BAD:** "Worked on the migration, made some progress." **GOOD:** "Migrated 3 of 5 tables (commit `a1b2c3d`); `bun test db/`: 12 pass, 0 fail; remaining 2 tables blocked on schema decision, see
Outstanding."

**NEVER** edit an existing handover in place to reflect a second session's progress. **WHY:** unlike a tool assessment, a handover is a snapshot of one session's end state; silently rewriting it loses
the audit trail of what the first session actually left behind. **BAD:** editing `2026-09-01-ticket-123-handover.md` directly when a second session continues the work. **GOOD:** writing
`2026-09-03-ticket-123-handover.md` with `supersedes: .context/handovers/2026-09-01-ticket-123-handover.md`.

**NEVER** duplicate a full `.context/follow-ups/` entry's description inside `## Outstanding` instead of linking it. **WHY:** two descriptions of the same open item drift apart over time and nobody
knows which is current. **BAD:** re-explaining a follow-up's whole context inline. **GOOD:** "See `.context/follow-ups/2026-09-08-x.md` - status unchanged since filing."

**NEVER** use a bare `git stash` or `git stash pop` while gathering Current State facts. **WHY:** the stash stack is shared by every worktree of a repository; a bare pop can steal another session's
in-progress work. **BAD:** `git stash` / `git stash pop`. **GOOD:** `git stash list`, and if something must be set aside, `git stash push -u -m "<unique-tag>"` with the SHA captured immediately.

## References

| Topic                             | Reference                                       | When to Use                                                             |
| ---------------------------------- | ------------------------------------------------ | -------------------------------------------------------------------------- |
| Section-by-section detail          | `references/document-structure.md`               | Full required-section list and frontmatter fields, with a worked example   |
| Typology boundary and lifecycle    | `references/typology-and-lifecycle.md`           | Why handovers are separate from follow-ups/plans, and the status lifecycle |
| Frontmatter contract               | `assets/schemas/handover-frontmatter.schema.json` | The JSON Schema every handover's frontmatter must satisfy                  |
| Structure template                 | `assets/templates/handover.yaml`                  | Required sections and frontmatter, in order                                |
| Mechanical validator               | `scripts/validate-handover.sh`                    | Run before considering a handover done or a pickup closed out              |
| Adjacent typology, different job   | `create-context-file` skill                       | Single-item follow-ups/plans/findings, tracked in `.context/index.yaml`     |
