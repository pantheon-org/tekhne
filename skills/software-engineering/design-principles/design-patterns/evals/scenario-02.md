# Scenario 02: Apply Adapter Pattern to Integrate an Incompatible External API

## User Prompt

Your domain uses an `ILogger` interface but you must integrate a legacy logging library that has a different method signature. Apply the Adapter pattern.

## Domain Interface

```typescript
// src/ports/ILogger.ts
export interface ILogger {
  info(message: string, context?: Record<string, unknown>): void
  warn(message: string, context?: Record<string, unknown>): void
  error(message: string, error?: Error, context?: Record<string, unknown>): void
}
```

## Legacy Library (cannot be modified)

```typescript
// external/legacy-logger/index.ts
export class LegacyLogger {
  log(level: 'INFO' | 'WARN' | 'ERROR', msg: string, meta?: object): void {
    console.log(`[${level}] ${msg}`, meta ?? {})
  }

  logError(msg: string, err: Error, meta?: object): void {
    console.error(`[ERROR] ${msg}`, err, meta ?? {})
  }
}
```

## Output Specification

Produce:

1. `LegacyLoggerAdapter.ts` — an adapter class that implements `ILogger` and delegates to `LegacyLogger`. The adapter must translate each `ILogger` method to the correct `LegacyLogger` call.
2. `adapter-analysis.md` — one paragraph identifying the incompatibility between the two interfaces and explaining why an adapter is the right choice here (not a facade or a wrapper that changes behavior).

## Expected Behavior

1. `LegacyLoggerAdapter` declares `implements ILogger` and has `info`, `warn`, and `error` methods
2. The `info` method calls `this.legacy.log('INFO', message, ...)` or equivalent
3. The `warn` method calls `this.legacy.log('WARN', message, ...)` or equivalent
4. The `error` method calls `this.legacy.logError(message, error, ...)` rather than `this.legacy.log` with ERROR level
5. `LegacyLoggerAdapter` accepts a `LegacyLogger` instance as a constructor parameter (not instantiated inside the adapter)
6. `adapter-analysis.md` identifies the interface mismatch and explains the adapter preserves domain interface semantics without changing behavior

## Success Criteria

- **LegacyLoggerAdapter implements ILogger**: `LegacyLoggerAdapter.ts` declares `'implements ILogger'` and has `info`, `warn`, and `error` methods
- **info maps to LegacyLogger.log with INFO level**: The `info` method calls `this.legacy.log('INFO', message, ...)` or equivalent
- **warn maps to LegacyLogger.log with WARN level**: The `warn` method calls `this.legacy.log('WARN', message, ...)` or equivalent
- **error maps to LegacyLogger.logError**: The `error` method calls `this.legacy.logError(message, error, ...)` rather than `this.legacy.log` with ERROR level
- **LegacyLogger accepted via constructor**: `LegacyLoggerAdapter` accepts a `LegacyLogger` instance as a constructor parameter (not instantiated inside the adapter)
- **adapter-analysis.md correctly explains the pattern choice**: `adapter-analysis.md` identifies the interface mismatch as the problem and explains the adapter preserves domain interface semantics without changing behavior

## Failure Conditions

- `LegacyLoggerAdapter` does not declare `implements ILogger` or is missing any of the three methods
- `info` method does not call `log('INFO', ...)` on the legacy logger
- `warn` method does not call `log('WARN', ...)` on the legacy logger
- `error` method calls `log('ERROR', ...)` instead of `logError(...)`, losing the error object parameter
- `LegacyLogger` is instantiated inside the adapter instead of being injected via constructor
- `adapter-analysis.md` is missing or does not identify the interface incompatibility as the reason for the pattern
