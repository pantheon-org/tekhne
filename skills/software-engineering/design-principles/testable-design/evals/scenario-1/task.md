# Task: Identify Testability Blockers

You are reviewing a TypeScript service class before writing unit tests. The class below is difficult to test. Identify all testability blockers and produce a structured analysis in `analysis.md`.

## Code Under Review

```typescript
// src/services/InvoiceService.ts
import { Pool } from 'pg'
import { Stripe } from 'stripe'
import { sendEmail } from '../utils/mailer'
import * as fs from 'fs'

export class InvoiceService {
  private db = new Pool({ connectionString: process.env.DATABASE_URL })
  private stripe = new Stripe(process.env.STRIPE_KEY!, { apiVersion: '2023-10-16' })

  async processInvoice(userId: string, amount: number): Promise<void> {
    const result = await this.db.query('SELECT * FROM users WHERE id = $1', [userId])
    const user = result.rows[0]

    const charge = await this.stripe.charges.create({
      amount,
      currency: 'usd',
      customer: user.stripeCustomerId,
    })

    await sendEmail(user.email, `Invoice paid: ${charge.id}`)

    const receipt = `Receipt for ${user.email}: ${charge.id}`
    fs.writeFileSync(`/tmp/receipts/${userId}.txt`, receipt)
  }
}
```

## Output Specification

Produce a file `analysis.md` with:

1. A list of each testability blocker using the exact format:
   ```
   Testability blocker: <description>
   Problem: <why this prevents unit testing>
   Refactor: <recommended fix>
   ```
2. A summary section stating how many blockers were found and the primary category of each (concrete instantiation, side effect, global state, etc.).

Do not refactor the code — analysis only.
