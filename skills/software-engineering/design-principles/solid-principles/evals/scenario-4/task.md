# Task: Split a Fat Interface to Satisfy ISP

The following interface forces multiple client types to depend on methods they never use. Split it to satisfy the Interface Segregation Principle.

## Interface Under Review

```typescript
// src/IDataStore.ts
export interface IDataStore {
  // Used by read-only reporting services
  findById(id: string): Promise<Record<string, unknown> | null>
  findAll(): Promise<Record<string, unknown>[]>
  findByFilter(filter: Record<string, unknown>): Promise<Record<string, unknown>[]>

  // Used only by write-capable services
  save(id: string, data: Record<string, unknown>): Promise<void>
  delete(id: string): Promise<void>

  // Used only by admin tools
  truncate(): Promise<void>
  exportDump(): Promise<string>
  importDump(data: string): Promise<void>
}
```

## Client Context

- `ReportingService` calls only `findById`, `findAll`, and `findByFilter`.
- `CrudService` calls `findById`, `findAll`, `save`, and `delete`.
- `AdminMigrationTool` calls `truncate`, `exportDump`, and `importDump`.

## Output Specification

Produce:

1. Split interface files — one interface per client role. Name them after the role they serve (e.g., `IReadableStore.ts`, `IWritableStore.ts`, `IAdminStore.ts`).
2. `isp-analysis.md` — explain which client was forced to depend on methods it never uses, and how splitting resolves this.

Rules:
- Each interface must contain only methods actually used by its target client.
- Do NOT create one-method interfaces for methods that are always used together by the same client.
- `IReadableStore` must be referenced by `ReportingService` if you produce a refactored version.
