# Scenario 02: Design a Use Case with Input/Output Ports

## User Prompt

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

## Expected Behavior

1. Define a typed input interface (e.g. `RegisterUserInput`) with `email`, `password`, and `name` — no `req`/`res` objects
2. Define a typed output interface (e.g. `RegisterUserOutput`) with `userId` and `email` — no HTTP response formatting
3. Accept a user repository through an interface (`IUserRepository` or similar), not the concrete `UserRepository` class
4. Remove all imports from `express` and all `req`/`res` parameters from the use case
5. Identify in DESIGN.md: use case in application layer, repository interface in application layer, HTTP adapter in adapter layer

## Success Criteria

- **Input port interface defined**: `register-user.ts` defines a typed input interface (e.g. `RegisterUserInput`) with `email`, `password`, and `name` — no `req`/`res` objects
- **Output port interface defined**: `register-user.ts` defines a typed output interface (e.g. `RegisterUserOutput`) with `userId` and `email` — no HTTP response formatting
- **Use case depends on repository interface**: `register-user.ts` receives a user repository through an interface (`IUserRepository` or similar), not the concrete `UserRepository` class
- **No HTTP/Express imports in use case**: `register-user.ts` has no imports from `express` and no `req` or `res` parameters — HTTP concerns are removed
- **DESIGN.md explains layer placement**: DESIGN.md identifies at least: use case in application layer, repository interface in application layer, HTTP adapter in adapter layer

## Failure Conditions

- Input interface uses `req: any` or `req: Request` instead of typed domain fields
- Output interface includes HTTP status codes or Express response formatting
- Use case imports or instantiates `UserRepository` directly instead of depending on an interface
- Use case still imports or references `express`, `Request`, or `Response`
- DESIGN.md does not identify which layer each piece belongs to
