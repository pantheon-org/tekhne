# Scenario 04: Split a Fat Interface to Satisfy ISP

## User Prompt

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

1. Split interface files — one interface per client role. Name them after the role they serve (e.g. `IReadableStore.ts`, `IWritableStore.ts`, `IAdminStore.ts`).
2. `isp-analysis.md` — explain which client was forced to depend on methods it never uses, and how splitting resolves this.

Rules:
- Each interface must contain only methods actually used by its target client.
- Do NOT create one-method interfaces for methods that are always used together by the same client.
- `IReadableStore` must be referenced by `ReportingService` if you produce a refactored version.

## Expected Behavior

1. Produce a read-only interface containing `findById`, `findAll`, and `findByFilter` — no write or admin methods
2. Produce a writable interface containing at minimum `save` and `delete` (and may extend or include read methods)
3. Produce an admin interface containing `truncate`, `exportDump`, and `importDump` — no read or write methods
4. Avoid single-method interfaces for methods the client always uses together
5. `isp-analysis.md` names at least one client forced to depend on methods it never uses, and explains the resolution

## Success Criteria

- **Read-only interface produced with correct methods**: A read-only interface file exists containing `findById`, `findAll`, and `findByFilter` — no write or admin methods
- **Write interface produced with correct methods**: A writable interface file exists containing at minimum `save` and `delete` (and may extend or include read methods)
- **Admin interface produced with correct methods**: An admin interface file exists containing `truncate`, `exportDump`, and `importDump` — no read or write methods
- **No single-method interfaces created**: No produced interface contains only one method when the client that uses it always calls multiple methods from the same role group
- **isp-analysis.md identifies the ISP violation correctly**: `isp-analysis.md` names at least one client (e.g. `ReportingService` or `AdminMigrationTool`) that was forced to depend on methods it never uses
- **isp-analysis.md explains the resolution**: `isp-analysis.md` explains that splitting by client role means each client only depends on the interface relevant to its own usage

## Failure Conditions

- Read-only interface includes `save`, `delete`, `truncate`, `exportDump`, or `importDump`
- Write interface is missing or contains admin methods (`truncate`, `exportDump`, `importDump`)
- Admin interface is missing or contains read or write methods
- Single-method interfaces are created for methods clients always use together
- `isp-analysis.md` is missing or does not identify which client was forced to depend on unused methods
