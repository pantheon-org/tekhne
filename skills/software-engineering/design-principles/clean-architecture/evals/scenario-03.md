# Scenario 03: Separate Business Logic from a Fat Controller

## User Prompt

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

## Expected Behavior

1. The controller contains no business logic — it extracts `req.body` fields, delegates to the use case, then formats the HTTP response
2. The use case contains the inventory validation, price calculation, order creation, and email notification logic
3. The use case does not import from `express` and does not accept `Request` or `Response` parameters
4. The use case accepts typed input (`userId` and `items`) rather than the raw HTTP request object
5. The controller still returns 400 on insufficient stock and 201 with `orderId` and `total` on success

## Success Criteria

- **Controller is thin**: `order-controller.ts` contains no business logic — it extracts `req.body` fields and delegates to the use case, then formats the HTTP response
- **Business logic moved to use case**: `place-order.ts` contains the inventory validation, price calculation, order creation, and email notification logic
- **Use case has no HTTP imports**: `place-order.ts` does not import from `express` and does not accept `Request` or `Response` parameters
- **Use case accepts typed input**: `place-order.ts` function accepts a typed input (`userId` and `items`) rather than the raw HTTP request object
- **Same observable behavior preserved**: The controller still returns 400 on insufficient stock and 201 with `orderId` and `total` on success

## Failure Conditions

- Controller still contains business logic (inventory checks, price calculations, or email sending)
- Use case file is missing or contains no business logic
- Use case still imports `express` or accepts `Request`/`Response` parameters
- Use case accepts the raw `req` object instead of typed `userId` and `items`
- Observable HTTP behavior changes (different status codes or response shape)
