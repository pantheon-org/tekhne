# Scenario 05: Evaluate Whether an Architectural Boundary Is Justified

## User Prompt

A developer is building a small REST API for a startup's internal tool. The tool manages employee time-off requests. It has one team of two developers and is expected to support ~50 internal users.

The developer is considering implementing full hexagonal architecture with:
- Separate `domain/`, `application/`, `infrastructure/`, and `interface/` directories
- Repository interfaces with PostgreSQL and in-memory implementations
- Input/output port interfaces for every use case
- A DI container (InversifyJS) wiring everything together

They asked: "Should I implement full hexagonal architecture for this project?"

Their current codebase has 3 routes, 2 database tables, and no tests.

Produce an `ADVICE.md` that:
1. Evaluates whether full hexagonal architecture is justified at this stage
2. Recommends what level of structure is appropriate right now
3. Identifies what trigger would justify introducing proper layer boundaries later

## Expected Behavior

1. Recommend against implementing full hexagonal architecture immediately for a 2-person, 3-route internal tool
2. Explain why the boundary cost outweighs the benefit at this scale (small team, simple domain, no complex testing requirements yet)
3. Suggest a simpler, pragmatic structure appropriate for the current stage (e.g. basic service layer, simple folder structure)
4. Name a concrete trigger that would justify adding proper layer boundaries (e.g. needing to swap database, adding a second client, team growth, complex domain logic)
5. Apply the principle of solving the current need and refactoring when the trigger appears — not designing for imagined future requirements

## Success Criteria

- **Full hexagonal architecture not recommended at this stage**: ADVICE.md recommends against implementing full hexagonal architecture immediately for a 2-person, 3-route internal tool
- **Cost-vs-benefit reasoning provided**: ADVICE.md explains why the boundary cost outweighs the benefit at this scale (small team, simple domain, no complex testing requirements yet)
- **Appropriate current structure recommended**: ADVICE.md suggests a simpler, pragmatic structure appropriate for the current stage (e.g. basic service layer, simple folder structure)
- **Trigger for future boundary introduction identified**: ADVICE.md names a concrete trigger that would justify adding proper layer boundaries (e.g. needing to swap database, adding a second client, team growth, complex domain logic)
- **YAGNI principle referenced or applied**: ADVICE.md applies the principle of solving the current need and refactoring when the trigger appears — not designing for imagined future requirements

## Failure Conditions

- ADVICE.md recommends implementing full hexagonal architecture for this early-stage project
- ADVICE.md provides no reasoning about cost vs. benefit at this scale
- ADVICE.md does not recommend any current structure, leaving the developer without actionable guidance
- ADVICE.md identifies no concrete trigger that would justify introducing proper boundaries later
- ADVICE.md does not reference or apply YAGNI or an equivalent principle against premature architecture
