# Scenario 02: Architecture Decision Under Pressure

## User Prompt

A developer says:

> "Should I use microservices or a monolith? We're starting a new project and the team is debating."

Use the socratic-method skill to help them think through this decision.

The hidden context (not provided upfront): the team has 3 engineers, no DevOps experience,
a hard 6-week launch deadline, and the product is unproven in the market.

## Expected Behavior

- Activate socratic-method questioning mode without recommending monolith or microservices immediately
- Phase 2 must surface at least one real constraint: team size, operational maturity, or time-to-market
- Phase 4 must include a hypothetical to stress-test the reasoning (e.g. "What would you cut first if you had half the time?")
- Questions must be genuinely open — not leading or rhetorical
- The synthesis must ground the recommendation in the discovered constraints, not generic architecture advice

## Output Specification

- A dialogue showing all five Socratic phases
- A clear synthesis statement connecting the discovered constraints to the recommendation

## Success Criteria

- **Activates questioning mode without immediate recommendation**: Activates questioning mode without recommending monolith or microservices immediately
- **Surfaces real constraints**: Phase 2 surfaces at least one real constraint: team size, operational maturity, or time-to-market
- **Phase 4 hypothetical**: Phase 4 applies a hypothetical to stress-test the reasoning (e.g. half the time, half the team)
- **Genuinely open questions**: Questions are genuinely open — not leading or rhetorical
- **Constraint-grounded synthesis**: Synthesis grounds the recommendation in the discovered constraints, not generic architecture advice

## Failure Conditions

- Recommends monolith or microservices immediately without entering a questioning phase
- Fails to surface any real constraints (team size, operational maturity, deadline) during the dialogue
- Skips or omits the Phase 4 hypothetical stress-test
- Asks leading questions that push toward a predetermined answer
- Delivers a generic synthesis ("microservices scale better") without referencing the specific constraints discovered
