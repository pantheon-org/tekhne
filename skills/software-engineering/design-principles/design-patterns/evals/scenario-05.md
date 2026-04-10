# Scenario 05: Evaluate Pattern Fit and Recommend Against Premature Extraction

## User Prompt

A developer has asked you to apply the Strategy pattern to the following `TaxCalculator`. Review the code and determine whether the pattern is warranted.

## Code Under Review

```typescript
// src/TaxCalculator.ts
export class TaxCalculator {
  calculateVat(amount: number): number {
    return amount * 0.2
  }
}
```

The developer's rationale: "We might add other tax types in the future. I want to be prepared with a Strategy interface."

## Output Specification

Produce `pattern-evaluation.md` with:

1. **Problem Statement** — describe the concrete design problem (or lack thereof) in the current code, without using pattern names.
2. **Pattern Fit Assessment** — evaluate whether Strategy is the right choice *now* given the current code. Be explicit about the win condition that is or is not met.
3. **Recommendation** — state clearly whether to apply the pattern now or defer, and why. Reference the principle that patterns should not be applied preemptively.
4. **Trigger Condition** — describe the specific future event that *would* justify extracting the Strategy pattern (e.g. what would need to appear in the codebase).

Do NOT refactor the code.

## Expected Behavior

1. Correctly note in the Problem Statement that there is no current conditional logic, coupling problem, or varying algorithm — the code is a simple calculation
2. Explicitly state in the Pattern Fit Assessment that the Strategy win condition (eliminating if/else selection between algorithms) is not met because only one implementation exists
3. Recommend NOT applying the Strategy pattern now in the Recommendation section
4. Reference YAGNI or an equivalent statement about not applying patterns for imagined future needs
5. Describe a concrete code event in the Trigger Condition that would justify the pattern (e.g. "when a second tax type with a different formula must be added and selected conditionally")

## Success Criteria

- **Problem statement describes no concrete problem**: The Problem Statement section correctly notes there is no current conditional logic, coupling problem, or varying algorithm — the code is a simple calculation
- **Win condition correctly evaluated as unmet**: The Pattern Fit Assessment explicitly states the Strategy win condition (eliminating if/else selection between algorithms) is not met because only one implementation exists
- **Recommendation is to defer**: The Recommendation section recommends NOT applying the Strategy pattern now
- **Anti-preemptive principle referenced**: The recommendation references the principle of not applying patterns for imagined future needs (YAGNI, or equivalent statement about waiting for concrete need)
- **Trigger condition is concrete and correct**: The Trigger Condition describes a concrete code event (e.g. "when a second tax type with a different formula must be added and selected conditionally") that would justify the pattern

## Failure Conditions

- Problem Statement claims there is a design problem in the current single-method class
- Pattern Fit Assessment states the Strategy win condition is already met with a single implementation
- Recommendation is to apply the pattern now, despite no concrete need
- No reference to YAGNI or anti-preemptive pattern principle
- Trigger Condition is vague (e.g. "when we need it") rather than describing a specific code change
