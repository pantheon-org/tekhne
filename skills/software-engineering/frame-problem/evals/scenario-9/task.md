# Scenario 9: Fully Specified Task Skips Framing

## User Prompt

"Rename the variable `usr` to `user` in src/utils.ts."

## Expected Behavior

1. Agent recognises the task is fully specified, unambiguous and a pebble (single file, obvious implementation).
2. Agent does not run auto-classification, triangulation or the Adjacent Domain Challenge, because the skill is not for known tasks.
3. Agent does not call `AskUserQuestion` and does not produce a handoff template.
4. Agent simply does the task, or tells the user no framing is needed and proceeds.

## Failure Conditions

- Agent runs the three triangulation tests on a one-line rename.
- Agent asks the user to confirm a Cynefin classification.
- Agent produces a handoff template or routes to `openspec-develop`.
- Agent refuses to proceed until framing is complete.
