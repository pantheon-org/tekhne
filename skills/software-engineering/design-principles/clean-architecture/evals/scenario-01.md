# Scenario 01: Identify and Fix Dependency Direction Violations

## User Prompt

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

## Expected Behavior

1. Identify that the `Order` entity importing TypeORM decorators violates the dependency rule (inner layer importing from outer layer/framework)
2. Identify that `CreateOrderUseCase` directly instantiates `PostgresOrderRepository`, bypassing interface abstraction
3. Explain that `Order` should be a plain TypeScript class with no TypeORM imports; the repository adapter should handle the ORM mapping
4. Explain that the use case should receive `IOrderRepository` via constructor injection, not instantiate `PostgresOrderRepository` directly
5. State the general principle that dependencies point inward and inner layers must not import outer layers

## Success Criteria

- **TypeORM decorator violation identified**: VIOLATIONS.md identifies that the `Order` entity importing TypeORM decorators violates the dependency rule (inner layer importing from framework/outer layer)
- **Concrete repository instantiation violation identified**: VIOLATIONS.md identifies that `CreateOrderUseCase` directly instantiates `PostgresOrderRepository`, bypassing interface abstraction
- **Entity fix: remove decorators and persist logic**: VIOLATIONS.md explains that `Order` should be a plain TypeScript class with no TypeORM imports; the repository adapter handles mapping
- **Use case fix: depend on interface not implementation**: VIOLATIONS.md explains the use case should receive `IOrderRepository` via constructor injection, not instantiate `PostgresOrderRepository` directly
- **Inward dependency rule stated**: VIOLATIONS.md states the general principle (dependencies point inward; inner layers must not import outer layers)

## Failure Conditions

- VIOLATIONS.md misses the TypeORM decorator violation in the entity
- VIOLATIONS.md misses the concrete repository instantiation in the use case
- VIOLATIONS.md does not explain how to fix the entity (remove ORM decorators and persist logic)
- VIOLATIONS.md does not explain how to fix the use case (inject interface instead of instantiating concrete class)
- VIOLATIONS.md provides no statement of the general dependency inversion principle
