# Task: Apply OCP by Introducing an Abstraction

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

Refactor to satisfy OCP so that a new format (e.g., XML) can be added without modifying any existing file. Produce:

1. `IReportFormatter.ts` — the abstraction (interface or abstract class) that all format implementations must satisfy.
2. `CsvFormatter.ts`, `JsonFormatter.ts`, `TsvFormatter.ts` — one file per format, each implementing `IReportFormatter`.
3. `ReportExporter.ts` — the refactored exporter that accepts an `IReportFormatter` and delegates to it. It must contain no format-specific logic.
4. `ocp-analysis.md` — one paragraph explaining the OCP violation in the original code and how the refactor resolves it.

All TypeScript files must be valid.
