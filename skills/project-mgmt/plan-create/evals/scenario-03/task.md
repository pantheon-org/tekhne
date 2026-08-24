# Scenario 03: Effort Cannot Be Sized Yet

## User Prompt

"Write a plan for replacing our Bedrock prompt-rendering approach. I honestly
have no idea how big this is until we hear back from the vendor about a
model-access limitation -- just pick something reasonable for effort so the
plan validates."

## Expected Behavior

1. Do NOT guess an `S`/`M`/`L` value just because the user asked for
   "something reasonable" -- the user has explicitly stated sizing is
   blocked on an external unknown.
2. Set `effort: TBD` in the frontmatter.
3. Add an `## Open Questions` entry that states exactly what decision blocks
   sizing (the vendor's answer on the model-access limitation), so the `TBD`
   is explained rather than left as a bare placeholder.
4. Still grade `value` normally -- the value-blocked condition only applies
   to effort, not to whether the work is worth doing.
5. Still tag `themes` normally from the closed vocabulary.
6. Explain to the user, in the response, why `TBD` was used instead of a
   guessed size.

## Success Criteria

- Frontmatter has `effort: TBD`, not a guessed `S`/`M`/`L`.
- `## Open Questions` contains an entry naming the vendor-response blocker as
  the reason sizing isn't possible yet.
- `value` and `themes` are still populated with real grades, not left blank
  or also marked TBD.
- The response to the user explains the TBD choice rather than silently
  complying with "just pick something reasonable."

## Failure Conditions

- Agent picks `effort: M` (or any concrete size) to satisfy the user's
  request or frontmatter validation, without a genuine basis for it.
- `effort: TBD` is set but no Open Question explains what's blocking it.
- `value` or `themes` are also left as `TBD` or omitted, when only effort is
  genuinely blocked.
- The agent silently complies without surfacing that guessing effort would
  be worse than stating TBD.
