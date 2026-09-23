# Scenario: Write an ADR for a Validation Library Decision

A team switched from hand-written runtime validation checks to Zod for validating API responses, and the decision was made in a meeting with no lasting record. A new team member later asks "why don't we just use manual `if` checks instead of pulling in a dependency?" — a question the original decision already answered, but nobody can find where.

Write an ADR (`0007-runtime-validation-with-zod.md`) that:

1. States the Context: what problem existed before Zod (e.g. hand-written checks were inconsistent, error messages varied, types drifted from checks).
2. States the Decision: Zod is adopted for runtime validation at system boundaries, with `z.infer` used to derive types from schemas.
3. States Alternatives Considered: at least two real alternatives (e.g. hand-written type predicates, a different library like Yup or io-ts) and a concrete reason each was not chosen.
4. States Consequences: at least one cost this decision accepts (e.g. an added dependency, a learning curve) alongside the benefits, not just the benefits alone.
5. Follows a standard ADR structure (Title, Status, Context, Decision, Alternatives Considered, Consequences) so it's findable and skimmable by a future reader who wasn't in the room.

## Output Specification

Produce a single file `0007-runtime-validation-with-zod.md` following the structure above.
