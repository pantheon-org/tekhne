---
name: risk-register
description: "Maintain docs/RISK_REGISTER.md, the living, append-only list of every deferred item, interim shortcut, and open architecture decision for this project. Use when a shortcut is taken, a decision is postponed, a follow-up is filed, or an existing entry is actually resolved. DO NOT use for accepted, finalized architecture decisions (use adr-capture instead), for session-local working notes that don't need to survive across the team (use context-file / the local scratch-notes directory instead), or for anything that must stay confidential (the register is a committed, team-visible doc). Triggers: 'add to risk register', 'log this as a risk', 'track this shortcut', 'this is a known gap', 'mark risk #N resolved', 'what's on the risk register', 'what's still open'."
---

# Risk Register

A single committed table (`docs/RISK_REGISTER.md`) of every deferred item, interim shortcut, and open architecture
decision that could otherwise get forgotten between sessions or team members. Unlike gitignored, session-local
scratch notes, this is a durable, shared, GitHub-visible record -- the register is the index; a scratch note or a PR
is where the extra detail lives, if there is more to say than fits in a row.

## Prerequisites

- `docs/RISK_REGISTER.md` exists. If this is the first entry in a project and the file doesn't exist yet, create it
  with a `# Risk Register` title, an intro paragraph (paraphrase the one above), and an empty table using the columns
  in `assets/templates/risk-register-row-template.yaml`.
- `assets/templates/risk-register-row-template.yaml` -- the shape of a single row, illustrated
- `assets/schemas/risk-register-row.schema.json` -- the same shape, as enforceable constraints
- `scripts/validate-risk-register-append-only.sh` and `scripts/validate-risk-register-schema.sh`, both wired into a
  local pre-commit hook so a row can never silently vanish, and a malformed row can never be committed

## When to Use

- The moment a shortcut is taken, a decision is postponed, or a known gap is discovered -- not retroactively.
- A scratch note is created for something that should also be durably tracked (most of them should; see
  [Relationship to Sibling Conventions](references/relationship-to-sibling-conventions.md)).
- Someone asks what's deferred, what's an accepted risk, or what's still open.
- An entry is actually resolved and needs to be marked as such.

## When NOT to Use

- A **finalized, accepted architectural decision** with no open question -- that's an ADR (`adr-capture`), not a risk.
- **Ephemeral, single-session scratch notes** that don't need to outlive the current conversation -- no register
  entry needed.
- Anything **confidential** -- this file is committed and visible to everyone with repo access.

## Row Schema

| Column               | Meaning                                                                                          |
| -------------------- | ------------------------------------------------------------------------------------------------- |
| `#`                  | Permanent id, assigned once, never reused or renumbered. Next id = current max + 1.               |
| Item                 | Short name, not a sentence.                                                                       |
| Type                 | `Deferred` (will do later) / `Shortcut` (interim workaround in place now) / `Accepted Risk` (known, decided to live with it) / `Open Decision` (a choice not yet made, blocking further work) |
| Description          | Self-contained: what it is and why it exists. A reader with no other context should understand it. |
| Risk if unaddressed  | One sentence: the concrete failure mode, not a vague "could be a problem."                        |
| Added                | `YYYY-MM-DD`, the date the row was created. Never changes.                                        |
| Status               | `Open` until resolved, then `Resolved`. Lifecycle state only -- no date or rationale appended here. |
| Date                 | `--` while `Status` is `Open`; the resolution date (`YYYY-MM-DD`) once `Status` flips to `Resolved`. Set together with Decision. |
| Decision             | `--` while `Status` is `Open`; a self-contained sentence on how it was resolved once `Status` flips to `Resolved`. Set together with Date. |

## Workflow

### Adding an entry

1. **Confirm this is genuinely unresolved before filing it.** A discrepancy you haven't seen before is a candidate,
   not a verdict -- search the project's root agent-guidance file and its instructions directory for an existing rule
   that already settles it before treating it as an open item. See
   [Case Study: Verify "Unresolved" Before Filing](references/unresolved-verification-case-study.md) for the concrete
   failure mode this step exists to prevent.
2. Read the current last row's `#` in `docs/RISK_REGISTER.md`; the new row's `#` is that value + 1.
3. Write a self-contained `Description` -- don't assume the reader has whatever prompted this in front of them.
   **Never cite a `.context/` file as where the extra detail lives** -- `.context/` is gitignored local scratch (see
   row 11), so a reference to it may be dead for every reader but the one who filed it. If there is more detail than
   fits in the row, inline the essential facts directly, or point at a committed doc (an ADR, an MR description,
   another `docs/*.md` page) instead.
4. Append the row, matching the row template's field shapes (`Type` from its enum, `Added` as `YYYY-MM-DD`, `Status`
   starting `Open` with `Date`/`Decision` both `--`). Never insert out of `#` order, never renumber existing rows.
5. Run both validation scripts and confirm each prints no error output and exits `0` before committing -- the
   pre-commit hook runs both too, but checking early avoids a bounced commit.

### Resolving an entry

1. Find the row by its `#` or Item name.
2. Amend `Status` to `Resolved`, `Date` to the resolution date (`YYYY-MM-DD`), and `Decision` to a self-contained
   sentence on how it was resolved -- all three in the same edit. The schema script rejects a row where `Status` is
   `Resolved` but `Date`/`Decision` are still `--`, or vice versa.
3. Never delete the row, never touch any column but `Status`/`Date`/`Decision`. If the original `Description` is now
   misleading given how it was actually resolved, add a short clarifying clause rather than rewriting it wholesale --
   the row is a history, not just a current-state snapshot.
4. Re-run `scripts/validate-risk-register-schema.sh` and confirm it passes now that all three cells moved together.

## Definition of Done

Before treating an add-or-resolve edit as finished, confirm all of the following, not just that the edit was typed:

- [ ] `scripts/validate-risk-register-schema.sh` passes with no output (content is well-formed).
- [ ] `scripts/validate-risk-register-append-only.sh` passes with no output (no previously-committed `#` went missing).
- [ ] For a new row: the `Description` and `Risk if unaddressed` stand alone without needing outside context.
- [ ] For a resolved row: `Status`, `Date`, and `Decision` all changed together, and nothing else did.

## Mindset

- **The Rule 2 discipline (only `Status`/`Date`/`Decision` change after commit) is convention, not a database
  constraint.** Nothing technically stops editing a row's other columns beyond that point; the point is that
  everyone treats it as if something did. An unenforced rule that's actually followed is worth more here than an
  enforced one that invites working around it -- the goal is a trustworthy history, not a tamper-proof one.
- A row with no clear "Risk if unaddressed" is a row nobody will ever prioritize -- don't file one that just restates
  the Description.
- File the row **when the shortcut happens**, not when you remember to write it up later. A register that only gets
  backfilled occasionally is worse than no register: it creates false confidence that everything deferred is tracked.
- This is a PoC-phase project: proportionate is fine. Not every minor TODO comment needs a row -- reserve it for
  things that could genuinely be forgotten and cause a real problem if they are. When in doubt, consider whether a
  teammate with no memory of this session would regret not knowing about it -- if yes, file it; if it's genuinely
  trivial, it's fine to let it go.

## Anti-Patterns

**NEVER** delete a row or renumber an existing `#`.
**WHY:** The `#` is the permanent identity other docs and conversations reference; deleting it destroys the history
the register exists to preserve. **Consequence:** anything that pointed at that `#` now references a gap, and nobody
can tell whether the risk was resolved or simply erased.
**BAD:** Removing row 5 because the risk "isn't relevant anymore" instead of marking it Resolved.
**GOOD:** `Status: Resolved`, `Date: 2026-08-01`, `Decision: No longer applicable because X`.

**NEVER** edit a row's `Item`, `Type`, `Description`, `Risk if unaddressed`, or `Added` after it's committed.
**WHY:** Nothing technically blocks this, which is exactly why it matters to hold the line by hand -- rewriting a
row's past turns the register into a current-state snapshot instead of a history. **Consequence:** a reader comparing
today's register against an old MR discussion finds the row described differently than what was true at the time.
**BAD:** Editing row 3's `Description` to reflect what you now know, erasing what was true when it was filed.
**GOOD:** Leave the original `Description` as-is; add a short clarifying clause if truly needed, and put the update
in `Decision` where it belongs.

**NEVER** file a vague row ("investigate later", "might be an issue").
**WHY:** An unactionable row is noise. **Consequence:** once a few vague rows accumulate, nobody trusts any row
enough to act on it without re-investigating from scratch, defeating the point of keeping a register at all.
**BAD:** `Description: Route fingerprinting stuff might need cleanup at some point.`
**GOOD:** A concrete description plus a concrete, one-sentence risk if it stays unaddressed.

**NEVER** cite a `.context/` file as where a row's extra detail lives.
**WHY:** `.context/` is gitignored local scratch, not a committed, durable record -- a reference to it may point at
something no other reader, machine, or fresh clone can ever open, the moment the row is committed.
**Consequence:** the register's own self-containment requirement is broken by the row that's supposed to enforce it.
**BAD:** `Description: ... see .context/findings/2026-07-31-foo.md for the exact detail.`
**GOOD:** Inline the essential fact in the row itself, or reference a committed doc (ADR, MR, another `docs/*.md`
page) if it doesn't fit.

**NEVER** put confidential or personal data in a register row.
**WHY:** This file is committed to the shared, GitHub-visible repo, unlike gitignored scratch notes.
**Consequence:** confidential detail leaks to every teammate and CI job with repo read access, with no way to
retroactively scrub it from git history.
**GOOD:** Reference a scratch-note file or an internal ticket for anything sensitive, and keep the row itself generic.

**NEVER** mention a shortcut, deferred item, or open decision in a reply without filing the row in the same turn.
**WHY:** No pre-commit hook can catch this -- the validation scripts only check a row that already exists, not
whether one *should* exist because something register-worthy got described in prose and then dropped.
**Consequence:** the register silently under-represents what's actually deferred, creating false confidence that
everything open is tracked when it isn't.
**BAD:** "One side note, not part of this task but worth flagging: ..." -- followed by moving on to something else.
**GOOD:** Stop, add the row right then (even mid-task), then proceed -- or say explicitly it isn't actionable yet
and confirm the row was filed rather than merely described.

## Scripts

Two independent checks; neither subsumes the other -- run both before committing (both are also wired into the
pre-commit hook):

```bash
scripts/validate-risk-register-append-only.sh   # History: did a previously-committed row id disappear vs. HEAD?
scripts/validate-risk-register-schema.sh        # Content: does every row match risk-register-row.schema.json?
```

```text
# Expected output on success (both scripts): nothing printed, exit code 0.
# Expected output on failure: a one-line reason on stderr, exit code 1 -- fix and re-run before committing.
```

`validate-risk-register-append-only.sh` is git-diff-based and would happily pass a row with a typo'd `Type`, since
nothing was deleted. `validate-risk-register-schema.sh` is point-in-time and has no git awareness, so it would
happily pass a file where an entire row vanished, as long as what remains is individually well-formed. Together they
cover "nothing was lost, and what's there is valid"; either alone does not.

## Troubleshooting

```text
Problem                                             | Solution
Pre-commit blocks: row id missing vs. HEAD          | validate-risk-register-append-only.sh caught a deletion --
                                                     | restore the row (mark Resolved instead) and re-stage.
Pre-commit blocks: type/added/status doesn't match  | validate-risk-register-schema.sh caught a malformed field --
                                                     | fix it to match risk-register-row.schema.json's constraints.
Pre-commit blocks: schema required fields mismatch  | The script's column order is out of sync with a schema edit --
                                                     | update validate-risk-register-schema.sh's expected_fields.
Not sure what the next # should be                  | Read the last row of the table; next # = its # + 1.
Unsure whether something belongs here or in an ADR  | Is it a decision that's actually been made and is final? -> ADR.
                                                     | Is it open, deferred, or a known accepted risk? -> register row.
```

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| How this register differs from `tech-debt` and from session-local scratch notes | [Relationship to Sibling Conventions](references/relationship-to-sibling-conventions.md) | Deciding which of the three a given item belongs in |
| A real filing mistake and the check that would have caught it | [Case Study: Verify "Unresolved" Before Filing](references/unresolved-verification-case-study.md) | Before filing a new "Open Decision" row that feels surprising or unfamiliar |
