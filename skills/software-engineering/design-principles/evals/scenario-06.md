# Scenario 06: Design Patterns Strategy

## User Prompt

"This discount calculation has complex conditional logic. Refactor it using an appropriate design pattern.

```typescript
class PricingService {
  calculateDiscount(customer: Customer, amount: number): number {
    if (customer.tier === 'gold') {
      return amount * 0.20
    } else if (customer.tier === 'silver') {
      return amount * 0.10
    } else if (customer.tier === 'bronze') {
      return amount * 0.05
    } else if (customer.isFirstPurchase) {
      return amount * 0.15
    } else {
      return 0
    }
  }
}
```"

## Expected Behavior

1. Identify conditional logic as a design problem
2. Suggest Strategy pattern
3. Define DiscountStrategy interface with calculate method
4. Propose implementations (GoldDiscount, SilverDiscount, BronzeDiscount, FirstPurchaseDiscount, NoDiscount)
5. Show how to eliminate if/else chain
6. Explain how to add new strategies without modifying existing code

## Success Criteria

- Identifies Strategy pattern as solution
- Defines strategy interface
- Proposes at least 3 concrete strategy implementations
- Shows elimination of if/else or switch statements
- Mentions Open-Closed Principle compliance

## Failure Conditions

- Replaces if/else with a switch statement and declares the problem solved
- Does not define a strategy interface; uses separate standalone functions instead
- Fewer than 3 concrete strategy implementations are shown
- Does not explain how to add new strategies without modifying existing code
- Omits any mention of the Open-Closed Principle or extensibility
