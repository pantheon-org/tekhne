# Suppress Biome Diagnostics Correctly

## Problem Description

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
