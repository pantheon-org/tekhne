# Categorization Decision Tree

Use these questions in order for every requirement.

1. Is it legally or contractually required?
2. Does the product fail to deliver core value without it?
3. Does absence block release approval or market entry?
4. Can it be deferred one release with acceptable risk?

## Mapping Rules

- If 1-3 are `yes`, classify as `Must`.
- If 4 is `yes` and value is high, classify as `Should`.
- If value is moderate or low and deferrable, classify as `Could`.
- If out of scope now, classify as `Won't` with revisit trigger.

## Must Challenge Questions

- "What breaks on release day if this is missing?"
- "Can a temporary workaround cover this release?"
- "Is the impact legal, security, revenue-critical, or reputation-critical?"

If answers are vague, downgrade from `Must` and document rationale.
