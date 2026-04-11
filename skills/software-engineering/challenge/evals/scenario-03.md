# Scenario 03: Wrong Problem Framing Detected — Framing Dispatch

## User Prompt

"/challenge framing We've been asked to speed up our database queries, and the AI recommended adding 12 new indexes. Something feels off."

## Expected Behavior

1. Agent parses `framing` as the subcommand from the first word of arguments.
2. Agent reads `references/protocols/framing.md` before executing.
3. Agent applies both framing patterns: Socratic and Steelman.
4. Socratic stage 1 (Definition): agent asks what "speed up queries" and "slow database" actually mean in this context.
5. Socratic stages 2-6: agent works through Elenchus, Dialectic, Maieutics (what is the real goal?), Generalization, and Counterfactual.
6. Steelman: agent constructs the strongest argument that adding indexes is NOT the right solution (e.g., write-heavy workload, missing query analysis, architecture-level problem).
7. Agent outputs Challenge Report with Error type: framing / wrong problem.
8. Verdict section indicates whether the framing holds or the wrong problem is being solved.

## Success Criteria

- **Framing dispatch**: Agent dispatches to framing protocol without presenting options.
- **Protocol file read**: Agent reads `references/protocols/framing.md` before executing.
- **Both patterns applied**: Socratic (all 6 stages) and Steelman both appear in output.
- **Real goal surfaced**: Maieutics stage produces a stripped-down actual objective distinct from "add indexes".
- **Steelman quality**: Counter-argument is substantive, not a strawman.
- **Report structure**: Framing verdict field present with one of: Framing holds / Framing needs revision / Wrong problem entirely.

## Failure Conditions

- Agent dispatches to anchor or verify instead of framing.
- Agent applies only Socratic or only Steelman, not both.
- Agent skips any of the 6 Socratic stages.
- Steelman is a weak objection rather than the strongest possible counter.
- Agent agrees with the index recommendation without challenging the framing.
- Framing verdict is absent from the Challenge Report.
