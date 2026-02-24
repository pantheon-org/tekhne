---
name: acceptance-criteria
description: Write clear, testable acceptance criteria for user stories and feature delivery; use when defining done conditions, creating measurable requirements, applying INVEST checks, documenting negative scenarios, and aligning product, engineering, and QA on expected outcomes.
type: skill
category: Development
version: 2.1.0
tags:
  - acceptance-criteria
  - user-stories
  - requirements
  - testing
  - agile
last_updated: 2026-02-23
---

# Acceptance Criteria

Navigation hub for writing measurable, test-ready acceptance criteria.

## Use When

- "Write acceptance criteria for this feature."
- "Define done conditions for this story."
- "Create testable requirements before implementation."
- "Apply INVEST quality checks to this story."

## When Not to Use

- Architecture decisions and system design tradeoffs.
- Deep technical specifications about implementation internals.

## Workflow

1. Capture user outcome and business intent.
2. Select criteria format (Given/When/Then or rule-oriented).
3. Draft must-pass criteria with measurable outcomes.
4. Add negative and boundary scenarios.
5. Validate with checklist before sign-off.

## Quick Commands

```bash
# Find vague language
grep -nE "\b(should be|user-friendly|fast|easy|good)\b" <file>.md
```

```bash
# Count checklist criteria lines
grep -nE "^- \[ \]" <file>.md | wc -l
```

```bash
# Detect missing negative/error words
grep -niE "invalid|error|timeout|denied|failed" <file>.md
```

## Expected Output

```markdown
User Story:
As a [role], I want [action], so that [benefit].

Acceptance Criteria (Must Have):
- [ ] ...
- [ ] ...

Negative/Edge Scenarios:
- [ ] ...

Out of Scope:
- ...
```

## Anti-Patterns

### NEVER use vague, non-measurable criteria

WHY: vague language cannot be tested objectively.
BAD: "The system should be fast." GOOD: "Search returns in <= 2s for 95% of requests."

### NEVER write implementation instructions as acceptance criteria

WHY: criteria define outcomes, not implementation details.
BAD: "Use JavaScript validation on submit button click." GOOD: "Invalid form input shows error message within 1 second."

### NEVER skip negative and boundary scenarios

WHY: real users hit error and edge conditions.
BAD: only valid-login scenario listed. GOOD: include invalid credentials, timeout, and expired session behavior.

### NEVER couple multiple behaviors into one ambiguous criterion

WHY: broad criteria hide failure source and reduce test clarity.
BAD: "Checkout and email and invoice should work." GOOD: split into separate measurable criteria.

### NEVER bypass audience alignment on definition of done

WHY: misalignment causes rework at QA/review time.
BAD: criteria written without stakeholder validation. GOOD: confirm criteria with product and QA before implementation.

## Verification Checklist

- [ ] Every criterion is measurable (pass/fail).
- [ ] Happy path and failure paths are both covered.
- [ ] Wording describes outcomes, not implementation.
- [ ] Scope boundaries are explicit.
- [ ] Criteria are understandable by product + QA.

## Quick Reference

| Topic | Reference |
| --- | --- |
| INVEST criteria | [references/invest-criteria.md](references/invest-criteria.md) |
| Gherkin examples | [references/gherkin-examples.md](references/gherkin-examples.md) |
| Pattern selection | [references/patterns-by-type.md](references/patterns-by-type.md) |
| Scenario examples | [references/examples.md](references/examples.md) |
| Reusable templates | [references/templates.md](references/templates.md) |

## References

- [Gherkin Reference](https://cucumber.io/docs/gherkin/)
