# Scenario 03: Apply the Humble Object Pattern

## User Prompt

The following controller class mixes HTTP-handling with business logic. Apply the Humble Object pattern to separate the testable logic from the hard-to-test infrastructure glue.

## Code to Refactor

```typescript
// src/controllers/CheckoutController.ts
import { Request, Response } from 'express'
import { db } from '../db'
import { stripe } from '../stripe'

export class CheckoutController {
  async handle(req: Request, res: Response): Promise<void> {
    const { userId, cartId } = req.body

    // Business logic mixed in
    const cart = await db.query('SELECT * FROM carts WHERE id = $1', [cartId])
    if (cart.rows.length === 0) {
      res.status(404).json({ error: 'Cart not found' })
      return
    }

    const items = cart.rows[0].items as Array<{ price: number; quantity: number }>
    const total = items.reduce((sum, item) => sum + item.price * item.quantity, 0)

    if (total <= 0) {
      res.status(400).json({ error: 'Cart is empty or invalid' })
      return
    }

    const charge = await stripe.charges.create({
      amount: Math.round(total * 100),
      currency: 'usd',
      customer: userId,
    })

    await db.query('UPDATE carts SET status = $1 WHERE id = $2', ['checked_out', cartId])
    res.json({ chargeId: charge.id, total })
  }
}
```

## Output Specification

Produce two files:

1. `CheckoutUseCase.ts` — the testable use case containing all business logic. It must not import `express`, the `db` singleton, or the `stripe` singleton. It must accept its dependencies via constructor injection.
2. `CheckoutController.ts` — the humble object that only maps the HTTP request to use case input, calls the use case, and maps the output to an HTTP response. It must contain no calculation logic.

Both files must be valid TypeScript.

## Expected Behavior

1. Produce `CheckoutUseCase.ts` exporting a class or function named `CheckoutUseCase`
2. `CheckoutUseCase.ts` must not import `express`, the `db` singleton, or the `stripe` singleton
3. `CheckoutUseCase` must receive its cart repository and payment gateway as constructor parameters
4. The total calculation (`reduce` over items) and the empty-cart validation must be in `CheckoutUseCase`, not the controller
5. `CheckoutController.ts` must contain no arithmetic or validation logic — only maps `req.body` to use case input and maps output to `res.json`
6. `CheckoutController.ts` instantiates or receives a `CheckoutUseCase` and calls its `execute` (or equivalent) method

## Success Criteria

- **CheckoutUseCase file produced**: A file named `CheckoutUseCase.ts` exists and exports a class or function named `CheckoutUseCase`
- **Use case has no express/db/stripe imports**: `CheckoutUseCase.ts` does not import `express`, the `db` singleton, or the `stripe` singleton at the top level
- **Use case accepts dependencies via constructor**: `CheckoutUseCase` receives its cart repository and payment gateway as constructor parameters
- **Business logic in use case**: The total calculation (`reduce` over items) and the empty-cart validation are present in `CheckoutUseCase`, not in the controller
- **Controller is thin**: `CheckoutController.ts` contains no arithmetic or validation logic — it only maps `req.body` to use case input and maps the output to `res.json`
- **Controller delegates to use case**: `CheckoutController.ts` instantiates or receives a `CheckoutUseCase` and calls its `execute` (or equivalent) method

## Failure Conditions

- `CheckoutUseCase.ts` is missing or does not export `CheckoutUseCase`
- `CheckoutUseCase.ts` imports `express`, `db`, or `stripe` directly
- `CheckoutUseCase` does not accept dependencies via constructor, keeping hard-coded external bindings
- The total calculation or empty-cart validation remains in the controller
- `CheckoutController.ts` still contains arithmetic (e.g. `reduce` over items) or validation logic
- `CheckoutController.ts` does not delegate to a `CheckoutUseCase`
