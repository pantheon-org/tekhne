# Task: Apply Adapter Pattern to Integrate an Incompatible External API

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
