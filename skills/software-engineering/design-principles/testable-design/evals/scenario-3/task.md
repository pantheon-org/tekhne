# Task: Apply the Humble Object Pattern

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

1. `CheckoutUseCase.ts` — the testable use case containing all business logic. It must not import express, the db singleton, or the stripe singleton. It must accept its dependencies via constructor injection.
2. `CheckoutController.ts` — the humble object that only maps the HTTP request to use case input, calls the use case, and maps the output to an HTTP response. It must contain no calculation logic.

Both files must be valid TypeScript.
