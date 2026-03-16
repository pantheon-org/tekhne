# Task: Apply Strategy Pattern to Eliminate an If/Else Chain

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
