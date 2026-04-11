# Scenario 02: Flaky Test That Fails Intermittently

## User Prompt

"One of our Vitest tests fails about 1 in 5 runs but I can't reproduce it reliably. It's `src/services/queue.test.ts` — the test `processes items in order` sometimes times out."

## Expected Behavior

1. Agent performs a WebSearch: `Vitest flaky test intermittent timeout "processes in order"` and related queries.
2. Agent scans results for common flaky test patterns (race conditions, shared state, async timing issues).
3. Agent invokes `AskUserQuestion` to qualify: language/framework/runtime, environment (CI vs local), what changed recently.
4. Agent checks for empty answers after `AskUserQuestion` (AskUserQuestion guard).
5. Agent applies Diagnose protocol: pattern matching for timing/async issues, isolation via Swap One Variable (run test in isolation vs full suite).
6. Agent applies Minimal Repro isolation: suggests running the test alone, with different timeouts, with forced concurrency.
7. Agent applies 5 Whys or Fishbone to drill to root cause (e.g., shared mutable state, missing `await`, non-deterministic data).
8. Agent enters OODA loop only if diagnosis is inconclusive after isolation.
9. Agent writes thinking artifact after resolution and offers Learn phase.

## Success Criteria

- **Search-first**: Agent searches for flaky test patterns before qualifying.
- **Intermittent pattern recognized**: Agent identifies timing/async/shared-state as the primary suspect category.
- **Isolation technique used**: Agent applies Swap One Variable (isolated run vs suite) or Minimal Repro.
- **Diagnose protocol**: Agent references `references/protocols/diagnose.md` for isolation steps.
- **OODA only if inconclusive**: Agent does not enter OODA before exhausting diagnosis phase.
- **Thinking artifact**: Agent writes artifact after root cause confirmed.
- **Learn phase offered**: Agent asks user whether to save the learning.

## Failure Conditions

- Agent jumps to OODA without attempting diagnosis phase first.
- Agent suggests only "increase the timeout" without investigating root cause.
- Agent does not check for empty `AskUserQuestion` responses.
- Agent skips isolation entirely and goes straight to root cause hypothesis.
- Agent fails to offer the Learn phase.
- Thinking artifact is skipped without confirming no store is configured.
