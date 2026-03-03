# Interface Design: Payment Provider Integration

## Problem/Feature Description

Your e-commerce platform needs to integrate with multiple payment providers. Currently, the checkout service directly instantiates and calls specific provider SDKs, making it difficult to:

- Add new payment providers
- Switch providers during outages
- Test the checkout flow without hitting real APIs

The team has decided to create an abstraction layer for payment processing. Your task is to design the interfaces and contracts that will enable flexible payment provider integration while maintaining testability and clean separation between domain logic and infrastructure.

## Output Specification

Create an interface design document called `payment-interfaces.md` that:

1. **Defines the abstraction**:
   - Create interfaces/ports that define the contract for payment processing
   - Apply Dependency Inversion Principle - depend on abstractions, not concrete details
   - Apply Interface Segregation Principle - create focused interfaces for different client needs

2. **Defines ownership**:
   - Specify who owns the interface contracts
   - Define how new providers can be added

3. **Shows implementation guidance**:
   - Use BAD/GOOD format to show problematic direct coupling vs. proper abstraction
   - Show how a concrete provider implements the interface

4. **Documents tradeoffs**:
   - List alternatives considered (e.g., one big interface vs. focused interfaces)
   - Explain your design choices and associated risks

5. **Includes validation** step before proceeding to implementation

## Current Code Causing Issues

=============== FILE: checkout-service.ts (problematic) ===============
export class CheckoutService {
  async processPayment(orderId: string, amount: number, provider: string) {
    // Direct coupling to concrete implementations!
    if (provider === 'stripe') {
      const stripe = new StripeGateway('sk_live_xxx');
      return await stripe.charge(orderId, amount);
    } else if (provider === 'paypal') {
      const paypal = new PayPalClient('client_id', 'secret');
      return await paypal.executePayment(orderId, amount);
    } else if (provider === 'square') {
      const square = new SquareClient('access_token');
      return await square.charge(orderId, amount);
    }
    // This if-else chain will keep growing
    // Testing is impossible without real SDKs
    throw new Error('Unknown provider');
  }
}
