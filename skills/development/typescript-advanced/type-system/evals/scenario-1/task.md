# Scenario: Extract Types with `infer` Instead of Duplicating Them

A codebase has several hand-written type aliases that duplicate what could be derived directly from existing functions and structures, and they have already drifted out of sync at least once.

```typescript
// derive.ts

async function fetchOrder(id: string) {
  return { id, total: 42.5, items: ["sku-1", "sku-2"] };
}

// Hand-written, already stale — fetchOrder actually returns a Promise
interface Order {
  id: string;
  total: number;
  items: string[];
}

function firstItem(items: string[]): string {
  return items[0];
}

// Hand-written duplicate of firstItem's parameter type
type ItemsParam = string[];
```

Rewrite `derive.ts` so that:

1. `Order` is derived from `fetchOrder`'s actual return type using `ReturnType` and `Awaited` (or an equivalent conditional type with `infer`), not hand-written.
2. `ItemsParam` is derived from `firstItem`'s actual parameter type using `Parameters`, not hand-written.
3. A new type `ItemElement<T>` is added that uses a conditional type with `infer` to extract the element type of an array type `T` (e.g. `ItemElement<string[]>` is `string`).
4. No behaviour changes — `fetchOrder` and `firstItem` keep their original implementations.

## Output Specification

Produce a single file `derive.ts` containing the original functions plus the corrected, derived type aliases and the new `ItemElement<T>` conditional type.
