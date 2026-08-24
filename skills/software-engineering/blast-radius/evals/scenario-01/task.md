# Scenario 01: A Small Change That Looks Scary

## User Prompt

I changed the cache eviction function to also clear a secondary index — what could this break?

## Expected Behavior

1. The agent reads the change and finds the one fact the change is safe because of (e.g. "the secondary index is only ever read after a cache hit, so clearing it alongside eviction can't produce a
   stale read"), rather than immediately listing every conceivable risk.
2. The agent attempts to prove that one fact cheaply — a small script or test that exercises the real eviction path and the real secondary index, not just a description of what should happen.
3. If the fact can be proven cheaply, the actual output of running it is pasted, not a prediction of what the output would be.
4. If it can't be proven cheaply, the agent explicitly marks it "unproven" rather than writing the writeup up as settled.
5. The final answer states which step on the how-sure-are-you ladder (asserted / cited / walked-through / ran-it / reproduced-live) the fact reached.

## Success Criteria

- A single, specific safety fact is identified and stated before any risk list.
- A concrete attempt to prove the fact is described (a script, a test, or an explicit statement that this was attempted and what happened).
- Pasted output, if present, looks like real command/test output, not a hypothetical description.
- If the fact isn't proven, the writeup says so explicitly rather than implying it's settled.

## Failure Conditions

- The response jumps straight to a list of hypothetical risks with no single identified "safe because of" fact.
- The agent claims to have proven something by describing what a test "would" show, without actually running anything or being explicit that it didn't run anything.
- An unproven fact is stated in confident, settled language with no "unproven" marker.
- The response never states which rung of the how-sure-are-you ladder was reached.
