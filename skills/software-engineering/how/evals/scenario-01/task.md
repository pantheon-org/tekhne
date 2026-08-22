# Scenario 01: Explain a Single Function

## User Prompt

How does the retry backoff calculation work in this codebase?

## Expected Behavior

1. The agent recognizes this as a narrow, single-function question and takes the simple path: explore and explain in one pass, without spawning multiple parallel explorer agents.
2. The agent explores using `tokensave_context` (and, if needed, `tokensave_search`/`tokensave_callees`/`tokensave_callers`) rather than `Glob`, `Grep`, `Read`, or `list_directory`.
3. The explanation follows the Overview / Key Concepts / How It Works / Where Things Live / Gotchas structure, adapted to what's needed for a narrow question.
4. The explanation is written in prose that references specific files and functions, not a dump of the function's full source pasted as a code block.
5. The agent does not describe the function's behavior based on its name (e.g. assuming "exponential" backoff just because the function is named that) without confirming it against the source
   `tokensave_context` returned.

## Success Criteria

- No parallel explorer agents are spawned for this narrow question.
- Exploration is attributed to `tokensave_context` (or its companion tools), not raw file search.
- The explanation is prose-first with file/function references, not a large pasted code block.
- Any claim about the backoff behavior (e.g. the growth pattern, the retry count, the cap) is grounded in the actual returned source, not inferred from the function's name.

## Failure Conditions

- The agent spawns 2+ explorer subagents for what is clearly a single-function question.
- The agent's exploration description mentions Glob, Grep, Read, or listing directories as the way it found the code.
- The response is dominated by a large pasted code block instead of a prose explanation.
- The agent states what the function does based on its name without checking the source (e.g. "it's called `linearBackoff` so it must increase linearly" without verifying).
