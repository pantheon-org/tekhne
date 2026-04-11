# Scenario 05: "I Don't Know Where to Start" — Triangulation Before Routing

## User Prompt

"I don't know where to start. We need to improve the developer experience on our platform, reduce CI build times, fix some tech debt in the auth service, and also maybe explore whether we should move to a different database. There's a lot."

## Expected Behavior

1. Agent detects that $ARGUMENTS is broad, multi-domain, and that auto-classification confidence is below 80%.
2. Agent applies Question Refinement: generates 2-3 clarifying sub-questions to sharpen the problem statement before running triangulation.
3. Agent presents sub-questions inline and asks them alongside T1/T2/T3/Q-Scale via a single AskUserQuestion call.
4. User answers reveal a composite problem: CI build times are Complicated-Degraded, auth tech debt is Complicated-Evolving, database migration is Complex (no clear hypothesis), DX improvement is Complex (enabling, emergent).
5. Agent runs Step 1.5 Decompose: breaks the problem into 4 sub-problems and maps each to a domain.
6. Agent presents domain map with suggested sequence (highest-risk/Complex first).
7. Agent confirms decomposition with AskUserQuestion before routing to the first sub-problem.
8. Agent routes Complex sub-problems before Complicated ones per sequencing guidance.

## Success Criteria

- **Question Refinement applied**: Agent generates clarifying sub-questions before or alongside triangulation.
- **Single AskUserQuestion call**: All questions (clarifying + T1/T2/T3/Q-Scale) batched into one call.
- **Decompose step triggered**: Step 1.5 is invoked because triangulation doesn't converge on a single domain.
- **Domain map produced**: All 4 sub-problems are mapped with domain, verb, and skill.
- **Sequencing follows risk guidance**: Complex sub-problems appear first in the suggested sequence.
- **Confirmation before routing**: Agent uses AskUserQuestion to confirm decomposition before proceeding.

## Failure Conditions

- Agent picks one sub-problem arbitrarily and ignores the rest.
- Agent classifies the entire request as a single domain (e.g., all Complicated) without decomposing.
- Agent asks multiple separate AskUserQuestion calls instead of batching into one.
- Agent skips Question Refinement and runs triangulation on the raw vague prompt.
- Agent proceeds to routing without confirming decomposition with the user.
- Agent routes the easiest (Complicated) sub-problems first, violating the risk-first sequencing guidance.
