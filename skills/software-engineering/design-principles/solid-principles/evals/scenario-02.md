# Scenario 02: Apply OCP by Introducing an Abstraction

## User Prompt

The following `ReportExporter` class violates the Open/Closed Principle because adding a new export format requires editing the existing class.

## Code to Refactor

```typescript
// src/ReportExporter.ts
export class ReportExporter {
  export(data: Record<string, unknown>[], format: string): string {
    if (format === 'csv') {
      const header = Object.keys(data[0]).join(',')
      const rows = data.map(row => Object.values(row).join(','))
      return [header, ...rows].join('\n')
    } else if (format === 'json') {
      return JSON.stringify(data, null, 2)
    } else if (format === 'tsv') {
      const header = Object.keys(data[0]).join('\t')
      const rows = data.map(row => Object.values(row).join('\t'))
      return [header, ...rows].join('\n')
    } else {
      throw new Error(`Unsupported format: ${format}`)
    }
  }
}
```

## Output Specification

Refactor to satisfy OCP so that a new format (e.g. XML) can be added without modifying any existing file. Produce:

1. `IReportFormatter.ts` — the abstraction (interface or abstract class) that all format implementations must satisfy.
2. `CsvFormatter.ts`, `JsonFormatter.ts`, `TsvFormatter.ts` — one file per format, each implementing `IReportFormatter`.
3. `ReportExporter.ts` — the refactored exporter that accepts an `IReportFormatter` and delegates to it. It must contain no format-specific logic.
4. `ocp-analysis.md` — one paragraph explaining the OCP violation in the original code and how the refactor resolves it.

All TypeScript files must be valid.

## Expected Behavior

1. `IReportFormatter.ts` exists and exports an interface with at least a `format` or `export` method that takes data and returns a string
2. `CsvFormatter.ts`, `JsonFormatter.ts`, and `TsvFormatter.ts` each implement `IReportFormatter`
3. Refactored `ReportExporter.ts` has no `if/else` or `switch` on format string; it only delegates to the injected `IReportFormatter`
4. `ReportExporter.ts` receives an `IReportFormatter` instance as a constructor parameter or method argument
5. `ocp-analysis.md` identifies the if/else chain as the violation and explains that the abstraction allows adding formats without editing existing files

## Success Criteria

- **IReportFormatter interface produced**: `IReportFormatter.ts` exists and exports an interface with at least a `format` or `export` method that takes data and returns a string
- **Three format implementations produced**: `CsvFormatter.ts`, `JsonFormatter.ts`, and `TsvFormatter.ts` each exist and implement `IReportFormatter`
- **Refactored ReportExporter contains no format-specific logic**: `ReportExporter.ts` has no `if/else` or `switch` on format string; it only delegates to the injected `IReportFormatter`
- **ReportExporter accepts formatter via constructor or method parameter**: `ReportExporter.ts` receives an `IReportFormatter` instance as a constructor parameter or method argument
- **ocp-analysis.md correctly explains the violation and fix**: `ocp-analysis.md` identifies the `if/else` chain as the violation and explains that the abstraction allows adding formats without editing existing files

## Failure Conditions

- `IReportFormatter.ts` is missing or has no `format`/`export` method
- Any of the three formatter files is missing or does not implement `IReportFormatter`
- Refactored `ReportExporter.ts` still contains `if/else` or `switch` on a format string
- `ReportExporter.ts` does not accept an `IReportFormatter` via constructor or method parameter
- `ocp-analysis.md` is missing or does not identify the if/else chain as the OCP violation
