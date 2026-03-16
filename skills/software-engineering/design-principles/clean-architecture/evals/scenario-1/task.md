# Identify and Fix Dependency Direction Violations

## Problem Description

Review the following TypeScript module structure and identify all dependency direction violations. The project claims to follow Clean Architecture.

**File: `src/domain/entities/order.ts`**
```typescript
import { Entity, Column, PrimaryGeneratedColumn } from 'typeorm';
import { OrderRepository } from '../../infrastructure/repositories/order-repository';

@Entity()
export class Order {
  @PrimaryGeneratedColumn()
  id: string;

  @Column()
  userId: string;

  @Column()
  total: number;

  async save(): Promise<void> {
    const repo = new OrderRepository();
    await repo.persist(this);
  }
}
```

**File: `src/application/use-cases/create-order.ts`**
```typescript
import { PostgresOrderRepository } from '../../infrastructure/repositories/postgres-order-repository';

export class CreateOrderUseCase {
  private repo = new PostgresOrderRepository();

  async execute(userId: string, items: string[]): Promise<string> {
    // ... business logic
    await this.repo.save(order);
    return order.id;
  }
}
```

Produce a `VIOLATIONS.md` that:
1. Lists each dependency direction violation found
2. Explains why each violates the dependency rule
3. Shows how to fix each one (what should import what instead)
