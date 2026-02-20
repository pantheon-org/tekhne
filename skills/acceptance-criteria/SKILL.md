---
title: Writing Effective Acceptance Criteria
description: >
  Write clear, testable acceptance criteria using Given/When/Then or rule-oriented formats. Use when: (1) defining
  "done" for user stories, (2) creating testable requirements for QA, (3) aligning stakeholder expectations, (4)
  preparing for sprint planning, (5) documenting edge cases. Keywords: acceptance criteria, AC, Given/When/Then, BDD,
  Gherkin, user story, conditions of satisfaction, MoSCoW, scenario, rule-oriented
type: skill
category: Development
version: 2.0.0
tags:
  - requirements
  - testing
  - agile
  - user-stories
  - quality-assurance
last_updated: 2026-02-20
---

## Overview

Use this skill to define feature completion criteria that are clear, testable, and aligned with product intent.

## When to Use This Skill

Use this skill when you need to:

- Define what "done" means for a user story
- Create testable requirements for QA and automation
- Align developers, product, and stakeholders on expected behavior
- Make scope and edge cases explicit before implementation
- Support sprint planning with verifiable outcomes

## Inputs Required

- User story in the format: `As a [role], I want [action], so that [benefit]`
- Feature constraints (policy, legal, performance, platform)
- Primary user flows and failure paths

## Format Selection Decision Tree

1. User interaction with clear sequence?
   - Yes: use `Given/When/Then`
   - No: continue
2. Intended for automated scenario tests?
   - Yes: use `Given/When/Then`
   - No: continue
3. Mostly independent UI or validation rules?
   - Yes: use rule-oriented checklist
   - No: continue
4. Mixed behavior and constraints?
   - Use both formats in one story

## Workflow

1. Clarify the user outcome
   - Capture the intended business/user result before writing AC
2. Select format
   - Use the decision tree above
3. Draft core criteria
   - Start with must-pass behavior
4. Add negative and boundary scenarios
   - Include invalid input, permissions, and failure states
5. Validate each criterion
   - Ensure criteria are independent, measurable, and testable
6. Prioritize with MoSCoW
   - Convert "Must Have" items into required acceptance criteria

## Four Essential Qualities

Each criterion must be:

| Quality | Validation Question | Example |
| --- | --- | --- |
| Clear | Could two reviewers interpret this the same way? | "Show `Invalid email` below email field" |
| Concise | Is there only test-relevant detail? | "Returns results in <= 2s for 95% of queries" |
| Testable | Is pass/fail objective? | "Rate limit at 100 requests/min per API key" |
| Result-oriented | Does it describe outcome, not implementation? | "Session persists for 24h" |

## Anti-Patterns

NEVER include implementation details (frameworks, libraries, internal architecture).
NEVER use vague language (`fast`, `intuitive`, `good UX`, `efficient`).
NEVER define only happy-path behavior.
NEVER couple one criterion to another criterion for validation.
NEVER write AC without understanding the user outcome (`so that`).
NEVER specify pixel-level UI values unless they are true contractual constraints.
NEVER keep criteria that cannot be verified by a human tester or automated test.

## Common Mistakes

| Mistake | Bad Example | Better Example |
| --- | --- | --- |
| Vague metric | "System should be fast" | "Search returns in <= 2s for 95% of requests" |
| Implementation detail | "Use Redis for caching" | "Dashboard data remains available across refresh" |
| Untestable wording | "Provide good UX" | "Checkout can be completed in <= 3 steps" |
| Missing failures | Only happy path listed | Include invalid input, timeout, and permission denial |

## Scenario Coverage Checklist

For each story, include:

- [ ] Happy path
- [ ] Invalid input
- [ ] Missing required data
- [ ] Boundary conditions
- [ ] Permission or authorization failures
- [ ] Timeout or network failures
- [ ] Error messaging and recovery path

## Templates

### Given/When/Then (Sequential Flows)

```gherkin
Scenario: [Behavior name]
Given [starting context]
When [user/system action]
Then [observable outcome]
And [additional verifiable outcome]
```

### Rule-Oriented (Independent Constraints)

```markdown
Acceptance Criteria:
- [Area] [specific measurable requirement]
- [Area] [specific measurable requirement]
- [Errors] [message + condition + recovery]
- [Limits] [boundary + expected behavior]
```

## MoSCoW Integration

Use MoSCoW to control scope:

- Must Have: required for acceptance now
- Should Have: important but can ship later
- Could Have: optional enhancement
- Won't Have: explicitly out of this iteration

Only Must Have items become blocking acceptance criteria for current delivery.

## Output Format

Produce acceptance criteria in this structure:

```markdown
User Story:
As a [role], I want [action], so that [benefit].

Acceptance Criteria (Must Have):
1. ...
2. ...

Negative/Edge Scenarios:
1. ...
2. ...

Out of Scope (Won't Have):
- ...
```

## References

Load detailed examples only when needed:

- `references/gherkin-examples.md`
- `references/patterns-by-type.md`
