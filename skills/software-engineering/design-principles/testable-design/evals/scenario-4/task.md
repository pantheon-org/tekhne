# Task: Design Test Doubles for a Repository Interface

Given the following repository interface and use case, produce the three standard test doubles: a Stub, a Mock, and a Fake.

## Interface and Use Case

```typescript
// src/ports/IProductRepository.ts
export interface Product {
  id: string
  name: string
  price: number
  stock: number
}

export interface IProductRepository {
  findById(id: string): Promise<Product | null>
  save(product: Product): Promise<void>
  findAll(): Promise<Product[]>
}

// src/use-cases/ReserveProductUseCase.ts
export class ReserveProductUseCase {
  constructor(private repo: IProductRepository) {}

  async execute(productId: string, quantity: number): Promise<void> {
    const product = await this.repo.findById(productId)
    if (!product) throw new Error('Product not found')
    if (product.stock < quantity) throw new Error('Insufficient stock')
    product.stock -= quantity
    await this.repo.save(product)
  }
}
```

## Output Specification

Produce a single file `test-doubles.ts` that contains and exports all three implementations:

1. **StubProductRepository** — returns fixed product data from `findById` (a product with id `"prod-1"`, name `"Widget"`, price `9.99`, stock `10`). `save` and `findAll` are no-ops.
2. **MockProductRepository** — records whether `save` was called and with what product. Exposes a `savedProduct` property (initially `null`). `findById` returns a product with stock `5`.
3. **InMemoryProductRepository** — a full in-memory implementation that stores products in a `Map`. All three methods must work correctly.

All three classes must implement `IProductRepository`.
