# Scenario 04: Design Test Doubles for a Repository Interface

## User Prompt

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

## Expected Behavior

1. All three classes (`StubProductRepository`, `MockProductRepository`, `InMemoryProductRepository`) each have `implements IProductRepository` in their class declaration
2. `StubProductRepository.findById` returns a hardcoded `Product` object regardless of the `id` argument
3. `MockProductRepository` has a `savedProduct` property (or equivalent) that is set to the product argument when `save` is called, and starts as `null`
4. `InMemoryProductRepository` stores products in a `Map<string, Product>` and all three methods operate against that map
5. `InMemoryProductRepository.findAll` returns `Array.from` of the map's values (or equivalent), not a hardcoded array
6. `findById` return types are `Promise<Product | null>`, `save` returns `Promise<void>`, and `findAll` returns `Promise<Product[]>` across all three

## Success Criteria

- **All three classes implement IProductRepository**: `StubProductRepository`, `MockProductRepository`, and `InMemoryProductRepository` each have `'implements IProductRepository'` in their class declaration
- **Stub returns fixed data**: `StubProductRepository.findById` returns a hardcoded `Product` object (not `null`, not dynamic) regardless of the `id` argument
- **Mock records save call**: `MockProductRepository` has a property (`savedProduct` or equivalent) that is set to the product argument when `save` is called, and starts as `null`
- **Fake uses a Map for storage**: `InMemoryProductRepository` stores products in a `Map<string, Product>` and all three methods (`findById`, `save`, `findAll`) operate against that map
- **Fake findAll returns all stored products**: `InMemoryProductRepository.findAll` returns `Array.from` of the map's values (or equivalent), not a hardcoded array
- **Types are correct**: `findById` return types are `Promise<Product | null>`, `save` returns `Promise<void>`, and `findAll` returns `Promise<Product[]>` across all three implementations

## Failure Conditions

- Any class does not declare `implements IProductRepository`
- `StubProductRepository.findById` returns `null` or a dynamic value instead of fixed hardcoded data
- `MockProductRepository` has no property to record the `save` call, or `savedProduct` does not start as `null`
- `InMemoryProductRepository` uses an array instead of a `Map`, breaking `findById` by key
- `InMemoryProductRepository.findAll` returns a hardcoded array instead of the map's current contents
- Any method has incorrect return types (e.g. `findById` returning `Promise<Product>` instead of `Promise<Product | null>`)
