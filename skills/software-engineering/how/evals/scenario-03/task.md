# Scenario 03: Critique an Architecture

## User Prompt

How does the job queue work, and are there any architectural problems with it?

## Expected Behavior

1. The agent runs the full explain flow first, producing an explanation that stands on its own before any critique begins.
2. After the explanation, the agent spawns several independent critics in a single message, each given a distinct `model`/`effort` combination so they read the architecture from genuinely different
   angles.
3. Critics do not see each other's findings before submitting their own.
4. Critics are scoped to architectural problems (abstraction fit, data model, boundary discipline, evolution readiness, complexity vs. value, consistency) — not line-level code review.
5. A tradeoff with a demonstrated benefit is not flagged as an issue merely because an alternative approach exists.
6. The lead categorizes the resulting findings into Act on / Consider / Noted / Dismissed rather than accepting every critic finding at face value.
7. The explanation is presented first, with the critique verdict below it, so a reader who only wants to understand the system isn't forced through the critique.

## Success Criteria

- The explanation section is complete and coherent on its own, appearing before any critique content.
- At least two critics are used, with distinct model/effort combinations named.
- No finding in the critique section is a line-level style/naming nitpick.
- Any tradeoff-related finding either demonstrates a concrete cost or is explicitly not flagged as an issue.
- The final output visibly sorts findings into the four categories.

## Failure Conditions

- The critique is interleaved with the explanation instead of following it as a separate section.
- Only one critic model/effort combination is used where the skill calls for several independent ones.
- A finding amounts to a line-level nit (e.g. variable naming, formatting) presented as an architectural problem.
- A deliberate tradeoff is flagged as a problem with no demonstrated cost, just because a different approach was possible.
- Findings are listed without any Act on/Consider/Noted/Dismissed categorization.
