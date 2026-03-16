# Separate Business Logic from a Fat Controller

## Problem Description

A team has all logic in an Express controller:

```typescript
import { Request, Response } from 'express';
import { db } from '../db';
import { sendEmail } from '../email';

export const placeOrder = async (req: Request, res: Response) => {
  const { userId, items } = req.body;

  // Validate inventory
  for (const item of items) {
    const stock = await db.query('SELECT quantity FROM inventory WHERE id = $1', [item.id]);
    if (stock.rows[0].quantity < item.quantity) {
      return res.status(400).json({ error: `Insufficient stock for item ${item.id}` });
    }
  }

  // Calculate total
  let total = 0;
  for (const item of items) {
    const product = await db.query('SELECT price FROM products WHERE id = $1', [item.id]);
    total += product.rows[0].price * item.quantity;
  }

  // Create order
  const order = await db.query(
    'INSERT INTO orders (user_id, total, status) VALUES ($1, $2, $3) RETURNING id',
    [userId, total, 'pending']
  );

  // Send confirmation
  const user = await db.query('SELECT email FROM users WHERE id = $1', [userId]);
  await sendEmail(user.rows[0].email, `Order ${order.rows[0].id} placed for $${total}`);

  res.status(201).json({ orderId: order.rows[0].id, total });
};
```

Produce:
1. `controllers/order-controller.ts` — a thin controller that delegates to the use case
2. `use-cases/place-order.ts` — the use case with no HTTP coupling

The refactored code should not change the observable HTTP behavior.
