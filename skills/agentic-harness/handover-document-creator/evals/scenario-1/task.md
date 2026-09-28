# Scenario 1: Writing a Handover at Session End

## User Prompt

"I'm about to run out of context. Write a handover for this session before we stop."

## Repo State

- Branch: `feat/ticket-456-example-service-retry`.
- This session migrated 3 of 5 config tables to the new retry schema. Commit `d4e5f6a` landed the
  change. The session ran `bun test config/` and got 18 pass, 0 fail.
- The remaining 2 tables are blocked on a naming decision the user has not made yet.
- A follow-up entry already exists at `.context/follow-ups/2026-09-20-example-service-naming-decision.md`
  (status: active) describing that exact naming decision in full.
- `.context/handovers/` does not yet contain a file for this topic.
- The session has one WIP change (an experimental retry-count constant) it wants to set aside before
  finishing up, not commit.

## Expected Behavior

1. Gather the facts before writing: `git status`, `git log -1`, `git stash list`, and check
   `.context/follow-ups/` for entries filed this session.
2. If something needs setting aside, use `git stash push -u -m "<unique-tag>"` (and capture the
   resulting SHA), never a bare `git stash` or `git stash pop` -- the stash stack is shared across
   worktrees.
3. Write `.context/handovers/YYYY-MM-DD-ticket-456-example-service-retry.md` (or a similarly specific
   slug) with the required frontmatter (`title`, `type: handover`, `date` matching the filename,
   `branch: feat/ticket-456-example-service-retry`, `status: active`).
4. In `## Completed`, record the 3-of-5 migration with its evidence: commit `d4e5f6a` and the test
   result `bun test config/: 18 pass, 0 fail`. Do not just say "made progress" or "ran the tests".
5. In `## Outstanding`, link to the existing follow-up
   (`.context/follow-ups/2026-09-20-example-service-naming-decision.md`) rather than re-describing the
   naming decision in full.
6. Include all other required sections (`# Handover:` title, `## Session Summary`, `## Current State`,
   `## Next Steps`, `## References`), and run `scripts/validate-handover.sh` against the new file.

## Success Criteria

- A single new file is created under `.context/handovers/` following `YYYY-MM-DD-<slug>.md`.
- Frontmatter carries all five required fields, with `date` matching the filename.
- `## Completed` cites the commit SHA and the test result as evidence, not a bare claim of progress.
- `## Outstanding` links the existing follow-up file path rather than duplicating its description.
- No bare `git stash` or `git stash pop` is used; if a stash is needed, it carries a unique `-m` tag.
- The validator script is run (or its command is shown) against the new file before calling the work
  done.

## Failure Conditions

- Writes "made progress on the migration" or similar with no commit SHA, file path, or test count.
- Re-explains the full naming decision inline in `## Outstanding` instead of linking the follow-up.
- Runs a bare `git stash` or `git stash pop`.
- Skips a required section, or omits the validator step entirely.
- Edits some other unrelated existing handover file instead of creating a new one.
