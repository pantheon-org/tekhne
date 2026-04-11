# Scenario 03: npm Install Dependency Conflict

## User Prompt

"npm install is failing with: `npm error ERESOLVE unable to resolve dependency tree` — trying to install `react-query@5` but something conflicts with `@tanstack/react-table@8`"

## Expected Behavior

1. Agent performs a WebSearch using constructed query: `"ERESOLVE unable to resolve dependency tree" "react-query" "tanstack" npm conflict fix`.
2. Agent searches Stack Overflow and GitHub Issues for this specific conflict pattern.
3. If solution found (e.g., `--legacy-peer-deps`, specific version pinning), agent verifies it applies before suggesting.
4. Agent invokes `AskUserQuestion` to qualify: Node/npm version, environment (local vs CI), what changed recently.
5. Agent checks for empty answers after `AskUserQuestion` (AskUserQuestion guard).
6. Agent applies Diagnose protocol: Swap One Variable (install packages separately to identify which pair conflicts).
7. Agent reads `references/protocols/search-multi-source.md` to guide multi-source search if initial search inconclusive.
8. Agent provides a concrete resolution (e.g., version downgrade, peer dep override, `--force` caveat).
9. After resolution, agent writes thinking artifact and offers Learn phase.

## Success Criteria

- **Search-first**: Agent searches before asking qualifying questions.
- **Exact error in query**: Search query includes `"ERESOLVE"` and the specific package names.
- **Multi-source protocol**: Agent references `references/protocols/search-multi-source.md` if search iterates.
- **Isolation technique**: Agent applies Swap One Variable to identify the conflicting pair.
- **Concrete resolution**: Agent suggests a specific npm command or version change, not just "try --force".
- **AskUserQuestion guard**: Agent checks for empty answers.
- **Thinking artifact**: Written after resolution.

## Failure Conditions

- Agent immediately suggests `--legacy-peer-deps` or `--force` without diagnosing which packages conflict.
- Agent skips WebSearch and goes directly to qualifying questions.
- Agent does not check for empty `AskUserQuestion` responses.
- Agent does not apply any isolation technique to identify the conflicting package pair.
- Agent provides no concrete resolution command.
- Agent skips thinking artifact without confirming no store is configured.
