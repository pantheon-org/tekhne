# Scenario 3: Picking Up and Closing Out a Handover

## User Prompt

"Pick up `.context/handovers/2026-09-10-ticket-321-example-worker-timeout.md` and finish the
outstanding work."

## Repo State

- The handover's frontmatter: `title: "TICKET-321 example-worker timeout fix - mid-fix"`,
  `type: handover`, `date: 2026-09-10`, `branch: fix/ticket-321-example-worker-timeout`,
  `status: active`.
- Its `## Outstanding` lists exactly one item: "Increase the retry timeout constant in
  `worker/config.py` from 5s to 15s and re-run the timeout integration test."
- Its `## Current State` says the branch was clean with commit `1a2b3c4` as of 2026-09-10.
- In the time since, one more commit (`5d6e7f8`) has landed on the branch from unrelated work, so the
  handover's `## Current State` is now stale.

## Expected Behavior

1. Read the whole handover file before touching any code, not only `## Next Steps`.
2. Verify `## Current State` against reality (`git status`, `git log -1`) before acting on it, and
   notice that the branch has moved on since the handover was written (new commit `5d6e7f8`).
3. Make the described change and re-run the timeout integration test.
4. Once the outstanding item is fully resolved, flip the handover's `status` from `active` to `done`
   in the same change that resolves the work (edit the existing handover file's frontmatter and
   `## Outstanding`/`## Completed` to reflect this; do not delete the file).
5. Re-run `scripts/validate-handover.sh` against the file to confirm the frontmatter edit didn't break
   the schema.

## Success Criteria

- The response reads the full handover before acting, and explicitly checks `git status` / `git log -1`
  against the handover's recorded `## Current State` rather than trusting it blindly.
- The response notes or accounts for the branch having moved on (the extra commit) since the handover
  was written.
- The single outstanding item is resolved and the handover's own `status` field is changed to `done` in
  the same change, not left as `active` with a separate note that it's finished.
- The handover file is not deleted.
- The validator is re-run against the updated file.

## Failure Conditions

- Acts on the outstanding item without checking whether Current State still matches reality.
- Resolves the work but leaves `status: active`, or leaves the status change to "later" / a separate
  follow-up turn.
- Deletes the handover file instead of updating its status.
- Skips re-running the validator after editing the frontmatter.
