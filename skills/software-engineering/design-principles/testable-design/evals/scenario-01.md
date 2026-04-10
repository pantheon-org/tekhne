# Scenario 01: Identify Testability Blockers

## User Prompt

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

## Expected Behavior

1. Identify `new Pool(...)` or the PostgreSQL connection being instantiated inside the class as a blocker
2. Identify `new Stripe(...)` being instantiated inside the class as a blocker
3. Identify the direct call to `sendEmail` as a hard-coded side effect blocker
4. Identify `fs.writeFileSync` as a hard-coded filesystem side effect blocker
5. Use the three-line format (`Testability blocker: / Problem: / Refactor:`) for each entry
6. Recommend injecting an interface or abstract type (not just "use a mock") in at least two Refactor lines

## Success Criteria

- **Concrete DB instantiation identified**: `analysis.md` includes a blocker entry for `new Pool(...)` or the PostgreSQL connection being instantiated inside the class
- **Concrete Stripe instantiation identified**: `analysis.md` includes a blocker entry for `new Stripe(...)` being instantiated inside the class
- **Side effect: email sending identified**: `analysis.md` includes a blocker entry for the direct call to `sendEmail` as a hard-coded side effect
- **Side effect: filesystem write identified**: `analysis.md` includes a blocker entry for `fs.writeFileSync` as a hard-coded side effect
- **Correct structured format used**: Each blocker uses the three-line format: `Testability blocker: / Problem: / Refactor:`
- **Refactor recommendation names interface injection**: At least two of the Refactor lines recommend injecting an interface or abstract type rather than just "use a mock"

## Failure Conditions

- `analysis.md` misses the `new Pool(...)` blocker
- `analysis.md` misses the `new Stripe(...)` blocker
- `analysis.md` misses the `sendEmail` side effect blocker
- `analysis.md` misses the `fs.writeFileSync` side effect blocker
- Entries do not use the required three-line format
- No Refactor line recommends interface injection as the solution
