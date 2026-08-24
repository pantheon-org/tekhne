# Scenario 03: A Decision Already Settled by an Existing ADR

## User Prompt

"Can you properly evaluate whether we should switch from Bedrock to calling the Anthropic API directly? I want a
full write-up."

## Input

`docs/adr/adr-002-bedrock-for-llm.md` exists and is `status: accepted`, recording the decision to use AWS Bedrock
rather than the direct Anthropic API, with rationale tied to the project's single-region data-residency
requirement (ADR-020).

## Expected Behavior

1. Recognize this falls under "When NOT to Use": a technology choice already settled by an existing ADR should be
   superseded, not re-litigated via a new finding.
2. Do not dispatch a research subagent or produce a new tech-evaluation YAML for this request.
3. Point the user at the existing ADR-002 and ADR-020, and explain that changing this decision requires
   superseding the ADR (a different skill/process), not a fresh evaluation finding.
4. If the user has new evidence that specifically invalidates ADR-002's stated rationale (e.g. the residency
   requirement changed), say so explicitly and note that even then the correct output is an ADR supersession, not
   a duplicate finding.

## Success Criteria

- Agent identifies the existing ADR before doing any research work.
- No tech-evaluation YAML or subagent dispatch produced for an already-settled decision.
- Response clearly redirects to the ADR-supersession path rather than re-evaluating from scratch.

## Failure Conditions

- Agent runs a full tech evaluation without checking whether an ADR already covers this decision.
- A new finding is created that duplicates or contradicts the existing ADR without addressing it.
- The agent produces a "comparison" write-up instead of recognizing the out-of-scope condition.
