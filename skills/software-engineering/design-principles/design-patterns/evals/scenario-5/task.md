# Task: Evaluate Pattern Fit and Recommend Against Premature Extraction

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
4. **Trigger Condition** — describe the specific future event that *would* justify extracting the Strategy pattern (e.g., what would need to appear in the codebase).

Do NOT refactor the code.
