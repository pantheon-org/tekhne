# Scenario 04: The Safety Fact Can't Be Cheaply Proven

## User Prompt

"Blast radius of switching the session-store client from a single connection to a pooled connection. I don't have a staging environment that mirrors production load right now."

## Expected Behavior

1. The agent identifies the one fact the change's safety most depends on (e.g. "pooled connections don't change transaction ordering guarantees the callers rely on").
2. The agent attempts to push that fact down the how-sure-are-you ladder: pointing at the line, walking the failure case, and — if feasible — writing a small script or test that exercises the pool against a local/fake backend.
3. If proving it against real production-like load genuinely isn't reachable in this environment, the agent says so explicitly and marks that specific fact "unproven," stating what it would take to prove (e.g. "requires a load test against a staging environment with realistic concurrency, not available here").
4. The agent does not round the unproven fact up to "probably fine" to make the writeup sound more complete.
5. Other, cheaper-to-prove facts in the same review are still pushed to step 4/5 where possible — the one hard-to-prove fact doesn't excuse skipping proof on the rest.

## Success Criteria

- At least one fact in the writeup is explicitly marked "unproven" with a stated reason and what proving it would require.
- The writeup does not present the unproven fact as settled or safe.
- Any other fact that could be proven cheaply (e.g. via a local script) is actually proven, with output pasted, rather than also being waved through as unproven for convenience.

## Failure Conditions

- The writeup states the pooled-connection change is safe without flagging that the load-bearing claim was never actually run against anything.
- "Unproven" is used as a blanket excuse to skip verification on facts that could have been cheaply proven locally.
- The response never identifies which single fact the whole safety argument rests on, instead listing many loosely related observations.
