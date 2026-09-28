# Scenario 2: A Second Session Hands Off Again on the Same Topic

## User Prompt

"I picked up the handover from last week and made more progress, but I need to hand off again before
I lose context. Write the next handover."

## Repo State

- `.context/handovers/2026-09-01-ticket-789-example-gateway-migration.md` exists, with frontmatter
  `status: active`, `branch: feat/ticket-789-example-gateway-migration`. Its `## Session Summary`,
  `## Completed`, and `## Outstanding` describe the first session's work only.
- This (second) session picked up that handover, resolved 2 of the 4 items in its `## Outstanding`
  section (commit `9c8b7a6`, `bun test gateway/`: 9 pass, 0 fail), and has 2 new outstanding items of
  its own plus 1 of the original 4 still unresolved.
- No `.context/follow-ups/` entries exist yet for the remaining items.

## Expected Behavior

1. Do not edit `.context/handovers/2026-09-01-ticket-789-example-gateway-migration.md` in place to
   describe this second session's work.
2. Write a **new** dated file, e.g.
   `.context/handovers/2026-09-08-ticket-789-example-gateway-migration.md`, with its own full set of
   required sections describing this session's own Session Summary, Completed (with the commit and test
   evidence), and current Outstanding (the 1 unresolved original item plus the 2 new ones).
3. Set `supersedes: .context/handovers/2026-09-01-ticket-789-example-gateway-migration.md` in the new
   file's frontmatter.
4. The new file's frontmatter otherwise follows the same required-field contract (`title`, `type:
   handover`, `date` matching filename, `branch`, `status: active`).
5. Run the validator against the new file.

## Success Criteria

- A new, separate handover file is created rather than the original 2026-09-01 file being rewritten
  with second-session content.
- The new file's frontmatter includes `supersedes` pointing at the original file's repo-relative path.
- The new file's `## Completed` cites commit `9c8b7a6` and the test result as evidence for the 2
  resolved items.
- The new file's `## Outstanding` reflects the current remaining state (1 leftover original item plus 2
  new items), not the original session's full outstanding list.
- The original 2026-09-01 file's own Session Summary / Completed / Outstanding text is not rewritten to
  narrate the second session's work.

## Failure Conditions

- Edits `2026-09-01-ticket-789-example-gateway-migration.md`'s body sections to add the second session's
  narrative.
- Creates a new file but omits the `supersedes` field.
- Overwrites the original file's `## Outstanding` with the second session's own outstanding items instead
  of writing a new file.
- Copies the entire original `## Outstanding` list into the new file unchanged, rather than reflecting
  what has actually been resolved.
