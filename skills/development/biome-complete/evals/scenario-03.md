# Scenario 03: Suppress Biome Diagnostics Correctly

## User Prompt

A developer is trying to silence two Biome diagnostics quickly to unblock CI. They want to know the right way to suppress them without degrading code quality.

The two issues are:

1. **File:** `src/legacy/parser.ts`, **Line 42** — `noExplicitAny` violation (a `@ts-ignore`-heavy parsing function that will be rewritten next sprint, tracked in JIRA-1234)
2. **File:** `src/utils/hash.ts`, **Line 17** — `noConsole` violation (a deliberate `console.warn` that alerts operators about hash collisions; removing it would hide a real issue)

The developer's first instinct was to add this to `biome.json`:

```json
{
  "linter": {
    "rules": {
      "suspicious": { "noExplicitAny": "off" },
      "suspicious": { "noConsole": "off" }
    }
  }
}
```

Explain why this approach is wrong and provide the correct suppression for each case.

Produce a `SUPPRESSIONS.md` file that:
1. Explains why the blanket approach is wrong
2. Shows the correct inline suppression for each case with a reason comment
3. Notes that the `noExplicitAny` suppression should reference the follow-up ticket

## Expected Behavior

1. Explain that disabling rules globally in `biome.json` hides defects across all files, not just the intended location
2. Show `biome-ignore` comment syntax for at least one case (e.g., `// biome-ignore lint/suspicious/noExplicitAny: <reason>`)
3. Include the JIRA-1234 ticket reference in the `noExplicitAny` suppression reason
4. Include an operational reason (operator alert / hash collision warning) in the `noConsole` suppression
5. Use line-level comment suppressions, not `biome.json` rule changes, for both cases

## Success Criteria

- **Blanket approach explained as wrong**: `SUPPRESSIONS.md` explains that disabling rules globally in `biome.json` hides defects across all files, not just the intended location
- **Correct biome-ignore syntax shown**: `SUPPRESSIONS.md` shows the `biome-ignore` comment syntax (e.g., `// biome-ignore lint/suspicious/noExplicitAny: <reason>`) for at least one case
- **noExplicitAny suppression references ticket**: The suppression for `noExplicitAny` includes a reason that mentions the follow-up ticket (JIRA-1234 or similar reference)
- **noConsole suppression includes operational reason**: The suppression for `noConsole` includes a reason explaining the intentional nature (e.g., operator alert, hash collision warning)
- **Both suppressions are inline (not global)**: Both suppressions are shown as line-level comments, not as `biome.json` rule changes

## Failure Conditions

- Agent recommends disabling rules in `biome.json` (the blanket approach the developer initially proposed)
- Agent does not show the `biome-ignore` comment syntax
- Agent omits the JIRA-1234 ticket reference from the `noExplicitAny` suppression reason
- Agent omits an operational justification from the `noConsole` suppression
- Agent produces `biome.json` changes instead of inline per-line suppressions
