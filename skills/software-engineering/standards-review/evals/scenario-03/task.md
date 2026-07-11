# Scenario 03: User Corrects the Recap

## Setup

The user asked to review their `.agents/RULES.md`. The file has 2 standards:

```markdown
### Rule: Always use structured logging

ALWAYS use structured logging for production services.

### Rule: No console.log in production code

NEVER commit console.log statements. Use the logger instead.
```

The agent has already:
1. Read the file and parsed 2 standards.
2. Presented standard #1 → user answered "Accept".
3. Presented standard #2 → user answered "Revise — needs examples of what to use instead".
4. Now the agent must present the recap.

## User Prompt (after recap is presented)

"Wait, I said Revise for the second one, not Accept. Please fix that."

## Expected Behavior

1. Accept the correction without argument.
2. Update the recap: standard #2 is now correctly shown as "Revise" with the user's note.
3. Re-present the corrected recap in a new message.
4. Ask for confirmation again.
5. After confirmation, write the report reflecting the corrected data.

## Success Criteria

- The agent acknowledges the correction without debating it.
- The corrected recap shows standard #2 as "Revise", not "Accept".
- The corrected recap is shown again before writing.
- The final report matches the corrected recap.
- The agent does not loop back and re-ask standard #2's question.

## Failure Conditions

- The agent argues that the user originally said something different.
- The agent silently updates without re-presenting the recap for confirmation.
- The agent asks to re-review standard #2 instead of just updating the status.
- The final report still shows the wrong action for standard #2.
