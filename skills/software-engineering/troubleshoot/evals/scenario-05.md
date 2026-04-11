# Scenario 05: Performance Regression After a Code Change

## User Prompt

"Our API response times went from ~50ms to ~800ms after yesterday's merge. The only change was refactoring the product listing endpoint to use a new `ProductRepository` class."

## Expected Behavior

1. Agent performs a WebSearch: `API response time regression after refactor Node.js repository pattern performance`.
2. Agent scans results for patterns (N+1 queries, missing eager loading, lost connection pooling, removed caching).
3. Agent invokes `AskUserQuestion` to qualify: stack, environment (local vs production), what changed exactly.
4. Agent checks for empty answers after `AskUserQuestion` (AskUserQuestion guard).
5. Agent applies Diagnose protocol: Swap One Variable — isolate whether the regression is in the `ProductRepository` class specifically (revert one thing at a time).
6. Agent suggests `git bisect` or a targeted diff of the `ProductRepository` implementation against the previous approach.
7. Agent applies Fishbone: Machine (DB connection pool?), Method (N+1 queries?), Material (missing index?), Milieu (ORM config changed?).
8. Agent enters OODA if inconclusive: asks user to run query profiling or add timing logs at specific points.
9. Agent writes thinking artifact with all required sections including Hypotheses tested.
10. Agent offers Learn phase.

## Success Criteria

- **Search-first**: Agent searches before asking questions.
- **Swap One Variable**: Agent applies the isolation technique to the `ProductRepository` change specifically.
- **git bisect suggested**: Agent mentions `git bisect` or targeted diff as an isolation approach.
- **Fishbone applied**: Agent brainstorms across at least 2-3 of the 6 M categories.
- **Diagnose protocol referenced**: Agent uses `references/protocols/diagnose.md` for isolation steps.
- **Thinking artifact**: Written with Hypotheses tested section showing what was ruled out.
- **OODA reserved**: Agent does not enter OODA before exhausting Diagnose phase.
- **Learn phase offered**: Agent asks user whether to save the learning.

## Failure Conditions

- Agent immediately blames N+1 queries without applying any isolation.
- Agent skips the search phase entirely.
- Agent does not check for empty `AskUserQuestion` responses.
- Agent enters OODA as the first investigative step.
- Thinking artifact is missing the Hypotheses tested section.
- Agent does not offer the Learn phase after resolution.
- Agent suggests only profiling tools without identifying the root cause mechanism.
