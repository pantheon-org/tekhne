# Scenario 1: Apply Strategy Pattern to Eliminate an If/Else Chain

## User Prompt

The following `ShippingCalculator` uses an if/else chain to select between shipping strategies. Refactor it using the Strategy pattern.

## Expected Behavior

1. Produce `IShippingStrategy.ts` with a `calculate(weightKg, distanceKm)` interface method
2. Produce four strategy implementation files, each containing the corresponding arithmetic formula
3. Refactor `ShippingCalculator.ts` to accept an injected `IShippingStrategy` and delegate to it with no arithmetic
4. State in `pattern-analysis.md` that the if/else chain is the problem and that new methods can be added without editing `ShippingCalculator`
5. Acknowledge in `pattern-analysis.md` the added complexity (one interface plus four classes) as the cost

## Failure Conditions

- `IShippingStrategy.ts` is missing or does not export a `calculate` interface method
- Any of the four strategy files is missing or does not implement `IShippingStrategy`
- Any implementation uses a different arithmetic formula than the original code branch
- Refactored `ShippingCalculator.ts` still contains arithmetic or a conditional on `method`
- `pattern-analysis.md` is missing or does not describe the problem, win condition, and cost
