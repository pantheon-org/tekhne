# Scenario 02: Explain a Cross-Cutting Subsystem

## User Prompt

How does the authentication middleware work end to end, from an incoming request to the session lookup?

## Expected Behavior

1. The agent recognizes this as a subsystem spanning multiple files/services and decomposes it into 2-4 distinct, non-overlapping exploration angles (e.g. request-path enforcement, session/token
   storage, configuration) rather than one broad unscoped pass.
2. Each exploration angle is handed to a separate `Explore` agent in a single concurrent `Agent` message.
3. Every explorer uses `tokensave_context` (and its companion tools) as its only exploration tool — no explorer is told to use `Glob`, `Grep`, or `Read`.
4. A synthesizer reconciles the explorers' findings into one coherent explanation, resolving any contradictions between explorers rather than concatenating their raw output.
5. The final explanation follows Overview / Key Concepts / How It Works / Where Things Live / Gotchas, and includes a diagram only if the multi-component flow genuinely needs one to be clear.

## Success Criteria

- The question is split into at least 2 distinct exploration angles with no significant overlap in what each is asked to cover.
- All explorers are described as using `tokensave_context`-based tools, not raw file search.
- The synthesizer's output reads as one unified explanation, not a concatenation of per-explorer findings.
- The output structure matches the documented sections, in the documented order.

## Failure Conditions

- The agent treats this broad, multi-file question as a single narrow pass with no decomposition.
- Two or more exploration angles are near-duplicates of each other (e.g. both angles are "trace the request path").
- Any explorer's instructions mention Glob, Grep, Read, or listing directories.
- The final output is visibly just the explorers' findings pasted one after another, with unresolved contradictions left standing.
