# Scenario 01: Apply Strategy Pattern to Eliminate an If/Else Chain

## User Prompt

The following `ShippingCalculator` uses an if/else chain to select between shipping strategies. Refactor it using the Strategy pattern.

## Code to Refactor

```typescript
// src/ShippingCalculator.ts
export class ShippingCalculator {
  calculate(method: string, weightKg: number, distanceKm: number): number {
    if (method === 'standard') {
      return weightKg * 0.5 + distanceKm * 0.1
    } else if (method === 'express') {
      return weightKg * 1.2 + distanceKm * 0.3 + 5.0
    } else if (method === 'overnight') {
      return weightKg * 2.0 + distanceKm * 0.5 + 15.0
    } else if (method === 'economy') {
      return weightKg * 0.3 + distanceKm * 0.05
    } else {
      throw new Error(`Unknown shipping method: ${method}`)
    }
  }
}
```

## Output Specification

Produce:

1. `IShippingStrategy.ts` — the strategy interface with a `calculate(weightKg: number, distanceKm: number): number` method.
2. One implementation file per strategy: `StandardShipping.ts`, `ExpressShipping.ts`, `OvernightShipping.ts`, `EconomyShipping.ts`.
3. `ShippingCalculator.ts` — refactored to accept an `IShippingStrategy` and delegate to it. Must contain no arithmetic.
4. `pattern-analysis.md` — describe the problem (two sentences), the win condition (one sentence), and the cost (one sentence).

All TypeScript files must be valid.

## Expected Behavior

1. Produce `IShippingStrategy.ts` with a `calculate(weightKg, distanceKm)` interface method
2. Produce four strategy implementation files, each containing the corresponding arithmetic formula
3. Refactor `ShippingCalculator.ts` to accept an injected `IShippingStrategy` and delegate to it with no arithmetic
4. State in `pattern-analysis.md` that the if/else chain is the problem and that new methods can be added without editing `ShippingCalculator`
5. Acknowledge in `pattern-analysis.md` the added complexity (one interface plus four classes) as the cost

## Success Criteria

- **IShippingStrategy interface produced**: `IShippingStrategy.ts` exists and exports an interface with a `calculate` method taking `weightKg` and `distanceKm` returning `number`
- **Four strategy implementations produced**: `StandardShipping.ts`, `ExpressShipping.ts`, `OvernightShipping.ts`, and `EconomyShipping.ts` each exist and implement `IShippingStrategy`
- **Calculation formulas preserved in implementations**: Each implementation contains the same arithmetic formula as the corresponding branch in the original code
- **Refactored ShippingCalculator contains no arithmetic**: `ShippingCalculator.ts` delegates to the injected strategy and contains no `+`, `*`, or numeric literals
- **pattern-analysis.md states problem and win condition**: `pattern-analysis.md` describes the if/else chain as the problem and states that new shipping methods can now be added without editing `ShippingCalculator`
- **pattern-analysis.md acknowledges cost**: `pattern-analysis.md` mentions the added complexity (one interface + four classes) as the cost of the pattern

## Failure Conditions

- `IShippingStrategy.ts` is missing or does not export a `calculate` interface method
- Any of the four strategy files is missing or does not implement `IShippingStrategy`
- Any implementation uses a different arithmetic formula than the original code branch
- Refactored `ShippingCalculator.ts` still contains arithmetic or a conditional on `method`
- `pattern-analysis.md` is missing or does not describe the problem, win condition, and cost
