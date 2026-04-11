# Scenario 01: TypeScript Compilation Error

## User Prompt

"I'm getting this TS error and can't figure it out: `Type 'string | undefined' is not assignable to type 'string'` on line 42 of src/api/user.ts"

## Expected Behavior

1. Agent performs a WebSearch using a constructed query: `"Type 'string | undefined' is not assignable to type 'string'" TypeScript fix`.
2. Agent scans results from Stack Overflow, GitHub Issues, and TypeScript docs.
3. If a solution is found online, agent verifies it applies to the user's context and moves to Learn phase.
4. If no match, agent invokes `AskUserQuestion` to qualify: stack/runtime, environment, what changed recently.
5. Agent checks for empty/blank answers after `AskUserQuestion` (AskUserQuestion guard).
6. Agent applies Diagnose protocol from `references/protocols/diagnose.md`: mental model match, isolation, root cause drill.
7. Agent suggests a specific fix (e.g., non-null assertion, optional chaining, type narrowing guard).
8. After resolution, agent writes a thinking artifact if a persistent store is configured.
9. Agent offers to save the learning (Global / Project / Skip).

## Success Criteria

- **Search-first**: Agent performs WebSearch before asking qualifying questions.
- **Query construction**: Search query includes the exact error message, "TypeScript", and version if known.
- **AskUserQuestion guard**: Agent checks for empty answers and falls back to numbered list.
- **Diagnose protocol referenced**: Agent reads `references/protocols/diagnose.md` if pattern match not found online.
- **Concrete fix suggested**: Agent provides a specific TypeScript code change, not generic advice.
- **Thinking artifact**: Agent writes the artifact after resolution (or skips silently if no store configured).
- **Learn phase**: Agent asks user whether to save the learning.

## Failure Conditions

- Agent asks qualifying questions before attempting WebSearch.
- Agent provides a generic "add `!` non-null assertion" without verifying it applies.
- Agent skips the thinking artifact step without confirming no store is configured.
- Agent does not offer the Learn phase after resolution.
- Agent fails to check for empty `AskUserQuestion` responses.
- Agent performs only one search iteration when results are inconclusive.
