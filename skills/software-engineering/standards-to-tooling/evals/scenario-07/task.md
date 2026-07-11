# Scenario 07: Verify Config With Lint Commands

## Setup

The agent has just added an ESLint rule for `no-console` to the project config.

## User Prompt

"Great, the rule is added. Is it working correctly?"

## Setup

The project has existing code with `console.log` statements.

## Expected Behavior

1. Run the appropriate lint command (e.g., `npx eslint 'src/**/*.{ts,js,vue}'`).
2. Report the results: how many violations were found, where they are.
3. If violations exist, note that the rule is working as expected.
4. Offer to auto-fix if `--fix` is available.
5. Do NOT modify the rule severity based on false positives — let the user decide.

## Success Criteria

- A lint command is run to verify the rule.
- Results are reported with file paths and violation counts.
- The user is informed whether the rule is working as expected.

## Failure Conditions

- The agent says "it should work" without running a verification command.
- The agent changes the rule from error to warn based on violation count without asking.
- The agent auto-fixes without offering or asking.
