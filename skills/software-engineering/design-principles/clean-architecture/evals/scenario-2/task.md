# Design a Use Case with Input/Output Ports

## Problem Description

A team is building a user registration feature. Currently their logic lives in a fat service class:

```typescript
import { UserRepository } from '../database/user-repository';
import { EmailService } from '../email/email-service';
import { bcrypt } from 'bcrypt';
import { Response } from 'express';

export class UserService {
  async register(req: any, res: Response): Promise<void> {
    const { email, password, name } = req.body;

    const existing = await UserRepository.findByEmail(email);
    if (existing) {
      res.status(409).json({ error: 'Email already taken' });
      return;
    }

    const hashedPassword = await bcrypt.hash(password, 10);
    const user = await UserRepository.create({ email, hashedPassword, name });

    await EmailService.sendWelcomeEmail(user.email);

    res.status(201).json({ userId: user.id, email: user.email });
  }
}
```

Refactor this into a proper Clean Architecture use case. Produce:
1. `use-cases/register-user.ts` — the use case with explicit input/output port interfaces, depending on abstractions
2. `DESIGN.md` — briefly explaining the layer placement of each piece and what was moved where
