# Scenario 2: Apply Adapter Pattern to Integrate an Incompatible External API

## User Prompt

Your domain uses an `ILogger` interface but you must integrate a legacy logging library that has a different method signature. Apply the Adapter pattern.

## Expected Behavior

1. `LegacyLoggerAdapter` declares `implements ILogger` and has `info`, `warn`, and `error` methods
2. The `info` method calls `this.legacy.log('INFO', message, ...)` or equivalent
3. The `warn` method calls `this.legacy.log('WARN', message, ...)` or equivalent
4. The `error` method calls `this.legacy.logError(message, error, ...)` rather than `this.legacy.log` with ERROR level
5. `LegacyLoggerAdapter` accepts a `LegacyLogger` instance as a constructor parameter (not instantiated inside the adapter)
6. `adapter-analysis.md` identifies the interface mismatch and explains the adapter preserves domain interface semantics without changing behavior

## Failure Conditions

- `LegacyLoggerAdapter` does not declare `implements ILogger` or is missing any of the three methods
- `info` method does not call `log('INFO', ...)` on the legacy logger
- `warn` method does not call `log('WARN', ...)` on the legacy logger
- `error` method calls `log('ERROR', ...)` instead of `logError(...)`, losing the error object parameter
- `LegacyLogger` is instantiated inside the adapter instead of being injected via constructor
- `adapter-analysis.md` is missing or does not identify the interface incompatibility as the reason for the pattern
