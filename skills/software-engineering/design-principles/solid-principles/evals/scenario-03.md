# Scenario 03: Detect and Fix an LSP Violation

## User Prompt

The following code has an LSP violation. A subtype breaks the contract of its base type. Identify the violation, explain it, and produce a fix.

## Code Under Review

```typescript
// src/storage/FileStorage.ts
export abstract class FileStorage {
  /**
   * Saves data to storage.
   * Contract: always resolves; never throws.
   */
  abstract save(key: string, data: string): Promise<void>

  /**
   * Reads data from storage.
   * Contract: returns the stored string, or null if not found.
   */
  abstract read(key: string): Promise<string | null>
}

// src/storage/DiskStorage.ts
export class DiskStorage extends FileStorage {
  async save(key: string, data: string): Promise<void> {
    const fs = await import('fs/promises')
    await fs.writeFile(`/tmp/${key}`, data)
    // Fulfils contract: resolves or rejects on IO error
  }

  async read(key: string): Promise<string | null> {
    const fs = await import('fs/promises')
    try {
      return await fs.readFile(`/tmp/${key}`, 'utf-8')
    } catch {
      return null
    }
  }
}

// src/storage/ReadOnlyStorage.ts
export class ReadOnlyStorage extends FileStorage {
  private store: Map<string, string>

  constructor(seed: Record<string, string>) {
    super()
    this.store = new Map(Object.entries(seed))
  }

  async save(key: string, data: string): Promise<void> {
    throw new Error('ReadOnlyStorage does not support writes')
  }

  async read(key: string): Promise<string | null> {
    return this.store.get(key) ?? null
  }
}
```

## Output Specification

Produce:

1. `lsp-analysis.md` — identify the LSP violation: which subtype breaks which contract, how callers are affected, and whether clients must add `instanceof` checks to avoid the problem.
2. `IReadableStorage.ts` and `IWritableStorage.ts` — two separate interfaces that split the responsibilities so that `ReadOnlyStorage` only implements what it can genuinely support.
3. Optionally, update `ReadOnlyStorage.ts` and `DiskStorage.ts` to implement the correct interface(s).

## Expected Behavior

1. `lsp-analysis.md` names `ReadOnlyStorage.save` as the violating method and states it breaks the base contract (which promises no throws)
2. `lsp-analysis.md` explains that callers of `FileStorage.save` cannot substitute `ReadOnlyStorage` without catching unexpected exceptions or adding `instanceof` checks
3. `IReadableStorage.ts` exists and declares only a `read` method returning `Promise<string | null>`
4. `IWritableStorage.ts` exists and declares only a `save` method returning `Promise<void>`
5. The updated or described `ReadOnlyStorage` implements `IReadableStorage` (not `IWritableStorage`) so the save contract violation is eliminated

## Success Criteria

- **LSP violation correctly identified**: `lsp-analysis.md` names `ReadOnlyStorage.save` as the violating method and states it breaks the base contract (which promises no throws)
- **Impact on callers explained**: `lsp-analysis.md` explains that callers of `FileStorage.save` cannot substitute `ReadOnlyStorage` without catching unexpected exceptions or adding `instanceof` checks
- **IReadableStorage interface produced**: `IReadableStorage.ts` exists and declares only a `read` method returning `Promise<string | null>`
- **IWritableStorage interface produced**: `IWritableStorage.ts` exists and declares only a `save` method returning `Promise<void>`
- **ReadOnlyStorage implements only IReadableStorage**: The updated or described `ReadOnlyStorage` implements `IReadableStorage` (not `IWritableStorage`) so the save contract violation is eliminated

## Failure Conditions

- `lsp-analysis.md` does not identify `ReadOnlyStorage.save` as the specific LSP violation
- `lsp-analysis.md` does not explain the impact on callers (unexpected exception, need for `instanceof` checks)
- `IReadableStorage.ts` is missing or contains a `save` method
- `IWritableStorage.ts` is missing or contains a `read` method
- `ReadOnlyStorage` is shown implementing both `IReadableStorage` and `IWritableStorage`, perpetuating the violation
