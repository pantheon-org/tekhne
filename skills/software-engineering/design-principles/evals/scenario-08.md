# Scenario 08: Solid Ocp Violation

## User Prompt

"This payment service requires modification every time a new payment processor is added. Refactor it to follow the Open-Closed Principle.

```typescript
class PaymentService {
  async processPayment(type: string, amount: number): Promise<Receipt> {
    if (type === 'stripe') {
      return await stripe.charge(amount)
    } else if (type === 'paypal') {
      return await paypal.createPayment(amount)
    } else if (type === 'crypto') {
      return await crypto.transfer(amount)
    } else {
      throw new Error('Unknown payment type')
    }
  }
}
```"

## Expected Behavior

1. Identify OCP violation (modifying existing code to add features)
2. Suggest extracting PaymentProcessor interface
3. Propose implementations (StripeProcessor, PayPalProcessor, CryptoProcessor)
4. Recommend strategy pattern or factory for processor selection

## Success Criteria

- Identifies OCP violation or mentions "open for extension, closed for modification"
- Proposes interface or abstraction for payment processors
- Suggests polymorphism to eliminate if/else chain
- Shows how to add new processors without editing PaymentService

## Failure Conditions

- Adds a new processor by adding another else-if branch to the existing method
- Does not define a PaymentProcessor interface or equivalent abstraction
- Uses a string-keyed map of function calls without explaining the pattern
- Does not demonstrate how the refactored design handles a new processor type
- Conflates OCP with SRP or DIP without addressing the modification problem
