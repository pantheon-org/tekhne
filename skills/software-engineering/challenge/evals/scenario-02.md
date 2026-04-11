# Scenario 02: Factual Claim Needs Verification — Verify Dispatch

## User Prompt

"/challenge verify The AI just told me that React's useEffect runs synchronously after every render, so there's no need to clean up subscriptions."

## Expected Behavior

1. Agent parses `verify` as the subcommand from the first word of arguments.
2. Agent reads `references/protocols/verify.md` before executing.
3. Agent applies all 3 verify patterns: Proof Demand, CoVe, Fact Check List.
4. Proof Demand: agent extracts the factual claims ("useEffect runs synchronously", "no need to clean up subscriptions") and classifies each as Sourced, Unsourced, or Contradicted.
5. CoVe: agent writes independent verification questions and answers them without referencing the original claim.
6. Fact Check List: agent decomposes into atomic assertions, rates confidence (High/Medium/Low/Unknown), and for Low/Unknown writes a concrete verification action.
7. Agent outputs a Challenge Report with Error type: factual.

## Success Criteria

- **Verify dispatch**: Agent dispatches to verify subcommand without presenting options to the user.
- **Protocol file read**: Agent reads `references/protocols/verify.md` before executing.
- **Proof Demand applied**: Claims are classified as Sourced/Unsourced/Contradicted.
- **CoVe applied**: Independent verification questions are generated and answered separately.
- **Fact Check List applied**: Atomic assertions listed with confidence ratings.
- **Verification actions**: At least one concrete verification action provided for uncertain claims.
- **Report structure**: Challenge Report follows format from `references/reference.md`.

## Failure Conditions

- Agent skips reading `references/protocols/verify.md`.
- Agent applies only one or two of the three verify patterns.
- Agent marks the claim as correct without independent verification.
- CoVe step answers questions by referencing the original claim rather than independently.
- Agent produces a generic "let me check that" response without structured output.
- Agent omits the Contradicted classification despite the claim being demonstrably false.
