# Task: Identify and Refactor SRP Violations

The following `UserManager` class has multiple responsibilities. Identify each SRP violation and refactor into focused classes.

## Code to Refactor

```typescript
// src/UserManager.ts
import { db } from './db'
import { mailer } from './mailer'
import { logger } from './logger'
import * as bcrypt from 'bcrypt'
import * as jwt from 'jsonwebtoken'

export class UserManager {
  async register(email: string, password: string): Promise<string> {
    const hashed = await bcrypt.hash(password, 10)
    const result = await db.query(
      'INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id',
      [email, hashed]
    )
    const userId = result.rows[0].id
    await mailer.send(email, 'Welcome!', 'Thanks for signing up.')
    logger.info(`User registered: ${email}`)
    return userId
  }

  async login(email: string, password: string): Promise<string> {
    const result = await db.query('SELECT * FROM users WHERE email = $1', [email])
    const user = result.rows[0]
    if (!user) throw new Error('User not found')
    const valid = await bcrypt.compare(password, user.password_hash)
    if (!valid) throw new Error('Invalid password')
    logger.info(`User logged in: ${email}`)
    return jwt.sign({ userId: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' })
  }

  async updateProfile(userId: string, name: string, bio: string): Promise<void> {
    await db.query('UPDATE users SET name = $1, bio = $2 WHERE id = $3', [name, bio, userId])
    logger.info(`Profile updated: ${userId}`)
  }

  async deleteAccount(userId: string): Promise<void> {
    await db.query('DELETE FROM users WHERE id = $1', [userId])
    await mailer.send('admin@company.com', 'Account deleted', `User ${userId} deleted their account.`)
    logger.info(`Account deleted: ${userId}`)
  }
}
```

## Output Specification

Produce:

1. `violations.md` — list each SRP violation using the format:
   ```
   SRP violation: <class> handles <concern>.
   Refactor: Extract <NewClass>.
   Validation: <how to verify each new class has one reason to change>.
   ```

2. One TypeScript file per extracted class. Each file must contain only one concern. The original `UserManager.ts` may be retained but must be stripped down to a thin coordinator if needed, or removed if all concerns are fully extracted.
