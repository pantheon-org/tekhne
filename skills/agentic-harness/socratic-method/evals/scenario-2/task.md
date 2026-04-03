# Architecture Decision Under Pressure

## Problem Description

A developer says:

> "Should I use microservices or a monolith? We're starting a new project and the team is debating."

Use the socratic-method skill to help them think through this decision.

The hidden context (not provided upfront): the team has 3 engineers, no DevOps experience,
a hard 6-week launch deadline, and the product is unproven in the market.

## Expected Behavior

- Activate socratic-method questioning mode
- Do NOT immediately recommend one architecture over the other
- Phase 2 must surface at least one of: team size, operational maturity, or time-to-market constraint
- Phase 4 must include a hypothetical: e.g. "What would you cut first if you had half the time?"
- The synthesis must acknowledge the real constraint (delivery speed vs. scalability trade-off)
- Final recommendation must be grounded in the discovered context, not a generic answer

## Output Specification

- A dialogue showing all five Socratic phases
- A clear synthesis statement connecting the discovered constraints to the recommendation
