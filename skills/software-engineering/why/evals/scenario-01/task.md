# Scenario 01: Investigate a Design Decision with Mixed Evidence

## User Prompt

Why does the retry logic in the upload handler wait exactly 3 times before giving up?

## Expected Behavior

1. The agent establishes a code anchor (file paths, symbols, commit history via `tokensave_blame`/`tokensave_log`) before drawing conclusions.
2. Any claim placed in "What We Found" carries an explicit citation (commit hash, PR/MR number, ticket ID, comment, or doc reference).
3. Claims that aren't explicitly stated anywhere, but are supported by indirect evidence, go in "What We Can Reasonably Infer" with hedged language ("appears to", "likely"), not confident language.
4. The agent does not cite the code's own shape (e.g. "the constant is 3 so the author wanted 3") as evidence of intent.
5. A "What We Don't Know" section is present, naming specifically what was searched and came up empty, not a vague "we don't know why."

## Success Criteria

- The code anchor is built before conclusions are drawn.
- Every "What We Found" bullet has a specific citation.
- Inferred claims use hedged phrasing and make the inference chain explicit ("given A and B, C seems likely because D").
- No bullet treats the code's own value or structure as self-evidence of its motivation.
- "What We Don't Know" names specific unanswered questions or empty searches, not a generic disclaimer.

## Failure Conditions

- A claim appears in "What We Found" with no citation at all.
- An inferred claim is phrased with confident language ("because", "the reason is") rather than hedged language.
- The agent writes something like "it checks for null because it handles the null case" as if that explained intent.
- The "What We Don't Know" section is missing, or is a single generic sentence with no specifics.
